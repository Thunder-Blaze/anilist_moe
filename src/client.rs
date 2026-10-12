use crate::endpoints::{
    ActivityEndpoint, AiringEndpoint, CharacterEndpoint, CommonEndpoint, ForumEndpoint,
    MediaEndpoint, MediaListEndpoint, NotificationEndpoint, RecommendationEndpoint, ReviewEndpoint,
    StaffEndpoint, StudioEndpoint, UserEndpoint,
};
use crate::errors::AniListError;
use crate::objects::responses::GraphQLResponse;
use crate::transport::{HttpRequest, HttpResponse, ReqwestTransport, Transport};
use crate::utils::{retry_with_backoff, RetryConfig};
use serde::Serialize;
use serde_json::{from_value, Value};
use std::borrow::Cow;
use std::sync::Arc;
use std::time::Duration;

use std::fmt;

/// The default AniList GraphQL API endpoint
const ANILIST_API_URL: &str = "https://graphql.anilist.co";

/// Content-Type header value (reused to avoid allocations)
const CONTENT_TYPE_JSON: &str = "application/json";

/// Authorization header prefix
const BEARER_PREFIX: &str = "Bearer ";

/// Internal shared state for the client
struct ClientInner {
    transport: Arc<dyn Transport>,
    token: Option<String>,
    retry_config: RetryConfig,
    base_url: Cow<'static, str>,
}

/// Builds an [`AniListClient`].
///
/// The constructors on [`AniListClient`] cover the common cases; reach for the
/// builder when the transport has to be configured differently — a longer
/// timeout, a different user agent, or a transport of your own.
///
/// # Examples
///
/// ```rust
/// use anilist_moe::AniListClient;
/// use std::time::Duration;
///
/// let client = AniListClient::builder()
///     .token("your_token_here")
///     .timeout(Duration::from_secs(60))
///     .build()?;
/// # Ok::<(), anilist_moe::AniListError>(())
/// ```
#[derive(Clone)]
pub struct AniListClientBuilder {
    token: Option<String>,
    base_url: Cow<'static, str>,
    retry_config: RetryConfig,
    timeout: Option<Duration>,
    user_agent: Option<String>,
    transport: Option<Arc<dyn Transport>>,
}

// Implement Debug manually to avoid exposing sensitive token information
impl fmt::Debug for AniListClientBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AniListClientBuilder")
            .field("base_url", &self.base_url)
            .field("has_token", &self.token.is_some())
            .field("retry_config", &self.retry_config)
            .field("timeout", &self.timeout)
            .field("user_agent", &self.user_agent)
            .field("has_custom_transport", &self.transport.is_some())
            .finish()
    }
}

impl AniListClientBuilder {
    /// A builder with the crate defaults: no token, the public API, and the
    /// default retry configuration.
    #[must_use]
    pub fn new() -> Self {
        Self {
            token: None,
            base_url: Cow::Borrowed(ANILIST_API_URL),
            retry_config: RetryConfig::default(),
            timeout: None,
            user_agent: None,
            transport: None,
        }
    }

    /// Reuses the settings of `inner`, keeping its transport.
    fn from_inner(inner: &Arc<ClientInner>) -> Self {
        Self {
            token: inner.token.clone(),
            base_url: inner.base_url.clone(),
            retry_config: inner.retry_config,
            timeout: None,
            user_agent: None,
            transport: Some(Arc::clone(&inner.transport)),
        }
    }

    /// Authenticates every request with `token`.
    #[must_use]
    pub fn token(mut self, token: impl Into<String>) -> Self {
        self.token = Some(token.into());
        self
    }

    /// Sends requests unauthenticated, dropping any token set earlier.
    #[must_use]
    pub fn without_token(mut self) -> Self {
        self.token = None;
        self
    }

    /// Sends requests to `base_url` instead of the public API.
    #[must_use]
    pub fn base_url(mut self, base_url: impl Into<String>) -> Self {
        self.base_url = Cow::Owned(base_url.into());
        self
    }

    /// Replaces the retry behavior.
    #[must_use]
    pub fn retry_config(mut self, config: RetryConfig) -> Self {
        self.retry_config = config;
        self
    }

    /// How long a request may take before it is given up on.
    ///
    /// Ignored when a `transport` is supplied.
    #[must_use]
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }

    /// The `User-Agent` requests are sent with.
    ///
    /// Ignored when a `transport` is supplied.
    #[must_use]
    pub fn user_agent(mut self, user_agent: impl Into<String>) -> Self {
        self.user_agent = Some(user_agent.into());
        self
    }

    /// Sends requests through `transport` instead of `reqwest`.
    ///
    /// Use this to route requests through your own HTTP stack, or to answer them
    /// from memory in tests.
    #[must_use]
    pub fn transport(mut self, transport: impl Transport + 'static) -> Self {
        self.transport = Some(Arc::new(transport));
        self
    }

    /// Builds the client.
    ///
    /// # Errors
    ///
    /// [`AniListError::Transport`] when no transport was supplied and the default
    /// `reqwest` client cannot be created.
    pub fn build(self) -> Result<AniListClient, AniListError> {
        let transport = match self.transport {
            Some(transport) => transport,
            None => Arc::new(ReqwestTransport::with_options(
                self.timeout,
                self.user_agent.as_deref(),
            )?),
        };

        Ok(AniListClient {
            inner: Arc::new(ClientInner {
                transport,
                token: self.token,
                retry_config: self.retry_config,
                base_url: self.base_url,
            }),
        })
    }
}

impl Default for AniListClientBuilder {
    fn default() -> Self {
        Self::new()
    }
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
#[derive(Clone)]
pub struct AniListClient {
    inner: Arc<ClientInner>,
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
    ///
    /// # Panics
    ///
    /// Panics when the default HTTP client cannot be built; use
    /// [`AniListClient::builder`] to handle that failure.
    #[must_use]
    pub fn new() -> Self {
        Self::builder()
            .build()
            .expect("Failed to build HTTP client")
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
    ///
    /// # Panics
    ///
    /// Panics when the default HTTP client cannot be built; use
    /// [`AniListClient::builder`] to handle that failure.
    #[must_use]
    pub fn with_token(token: impl Into<String>) -> Self {
        Self::builder()
            .token(token)
            .build()
            .expect("Failed to build HTTP client")
    }

    /// Starts building a client with settings of your own.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use anilist_moe::{AniListClient, transport::{HttpResponse, Transport}};
    /// use anilist_moe::AniListError;
    /// use std::future::Future;
    /// use std::pin::Pin;
    ///
    /// struct Offline;
    ///
    /// impl Transport for Offline {
    ///     fn execute(
    ///         &self,
    ///         _request: anilist_moe::transport::HttpRequest,
    ///     ) -> Pin<Box<dyn Future<Output = Result<HttpResponse, AniListError>> + Send + '_>> {
    ///         Box::pin(async { Ok(HttpResponse::new(200, "{\"data\":null}")) })
    ///     }
    /// }
    ///
    /// let client = AniListClient::builder().transport(Offline).build()?;
    /// # Ok::<(), AniListError>(())
    /// ```
    #[must_use]
    pub fn builder() -> AniListClientBuilder {
        AniListClientBuilder::new()
    }

    /// The transport this client sends requests through.
    #[must_use]
    pub fn transport(&self) -> &Arc<dyn Transport> {
        &self.inner.transport
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
        AniListClientBuilder::from_inner(&self.inner)
            .retry_config(config)
            .build()
            .expect("reusing the existing transport cannot fail")
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
        AniListClientBuilder::from_inner(&self.inner)
            .base_url(base_url)
            .build()
            .expect("reusing the existing transport cannot fail")
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
        *self = AniListClientBuilder::from_inner(&self.inner)
            .token(token)
            .build()
            .expect("reusing the existing transport cannot fail");
    }

    /// Clears the authentication token from this client.
    ///
    /// Note: This creates a new client without the token due to Arc sharing.
    pub fn clear_token(&mut self) {
        *self = AniListClientBuilder::from_inner(&self.inner)
            .without_token()
            .build()
            .expect("reusing the existing transport cannot fail");
    }

    /// Returns whether this client has an authentication token.
    #[inline]
    pub fn has_token(&self) -> bool {
        self.inner.token.is_some()
    }

    /// Returns the retry configuration for this client.
    #[inline]
    pub fn retry_config(&self) -> RetryConfig {
        self.inner.retry_config
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
            self.inner.retry_config,
        )
        .await
    }

    async fn raw_query<V: Serialize>(
        &self,
        query: &'static str,
        variables: Option<&V>,
    ) -> Result<Value, AniListError> {
        let body = serde_json::to_vec(&RequestBody { query, variables }).map_err(|error| {
            AniListError::ParseError {
                message: format!("Failed to serialize request: {error}"),
            }
        })?;

        let mut request = HttpRequest::new(self.inner.base_url.as_ref(), body)
            .header("Content-Type", CONTENT_TYPE_JSON);

        if let Some(token) = &self.inner.token {
            // Preallocate the authorization header to avoid repeated allocations
            let mut auth_header = String::with_capacity(BEARER_PREFIX.len() + token.len());
            auth_header.push_str(BEARER_PREFIX);
            auth_header.push_str(token);
            request = request.header("Authorization", auth_header);
        }

        let response = self.inner.transport.execute(request).await?;

        Self::interpret_response(response)
    }

    /// Turns an HTTP response into the GraphQL `data` value or an error.
    fn interpret_response(response: HttpResponse) -> Result<Value, AniListError> {
        if response.is_success() {
            let json = serde_json::from_slice::<Value>(&response.body)?;
            Self::handle_graphql_errors(json)
        } else {
            Err(Self::error_from_status(&response))
        }
    }

    /// Maps a non-2xx response onto an error, reading the rate limit headers.
    fn error_from_status(response: &HttpResponse) -> AniListError {
        match response.status {
            400 => AniListError::BadRequest {
                message: response.text().into_owned(),
            },
            401 => AniListError::AuthenticationRequired,
            403 => AniListError::AccessDenied,
            404 => AniListError::NotFound,
            429 => Self::parse_rate_limit_error(response),
            500..=599 => AniListError::ServerError {
                status: response.status,
                message: response.text().into_owned(),
            },
            status => AniListError::ServerError {
                status,
                message: response.text().into_owned(),
            },
        }
    }

    fn parse_rate_limit_error(response: &HttpResponse) -> AniListError {
        fn header<T: std::str::FromStr>(response: &HttpResponse, key: &str) -> Option<T> {
            response
                .header_value(key)
                .and_then(|value| value.parse().ok())
        }

        if let (Some(limit), Some(remaining), Some(reset), Some(retry_after)) = (
            header::<u32>(response, "X-RateLimit-Limit"),
            header::<u32>(response, "X-RateLimit-Remaining"),
            header::<u64>(response, "X-RateLimit-Reset"),
            header::<u32>(response, "Retry-After"),
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

    fn handle_graphql_errors(json: Value) -> Result<Value, AniListError> {
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
    query: &'static str,
    #[serde(skip_serializing_if = "Option::is_none")]
    variables: Option<&'a V>,
}

impl Default for AniListClient {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::transport::Transport;
    use parking_lot::Mutex;
    use std::future::Future;
    use std::pin::Pin;

    /// Answers with the queued responses and records what was sent.
    #[derive(Debug, Default)]
    struct RecordingTransport {
        responses: Mutex<Vec<HttpResponse>>,
        requests: Mutex<Vec<HttpRequest>>,
    }

    impl RecordingTransport {
        fn with_responses(responses: Vec<HttpResponse>) -> Arc<Self> {
            Arc::new(Self {
                responses: Mutex::new(responses),
                requests: Mutex::new(Vec::new()),
            })
        }

        fn requests(&self) -> Vec<HttpRequest> {
            self.requests.lock().clone()
        }
    }

    impl Transport for Arc<RecordingTransport> {
        fn execute(
            &self,
            request: HttpRequest,
        ) -> Pin<Box<dyn Future<Output = Result<HttpResponse, AniListError>> + Send + '_>> {
            Box::pin(async move {
                self.requests.lock().push(request);
                let mut responses = self.responses.lock();
                if responses.is_empty() {
                    return Err(AniListError::Transport {
                        message: "no queued response".to_string(),
                    });
                }
                Ok(responses.remove(0))
            })
        }
    }

    /// A retry configuration that never sleeps for long.
    fn fast_retries(max_retries: u32) -> RetryConfig {
        RetryConfig {
            max_retries,
            base_delay_ms: 1,
            exponential_backoff: false,
            max_delay_ms: 2,
        }
    }

    fn client_with(transport: Arc<RecordingTransport>) -> AniListClient {
        AniListClient::builder()
            .transport(transport)
            .retry_config(fast_retries(0))
            .build()
            .expect("a supplied transport cannot fail to build")
    }

    fn json_response(status: u16, body: &str) -> HttpResponse {
        HttpResponse::new(status, body.as_bytes().to_vec())
    }

    #[test]
    fn data_is_returned_from_a_successful_response() {
        let response = json_response(200, r#"{"data":{"Media":{"id":1}}}"#);
        let value = AniListClient::interpret_response(response).expect("data");
        assert_eq!(value["data"]["Media"]["id"], 1);
    }

    #[test]
    fn graphql_errors_are_joined_into_one_message() {
        let response = json_response(
            200,
            r#"{"errors":[{"message":"first"},{"message":"second"}]}"#,
        );
        let error = AniListClient::interpret_response(response).expect_err("must fail");
        assert!(
            matches!(&error, AniListError::GraphQL { message } if message == "first, second"),
            "unexpected error: {error:?}"
        );
    }

    #[test]
    fn a_rate_limit_message_becomes_a_burst_limit_error() {
        let response = json_response(200, r#"{"errors":[{"message":"Too Many Requests"}]}"#);
        assert!(matches!(
            AniListClient::interpret_response(response),
            Err(AniListError::BurstLimit)
        ));
    }

    #[test]
    fn malformed_json_on_a_successful_response_is_a_json_error() {
        let response = json_response(200, "not json");
        assert!(matches!(
            AniListClient::interpret_response(response),
            Err(AniListError::Json(_))
        ));
    }

    #[test]
    fn rate_limit_headers_are_read_off_a_429() {
        let response = json_response(429, "")
            .header("X-RateLimit-Limit", "90")
            .header("X-RateLimit-Remaining", "0")
            .header("X-RateLimit-Reset", "1700000000")
            .header("Retry-After", "42");
        let error = AniListClient::interpret_response(response).expect_err("must fail");
        match error {
            AniListError::RateLimit {
                limit,
                remaining,
                reset_at,
                retry_after,
            } => {
                assert_eq!(
                    (limit, remaining, reset_at, retry_after),
                    (90, 0, 1_700_000_000, 42)
                );
            }
            other => panic!("expected a detailed rate limit, got {other:?}"),
        }
    }

    #[test]
    fn a_429_without_headers_falls_back_to_the_simple_error() {
        let error = AniListClient::interpret_response(json_response(429, "slow down"))
            .expect_err("must fail");
        assert!(matches!(error, AniListError::RateLimitSimple));
    }

    #[test]
    fn status_codes_map_to_their_errors() {
        let cases = [
            (400, "bad"),
            (401, "unauthenticated"),
            (403, "forbidden"),
            (404, "missing"),
            (503, "unavailable"),
        ];

        for (status, body) in cases {
            let error = AniListClient::interpret_response(json_response(status, body))
                .expect_err("must fail");
            let expected = match status {
                400 => matches!(&error, AniListError::BadRequest { message } if message == body),
                401 => matches!(error, AniListError::AuthenticationRequired),
                403 => matches!(error, AniListError::AccessDenied),
                404 => matches!(error, AniListError::NotFound),
                _ => {
                    matches!(&error, AniListError::ServerError { status: s, message } if *s == status && message == body)
                }
            };
            assert!(expected, "status {status} mapped to {error:?}");
        }
    }

    #[tokio::test]
    async fn a_query_sends_the_body_token_and_content_type() {
        let transport =
            RecordingTransport::with_responses(vec![json_response(200, r#"{"data":{"ok":true}}"#)]);
        let client = AniListClient::builder()
            .transport(Arc::clone(&transport))
            .token("secret-token")
            .base_url("https://example.test/graphql")
            .retry_config(RetryConfig {
                max_retries: 0,
                base_delay_ms: 1,
                exponential_backoff: false,
                max_delay_ms: 1,
            })
            .build()
            .unwrap();

        let value = client
            .query("query { ok }", Some(&serde_json::json!({ "a": 1 })))
            .await
            .expect("query succeeds");
        assert_eq!(value["data"]["ok"], true);

        let requests = transport.requests();
        assert_eq!(requests.len(), 1);
        let sent = &requests[0];
        assert_eq!(sent.url, "https://example.test/graphql");
        assert_eq!(sent.header_value("Content-Type"), Some("application/json"));
        assert_eq!(
            sent.header_value("Authorization"),
            Some("Bearer secret-token")
        );

        let body: Value = serde_json::from_slice(&sent.body).expect("json body");
        assert_eq!(body["query"], "query { ok }");
        assert_eq!(body["variables"]["a"], 1);
    }

    #[tokio::test]
    async fn an_unauthenticated_query_sends_no_authorization_header() {
        let transport =
            RecordingTransport::with_responses(vec![json_response(200, r#"{"data":null}"#)]);
        let client = client_with(Arc::clone(&transport));

        client.query("query { ok }", None::<&()>).await.unwrap();

        assert_eq!(transport.requests()[0].header_value("Authorization"), None);
    }

    #[tokio::test]
    async fn a_rate_limited_request_is_retried_when_configured_to() {
        let transport = RecordingTransport::with_responses(vec![
            json_response(429, "").header("Retry-After", "0"),
            json_response(200, r#"{"data":{"ok":true}}"#),
        ]);
        let client = AniListClient::builder()
            .transport(Arc::clone(&transport))
            .retry_config(RetryConfig {
                max_retries: 1,
                base_delay_ms: 1,
                exponential_backoff: false,
                max_delay_ms: 1,
            })
            .build()
            .unwrap();

        let value = client.query("query { ok }", None::<&()>).await.unwrap();
        assert_eq!(value["data"]["ok"], true);
        assert_eq!(transport.requests().len(), 2, "the request is sent twice");
    }

    #[test]
    fn token_helpers_round_trip() {
        let mut client = AniListClient::new();
        assert!(!client.has_token());

        client.set_token("token");
        assert!(client.has_token());

        client.clear_token();
        assert!(!client.has_token());
    }

    #[tokio::test]
    async fn token_helpers_keep_the_transport() {
        let transport =
            RecordingTransport::with_responses(vec![json_response(200, r#"{"data":null}"#)]);
        let mut client = client_with(Arc::clone(&transport));

        client.set_token("token");
        client.clear_token();
        // The response queued on the original transport is still served, so the
        // token helpers rebuilt the client without replacing the transport.
        client.query("query { ok }", None::<&()>).await.unwrap();

        let requests = transport.requests();
        assert_eq!(requests.len(), 1);
        assert_eq!(requests[0].header_value("Authorization"), None);
    }

    #[tokio::test]
    async fn modifiers_keep_the_transport() {
        let transport =
            RecordingTransport::with_responses(vec![json_response(200, r#"{"data":null}"#)]);
        let client = client_with(Arc::clone(&transport))
            .with_base_url("https://example.test")
            .with_retry_config(fast_retries(2));

        assert_eq!(client.retry_config(), fast_retries(2));
        client.query("query { ok }", None::<&()>).await.unwrap();

        let requests = transport.requests();
        assert_eq!(requests.len(), 1);
        assert_eq!(requests[0].url, "https://example.test");
    }

    #[test]
    fn builder_settings_reach_the_client() {
        let client = AniListClient::builder()
            .token("t")
            .base_url("https://example.test")
            .retry_config(RetryConfig::aggressive())
            .build()
            .unwrap();

        assert!(client.has_token());
        assert_eq!(client.retry_config(), RetryConfig::aggressive());
    }

    #[test]
    fn debug_does_not_leak_the_token() {
        let client = AniListClient::builder()
            .token("super-secret")
            .build()
            .unwrap();
        let debug = format!("{client:?}");
        assert!(!debug.contains("super-secret"), "token leaked: {debug}");
        assert!(debug.contains("has_token: true"));
    }

    #[test]
    fn builder_debug_does_not_leak_the_token() {
        let builder = AniListClientBuilder::new().token("super-secret");
        let debug = format!("{builder:?}");
        assert!(!debug.contains("super-secret"), "token leaked: {debug}");
        assert!(debug.contains("has_token: true"));
    }
}
