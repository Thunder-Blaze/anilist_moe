//! # Utility Functions
//!
//! This module provides utility functions for handling rate limiting, retries,
//! and other common operations when working with the AniList API.

use crate::errors::AniListError;
use serde_json::Value;
use std::collections::HashMap;
use std::time::Duration;
use tokio::time::sleep;

/// Configuration for retry behavior when requests fail.
///
/// This struct controls how the client retries failed requests, including
/// rate limit errors and transient network failures.
///
/// # Examples
///
/// ```rust
/// use anilist_moe::utils::RetryConfig;
///
/// let config = RetryConfig {
///     max_retries: 5,
///     base_delay_ms: 2000,
///     exponential_backoff: true,
///     max_delay_ms: 60000,
/// };
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

pub async fn retry_with_backoff<F, Fut, T>(
    mut operation: F,
    config: RetryConfig,
) -> Result<T, AniListError>
where
    F: FnMut() -> Fut,
    Fut: std::future::Future<Output = Result<T, AniListError>>,
{
    let mut attempts = 0;
    let mut delay = config.base_delay_ms;

    loop {
        match operation().await {
            Ok(result) => return Ok(result),
            Err(AniListError::RateLimit { retry_after, .. }) => {
                if attempts >= config.max_retries {
                    return Err(AniListError::RateLimit {
                        limit: 90,
                        remaining: 0,
                        reset_at: 0,
                        retry_after,
                    });
                }

                // Use the Retry-After header if available, otherwise use exponential backoff
                let sleep_duration = if retry_after > 0 {
                    Duration::from_secs(retry_after as u64)
                } else {
                    Duration::from_millis(delay.min(config.max_delay_ms))
                };

                log::warn!(
                    "Rate limited. Retrying in {} seconds... (attempt {}/{})",
                    sleep_duration.as_secs(),
                    attempts + 1,
                    config.max_retries
                );

                sleep(sleep_duration).await;

                attempts += 1;
                if config.exponential_backoff {
                    delay = (delay * 2).min(config.max_delay_ms);
                }
            }
            Err(AniListError::RateLimitSimple) => {
                if attempts >= config.max_retries {
                    return Err(AniListError::RateLimitSimple);
                }

                let sleep_duration = Duration::from_millis(delay.min(config.max_delay_ms));
                log::warn!(
                    "Rate limited. Retrying in {} seconds... (attempt {}/{})",
                    sleep_duration.as_secs(),
                    attempts + 1,
                    config.max_retries
                );

                sleep(sleep_duration).await;

                attempts += 1;
                if config.exponential_backoff {
                    delay = (delay * 2).min(config.max_delay_ms);
                }
            }
            Err(AniListError::BurstLimit) => {
                if attempts >= config.max_retries {
                    return Err(AniListError::BurstLimit);
                }

                // For burst limits, wait a bit longer
                let sleep_duration = Duration::from_millis((delay * 2).min(config.max_delay_ms));
                log::warn!(
                    "Burst limit exceeded. Retrying in {} seconds... (attempt {}/{})",
                    sleep_duration.as_secs(),
                    attempts + 1,
                    config.max_retries
                );

                sleep(sleep_duration).await;

                attempts += 1;
                delay = (delay * 2).min(config.max_delay_ms);
            }
            Err(other_error) => return Err(other_error),
        }
    }
}

pub async fn rate_limit_delay(delay_ms: u64) {
    sleep(Duration::from_millis(delay_ms)).await;
}

pub fn calculate_delay(remaining: u32, reset_in_seconds: u64) -> Duration {
    if remaining == 0 {
        Duration::from_secs(reset_in_seconds)
    } else if remaining < 10 {
        Duration::from_millis(2000) // 2 seconds when getting low
    } else if remaining < 30 {
        Duration::from_millis(1000) // 1 second when moderate
    } else {
        Duration::from_millis(500) // 500ms when plenty remaining
    }
}

/// Converts a `serde_json::Value` to a `HashMap<String, Value>`.
///
/// If the input `Value` is not a JSON object, it returns an empty `HashMap`.
pub fn json_to_hashmap(value: Value) -> HashMap<String, Value> {
    match value {
        Value::Object(map) => map.into_iter().collect(),
        _ => HashMap::new(),
    }
}
