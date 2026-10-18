#[cfg(feature = "activity")]
use crate::endpoints::ActivityEndpoint;
#[cfg(feature = "airing")]
use crate::endpoints::AiringEndpoint;
#[cfg(feature = "character")]
use crate::endpoints::CharacterEndpoint;
#[cfg(feature = "common")]
use crate::endpoints::CommonEndpoint;
#[cfg(feature = "forum")]
use crate::endpoints::ForumEndpoint;
#[cfg(feature = "media")]
use crate::endpoints::MediaEndpoint;
#[cfg(feature = "medialist")]
use crate::endpoints::MediaListEndpoint;
#[cfg(feature = "notification")]
use crate::endpoints::NotificationEndpoint;
#[cfg(feature = "recommendation")]
use crate::endpoints::RecommendationEndpoint;
#[cfg(feature = "review")]
use crate::endpoints::ReviewEndpoint;
#[cfg(feature = "staff")]
use crate::endpoints::StaffEndpoint;
#[cfg(feature = "studio")]
use crate::endpoints::StudioEndpoint;
#[cfg(feature = "user")]
use crate::endpoints::UserEndpoint;
use crate::errors::{detect_maintenance, format_messages, AniListError, GraphQLError};
use crate::objects::responses::GraphQLResponse;
use crate::transport::{HttpRequest, HttpResponse, ReqwestTransport, Transport};
use crate::utils::{retry_with_idempotency, Idempotency, RetryConfig};
use serde::Serialize;
use serde_json::{from_value, Value};
use std::borrow::Cow;
use std::sync::Arc;
use std::time::Duration;
#[cfg(feature = "tracing")]
use tracing::Instrument;

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
    headers: Vec<(String, String)>,
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
    headers: Vec<(String, String)>,
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
            .field(
                "headers",
                &self
                    .headers
                    .iter()
                    .map(|(name, _)| name.as_str())
                    .collect::<Vec<_>>(),
            )
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
            headers: Vec::new(),
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
            headers: inner.headers.clone(),
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

    /// Sends an extra header with every request, replacing one of the same name.
    ///
    /// Headers set here are handed to whichever transport is in use, so a custom
    /// transport sees them in [`HttpRequest::headers`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use anilist_moe::AniListClient;
    ///
    /// let client = AniListClient::builder()
    ///     .header("X-Client", "my-app")
    ///     .header("X-Proxy-Token", "secret")
    ///     .build()?;
    /// # Ok::<(), anilist_moe::AniListError>(())
    /// ```
    #[must_use]
    pub fn header(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        let name = name.into();
        let value = value.into();

        match self
            .headers
            .iter_mut()
            .find(|(existing, _)| existing.eq_ignore_ascii_case(&name))
        {
            Some((_, existing)) => *existing = value,
            None => self.headers.push((name, value)),
        }
        self
    }

    /// Sends several extra headers with every request.
    #[must_use]
    pub fn headers<I, N, V>(self, headers: I) -> Self
    where
        I: IntoIterator<Item = (N, V)>,
        N: Into<String>,
        V: Into<String>,
    {
        headers
            .into_iter()
            .fold(self, |builder, (name, value)| builder.header(name, value))
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
    /// [`AniListError::Network`] when no transport was supplied and the default
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
                headers: self.headers,
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
    /// #[derive(Debug)]
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

    /// Creates a client without authentication, reporting a failure instead of
    /// panicking.
    ///
    /// # Errors
    ///
    /// [`AniListError::Network`] when the default HTTP client cannot be built.
    /// [`AniListClient::new`] is the panicking shorthand.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use anilist_moe::AniListClient;
    ///
    /// let client = AniListClient::try_new()?;
    /// # Ok::<(), anilist_moe::AniListError>(())
    /// ```
    pub fn try_new() -> Result<Self, AniListError> {
        Self::builder().build()
    }

    /// Creates an authenticated client, reporting a failure instead of panicking.
    ///
    /// # Errors
    ///
    /// [`AniListError::Network`] when the default HTTP client cannot be built.
    pub fn try_with_token(token: impl Into<String>) -> Result<Self, AniListError> {
        Self::builder().token(token).build()
    }

    /// The transport this client sends requests through.
    #[must_use]
    pub fn transport(&self) -> &Arc<dyn Transport> {
        &self.inner.transport
    }

    /// Walks the pages of a list query, asking for one page at a time.
    ///
    /// Available with the `pagination` feature.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # #[cfg(feature = "pagination")]
    /// # async fn example() -> Result<(), anilist_moe::AniListError> {
    /// use anilist_moe::endpoints::media::FetchMediaOptions;
    /// use anilist_moe::enums::media::MediaType;
    /// use anilist_moe::AniListClient;
    ///
    /// let client = AniListClient::new();
    /// let options = FetchMediaOptions::builder()
    ///     .media_type(MediaType::Anime)
    ///     .per_page(50)
    ///     .build();
    /// let everything = client
    ///     .paginate(options)
    ///     .collect_all()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    #[cfg(feature = "pagination")]
    #[must_use]
    pub fn paginate<Q: crate::pagination::Paginated>(
        &self,
        options: Q,
    ) -> crate::pagination::Paginator<Q> {
        crate::pagination::Paginator::new(self.clone(), options)
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
    ///     ..RetryConfig::default()
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
    #[cfg(feature = "media")]
    #[inline]
    #[must_use]
    pub fn media(&self) -> MediaEndpoint<'_> {
        MediaEndpoint::new(self)
    }

    /// Returns the media endpoint for anime operations (alias for media()).
    #[cfg(feature = "media")]
    #[inline]
    #[must_use]
    pub fn anime(&self) -> MediaEndpoint<'_> {
        self.media()
    }

    /// Returns the media endpoint for manga operations (alias for media()).
    #[cfg(feature = "media")]
    #[inline]
    #[must_use]
    pub fn manga(&self) -> MediaEndpoint<'_> {
        self.media()
    }

    /// Returns the medialist endpoint for user anime/manga list operations.
    #[cfg(feature = "medialist")]
    #[inline]
    #[must_use]
    pub fn medialist(&self) -> MediaListEndpoint<'_> {
        MediaListEndpoint::new(self)
    }

    /// Returns the character endpoint for character operations.
    #[cfg(feature = "character")]
    #[inline]
    #[must_use]
    pub fn character(&self) -> CharacterEndpoint<'_> {
        CharacterEndpoint::new(self)
    }

    /// Returns the common endpoint for likes, follows, and favorites.
    #[cfg(feature = "common")]
    #[inline]
    #[must_use]
    pub fn common(&self) -> CommonEndpoint<'_> {
        CommonEndpoint::new(self)
    }

    /// Returns the staff endpoint for staff member operations.
    #[cfg(feature = "staff")]
    #[inline]
    #[must_use]
    pub fn staff(&self) -> StaffEndpoint<'_> {
        StaffEndpoint::new(self)
    }

    /// Returns the user endpoint for user profile operations.
    #[cfg(feature = "user")]
    #[inline]
    #[must_use]
    pub fn user(&self) -> UserEndpoint<'_> {
        UserEndpoint::new(self)
    }

    /// Returns the studio endpoint for studio operations.
    #[cfg(feature = "studio")]
    #[inline]
    #[must_use]
    pub fn studio(&self) -> StudioEndpoint<'_> {
        StudioEndpoint::new(self)
    }

    /// Returns the forum endpoint for thread and comment operations.
    #[cfg(feature = "forum")]
    #[inline]
    #[must_use]
    pub fn forum(&self) -> ForumEndpoint<'_> {
        ForumEndpoint::new(self)
    }

    /// Returns the activity endpoint for activity feed operations.
    #[cfg(feature = "activity")]
    #[inline]
    #[must_use]
    pub fn activity(&self) -> ActivityEndpoint<'_> {
        ActivityEndpoint::new(self)
    }

    /// Returns the review endpoint for review operations.
    #[cfg(feature = "review")]
    #[inline]
    #[must_use]
    pub fn review(&self) -> ReviewEndpoint<'_> {
        ReviewEndpoint::new(self)
    }

    /// Returns the recommendation endpoint for recommendation operations.
    #[cfg(feature = "recommendation")]
    #[inline]
    #[must_use]
    pub fn recommendation(&self) -> RecommendationEndpoint<'_> {
        RecommendationEndpoint::new(self)
    }

    /// Returns the airing endpoint for airing schedule operations.
    #[cfg(feature = "airing")]
    #[inline]
    #[must_use]
    pub fn airing(&self) -> AiringEndpoint<'_> {
        AiringEndpoint::new(self)
    }

    /// Returns the notification endpoint for notification operations.
    #[cfg(feature = "notification")]
    #[inline]
    #[must_use]
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

    /// Sends a GraphQL document and returns the whole response body as JSON.
    ///
    /// Use this for documents the typed endpoints do not cover. `query` can be
    /// built at runtime, for example a batch of aliased fields.
    ///
    /// A document that declares a `mutation` is retried only when the server
    /// provably did not run it (a rate limit, or a connection that was never
    /// made), so a retry cannot apply a write twice.
    ///
    /// # Errors
    ///
    /// Any [`AniListError`]: a GraphQL `errors` list in the response becomes
    /// [`AniListError::GraphQL`], with the partial `data` kept.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # async fn example() -> Result<(), anilist_moe::AniListError> {
    /// use anilist_moe::AniListClient;
    ///
    /// let client = AniListClient::new();
    /// let ids = [1, 20];
    /// let document = format!(
    ///     "query {{ {} }}",
    ///     ids.iter()
    ///         .map(|id| format!("m{id}: Media(id: {id}) {{ id title {{ romaji }} }}"))
    ///         .collect::<Vec<_>>()
    ///         .join(" ")
    /// );
    /// let body = client.query(&document, None::<&()>).await?;
    /// println!("{}", body["data"]["m1"]["title"]["romaji"]);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn query<V: Serialize>(
        &self,
        query: &str,
        variables: Option<&V>,
    ) -> Result<Value, AniListError> {
        self.execute_query(query, variables).await
    }

    /// Sends a GraphQL document and deserialises the single root field of its
    /// `data` into `T`.
    ///
    /// This is what every typed endpoint uses. The root field's name is not
    /// checked, so `T` describes its value: `{"data": {"Media": {...}}}`
    /// deserialises the `{...}`. Use [`AniListClient::query`] when a document has
    /// several root fields.
    ///
    /// # Errors
    ///
    /// As [`AniListClient::query`], plus [`AniListError::ParseError`] when the
    /// data does not match `T`.
    pub async fn fetch<T, V>(&self, query: &str, variables: Option<&V>) -> Result<T, AniListError>
    where
        T: serde::de::DeserializeOwned,
        V: Serialize,
    {
        let response_data = self.execute_query(query, variables).await?;
        let wrapper: GraphQLResponse<T> =
            from_value(response_data).map_err(|e| AniListError::ParseError {
                message: format!("Failed to deserialize response: {e}"),
                source: Some(Box::new(e)),
            })?;
        Ok(wrapper.data)
    }

    async fn execute_query<V: Serialize>(
        &self,
        query: &str,
        variables: Option<&V>,
    ) -> Result<Value, AniListError> {
        let idempotency = if is_mutation(query) {
            Idempotency::Unsafe
        } else {
            Idempotency::Safe
        };
        let request = retry_with_idempotency(
            || async { self.raw_query(query, variables).await },
            self.inner.retry_config,
            idempotency,
        );

        // A span covering the whole request, including its retries. The
        // declared operation and the type of the variables are carried as
        // fields, so a subscriber sees one span per logical query and names the
        // options struct that was sent. Neither helper allocates.
        #[cfg(feature = "tracing")]
        let request = request.instrument(tracing::debug_span!(
            target: "anilist_moe::client",
            "query",
            operation = operation_name(query),
            variables = variables_type::<V>(),
            url = %self.inner.base_url,
        ));

        request.await
    }

    async fn raw_query<V: Serialize>(
        &self,
        query: &str,
        variables: Option<&V>,
    ) -> Result<Value, AniListError> {
        let body = serde_json::to_vec(&RequestBody { query, variables }).map_err(|error| {
            AniListError::ParseError {
                message: format!("Failed to serialize request: {error}"),
                source: Some(Box::new(error)),
            }
        })?;

        let mut request = HttpRequest::new(self.inner.base_url.as_ref(), body)
            .header("Content-Type", CONTENT_TYPE_JSON);

        for (name, value) in &self.inner.headers {
            request = request.header(name, value);
        }

        if let Some(token) = &self.inner.token {
            // Preallocate the authorization header to avoid repeated allocations
            let mut auth_header = String::with_capacity(BEARER_PREFIX.len() + token.len());
            auth_header.push_str(BEARER_PREFIX);
            auth_header.push_str(token);
            request = request.header("Authorization", auth_header);
        }

        // The token and the Authorization header value are never logged; only
        // whether the request was sent to which URL. The attempt number is
        // carried by the enclosing `attempt` span.
        // The body carries the query and the variables, so it is logged at the
        // trace level only, where a subscriber has to ask for it.
        #[cfg(feature = "tracing")]
        tracing::trace!(
            target: "anilist_moe::client",
            body = %String::from_utf8_lossy(&request.body),
            "request body",
        );

        #[cfg(feature = "tracing")]
        tracing::debug!(
            target: "anilist_moe::client",
            url = %self.inner.base_url,
            authenticated = self.inner.token.is_some(),
            body_bytes = request.body.len(),
            "sending request",
        );

        #[cfg(feature = "tracing")]
        let started = std::time::Instant::now();
        let response = self.inner.transport.execute(request).await?;
        #[cfg(feature = "tracing")]
        tracing::debug!(
            target: "anilist_moe::client",
            status = response.status,
            elapsed_ms = started.elapsed().as_millis() as u64,
            "received response",
        );

        let op = operation_name(query);
        Self::interpret_response(response).map_err(|err| err.with_operation(op))
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
        // The rate limit headers are authoritative, so they win over the body.
        if response.status == 429 {
            return Self::parse_rate_limit_error(response);
        }

        let body = response.text();

        // AniList reports an outage as a 503, and sometimes only in the body.
        if response.status == 503 || detect_maintenance(&body) {
            return AniListError::Maintenance {
                message: body.into_owned(),
            };
        }

        match response.status {
            // A malformed query comes back as 400 with a GraphQL `errors` array,
            // so keep the structure instead of handing over raw JSON.
            400 => match serde_json::from_str::<Value>(&body) {
                Ok(json) if json.get("errors").is_some() => Self::handle_graphql_errors(json)
                    .err()
                    .unwrap_or_else(|| AniListError::BadRequest {
                        message: body.into_owned(),
                    }),
                _ => AniListError::BadRequest {
                    message: body.into_owned(),
                },
            },
            401 => match serde_json::from_str::<Value>(&body) {
                Ok(json) if json.get("errors").is_some() => Self::handle_graphql_errors(json)
                    .err()
                    .unwrap_or(AniListError::AuthenticationRequired),
                _ => AniListError::AuthenticationRequired,
            },
            403 => match serde_json::from_str::<Value>(&body) {
                Ok(json) if json.get("errors").is_some() => Self::handle_graphql_errors(json)
                    .err()
                    .unwrap_or(AniListError::AccessDenied),
                _ => AniListError::AccessDenied,
            },
            404 => match serde_json::from_str::<Value>(&body) {
                Ok(json) if json.get("errors").is_some() => Self::handle_graphql_errors(json)
                    .err()
                    .unwrap_or(AniListError::NotFound),
                _ => AniListError::NotFound,
            },
            500..=599 => AniListError::ServerError {
                status: response.status,
                message: body.into_owned(),
            },
            status => AniListError::UnexpectedStatus {
                status,
                message: body.into_owned(),
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
        let Some(errors) = json.get("errors") else {
            return Ok(json);
        };

        let parsed = match serde_json::from_value::<Vec<GraphQLError>>(errors.clone()) {
            Ok(parsed) if !parsed.is_empty() => parsed,
            // The server sent something that is not a well formed error list;
            // keep whatever it said rather than losing it.
            _ => vec![GraphQLError::new(describe_errors(errors))],
        };

        // A burst of traffic is reported as a GraphQL error rather than a 429.
        if parsed.iter().any(|error| {
            let message = error.message.to_lowercase();
            message.contains("rate limit") || message.contains("too many requests")
        }) {
            return Err(AniListError::BurstLimit);
        }

        // An outage can arrive as a 200 with the notice in the errors.
        if parsed
            .iter()
            .any(|error| detect_maintenance(&error.message))
        {
            return Err(AniListError::Maintenance {
                message: format_messages(&parsed),
            });
        }

        // A partial answer carries data for the fields that did resolve.
        let data = json.get("data").filter(|value| !value.is_null()).cloned();

        Err(AniListError::GraphQL {
            operation: None,
            errors: parsed,
            data,
        })
    }
}

/// The name of the options struct the variables were built from, for tracing.
///
/// `std::any::type_name` renders the full path with its parameters, so
/// `anilist_moe::endpoints::media::FetchMediaOptions<'_>` is reported as
/// `FetchMediaOptions`. It borrows a compile-time constant and allocates
/// nothing.
#[cfg(feature = "tracing")]
fn variables_type<V>() -> &'static str {
    let full = std::any::type_name::<V>();
    let short = full.rsplit("::").next().unwrap_or(full);
    short.split('<').next().unwrap_or(short)
}

/// The operation name a GraphQL document declares, for tracing.
///
/// Returns the identifier that follows a `query`, `mutation`, or `subscription`
/// keyword, or `"query"` for an anonymous document such as `{ viewer { id } }`.
/// The result borrows from `query`, so this allocates nothing and is cheap
/// enough to call on every request.
fn operation_name(query: &str) -> &str {
    let trimmed = query.trim_start();

    // Skip a leading operation keyword, if the document has one. The keyword
    // only counts when a name, a variable list, or a selection set follows.
    let body = ["query", "mutation", "subscription"]
        .into_iter()
        .find_map(|keyword| {
            trimmed.strip_prefix(keyword).filter(|rest| {
                rest.starts_with(|c: char| c.is_whitespace() || c == '(' || c == '{')
            })
        })
        .unwrap_or(trimmed);

    // The first word after the keyword, if any; the name ends at the first
    // space, `(`, or `{`.
    let name = body
        .trim_start()
        .split(|c: char| c.is_whitespace() || c == '(' || c == '{')
        .next()
        .unwrap_or("");

    if name.is_empty() {
        "query"
    } else {
        name
    }
}

/// Renders an `errors` value that is not an array of error objects.
fn describe_errors(errors: &Value) -> String {
    match errors.as_array() {
        Some(entries) => entries
            .iter()
            .map(|entry| {
                entry
                    .get("message")
                    .and_then(|message| message.as_str())
                    .unwrap_or("Unknown error")
                    .to_string()
            })
            .collect::<Vec<_>>()
            .join(", "),
        None => errors.to_string(),
    }
}

/// Whether a GraphQL document declares a `mutation` operation.
///
/// Scans the top level of the document — outside every selection set, string
/// and comment — for the `mutation` keyword, so a fragment defined first, a
/// leading comment, or a field that happens to be named `mutation` cannot
/// mislead it. Any mutation in the document counts, which errs on the side of
/// not repeating a write.
fn is_mutation(document: &str) -> bool {
    let bytes = document.as_bytes();
    let mut depth = 0usize;
    let mut index = 0;

    while index < bytes.len() {
        match bytes[index] {
            b'#' => {
                while index < bytes.len() && bytes[index] != b'\n' {
                    index += 1;
                }
            }
            b'"' => {
                let block = bytes[index..].starts_with(b"\"\"\"");
                index += if block { 3 } else { 1 };
                while index < bytes.len() {
                    if block && bytes[index..].starts_with(b"\"\"\"") {
                        index += 2;
                        break;
                    }
                    if !block && bytes[index] == b'\\' {
                        index += 1;
                    } else if !block && bytes[index] == b'"' {
                        break;
                    }
                    index += 1;
                }
            }
            // `$mutation` and `@mutation` are a variable and a directive, not the keyword.
            b'$' | b'@' => {
                index += 1;
                while index < bytes.len()
                    && (bytes[index].is_ascii_alphanumeric() || bytes[index] == b'_')
                {
                    index += 1;
                }
                continue;
            }
            b'{' => depth += 1,
            b'}' => depth = depth.saturating_sub(1),
            byte if depth == 0 && (byte.is_ascii_alphabetic() || byte == b'_') => {
                let start = index;
                while index < bytes.len()
                    && (bytes[index].is_ascii_alphanumeric() || bytes[index] == b'_')
                {
                    index += 1;
                }
                if &bytes[start..index] == b"mutation" {
                    return true;
                }
                continue;
            }
            _ => {}
        }
        index += 1;
    }

    false
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
                        source: None,
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
            jitter: false,
            retry_transient_errors: true,
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
    fn graphql_errors_keep_their_structure() {
        let response = json_response(
            200,
            r#"{
                "errors": [
                    {
                        "message": "Cannot query field \"nope\" on type \"Media\".",
                        "path": ["Media", "nope"],
                        "locations": [{"line": 3, "column": 5}],
                        "extensions": {"code": "GRAPHQL_VALIDATION_FAILED"}
                    },
                    {"message": "second"}
                ],
                "data": null
            }"#,
        );

        let error = AniListClient::interpret_response(response).expect_err("must fail");
        let errors = error.graphql_errors().expect("structured errors");
        assert_eq!(errors.len(), 2, "every error is kept");
        assert_eq!(errors[0].path.as_ref().unwrap().len(), 2);
        assert_eq!(
            errors[0].locations.as_ref().unwrap()[0],
            crate::errors::GraphQLLocation::new(3, 5)
        );
        assert_eq!(
            errors[0].extensions.as_ref().unwrap()["code"],
            "GRAPHQL_VALIDATION_FAILED"
        );
        assert_eq!(errors[1].message(), "second");
        assert!(
            error.to_string().contains("Cannot query field")
                && error.to_string().contains("second"),
            "the display string still carries every message: {error}"
        );
    }

    #[test]
    fn a_partial_answer_keeps_the_data_that_resolved() {
        let response = json_response(
            200,
            r#"{"data":{"Media":{"id":1,"title":null}},"errors":[{"message":"title failed"}]}"#,
        );

        let error = AniListClient::interpret_response(response).expect_err("must fail");
        match error {
            AniListError::GraphQL { errors, data, .. } => {
                assert_eq!(errors[0].message(), "title failed");
                assert_eq!(data.expect("partial data")["Media"]["id"], 1);
            }
            other => panic!("expected a GraphQL error, got {other:?}"),
        }
    }

    #[test]
    fn an_errors_value_that_is_not_a_list_is_still_reported() {
        let response = json_response(200, r#"{"errors":[{"nope":true}]}"#);

        let error = AniListClient::interpret_response(response).expect_err("must fail");
        assert_eq!(
            error.graphql_errors().unwrap()[0].message(),
            "Unknown error",
            "a missing message field must not lose the error"
        );

        let odd = AniListClient::interpret_response(json_response(200, r#"{"errors":"boom"}"#))
            .expect_err("must fail");
        assert_eq!(odd.graphql_errors().unwrap()[0].message(), "\"boom\"");
    }

    #[test]
    fn graphql_message_lookup_is_case_insensitive() {
        let error = AniListError::GraphQL {
            operation: None,
            errors: vec![GraphQLError::new("Cannot query field \"nope\".")],
            data: None,
        };
        assert!(error.graphql_message_contains("CANNOT"));
        assert!(!error.graphql_message_contains("something else"));
    }

    #[test]
    fn a_400_with_graphql_errors_keeps_the_structure() {
        let response = json_response(
            400,
            r#"{"errors":[{"message":"Cannot query field \"nope\" on type \"Media\".","locations":[{"line":2,"column":3}]}]}"#,
        );

        let error = AniListClient::interpret_response(response).expect_err("must fail");
        let errors = error.graphql_errors().expect("structured errors");
        assert_eq!(errors[0].locations.as_ref().unwrap()[0].line, 2);
        assert!(
            error.graphql_message_contains("cannot query field"),
            "a validation error should not arrive as a raw body: {error}"
        );
    }

    #[test]
    fn a_400_without_graphql_errors_stays_a_bad_request() {
        let error =
            AniListClient::interpret_response(json_response(400, "plain text")).expect_err("fails");
        assert!(matches!(&error, AniListError::BadRequest { message } if message == "plain text"));
    }

    #[test]
    fn an_outage_is_reported_as_maintenance() {
        let down = AniListClient::interpret_response(json_response(503, "service unavailable"))
            .expect_err("must fail");
        assert!(
            matches!(down, AniListError::Maintenance { .. }),
            "got {down:?}"
        );

        // The notice can also arrive in a 200 with the errors.
        let notice = AniListClient::interpret_response(json_response(
            200,
            r#"{"errors":[{"message":"AniList is down due to maintenance"}]}"#,
        ))
        .expect_err("must fail");
        assert!(
            matches!(notice, AniListError::Maintenance { .. }),
            "got {notice:?}"
        );

        // A 5xx that is not an outage stays a server error.
        let other =
            AniListClient::interpret_response(json_response(500, "boom")).expect_err("must fail");
        assert!(matches!(
            other,
            AniListError::ServerError { status: 500, .. }
        ));
    }

    #[test]
    fn an_unmapped_status_says_so() {
        let error =
            AniListClient::interpret_response(json_response(418, "teapot")).expect_err("must fail");
        assert!(
            matches!(&error, AniListError::UnexpectedStatus { status: 418, message } if message == "teapot"),
            "got {error:?}"
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
            (500, "boom"),
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
                // A 503 reads as an outage; other 5xx stay server errors.
                503 => matches!(&error, AniListError::Maintenance { message } if message == body),
                _ => {
                    matches!(&error, AniListError::ServerError { status: s, message } if *s == status && message == body)
                }
            };
            assert!(expected, "status {status} mapped to {error:?}");
        }
    }
    #[test]
    fn auth_and_not_found_errors_preserve_graphql_errors() {
        let cases = [
            (401, "unauthorized with detail"),
            (403, "forbidden with detail"),
            (404, "Not Found."),
        ];

        for (status, message) in cases {
            let body = format!(r#"{{"errors":[{{"message":"{message}"}}]}}"#);
            let error = AniListClient::interpret_response(json_response(status, &body))
                .expect_err("must fail");
            assert!(
                error.graphql_message_contains(message),
                "status {status} should preserve GraphQL error message '{message}', got: {error:?}"
            );
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
            .retry_config(fast_retries(0))
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
            .retry_config(fast_retries(1))
            .build()
            .unwrap();

        let value = client.query("query { ok }", None::<&()>).await.unwrap();
        assert_eq!(value["data"]["ok"], true);
        assert_eq!(transport.requests().len(), 2, "the request is sent twice");
    }

    #[tokio::test]
    async fn a_rate_limited_request_is_not_retried_by_default() {
        let transport = RecordingTransport::with_responses(vec![
            json_response(429, "").header("Retry-After", "0"),
            json_response(200, r#"{"data":{"ok":true}}"#),
        ]);
        let client = AniListClient::builder()
            .transport(Arc::clone(&transport))
            .build()
            .unwrap();

        let error = client
            .query("query { ok }", None::<&()>)
            .await
            .expect_err("the 429 is handed back to the caller");

        assert!(matches!(error, AniListError::RateLimitSimple));
        assert_eq!(
            transport.requests().len(),
            1,
            "the default configuration does not retry"
        );
    }

    #[tokio::test]
    async fn builder_headers_reach_the_request() {
        let transport =
            RecordingTransport::with_responses(vec![json_response(200, r#"{"data":null}"#)]);
        let client = AniListClient::builder()
            .transport(Arc::clone(&transport))
            .header("X-Client", "my-app")
            .headers([("X-Trace", "abc"), ("x-client", "replaced")])
            .retry_config(fast_retries(0))
            .build()
            .unwrap();

        client.query("query { ok }", None::<&()>).await.unwrap();

        let request = &transport.requests()[0];
        assert_eq!(request.header_value("X-Client"), Some("replaced"));
        assert_eq!(request.header_value("x-trace"), Some("abc"));
        assert_eq!(
            request.header_value("Content-Type"),
            Some("application/json")
        );
    }

    #[test]
    fn builder_debug_lists_header_names_without_values() {
        let builder = AniListClientBuilder::new().header("X-Proxy-Token", "super-secret");
        let debug = format!("{builder:?}");

        assert!(
            debug.contains("X-Proxy-Token"),
            "the name is useful: {debug}"
        );
        assert!(
            !debug.contains("super-secret"),
            "the value must not leak: {debug}"
        );
    }

    #[test]
    fn the_fallible_constructors_work() {
        assert!(AniListClient::try_new().is_ok());
        assert!(AniListClient::try_with_token("token").is_ok());
        assert!(AniListClient::try_with_token("token").unwrap().has_token());
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

    #[test]
    fn mutations_are_recognised_at_the_top_level_only() {
        assert!(is_mutation(
            "mutation ToggleLike($id: Int) { ToggleLikeV2(id: $id) { __typename } }"
        ));
        assert!(is_mutation(
            "\n  # posts a status\n  mutation { SaveTextActivity(text: \"x\") { id } }"
        ));
        assert!(
            is_mutation("fragment F on Media { id }\nmutation M { SaveMediaListEntry(mediaId: 1) { ...F } }"),
            "a fragment defined before the operation must not hide it"
        );

        assert!(!is_mutation("query { Media(id: 1) { id } }"));
        assert!(!is_mutation("{ viewer { id } }"));
        assert!(
            !is_mutation("query { Page { mutation: media { id } } }"),
            "a field or alias named mutation is not an operation"
        );
        assert!(
            !is_mutation("query Q($mutation: Int) @mutation { Media(id: $mutation) { id } }"),
            "variables and directives are not the keyword"
        );
        assert!(
            !is_mutation("query { Media(search: \"mutation { x }\") { id } }"),
            "string contents are not code"
        );
        assert!(!is_mutation(
            "query { Media(search: \"\"\"\nmutation\n\"\"\") { id } }"
        ));
        assert!(!is_mutation(
            "# mutation in a comment\nquery { Media { id } }"
        ));
        assert!(!is_mutation(""));
    }

    #[test]
    fn every_bundled_mutation_is_recognised() {
        #[cfg(feature = "common")]
        assert!(is_mutation(crate::queries::common::TOGGLE_LIKE));
        #[cfg(feature = "activity")]
        assert!(is_mutation(crate::queries::activity::SAVE_TEXT_ACTIVITY));
        #[cfg(feature = "medialist")]
        assert!(is_mutation(crate::queries::medialist::SAVE));
        #[cfg(feature = "media")]
        assert!(!is_mutation(crate::queries::media::FETCH));
    }

    /// Every `.graphql` file under `src/queries`, recursively.
    fn bundled_documents() -> Vec<std::path::PathBuf> {
        fn walk(dir: &std::path::Path, found: &mut Vec<std::path::PathBuf>) {
            for entry in std::fs::read_dir(dir).expect("the queries directory is readable") {
                let path = entry.expect("a directory entry").path();
                if path.is_dir() {
                    walk(&path, found);
                } else if path.extension().is_some_and(|ext| ext == "graphql") {
                    found.push(path);
                }
            }
        }

        let mut found = Vec::new();
        walk(
            std::path::Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/src/queries")),
            &mut found,
        );
        found.sort();
        found
    }

    #[test]
    fn every_bundled_document_parses_and_is_classified_correctly() {
        use graphql_parser::query::{Definition, OperationDefinition};

        let documents = bundled_documents();
        assert!(documents.len() > 60, "found {} documents", documents.len());

        for path in documents {
            let text = std::fs::read_to_string(&path).expect("a readable document");
            let parsed = graphql_parser::parse_query::<String>(&text)
                .unwrap_or_else(|error| panic!("{} does not parse: {error}", path.display()));

            let declares_mutation = parsed.definitions.iter().any(|definition| {
                matches!(
                    definition,
                    Definition::Operation(OperationDefinition::Mutation(_))
                )
            });
            assert_eq!(
                is_mutation(&text),
                declares_mutation,
                "{} is classified wrongly, so its retries would be wrong",
                path.display()
            );
        }
    }

    /// The variables each document requires (`!` with no default), by path below
    /// `src/queries`.
    ///
    /// AniList rejects a request that leaves a required variable out, and an
    /// options field that is an `Option` can be left out. So a variable belongs
    /// here only when its options field is required too (a plain type with
    /// `#[builder(!default)]`). `$id: Int!` on `GetUserById` broke
    /// `get_by_name` for exactly this reason: AniList itself accepts the name alone.
    const REQUIRED_VARIABLES: &[(&str, &[&str])] = &[
        ("activity/delete.graphql", &["id"]),
        ("activity/delete_reply.graphql", &["id"]),
        ("activity/pin.graphql", &["id", "pinned"]),
        (
            "activity/save_message_activity.graphql",
            &["message", "recipientId"],
        ),
        ("activity/save_reply.graphql", &["activityId", "text"]),
        ("activity/save_text_activity.graphql", &["text"]),
        ("activity/subscribe.graphql", &["activityId", "subscribe"]),
        ("common/markdown.graphql", &["markdown"]),
        ("common/toggle_follow.graphql", &["userId"]),
        ("forum/delete.graphql", &["id"]),
        ("forum/delete_comment.graphql", &["id"]),
        ("forum/subscription.graphql", &["threadId", "subscribe"]),
        ("medialist/delete.graphql", &["id"]),
        ("medialist/save_multiple.graphql", &["ids"]),
        (
            "recommendation/save.graphql",
            &["mediaId", "mediaRecommendationId", "rating"],
        ),
        ("review/delete.graphql", &["id"]),
        ("review/rate.graphql", &["reviewId", "rating"]),
        ("review/save.graphql", &["mediaId"]),
        ("user/favorites.graphql", &["userId"]),
        ("user/followers.graphql", &["userId"]),
        ("user/following.graphql", &["userId"]),
        ("user/stats.graphql", &["userId"]),
    ];

    #[test]
    fn required_variables_are_exactly_the_reviewed_ones() {
        use graphql_parser::query::{Definition, OperationDefinition, Type};

        let root = std::path::Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/src/queries"));
        let mut found: Vec<(String, Vec<String>)> = Vec::new();

        for path in bundled_documents() {
            let text = std::fs::read_to_string(&path).expect("a readable document");
            let parsed = graphql_parser::parse_query::<String>(&text).expect("parses");
            let mut required = Vec::new();
            for definition in &parsed.definitions {
                let variables = match definition {
                    Definition::Operation(OperationDefinition::Query(query)) => {
                        &query.variable_definitions
                    }
                    Definition::Operation(OperationDefinition::Mutation(mutation)) => {
                        &mutation.variable_definitions
                    }
                    _ => continue,
                };
                required.extend(
                    variables
                        .iter()
                        .filter(|variable| {
                            matches!(variable.var_type, Type::NonNullType(_))
                                && variable.default_value.is_none()
                        })
                        .map(|variable| variable.name.clone()),
                );
            }
            if !required.is_empty() {
                let relative = path
                    .strip_prefix(root)
                    .expect("below src/queries")
                    .to_string_lossy()
                    .replace('\\', "/");
                found.push((relative, required));
            }
        }

        let expected: Vec<(String, Vec<String>)> = REQUIRED_VARIABLES
            .iter()
            .map(|(path, names)| {
                (
                    (*path).to_string(),
                    names.iter().map(|name| (*name).to_string()).collect(),
                )
            })
            .collect();
        assert_eq!(
            found, expected,
            "a required variable changed; check that its options field is required too"
        );
    }

    #[tokio::test]
    async fn a_mutation_is_not_resent_after_a_server_error() {
        let transport = RecordingTransport::with_responses(vec![
            json_response(500, "boom"),
            json_response(200, r#"{"data":{"ToggleLikeV2":{"id":1}}}"#),
        ]);
        let client = AniListClient::builder()
            .transport(Arc::clone(&transport))
            .retry_config(fast_retries(3))
            .build()
            .unwrap();

        let error = client
            .query(
                "mutation { ToggleLikeV2(id: 1, type: ACTIVITY) { __typename } }",
                None::<&()>,
            )
            .await
            .expect_err("the server error is handed back");

        assert!(matches!(
            error,
            AniListError::ServerError { status: 500, .. }
        ));
        assert_eq!(
            transport.requests().len(),
            1,
            "a second toggle would undo the first"
        );
    }

    #[tokio::test]
    async fn a_rate_limited_mutation_is_retried() {
        let transport = RecordingTransport::with_responses(vec![
            json_response(429, ""),
            json_response(200, r#"{"data":{"SaveTextActivity":{"id":7}}}"#),
        ]);
        let client = AniListClient::builder()
            .transport(Arc::clone(&transport))
            .retry_config(fast_retries(1))
            .build()
            .unwrap();

        let value = client
            .query(
                "mutation { SaveTextActivity(text: \"hi\") { id } }",
                None::<&()>,
            )
            .await
            .expect("the refused write is sent again");
        assert_eq!(value["data"]["SaveTextActivity"]["id"], 7);
        assert_eq!(transport.requests().len(), 2);
    }

    #[tokio::test]
    async fn a_query_is_retried_after_an_outage() {
        let transport = RecordingTransport::with_responses(vec![
            json_response(503, "service unavailable"),
            json_response(200, r#"{"data":{"ok":true}}"#),
        ]);
        let client = AniListClient::builder()
            .transport(Arc::clone(&transport))
            .retry_config(fast_retries(1))
            .build()
            .unwrap();

        let value = client.query("query { ok }", None::<&()>).await.unwrap();
        assert_eq!(value["data"]["ok"], true);
        assert_eq!(transport.requests().len(), 2);
    }

    #[tokio::test]
    async fn a_document_built_at_runtime_can_be_sent() {
        let transport = RecordingTransport::with_responses(vec![json_response(
            200,
            r#"{"data":{"m1":{"id":1}}}"#,
        )]);
        let client = client_with(Arc::clone(&transport));

        let document = format!("query {{ m{0}: Media(id: {0}) {{ id }} }}", 1);
        let value = client.query(&document, None::<&()>).await.unwrap();
        assert_eq!(value["data"]["m1"]["id"], 1);
    }

    #[cfg(feature = "tracing")]
    #[test]
    fn the_variables_type_is_reported_by_its_short_name() {
        use crate::endpoints::media::FetchMediaOptions;

        assert_eq!(
            variables_type::<FetchMediaOptions<'_>>(),
            "FetchMediaOptions"
        );
        assert_eq!(variables_type::<()>(), "()");
    }

    #[cfg(feature = "tracing")]
    #[test]
    fn operation_names_are_read_from_the_document() {
        assert_eq!(operation_name("query GetUser { viewer { id } }"), "GetUser");
        assert_eq!(
            operation_name("mutation CreatePost($x: Int) { x }"),
            "CreatePost"
        );
        assert_eq!(operation_name("subscription OnUpdate { x }"), "OnUpdate");
        // Anonymous operations have no name to show.
        assert_eq!(operation_name("{ viewer { id } }"), "query");
        assert_eq!(operation_name("query { viewer { id } }"), "query");
        assert_eq!(operation_name("query($id: Int!) { x }"), "query");
        // Leading whitespace and newlines are ignored.
        assert_eq!(operation_name("\n  mutation DoThing { x }"), "DoThing");
        assert_eq!(operation_name(""), "query");
    }
}
