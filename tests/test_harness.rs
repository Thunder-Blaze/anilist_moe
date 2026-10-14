//! Test utilities for AniList rate limits.
//!
//! Coordinates pauses across threads on 429 responses and retries
//! transient network failures.

// This module is shared by several test binaries and each uses a different part
// of it, so the unused helpers are expected rather than dead code.
#![allow(dead_code)]

use anilist_moe::{AniListClient, AniListError};
use std::future::Future;
use std::sync::atomic::{AtomicBool, AtomicU32, AtomicU64, Ordering};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
use tokio::sync::Mutex;
use tokio::time::sleep;

/// Runtime configuration for the harness.
///
/// Read once from the environment, so a test run can be tuned without a rebuild.
/// AniList allows 90 requests per minute per IP; the defaults stay a little under
/// that so a full run does not trip the limit.
///
/// | Variable | Default | Meaning |
/// |---|---|---|
/// | `ANILIST_TEST_WINDOW_REQUESTS` | 80 | requests the harness lets through per window |
/// | `ANILIST_TEST_WINDOW_SECS` | 60 | length of that window |
/// | `ANILIST_TEST_PAUSE_SECS` | 61 | pause after a 429 that carries no `Retry-After` |
/// | `ANILIST_TEST_MIN_DELAY_MS` | 700 | minimum delay between two requests |
/// | `ANILIST_TEST_MAX_RETRIES` | 3 | transient network failures retried per request |
/// | `ANILIST_TEST_RETRY_DELAY_MS` | 2000 | delay between those retries |
/// | `ANILIST_TEST_MAX_RATE_LIMIT_RETRIES` | 5 | maximum rate limit pauses before failing |
#[derive(Debug, Clone, Copy)]
pub struct TestConfig {
    /// Requests allowed inside `window_secs` before the harness waits.
    pub window_requests: u32,
    /// Length of the rate limit window, in seconds.
    pub window_secs: u64,
    /// Pause applied after a 429 that carries no `Retry-After` header.
    pub pause_secs: u64,
    /// Minimum delay between two requests, in milliseconds.
    pub min_delay_ms: u64,
    /// Transient network failures retried per request.
    pub max_network_retries: u32,
    /// Delay between those retries, in milliseconds.
    pub retry_delay_ms: u64,
    /// Maximum rate limit pauses before failing the test.
    pub max_rate_limit_retries: u32,
}

impl TestConfig {
    /// Reads the configuration from the environment, falling back to the defaults.
    #[must_use]
    pub fn from_env() -> Self {
        Self::from_lookup(|key| std::env::var(key).ok())
    }

    /// Builds a configuration from `lookup`, falling back to the documented defaults.
    fn from_lookup(mut lookup: impl FnMut(&str) -> Option<String>) -> Self {
        fn parse<T: std::str::FromStr>(value: Option<String>, default: T) -> T {
            value
                .and_then(|value| value.trim().parse().ok())
                .unwrap_or(default)
        }

        Self {
            window_requests: parse(lookup("ANILIST_TEST_WINDOW_REQUESTS"), 80),
            window_secs: parse(lookup("ANILIST_TEST_WINDOW_SECS"), 60),
            pause_secs: parse(lookup("ANILIST_TEST_PAUSE_SECS"), 61),
            min_delay_ms: parse(lookup("ANILIST_TEST_MIN_DELAY_MS"), 700),
            max_network_retries: parse(lookup("ANILIST_TEST_MAX_RETRIES"), 3),
            retry_delay_ms: parse(lookup("ANILIST_TEST_RETRY_DELAY_MS"), 2000),
            max_rate_limit_retries: parse(lookup("ANILIST_TEST_MAX_RATE_LIMIT_RETRIES"), 5),
        }
    }

    /// The configuration for this test process.
    pub fn get() -> &'static Self {
        &CONFIG
    }
}

/// Parsed once per test process; the initializer is declared with the cell.
static CONFIG: std::sync::LazyLock<TestConfig> = std::sync::LazyLock::new(|| {
    let config = TestConfig::from_env();
    eprintln!(
        "Test harness: {} requests / {}s window, {}s pause on 429, {}ms between requests",
        config.window_requests, config.window_secs, config.pause_secs, config.min_delay_ms
    );
    config
});

/// Global rate limiter shared across tests
static GLOBAL_RATE_LIMITER: GlobalRateLimiter = GlobalRateLimiter::new();

/// Coordinates rate limit state across threads
pub struct GlobalRateLimiter {
    /// Whether a pause is active
    is_paused: AtomicBool,
    /// Pause end (unix seconds)
    pause_until: AtomicU64,
    /// Requests in current window
    request_count: AtomicU32,
    /// Window start (unix seconds)
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

    /// Current unix timestamp (seconds)
    fn now_secs() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
    }

    /// Start a global pause for `secs` seconds.
    pub fn trigger_pause(&self, secs: u64) {
        let pause_end = Self::now_secs() + secs;
        self.pause_until.store(pause_end, Ordering::SeqCst);
        self.is_paused.store(true, Ordering::SeqCst);
        eprintln!("Rate limit hit. Pausing {secs}s");
    }

    /// Wait while a pause is active
    pub async fn wait_if_paused(&self) {
        loop {
            let pause_until = self.pause_until.load(Ordering::SeqCst);
            let now = Self::now_secs();

            if pause_until > now {
                let wait_secs = pause_until - now;
                eprintln!("⏳ Waiting {} seconds for rate limit reset...", wait_secs);
                sleep(Duration::from_secs(wait_secs + 1)).await;
            } else {
                self.is_paused.store(false, Ordering::SeqCst);
                break;
            }
        }
    }

    /// Track a request and enforce delays
    pub async fn pre_request(&self) {
        self.wait_if_paused().await;

        let config = TestConfig::get();
        let now = Self::now_secs();
        let window_start = self.window_start.load(Ordering::SeqCst);

        if now - window_start >= config.window_secs {
            self.window_start.store(now, Ordering::SeqCst);
            self.request_count.store(0, Ordering::SeqCst);
        }

        let count = self.request_count.fetch_add(1, Ordering::SeqCst);
        if count >= config.window_requests {
            let window_start = self.window_start.load(Ordering::SeqCst);
            let elapsed = now - window_start;

            if elapsed < config.window_secs {
                let wait_secs = config.window_secs - elapsed;
                eprintln!(
                    "⚠️  Approaching rate limit ({}/{} requests). Waiting {} seconds...",
                    count, config.window_requests, wait_secs
                );
                sleep(Duration::from_secs(wait_secs + 1)).await;
                self.window_start.store(Self::now_secs(), Ordering::SeqCst);
                self.request_count.store(0, Ordering::SeqCst);
            }
        }

        sleep(Duration::from_millis(config.min_delay_ms)).await;
    }

    /// Handle a rate limit error (global pause).
    ///
    /// `server_retry_after` is the `Retry-After` the API sent, in seconds; the
    /// configured pause is used when it is missing.
    pub async fn handle_rate_limit(&self, server_retry_after: Option<u64>) {
        self.trigger_pause(server_retry_after.unwrap_or(TestConfig::get().pause_secs));
        self.wait_if_paused().await;
        self.window_start.store(Self::now_secs(), Ordering::SeqCst);
        self.request_count.store(0, Ordering::SeqCst);
    }
}

/// Test harness that manages rate limiting across tests
pub struct TestHarness {
    client: AniListClient,
    last_request: Mutex<Instant>,
}

/// Starts a `tracing` subscriber once, when the feature is on, so a test run
/// with `--features tracing` shows the spans the client emits.
///
/// The level comes from `RUST_LOG`, defaulting to `info`.
#[cfg(feature = "tracing")]
fn init_tracing() {
    static ONCE: std::sync::Once = std::sync::Once::new();

    ONCE.call_once(|| {
        use tracing_subscriber::EnvFilter;

        let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
        let _ = tracing_subscriber::fmt().with_env_filter(filter).try_init();
    });
}

impl TestHarness {
    /// Creates a new test harness with an unauthenticated client
    pub fn new() -> Self {
        #[cfg(feature = "tracing")]
        init_tracing();

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
    /// 4. Retries on transient network errors (up to `max_network_retries` times)
    /// 5. Triggers a GLOBAL pause (all threads) when rate limited, for as long as
    ///    the API asked for
    /// 6. Retries the operation after the pause
    pub async fn run<F, Fut, T>(&self, operation: F) -> Result<T, AniListError>
    where
        F: Fn() -> Fut,
        Fut: Future<Output = Result<T, AniListError>>,
    {
        let config = TestConfig::get();
        let mut network_retries = 0u32;
        let mut rate_limit_retries = 0u32;

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
                Ok(result) => {
                    if network_retries > 0 {
                        eprintln!("✅ Retry successful after {} attempts.", network_retries);
                    }
                    return Ok(result);
                }
                Err(e) if e.is_rate_limit() => {
                    if rate_limit_retries >= config.max_rate_limit_retries {
                        eprintln!(
                            "❌ Max rate limit retries ({}) reached. Failing test.",
                            config.max_rate_limit_retries
                        );
                        return Err(e);
                    }
                    rate_limit_retries += 1;
                    eprintln!(
                        "⏳ Rate limit hit ({}/{}), pausing for ALL threads...",
                        rate_limit_retries, config.max_rate_limit_retries
                    );
                    // Trigger global pause for ALL threads, honouring Retry-After
                    GLOBAL_RATE_LIMITER
                        .handle_rate_limit(e.retry_after_secs())
                        .await;
                    // Loop will retry after pause
                }
                Err(e) if e.is_network() && network_retries < config.max_network_retries => {
                    network_retries += 1;
                    eprintln!(
                        "🔄 Network error, retrying ({}/{})...: {:?}",
                        network_retries, config.max_network_retries, e
                    );
                    sleep(Duration::from_millis(config.retry_delay_ms)).await;
                    // Loop will retry
                }
                Err(e) => {
                    if network_retries > 0 {
                        eprintln!(
                            "❌ Max network retries ({}) reached. Failing test.",
                            config.max_network_retries
                        );
                    }
                    return Err(e);
                }
            }
        }
    }
}

impl Default for TestHarness {
    fn default() -> Self {
        Self::new()
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

/// Pauses for the configured rate limit window reset period (triggers global pause)
pub async fn pause_for_rate_limit() {
    GLOBAL_RATE_LIMITER.handle_rate_limit(None).await;
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
        assert!(AniListError::RateLimitSimple.is_rate_limit());
        assert!(AniListError::BurstLimit.is_rate_limit());
        assert!(AniListError::RateLimit {
            limit: 90,
            remaining: 0,
            reset_at: 0,
            retry_after: 60,
        }
        .is_rate_limit());
        assert!(!AniListError::NotFound.is_rate_limit());
        assert!(!AniListError::AuthenticationRequired.is_rate_limit());
    }

    #[test]
    fn test_is_network_error() {
        // Network errors should be detected
        assert!(!AniListError::NotFound.is_network());
        assert!(!AniListError::RateLimitSimple.is_network());
    }

    #[test]
    fn test_global_rate_limiter_now() {
        let now = GlobalRateLimiter::now_secs();
        assert!(now > 0);
    }

    #[test]
    fn test_config_defaults() {
        // No environment input: every field falls back to the documented default.
        let config = TestConfig::from_lookup(|_| None);
        assert_eq!(config.window_requests, 80);
        assert_eq!(config.window_secs, 60);
        assert_eq!(config.pause_secs, 61);
        assert_eq!(config.min_delay_ms, 700);
        assert_eq!(config.max_network_retries, 3);
        assert_eq!(config.retry_delay_ms, 2000);
        assert_eq!(config.max_rate_limit_retries, 5);
        assert!(
            config.window_requests < 90,
            "the default must stay under the API limit"
        );
    }

    #[test]
    fn test_config_overrides_and_bad_values() {
        let config = TestConfig::from_lookup(|key| match key {
            "ANILIST_TEST_WINDOW_REQUESTS" => Some("7".to_string()),
            "ANILIST_TEST_WINDOW_SECS" => Some("not a number".to_string()),
            "ANILIST_TEST_MAX_RETRIES" => Some(" 5 ".to_string()),
            _ => None,
        });
        assert_eq!(config.window_requests, 7, "override applies");
        assert_eq!(config.window_secs, 60, "unparsable values keep the default");
        assert_eq!(
            config.max_network_retries, 5,
            "surrounding space is trimmed"
        );
        assert_eq!(
            config.max_rate_limit_retries, 5,
            "unset values keep the default"
        );
        assert_eq!(config.pause_secs, 61, "unset values keep the default");
    }

    #[test]
    fn test_retry_after_comes_from_the_error() {
        let error = AniListError::RateLimit {
            limit: 90,
            remaining: 0,
            reset_at: 0,
            retry_after: 42,
        };
        assert_eq!(error.retry_after_secs(), Some(42));
        assert_eq!(AniListError::RateLimitSimple.retry_after_secs(), None);
        assert_eq!(AniListError::BurstLimit.retry_after_secs(), None);
        assert_eq!(AniListError::NotFound.retry_after_secs(), None);
    }

    #[test]
    fn test_pause_uses_the_requested_duration() {
        let limiter = GlobalRateLimiter::new();
        limiter.trigger_pause(5);
        let now = GlobalRateLimiter::now_secs();
        let deadline = limiter.pause_until.load(Ordering::SeqCst);
        assert!(limiter.is_paused.load(Ordering::SeqCst));
        assert!(
            deadline > now && deadline <= now + 6,
            "the pause must last the requested 5s, got {deadline} against {now}"
        );
    }
}
