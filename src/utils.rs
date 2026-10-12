//! Utility helpers for rate limits, retries, and JSON.

use crate::errors::AniListError;
use std::time::Duration;
use tokio::time::sleep;
#[cfg(feature = "tracing")]
use tracing::Instrument;

/// Retry configuration.
///
/// Retries are opt-in: [`RetryConfig::default`] retries nothing, and the
/// constructors below enable them. Controls retry count, delay strategy, and
/// backoff.
///
/// Example:
/// ```rust
/// use anilist_moe::utils::RetryConfig;
/// let config = RetryConfig { max_retries: 5, base_delay_ms: 2000, exponential_backoff: true, max_delay_ms: 60000, jitter: true, retry_transient_errors: true };
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RetryConfig {
    /// Maximum number of retry attempts before giving up. `0` disables retries.
    pub max_retries: u32,
    /// Initial delay in milliseconds before the first retry
    pub base_delay_ms: u64,
    /// Whether to use exponential backoff (doubles delay each retry)
    pub exponential_backoff: bool,
    /// Maximum delay in milliseconds between retries
    pub max_delay_ms: u64,
    /// Spreads each delay by up to 50%, so clients that failed together do not
    /// all come back at the same moment.
    ///
    /// Set this to `false` for a deterministic schedule.
    pub jitter: bool,
    /// Retries transient failures as well as rate limits: `5xx` responses and
    /// transport errors, which usually succeed on a second attempt.
    ///
    /// Off in [`RetryConfig::default`], on in the presets below, and ignored when
    /// `max_retries` is `0`.
    pub retry_transient_errors: bool,
}

impl Default for RetryConfig {
    /// Retries are off: a failed request is returned to the caller as is.
    ///
    /// Which failures are worth another attempt depends on the caller — its rate
    /// limit budget, how it cancels work, whether the request is idempotent — so
    /// the client does not guess. Opt in with [`RetryConfig::for_rate_limits`] or
    /// [`RetryConfig::aggressive`], or with a configuration of your own.
    ///
    /// ```rust
    /// use anilist_moe::{AniListClient, utils::RetryConfig};
    ///
    /// // No retries: a 429 comes straight back to the caller.
    /// let client = AniListClient::new();
    /// assert_eq!(client.retry_config().max_retries, 0);
    ///
    /// // Retry rate limits and transient failures.
    /// let client = AniListClient::new().with_retry_config(RetryConfig::aggressive());
    /// assert_eq!(client.retry_config().max_retries, 5);
    /// ```
    fn default() -> Self {
        Self {
            max_retries: 0,
            base_delay_ms: 1000,
            exponential_backoff: true,
            max_delay_ms: 30000,
            jitter: true,
            retry_transient_errors: false,
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
            jitter: true,
            retry_transient_errors: true,
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
            jitter: true,
            retry_transient_errors: true,
        }
    }

    /// The same configuration with a fixed delay schedule.
    #[must_use]
    pub fn without_jitter(self) -> Self {
        Self {
            jitter: false,
            ..self
        }
    }

    /// The same configuration, retrying only rate limits.
    #[must_use]
    pub fn without_transient_retries(self) -> Self {
        Self {
            retry_transient_errors: false,
            ..self
        }
    }

    /// The delay before retry number `attempts` (zero based), before jitter.
    #[must_use]
    pub fn delay_for(&self, attempts: u32) -> Duration {
        let delay = if self.exponential_backoff {
            self.base_delay_ms.saturating_mul(1u64 << attempts.min(10))
        } else {
            self.base_delay_ms
        };
        Duration::from_millis(delay.min(self.max_delay_ms))
    }

    /// The delay before retry number `attempts` (zero based), with jitter applied.
    fn delay_with_jitter(&self, attempts: u32) -> Duration {
        let delay = self.delay_for(attempts);
        if !self.jitter {
            return delay;
        }

        // Spread by up to 50% of the delay, never shortening it, so the schedule
        // stays at least as patient as configured.
        let spread = (delay.as_millis() as u64) / 2;
        if spread == 0 {
            return delay;
        }
        delay + Duration::from_millis(random_below(spread + 1))
    }
}

/// A pseudo random value below `bound`, seeded from the OS.
///
/// Retry jitter only needs to be unpredictable enough to spread clients out,
/// which does not warrant a dependency on a random number generator.
fn random_below(bound: u64) -> u64 {
    use std::hash::{BuildHasher, Hasher};

    let mut hasher = std::collections::hash_map::RandomState::new().build_hasher();
    hasher.write_u64(
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|since_epoch| since_epoch.as_nanos() as u64)
            .unwrap_or_default(),
    );
    hasher.write_u64(std::process::id() as u64);
    hasher.finish() % bound
}

/// How long to wait before retrying `error`, or `None` to give up.
fn retry_delay(error: &AniListError, attempts: u32, config: &RetryConfig) -> Option<Duration> {
    if attempts >= config.max_retries {
        return None;
    }

    match error {
        AniListError::RateLimit { retry_after, .. } => {
            // Honour Retry-After when it is a sane value to wait for.
            if *retry_after > 0 && *retry_after <= 300 {
                Some(Duration::from_secs(u64::from(*retry_after)))
            } else {
                Some(config.delay_with_jitter(attempts))
            }
        }
        AniListError::RateLimitSimple => Some(config.delay_with_jitter(attempts)),
        // A burst limit means the window has to drain, so wait a full step longer.
        AniListError::BurstLimit => Some(config.delay_with_jitter(attempts + 1)),
        AniListError::ServerError { status, .. }
            if config.retry_transient_errors && (500..=599).contains(status) =>
        {
            Some(config.delay_with_jitter(attempts))
        }
        AniListError::Network(_) | AniListError::Transport { .. }
            if config.retry_transient_errors =>
        {
            Some(config.delay_with_jitter(attempts))
        }
        _ => None,
    }
}

/// Retry an operation with backoff on rate limits and, when configured,
/// transient failures.
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
        // Each attempt runs inside a span carrying its one-based attempt
        // number, so the request events emitted by the operation inherit it.
        #[cfg(feature = "tracing")]
        let result = operation()
            .instrument(tracing::debug_span!(
                target: "anilist_moe::utils",
                "attempt",
                attempt = attempts + 1,
            ))
            .await;
        #[cfg(not(feature = "tracing"))]
        let result = operation().await;

        match result {
            Ok(result) => return Ok(result),
            Err(err) => {
                let Some(sleep_duration) = retry_delay(&err, attempts, &config) else {
                    return Err(err);
                };

                #[cfg(feature = "tracing")]
                tracing::warn!(
                    target: "anilist_moe::utils",
                    error = err.kind(),
                    attempt = attempts + 1,
                    max_retries = config.max_retries,
                    delay_ms = sleep_duration.as_millis() as u64,
                    "retrying after a failed attempt",
                );
                log::warn!(
                    "{}. Retrying in {} seconds... (attempt {}/{})",
                    err,
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

#[cfg(test)]
mod tests {
    use super::*;

    /// A configuration whose delays are exact and easy to reason about.
    fn exact() -> RetryConfig {
        RetryConfig {
            max_retries: 3,
            base_delay_ms: 100,
            exponential_backoff: true,
            max_delay_ms: 10_000,
            jitter: false,
            retry_transient_errors: true,
        }
    }

    #[test]
    fn deterministic_backoff_doubles_and_stops_at_the_cap() {
        let config = exact();
        assert_eq!(config.delay_for(0), Duration::from_millis(100));
        assert_eq!(config.delay_for(1), Duration::from_millis(200));
        assert_eq!(config.delay_for(2), Duration::from_millis(400));
        assert_eq!(config.delay_for(3), Duration::from_millis(800));

        let capped = RetryConfig {
            max_delay_ms: 300,
            ..config
        };
        assert_eq!(
            capped.delay_for(4),
            Duration::from_millis(300),
            "the cap applies"
        );

        let fixed = RetryConfig {
            exponential_backoff: false,
            ..config
        };
        assert_eq!(fixed.delay_for(0), fixed.delay_for(5));
    }

    #[test]
    fn jitter_stays_within_the_configured_band() {
        let config = RetryConfig {
            base_delay_ms: 1000,
            ..exact()
        };

        for _ in 0..32 {
            let delay = config.delay_with_jitter(0);
            assert!(
                delay >= Duration::from_millis(1000) && delay <= Duration::from_millis(1500),
                "delay {delay:?} left the 1000..=1500ms band"
            );
        }

        assert_eq!(
            config.without_jitter().delay_with_jitter(0),
            Duration::from_millis(1000),
            "the deterministic mode must not vary"
        );
    }

    #[test]
    fn jitter_does_not_amplify_a_zero_delay() {
        let config = RetryConfig {
            base_delay_ms: 0,
            ..exact()
        };
        assert_eq!(config.delay_with_jitter(0), Duration::ZERO);
    }

    #[test]
    fn rate_limit_delays_come_from_the_server() {
        let config = exact();
        let rate_limited = |retry_after| AniListError::RateLimit {
            limit: 90,
            remaining: 0,
            reset_at: 0,
            retry_after,
        };

        assert_eq!(
            retry_delay(&rate_limited(30), 0, &config),
            Some(Duration::from_secs(30)),
            "the server asked for 30s and gets it"
        );
        assert_eq!(
            retry_delay(&rate_limited(0), 0, &config),
            Some(config.delay_for(0)),
            "no Retry-After falls back to the backoff schedule"
        );
        assert_eq!(
            retry_delay(&rate_limited(99_999), 0, &config),
            Some(config.delay_for(0)),
            "an implausible Retry-After is ignored"
        );
    }

    #[test]
    fn burst_limits_wait_a_step_longer() {
        let config = exact();
        assert_eq!(
            retry_delay(&AniListError::BurstLimit, 0, &config),
            Some(config.delay_for(1))
        );
        assert_eq!(
            retry_delay(&AniListError::RateLimitSimple, 0, &config),
            Some(config.delay_for(0))
        );
    }

    #[test]
    fn transient_failures_are_retried_only_when_asked_for() {
        let config = exact();
        let unavailable = AniListError::ServerError {
            status: 503,
            message: "down".to_string(),
        };
        assert_eq!(
            retry_delay(&unavailable, 0, &config),
            Some(config.delay_for(0))
        );
        assert_eq!(
            retry_delay(&unavailable, 0, &config.without_transient_retries()),
            None,
            "opt out stops the retries"
        );

        let transport = AniListError::Transport {
            message: "connection reset".to_string(),
        };
        assert!(retry_delay(&transport, 0, &config).is_some());

        // Client mistakes are never retried.
        let bad_request = AniListError::BadRequest {
            message: "nope".to_string(),
        };
        assert_eq!(retry_delay(&bad_request, 0, &config), None);
        assert_eq!(retry_delay(&AniListError::NotFound, 0, &config), None);
        assert_eq!(
            retry_delay(
                &AniListError::GraphQL {
                    errors: vec![crate::errors::GraphQLError::new("bad query")],
                    data: None,
                },
                0,
                &config
            ),
            None
        );
    }

    #[test]
    fn the_retry_budget_is_respected() {
        let config = RetryConfig {
            max_retries: 2,
            ..exact()
        };
        let error = AniListError::RateLimitSimple;

        assert!(retry_delay(&error, 0, &config).is_some());
        assert!(retry_delay(&error, 1, &config).is_some());
        assert_eq!(retry_delay(&error, 2, &config), None, "budget is spent");
    }

    #[tokio::test]
    async fn a_failing_operation_is_retried_then_gives_up() {
        let config = RetryConfig {
            max_retries: 2,
            base_delay_ms: 1,
            exponential_backoff: false,
            max_delay_ms: 1,
            jitter: false,
            retry_transient_errors: true,
        };

        let mut calls = 0;
        let result: Result<(), AniListError> = retry_with_backoff(
            || {
                calls += 1;
                async { Err(AniListError::RateLimitSimple) }
            },
            config,
        )
        .await;

        assert!(matches!(result, Err(AniListError::RateLimitSimple)));
        assert_eq!(calls, 3, "the first attempt plus two retries");
    }

    #[tokio::test]
    async fn a_retried_operation_can_succeed() {
        let config = RetryConfig {
            max_retries: 3,
            base_delay_ms: 1,
            exponential_backoff: false,
            max_delay_ms: 1,
            jitter: false,
            retry_transient_errors: true,
        };

        let mut calls = 0;
        let result = retry_with_backoff(
            || {
                calls += 1;
                async move {
                    if calls == 1 {
                        Err(AniListError::ServerError {
                            status: 502,
                            message: "bad gateway".to_string(),
                        })
                    } else {
                        Ok("done")
                    }
                }
            },
            config,
        )
        .await;

        assert_eq!(result.unwrap(), "done");
        assert_eq!(calls, 2);
    }

    #[test]
    fn calculate_delay_follows_the_remaining_quota() {
        assert_eq!(
            calculate_delay(0, 60),
            Duration::from_secs(60),
            "with nothing left, wait out the window"
        );
        assert_eq!(calculate_delay(1, 60), Duration::from_millis(2000));
        assert_eq!(calculate_delay(9, 60), Duration::from_millis(2000));
        assert_eq!(calculate_delay(10, 60), Duration::from_millis(1000));
        assert_eq!(calculate_delay(29, 60), Duration::from_millis(1000));
        assert_eq!(calculate_delay(30, 60), Duration::from_millis(500));
        assert_eq!(calculate_delay(80, 60), Duration::from_millis(500));
    }
}
