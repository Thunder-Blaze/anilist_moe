//! Test utilities and harness for handling AniList API rate limits.
//!
//! This module provides a test runner that automatically handles rate limits
//! by pausing ALL threads for a minute when any thread encounters a 429 error.
//! It also retries on transient network errors.

use anilist_moe::{AniListClient, AniListError};
use std::future::Future;
use std::sync::atomic::{AtomicBool, AtomicU32, AtomicU64, Ordering};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
use tokio::sync::Mutex;
use tokio::time::sleep;

/// Rate limit configuration for tests
const RATE_LIMIT_PAUSE_SECS: u64 = 61; // Pause for 61 seconds on rate limit
const MIN_DELAY_BETWEEN_REQUESTS_MS: u64 = 700; // Minimum delay between API calls
const RATE_LIMIT_WINDOW_REQUESTS: u32 = 80; // Be conservative, AniList allows 90/min
const MAX_NETWORK_RETRIES: u32 = 3; // Max retries for network errors
const NETWORK_RETRY_DELAY_MS: u64 = 2000; // Delay before retrying network errors

/// Global rate limit coordinator - shared across ALL tests
static GLOBAL_RATE_LIMITER: GlobalRateLimiter = GlobalRateLimiter::new();

/// Global rate limiter that coordinates all test threads
pub struct GlobalRateLimiter {
    /// Whether we're currently in a rate limit pause
    is_paused: AtomicBool,
    /// Unix timestamp (seconds) when the pause ends
    pause_until: AtomicU64,
    /// Total request count in the current window
    request_count: AtomicU32,
    /// Unix timestamp when the current window started
    window_start: AtomicU64,
}

impl GlobalRateLimiter {
    const fn new() -> Self {
        Self {
            is_paused: AtomicBool::new(false),
            pause_until: AtomicU64::new(0),
            request_count: AtomicU32::new(0),
            window_start: AtomicU64::new(0),
        }
    }

    /// Get current unix timestamp in seconds
    fn now_secs() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
    }

    /// Trigger a global pause for all threads
    pub fn trigger_pause(&self) {
        let pause_end = Self::now_secs() + RATE_LIMIT_PAUSE_SECS;
        self.pause_until.store(pause_end, Ordering::SeqCst);
        self.is_paused.store(true, Ordering::SeqCst);
        eprintln!(
            "\n🚨 RATE LIMIT HIT! All threads pausing for {} seconds...\n",
            RATE_LIMIT_PAUSE_SECS
        );
    }

    /// Check if we need to pause, and if so, wait until pause is over
    pub async fn wait_if_paused(&self) {
        loop {
            let pause_until = self.pause_until.load(Ordering::SeqCst);
            let now = Self::now_secs();

            if pause_until > now {
                // We need to wait
                let wait_secs = pause_until - now;
                eprintln!("⏳ Waiting {} seconds for rate limit reset...", wait_secs);
                sleep(Duration::from_secs(wait_secs + 1)).await;
            } else {
                // Pause is over, reset the flag
                self.is_paused.store(false, Ordering::SeqCst);
                break;
            }
        }
    }

    /// Track a request and enforce delays
    pub async fn pre_request(&self) {
        // First, check if we need to wait for a global pause
        self.wait_if_paused().await;

        let now = Self::now_secs();
        let window_start = self.window_start.load(Ordering::SeqCst);

        // Check if we need to start a new window (60 seconds elapsed)
        if now - window_start >= 60 {
            self.window_start.store(now, Ordering::SeqCst);
            self.request_count.store(0, Ordering::SeqCst);
        }

        // Check if we're approaching the rate limit
        let count = self.request_count.fetch_add(1, Ordering::SeqCst);
        if count >= RATE_LIMIT_WINDOW_REQUESTS {
            let window_start = self.window_start.load(Ordering::SeqCst);
            let elapsed = now - window_start;

            if elapsed < 60 {
                let wait_secs = 60 - elapsed;
                eprintln!(
                    "⚠️  Approaching rate limit ({}/{} requests). Waiting {} seconds...",
                    count, RATE_LIMIT_WINDOW_REQUESTS, wait_secs
                );
                sleep(Duration::from_secs(wait_secs + 1)).await;
                // Reset the window
                self.window_start.store(Self::now_secs(), Ordering::SeqCst);
                self.request_count.store(0, Ordering::SeqCst);
            }
        }

        // Add small delay between requests
        sleep(Duration::from_millis(MIN_DELAY_BETWEEN_REQUESTS_MS)).await;
    }

    /// Handle a rate limit error - triggers global pause
    pub async fn handle_rate_limit(&self) {
        self.trigger_pause();
        self.wait_if_paused().await;
        // Reset counters after pause
        self.window_start.store(Self::now_secs(), Ordering::SeqCst);
        self.request_count.store(0, Ordering::SeqCst);
    }
}

/// Test harness that manages rate limiting across tests
pub struct TestHarness {
    client: AniListClient,
    last_request: Mutex<Instant>,
}

impl TestHarness {
    /// Creates a new test harness with an unauthenticated client
    pub fn new() -> Self {
        Self {
            client: AniListClient::new(),
            last_request: Mutex::new(Instant::now()),
        }
    }

    /// Creates a new test harness with an authenticated client
    pub fn with_token(token: &str) -> Self {
        Self {
            client: AniListClient::with_token(token),
            last_request: Mutex::new(Instant::now()),
        }
    }

    /// Gets a reference to the underlying client
    pub fn client(&self) -> &AniListClient {
        &self.client
    }

    /// Runs a test with global rate limit handling and network error retry
    ///
    /// This method:
    /// 1. Waits if there's a global pause in effect
    /// 2. Enforces a minimum delay between requests
    /// 3. Tracks request counts within a window
    /// 4. Retries on transient network errors (up to MAX_NETWORK_RETRIES times)
    /// 5. Triggers a GLOBAL pause (all threads) when rate limited
    /// 6. Retries the operation after the pause
    pub async fn run<F, Fut, T, E>(&self, operation: F) -> Result<T, E>
    where
        F: Fn() -> Fut,
        Fut: Future<Output = Result<T, E>>,
        E: std::fmt::Debug + IsRetryableError,
    {
        let mut network_retries = 0u32;

        loop {
            // Wait for any global pause and enforce pre-request delays
            GLOBAL_RATE_LIMITER.pre_request().await;

            // Update our local last request time
            {
                let mut last = self.last_request.lock().await;
                *last = Instant::now();
            }

            // Run the operation
            match operation().await {
                Ok(result) => return Ok(result),
                Err(e) if e.is_rate_limit_error() => {
                    // Trigger global pause for ALL threads
                    GLOBAL_RATE_LIMITER.handle_rate_limit().await;
                    // Loop will retry after pause
                }
                Err(e) if e.is_network_error() && network_retries < MAX_NETWORK_RETRIES => {
                    network_retries += 1;
                    eprintln!(
                        "🔄 Network error, retrying ({}/{})...: {:?}",
                        network_retries, MAX_NETWORK_RETRIES, e
                    );
                    sleep(Duration::from_millis(NETWORK_RETRY_DELAY_MS)).await;
                    // Loop will retry
                }
                Err(e) => return Err(e),
            }
        }
    }
}

impl Default for TestHarness {
    fn default() -> Self {
        Self::new()
    }
}

/// Trait to check if an error is retryable
pub trait IsRetryableError {
    fn is_rate_limit_error(&self) -> bool;
    fn is_network_error(&self) -> bool;
}

impl IsRetryableError for AniListError {
    fn is_rate_limit_error(&self) -> bool {
        matches!(
            self,
            AniListError::RateLimit { .. }
                | AniListError::RateLimitSimple
                | AniListError::BurstLimit
        )
    }

    fn is_network_error(&self) -> bool {
        matches!(self, AniListError::Network(_))
    }
}

/// Gets an authenticated test harness
pub fn get_authenticated_harness() -> Option<TestHarness> {
    use dotenv::dotenv;
    use std::env;

    dotenv().ok();
    env::var("ANILIST_TOKEN")
        .ok()
        .map(|token| TestHarness::with_token(&token))
}

/// Pauses for the rate limit window reset period (triggers global pause)
pub async fn pause_for_rate_limit() {
    GLOBAL_RATE_LIMITER.handle_rate_limit().await;
}

/// A simple delay between tests to avoid rate limiting
pub async fn delay_between_tests() {
    GLOBAL_RATE_LIMITER.pre_request().await;
}

/// Check if we're currently paused and wait if so
pub async fn wait_if_rate_limited() {
    GLOBAL_RATE_LIMITER.wait_if_paused().await;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_harness_creation() {
        let harness = TestHarness::new();
        assert!(!harness.client().has_token());
    }

    #[test]
    fn test_is_rate_limit_error() {
        assert!(AniListError::RateLimitSimple.is_rate_limit_error());
        assert!(AniListError::BurstLimit.is_rate_limit_error());
        assert!(
            AniListError::RateLimit {
                limit: 90,
                remaining: 0,
                reset_at: 0,
                retry_after: 60,
            }
            .is_rate_limit_error()
        );
        assert!(!AniListError::NotFound.is_rate_limit_error());
        assert!(!AniListError::AuthenticationRequired.is_rate_limit_error());
    }

    #[test]
    fn test_is_network_error() {
        // Network errors should be detected
        assert!(!AniListError::NotFound.is_network_error());
        assert!(!AniListError::RateLimitSimple.is_network_error());
    }

    #[test]
    fn test_global_rate_limiter_now() {
        let now = GlobalRateLimiter::now_secs();
        assert!(now > 0);
    }
}
