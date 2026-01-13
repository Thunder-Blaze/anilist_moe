//! Utility helpers for rate limits, retries, and JSON.

use crate::errors::AniListError;
use serde_json::Value;
use std::collections::HashMap;
use std::time::Duration;
use tokio::time::sleep;

/// Retry configuration.
///
/// Controls retry count, delay strategy, and backoff.
///
/// Example:
/// ```rust
/// use anilist_moe::utils::RetryConfig;
/// let config = RetryConfig { max_retries: 5, base_delay_ms: 2000, exponential_backoff: true, max_delay_ms: 60000 };
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RetryConfig {
    /// Maximum number of retry attempts before giving up
    pub max_retries: u32,
    /// Initial delay in milliseconds before the first retry
    pub base_delay_ms: u64,
    /// Whether to use exponential backoff (doubles delay each retry)
    pub exponential_backoff: bool,
    /// Maximum delay in milliseconds between retries
    pub max_delay_ms: u64,
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            max_retries: 3,
            base_delay_ms: 1000,
            exponential_backoff: true,
            max_delay_ms: 30000,
        }
    }
}

impl RetryConfig {
    /// Configuration tuned for API rate limits.
    #[must_use]
    pub fn for_rate_limits() -> Self {
        Self {
            max_retries: 5,
            base_delay_ms: 60000,       // 1 minute base delay for rate limits
            exponential_backoff: false, // Fixed delay for rate limits
            max_delay_ms: 120000,       // 2 minute max
        }
    }

    /// Configuration for aggressive retries (transient errors).
    #[must_use]
    pub fn aggressive() -> Self {
        Self {
            max_retries: 5,
            base_delay_ms: 500,
            exponential_backoff: true,
            max_delay_ms: 10000,
        }
    }

    /// Calculates the delay for a given attempt number.
    #[inline]
    fn calculate_delay(&self, attempts: u32) -> Duration {
        let delay = if self.exponential_backoff {
            self.base_delay_ms.saturating_mul(1u64 << attempts.min(10))
        } else {
            self.base_delay_ms
        };
        Duration::from_millis(delay.min(self.max_delay_ms))
    }
}

/// Retry an operation with backoff on rate limits.
pub async fn retry_with_backoff<F, Fut, T>(
    mut operation: F,
    config: RetryConfig,
) -> Result<T, AniListError>
where
    F: FnMut() -> Fut,
    Fut: std::future::Future<Output = Result<T, AniListError>>,
{
    let mut attempts = 0u32;

    loop {
        match operation().await {
            Ok(result) => return Ok(result),
            Err(err) => {
                let (should_retry, sleep_duration) = match &err {
                    AniListError::RateLimit { retry_after, .. } => {
                        if attempts >= config.max_retries {
                            return Err(err);
                        }
                        // Use the Retry-After header if available and reasonable
                        let duration = if *retry_after > 0 && *retry_after <= 300 {
                            Duration::from_secs(*retry_after as u64)
                        } else {
                            config.calculate_delay(attempts)
                        };
                        (true, duration)
                    }
                    AniListError::RateLimitSimple => {
                        if attempts >= config.max_retries {
                            return Err(err);
                        }
                        (true, config.calculate_delay(attempts))
                    }
                    AniListError::BurstLimit => {
                        if attempts >= config.max_retries {
                            return Err(err);
                        }
                        // For burst limits, wait longer
                        let duration = config.calculate_delay(attempts + 1);
                        (true, duration)
                    }
                    // Don't retry on other errors
                    _ => return Err(err),
                };

                if should_retry {
                    log::warn!(
                        "Rate limited. Retrying in {} seconds... (attempt {}/{})",
                        sleep_duration.as_secs(),
                        attempts + 1,
                        config.max_retries
                    );
                    sleep(sleep_duration).await;
                    attempts += 1;
                }
            }
        }
    }
}

/// Sleeps for the specified duration in milliseconds.
#[inline]
pub async fn rate_limit_delay(delay_ms: u64) {
    sleep(Duration::from_millis(delay_ms)).await;
}

/// Calculates an appropriate delay based on remaining rate limit quota.
#[inline]
pub fn calculate_delay(remaining: u32, reset_in_seconds: u64) -> Duration {
    match remaining {
        0 => Duration::from_secs(reset_in_seconds),
        1..=9 => Duration::from_millis(2000), // 2 seconds when getting low
        10..=29 => Duration::from_millis(1000), // 1 second when moderate
        _ => Duration::from_millis(500),      // 500ms when plenty remaining
    }
}

/// Converts a `serde_json::Value` to a `HashMap<String, Value>`.
///
/// If the input `Value` is not a JSON object, it returns an empty `HashMap`.
#[inline]
pub fn json_to_hashmap(value: Value) -> HashMap<String, Value> {
    match value {
        Value::Object(map) => map.into_iter().collect(),
        _ => HashMap::new(),
    }
}

/// Converts a `serde_json::Value` to a `HashMap<String, Value>` with capacity hint.
///
/// More efficient when you know the approximate size.
#[inline]
pub fn json_to_hashmap_with_capacity(value: Value, capacity: usize) -> HashMap<String, Value> {
    match value {
        Value::Object(map) => {
            let mut result = HashMap::with_capacity(capacity.max(map.len()));
            result.extend(map);
            result
        }
        _ => HashMap::with_capacity(capacity),
    }
}
