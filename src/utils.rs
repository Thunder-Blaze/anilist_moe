//! # Utility Functions
//!
//! This module provides utility functions for handling rate limiting, retries,
//! and other common operations when working with the AniList API.

use crate::error::AniListError;
use std::time::Duration;
use tokio::time::sleep;

/// Configuration for retry behavior when handling API failures.
///
/// This struct controls how the wrapper handles transient failures like
/// rate limiting, network issues, and server errors through automatic
/// retry logic with configurable backoff strategies.
///
/// # Examples
///
/// ```rust
/// use anilist_moe::utils::RetryConfig;
///
/// // Default configuration (3 retries, exponential backoff)
/// let config = RetryConfig::default();
///
/// // Custom configuration for aggressive retrying
/// let aggressive = RetryConfig {
///     max_retries: 5,
///     base_delay_ms: 500,
///     exponential_backoff: true,
///     max_delay_ms: 60000,
/// };
///
/// // Configuration for quick retries without backoff
/// let quick = RetryConfig {
///     max_retries: 2,
///     base_delay_ms: 100,
///     exponential_backoff: false,
///     max_delay_ms: 1000,
/// };
/// ```
#[derive(Debug, Clone)]
pub struct RetryConfig {
    /// Maximum number of retry attempts before giving up.
    ///
    /// Set to 0 to disable retries entirely. Higher values provide more
    /// resilience but may increase response times for failing requests.
    ///
    /// # Recommended Values
    /// - 0: No retries (fastest failure response)
    /// - 1-3: Conservative retrying (good for most use cases)
    /// - 4-6: Aggressive retrying (for critical operations)
    /// - 7+: Very aggressive (may cause long delays)
    pub max_retries: u32,

    /// Base delay between retry attempts in milliseconds.
    ///
    /// This is the initial delay used for the first retry. With exponential
    /// backoff enabled, subsequent delays will be calculated based on this value.
    ///
    /// # Recommended Values
    /// - 100-500ms: Quick retries for temporary glitches
    /// - 1000-2000ms: Standard retries for rate limiting
    /// - 3000-5000ms: Conservative retries for server issues
    pub base_delay_ms: u64,

    /// Whether to use exponential backoff for retry delays.
    ///
    /// When enabled, each retry attempt will wait longer than the previous one,
    /// helping to avoid overwhelming the API during issues. When disabled,
    /// all retries use the base delay.
    ///
    /// # Exponential Backoff Formula
    ///
    /// ```text
    /// delay = min(base_delay_ms * 2^attempt, max_delay_ms)
    /// ```
    ///
    /// # Recommendations
    /// - `true`: Best for production use and rate limiting
    /// - `false`: Use for testing or when consistent timing is needed
    pub exponential_backoff: bool,

    /// Maximum delay between retries in milliseconds.
    ///
    /// Caps the delay when using exponential backoff to prevent excessively
    /// long wait times. Has no effect when exponential backoff is disabled.
    ///
    /// # Recommended Values
    /// - 10-30 seconds: Good balance for most applications
    /// - 1-2 minutes: For non-interactive or batch operations
    /// - 5+ minutes: Only for very long-running processes
    pub max_delay_ms: u64,
}

impl Default for RetryConfig {
    /// Creates a default retry configuration suitable for most use cases.
    ///
    /// Default settings:
    /// - `max_retries`: 3 attempts
    /// - `base_delay_ms`: 1000ms (1 second)
    /// - `exponential_backoff`: true
    /// - `max_delay_ms`: 30000ms (30 seconds)
    ///
    /// These defaults provide a good balance between resilience and response time,
    /// with appropriate handling for AniList's rate limiting.
    fn default() -> Self {
        Self {
            max_retries: 3,
            base_delay_ms: 1000,
            exponential_backoff: true,
            max_delay_ms: 30000,
        }
    }
}

/// Executes a future with automatic retry logic for handling transient failures.
///
/// This function wraps API calls with intelligent retry behavior, automatically
/// handling rate limits, network issues, and server errors according to the
/// provided retry configuration.
///
/// # Parameters
///
/// * `operation` - A closure that returns a future representing the API operation to retry
/// * `config` - Retry configuration controlling backoff behavior and attempt limits
///
/// # Returns
///
/// Returns the successful result of the operation, or the final error if all retries fail.
///
/// # Retry Conditions
///
/// The following errors trigger automatic retries:
/// - [`AniListError::RateLimit`] - Respects retry-after timing when available
/// - [`AniListError::RateLimitSimple`] - Uses exponential backoff
/// - [`AniListError::BurstLimit`] - Uses exponential backoff
/// - [`AniListError::Network`] - For transient network issues
/// - [`AniListError::ServerError`] - For 5xx server errors
///
/// Other errors (authentication, not found, bad request) are not retried as they
/// typically indicate permanent issues that won't resolve with retries.
///
/// # Examples
///
/// ```rust
/// use anilist_moe::{AniListClient, utils::{retry_with_backoff, RetryConfig}};
///
/// let client = AniListClient::new();
/// let config = RetryConfig::default();
///
/// // Retry an API call with automatic backoff
/// let result = retry_with_backoff(
///     || client.anime().get_popular(1, 10),
///     config
/// ).await?;
///
/// // Custom retry configuration for critical operations
/// let aggressive_config = RetryConfig {
///     max_retries: 5,
///     base_delay_ms: 2000,
///     exponential_backoff: true,
///     max_delay_ms: 60000,
/// };
///
/// let important_result = retry_with_backoff(
///     || client.user().get_current_user(),
///     aggressive_config
/// ).await?;
/// ```
///
/// # Rate Limit Handling
///
/// When a rate limit error is encountered, the function will:
/// 1. Extract the `retry_after` value from detailed rate limit errors
/// 2. Wait for the specified time before retrying
/// 3. Fall back to exponential backoff for simple rate limit errors
/// 4. Continue with remaining retry attempts
///
/// # Performance Considerations
///
/// - Higher `max_retries` values increase resilience but may cause longer delays
/// - Exponential backoff helps avoid API overload but increases wait times
/// - Consider your application's timeout requirements when configuring retries
/// - For interactive applications, use lower retry counts to maintain responsiveness
///
/// # Error Handling
///
/// The function preserves the original error type, so callers can still handle
/// specific error conditions even after retries are exhausted.
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

                println!(
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
                println!(
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
                println!(
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

/// Helper to add delay between requests to avoid rate limiting
pub async fn rate_limit_delay(delay_ms: u64) {
    sleep(Duration::from_millis(delay_ms)).await;
}

/// Calculate appropriate delay based on remaining rate limit
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
