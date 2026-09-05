use crate::endpoints::{
    ActivityEndpoint, AiringEndpoint, CharacterEndpoint, CommonEndpoint, ForumEndpoint,
    MediaEndpoint, MediaListEndpoint, NotificationEndpoint, RecommendationEndpoint, ReviewEndpoint,
    StaffEndpoint, StudioEndpoint, UserEndpoint,
};
use crate::errors::AniListError;
use crate::objects::responses::GraphQLResponse;
use crate::utils::{retry_with_backoff, RetryConfig};
use serde::Serialize;
use serde_json::{from_value, Value};
use std::borrow::Cow;
use ureq::http::{Response, StatusCode};
use ureq::{Agent, Body};

use std::fmt;

/// The default AniList GraphQL API endpoint
const ANILIST_API_URL: &str = "https://graphql.anilist.co";

/// User-Agent header for identifying this library
const USER_AGENT: &str = concat!("anilist-moe/", env!("CARGO_PKG_VERSION"), " (Rust)");

/// Content-Type header value (reused to avoid allocations)
const CONTENT_TYPE_JSON: &str = "application/json";

/// Authorization header prefix
const BEARER_PREFIX: &str = "Bearer ";

/// Internal shared state for the client
struct ClientInner {
    client: Agent,
    token: Option<String>,
    retry_config: RetryConfig,
    base_url: Cow<'static, str>,
}

/// The main client for interacting with the AniList API.
///
/// This client handles all API requests, authentication, rate limiting,
/// and error handling. It provides access to all AniList API endpoints
/// through specialized endpoint methods.
///
/// # Examples
///
/// ```rust
/// use anilist_moe::AniListClient;
///
/// // Create a client without authentication
/// let client = AniListClient::new();
///
/// // Create a client with authentication
/// let authenticated_client = AniListClient::with_token("your_token_here");
/// ```
pub struct AniListClient {
    inner: ClientInner,
}

// Implement Debug manually to avoid exposing sensitive token information
impl fmt::Debug for AniListClient {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AniListClient")
            .field("base_url", &self.inner.base_url)
            .field("has_token", &self.inner.token.is_some())
            .field("retry_config", &self.inner.retry_config)
            .finish()
    }
}

impl AniListClient {
    /// Creates a new AniList client without authentication.
    ///
    /// This client can access all public endpoints but cannot perform
    /// authenticated actions like posting activities or managing lists.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use anilist_moe::AniListClient;
    ///
    /// let client = AniListClient::new();
    /// ```
    #[must_use]
    pub fn new() -> Self {
        Self::new_with_client(Self::build_client())
    }

    /// Creates a new AniList client with authentication.
    ///
    /// # Arguments
    ///
    /// * `token` - The OAuth2 Bearer token for authentication
    ///
    /// # Examples
    ///
    /// ```rust
    /// use anilist_moe::AniListClient;
    ///
    /// let client = AniListClient::with_token("your_access_token");
    /// ```
    #[must_use]
    pub fn with_token(token: impl Into<String>) -> Self {
        Self {
            inner: ClientInner {
                client: Self::build_client(),
                token: Some(token.into()),
                retry_config: RetryConfig::default(),
                base_url: Cow::Borrowed(ANILIST_API_URL),
            },
        }
    }

    /// Builds a configured reqwest client with optimal settings.
    fn build_client() -> Agent {
        Agent::new_with_config(
            Agent::config_builder()
                .timeout_global(Some(std::time::Duration::from_secs(30)))
                .user_agent(USER_AGENT)
                .no_delay(true)
                .build(),
        )
    }

    /// Creates a client with a custom ureq [`Agent`].
    fn new_with_client(client: Agent) -> Self {
        Self {
            inner: ClientInner {
                client,
                token: None,
                retry_config: RetryConfig::default(),
                base_url: Cow::Borrowed(ANILIST_API_URL),
            },
        }
    }

    /// Configures the retry behavior for failed requests.
    ///
    /// # Arguments
    ///
    /// * `config` - The retry configuration to use
    ///
    /// # Examples
    ///
    /// ```rust
    /// use anilist_moe::{AniListClient, utils::RetryConfig};
    ///
    /// let config = RetryConfig {
    ///     max_retries: 5,
    ///     base_delay_ms: 2000,
    ///     exponential_backoff: true,
    ///     max_delay_ms: 60000,
    /// };
    ///
    /// let client = AniListClient::new().with_retry_config(config);
    /// ```
    #[must_use]
    pub fn with_retry_config(self, config: RetryConfig) -> Self {
        let inner: ClientInner = self.inner;
        let (client, base_url, token) = (inner.client, inner.base_url, inner.token);

        Self {
            inner: ClientInner {
                client,
                base_url,
                token,
                retry_config: config,
            },
        }
    }

    /// Sets a custom base URL for the API (useful for testing or custom endpoints).
    ///
    /// # Arguments
    ///
    /// * `base_url` - The base URL to use for API requests
    ///
    /// # Examples
    ///
    /// ```rust
    /// use anilist_moe::AniListClient;
    ///
    /// let client = AniListClient::new()
    ///     .with_base_url("https://custom-api.example.com");
    /// ```
    #[must_use]
    pub fn with_base_url(self, base_url: impl Into<String>) -> Self {
        Self {
            inner: ClientInner {
                client: self.inner.client,
                token: self.inner.token,
                retry_config: self.inner.retry_config,
                base_url: Cow::Owned(base_url.into()),
            },
        }
    }

    /// Returns the media endpoint for anime and manga operations.
    #[inline]
    pub fn media(&self) -> MediaEndpoint<'_> {
        MediaEndpoint::new(self)
    }

    /// Returns the media endpoint for anime operations (alias for media()).
    #[inline]
    pub fn anime(&self) -> MediaEndpoint<'_> {
        self.media()
    }

    /// Returns the media endpoint for manga operations (alias for media()).
    #[inline]
    pub fn manga(&self) -> MediaEndpoint<'_> {
        self.media()
    }

    /// Returns the medialist endpoint for user anime/manga list operations.
    #[inline]
    pub fn medialist(&self) -> MediaListEndpoint<'_> {
        MediaListEndpoint::new(self)
    }

    /// Returns the character endpoint for character operations.
    #[inline]
    pub fn character(&self) -> CharacterEndpoint<'_> {
        CharacterEndpoint::new(self)
    }

    /// Returns the common endpoint for likes, follows, and favorites.
    #[inline]
    pub fn common(&self) -> CommonEndpoint<'_> {
        CommonEndpoint::new(self)
    }

    /// Returns the staff endpoint for staff member operations.
    #[inline]
    pub fn staff(&self) -> StaffEndpoint<'_> {
        StaffEndpoint::new(self)
    }

    /// Returns the user endpoint for user profile operations.
    #[inline]
    pub fn user(&self) -> UserEndpoint<'_> {
        UserEndpoint::new(self)
    }

    /// Returns the studio endpoint for studio operations.
    #[inline]
    pub fn studio(&self) -> StudioEndpoint<'_> {
        StudioEndpoint::new(self)
    }

    /// Returns the forum endpoint for thread and comment operations.
    #[inline]
    pub fn forum(&self) -> ForumEndpoint<'_> {
        ForumEndpoint::new(self)
    }

    /// Returns the activity endpoint for activity feed operations.
    #[inline]
    pub fn activity(&self) -> ActivityEndpoint<'_> {
        ActivityEndpoint::new(self)
    }

    /// Returns the review endpoint for review operations.
    #[inline]
    pub fn review(&self) -> ReviewEndpoint<'_> {
        ReviewEndpoint::new(self)
    }

    /// Returns the recommendation endpoint for recommendation operations.
    #[inline]
    pub fn recommendation(&self) -> RecommendationEndpoint<'_> {
        RecommendationEndpoint::new(self)
    }

    /// Returns the airing endpoint for airing schedule operations.
    #[inline]
    pub fn airing(&self) -> AiringEndpoint<'_> {
        AiringEndpoint::new(self)
    }

    /// Returns the notification endpoint for notification operations.
    #[inline]
    pub fn notification(&self) -> NotificationEndpoint<'_> {
        NotificationEndpoint::new(self)
    }

    /// Sets the authentication token for this client.
    ///
    /// Note: This creates a new client with the updated token due to Arc sharing.
    pub fn set_token(&mut self, token: &str) {
        self.inner.token = Some(token.to_string());
    }

    /// Clears the authentication token from this client.
    ///
    /// Note: This creates a new client without the token due to Arc sharing.
    pub fn clear_token(&mut self) {
        self.inner.token = None;
    }

    /// Returns whether this client has an authentication token.
    #[inline]
    pub fn has_token(&self) -> bool {
        self.inner.token.is_some()
    }

    /// Returns the retry configuration for this client.
    #[inline]
    pub fn retry_config(&self) -> &RetryConfig {
        &self.inner.retry_config
    }

    pub async fn query<V: Serialize>(
        &self,
        query: &'static str,
        variables: Option<&V>,
    ) -> Result<Value, AniListError> {
        self.execute_query(query, variables).await
    }

    pub async fn fetch<T, V>(
        &self,
        query: &'static str,
        variables: Option<&V>,
    ) -> Result<T, AniListError>
    where
        T: serde::de::DeserializeOwned,
        V: Serialize,
    {
        let response_data = self.execute_query(query, variables).await?;
        let wrapper: GraphQLResponse<T> =
            from_value(response_data).map_err(|e| AniListError::ParseError {
                message: format!("Failed to deserialize response: {}", e),
            })?;
        Ok(wrapper.data)
    }

    async fn execute_query<V: Serialize>(
        &self,
        query: &'static str,
        variables: Option<&V>,
    ) -> Result<Value, AniListError> {
        retry_with_backoff(
            || async { self.raw_query(query, variables).await },
            &self.inner.retry_config,
        )
        .await
    }

    async fn raw_query<V: Serialize>(
        &self,
        query: &'static str,
        variables: Option<&V>,
    ) -> Result<Value, AniListError> {
        let body = RequestBody { query, variables };
        let serialized_body = serde_json::to_string(&body)?;

        let mut request = self
            .inner
            .client
            .post(self.inner.base_url.as_ref())
            .header("Content-Type", CONTENT_TYPE_JSON);

        if let Some(token) = &self.inner.token {
            // Preallocate the authorization header to avoid repeated allocations
            let mut auth_header = String::with_capacity(BEARER_PREFIX.len() + token.len());
            auth_header.push_str(BEARER_PREFIX);
            auth_header.push_str(token);
            request = request.header("Authorization", auth_header);
        }

        let response = smol::unblock(move || request.send(serialized_body)).await?;
        self.handle_response(response).await
    }

    async fn handle_response(&self, response: Response<Body>) -> Result<Value, AniListError> {
        let status = response.status();
        if status.is_success() {
            let json: Value = smol::unblock(move || response.into_body().read_json()).await?;
            self.handle_graphql_errors(json)
        } else {
            Err(self.handle_http_error(status, response).await)
        }
    }

    async fn handle_http_error(
        &self,
        status: StatusCode,
        response: Response<Body>,
    ) -> AniListError {
        match status.as_u16() {
            400 => AniListError::BadRequest {
                message: smol::unblock(move || response.into_body().read_to_string())
                    .await
                    .unwrap_or_else(|_| "Bad Request".to_string()),
            },
            401 => AniListError::AuthenticationRequired,
            403 => AniListError::AccessDenied,
            404 => AniListError::NotFound,
            429 => self.parse_rate_limit_error(response),
            500..=599 => AniListError::ServerError {
                status: status.as_u16(),
                message: smol::unblock(move || response.into_body().read_to_string())
                    .await
                    .unwrap_or_else(|_| "Server Error".to_string()),
            },
            _ => AniListError::ServerError {
                status: status.as_u16(),
                message: smol::unblock(move || response.into_body().read_to_string())
                    .await
                    .unwrap_or_else(|_| "Unknown Error".to_string()),
            },
        }
    }

    fn parse_rate_limit_error(&self, response: Response<Body>) -> AniListError {
        let headers = response.headers();
        let get_header = |key: &str| headers.get(key).and_then(|v| v.to_str().ok());

        if let (Some(limit), Some(remaining), Some(reset), Some(retry_after)) = (
            get_header("X-RateLimit-Limit").and_then(|s| s.parse().ok()),
            get_header("X-RateLimit-Remaining").and_then(|s| s.parse().ok()),
            get_header("X-RateLimit-Reset").and_then(|s| s.parse().ok()),
            get_header("Retry-After").and_then(|s| s.parse().ok()),
        ) {
            AniListError::RateLimit {
                limit,
                remaining,
                reset_at: reset,
                retry_after,
            }
        } else {
            AniListError::RateLimitSimple
        }
    }

    fn handle_graphql_errors(&self, json: Value) -> Result<Value, AniListError> {
        if let Some(errors) = json.get("errors") {
            let error_message = if let Some(arr) = errors.as_array() {
                // Preallocate capacity for the joined string
                let estimated_size: usize = arr
                    .iter()
                    .map(|e| {
                        e.get("message")
                            .and_then(|m| m.as_str())
                            .map(|s| s.len() + 2)
                            .unwrap_or(15)
                    })
                    .sum();

                let mut result = String::with_capacity(estimated_size);
                for (i, e) in arr.iter().enumerate() {
                    if i > 0 {
                        result.push_str(", ");
                    }
                    result.push_str(
                        e.get("message")
                            .and_then(|m| m.as_str())
                            .unwrap_or("Unknown error"),
                    );
                }
                result
            } else {
                errors.to_string()
            };

            // Use bytes comparison for case-insensitive check to avoid allocation
            let lower = error_message.to_lowercase();
            if lower.contains("rate limit") || lower.contains("too many requests") {
                return Err(AniListError::BurstLimit);
            }

            Err(AniListError::GraphQL {
                message: error_message,
            })
        } else {
            Ok(json)
        }
    }
}

/// Optimized request body structure that avoids HashMap allocation
#[derive(Serialize)]
struct RequestBody<'a, V: Serialize> {
    query: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    variables: Option<&'a V>,
}

impl Default for AniListClient {
    fn default() -> Self {
        Self::new()
    }
}
