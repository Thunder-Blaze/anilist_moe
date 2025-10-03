use crate::endpoints::{
    ActivityEndpoint, AiringEndpoint, CharacterEndpoint, CommonEndpoint, ForumEndpoint,
    MediaEndpoint, MediaListEndpoint, NotificationEndpoint, RecommendationEndpoint, ReviewEndpoint,
    StaffEndpoint, StudioEndpoint, UserEndpoint,
};
use crate::errors::AniListError;
use crate::utils::{RetryConfig, retry_with_backoff};
use reqwest::{Client, Response, StatusCode};
use serde_json::{Value, from_value};
use std::collections::HashMap;
use std::fmt;

/// The default AniList GraphQL API endpoint
const ANILIST_API_URL: &str = "https://graphql.anilist.co";

/// User-Agent header for identifying this library
const USER_AGENT: &str = concat!("anilist-moe/", env!("CARGO_PKG_VERSION"), " (Rust)");

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
    client: Client,
    token: Option<String>,
    retry_config: RetryConfig,
    base_url: String,
}

// Implement Debug manually to avoid exposing sensitive token information
impl fmt::Debug for AniListClient {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AniListClient")
            .field("base_url", &self.base_url)
            .field("has_token", &self.token.is_some())
            .field("retry_config", &self.retry_config)
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
    pub fn with_token(token: &str) -> Self {
        let mut client = Self::new();
        client.token = Some(token.to_string());
        client
    }

    /// Builds a configured reqwest client with optimal settings.
    fn build_client() -> Client {
        Client::builder()
            .user_agent(USER_AGENT)
            .timeout(std::time::Duration::from_secs(30))
            .pool_max_idle_per_host(10)
            .build()
            .expect("Failed to build HTTP client")
    }

    /// Creates a client with a custom reqwest client.
    fn new_with_client(client: Client) -> Self {
        Self {
            client,
            token: None,
            retry_config: RetryConfig::default(),
            base_url: ANILIST_API_URL.to_string(),
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
    pub fn with_retry_config(mut self, config: RetryConfig) -> Self {
        self.retry_config = config;
        self
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
    pub fn with_base_url(mut self, base_url: &str) -> Self {
        self.base_url = base_url.to_string();
        self
    }

    pub fn media(&self) -> MediaEndpoint {
        MediaEndpoint::new(self.clone())
    }

    pub fn medialist(&self) -> MediaListEndpoint {
        MediaListEndpoint::new(self.clone())
    }

    pub fn character(&self) -> CharacterEndpoint {
        CharacterEndpoint::new(self.clone())
    }

    pub fn common(&self) -> CommonEndpoint {
        CommonEndpoint::new(self.clone())
    }

    pub fn staff(&self) -> StaffEndpoint {
        StaffEndpoint::new(self.clone())
    }

    pub fn user(&self) -> UserEndpoint {
        UserEndpoint::new(self.clone())
    }

    pub fn studio(&self) -> StudioEndpoint {
        StudioEndpoint::new(self.clone())
    }

    pub fn forum(&self) -> ForumEndpoint {
        ForumEndpoint::new(self.clone())
    }

    pub fn activity(&self) -> ActivityEndpoint {
        ActivityEndpoint::new(self.clone())
    }

    pub fn review(&self) -> ReviewEndpoint {
        ReviewEndpoint::new(self.clone())
    }

    pub fn recommendation(&self) -> RecommendationEndpoint {
        RecommendationEndpoint::new(self.clone())
    }

    pub fn airing(&self) -> AiringEndpoint {
        AiringEndpoint::new(self.clone())
    }

    pub fn notification(&self) -> NotificationEndpoint {
        NotificationEndpoint::new(self.clone())
    }

    pub fn set_token(&mut self, token: &str) {
        self.token = Some(token.to_string());
    }

    pub fn clear_token(&mut self) {
        self.token = None;
    }

    pub fn has_token(&self) -> bool {
        self.token.is_some()
    }

    #[allow(dead_code)]
    pub(crate) async fn query(
        &self,
        query: &'static str,
        variables: Option<&HashMap<String, Value>>,
    ) -> Result<Value, AniListError> {
        self.execute_query(query, variables).await
    }

    pub(crate) async fn query_typed<T>(
        &self,
        query: &'static str,
        variables: Option<&HashMap<String, Value>>,
    ) -> Result<T, AniListError>
    where
        T: serde::de::DeserializeOwned,
    {
        let response_data = self.execute_query(query, variables).await?;
        from_value(response_data).map_err(|e| AniListError::ParseError {
            message: format!("Failed to deserialize response: {}", e),
        })
    }

    async fn execute_query(
        &self,
        query: &'static str,
        variables: Option<&HashMap<String, Value>>,
    ) -> Result<Value, AniListError> {
        retry_with_backoff(
            || async { self.raw_query(query, variables).await },
            self.retry_config,
        )
        .await
    }

    async fn raw_query(
        &self,
        query: &'static str,
        variables: Option<&HashMap<String, Value>>,
    ) -> Result<Value, AniListError> {
        let body = self.build_request_body(query, variables);

        let mut request = self
            .client
            .post(&self.base_url)
            .header("Content-Type", "application/json");

        if let Some(token) = &self.token {
            request = request.header("Authorization", format!("Bearer {}", token));
        }

        let response = request.json(&body).send().await?;

        self.handle_response(response).await
    }

    fn build_request_body(
        &self,
        query: &'static str,
        variables: Option<&HashMap<String, Value>>,
    ) -> HashMap<&'static str, Value> {
        let mut body = HashMap::new();
        body.insert("query", Value::String(query.to_string()));

        if let Some(vars) = variables {
            body.insert(
                "variables",
                Value::Object(vars.clone().into_iter().collect()),
            );
        }
        body
    }

    async fn handle_response(&self, response: Response) -> Result<Value, AniListError> {
        let status = response.status();
        if status.is_success() {
            let json: Value = response.json().await?;
            self.handle_graphql_errors(json)
        } else {
            Err(self.handle_http_error(status, response).await)
        }
    }

    async fn handle_http_error(&self, status: StatusCode, response: Response) -> AniListError {
        match status.as_u16() {
            400 => AniListError::BadRequest {
                message: response
                    .text()
                    .await
                    .unwrap_or_else(|_| "Bad Request".to_string()),
            },
            401 => AniListError::AuthenticationRequired,
            403 => AniListError::AccessDenied,
            404 => AniListError::NotFound,
            429 => self.parse_rate_limit_error(response),
            500..=599 => AniListError::ServerError {
                status: status.as_u16(),
                message: response
                    .text()
                    .await
                    .unwrap_or_else(|_| "Server Error".to_string()),
            },
            _ => AniListError::ServerError {
                status: status.as_u16(),
                message: response
                    .text()
                    .await
                    .unwrap_or_else(|_| "Unknown Error".to_string()),
            },
        }
    }

    fn parse_rate_limit_error(&self, response: Response) -> AniListError {
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
            let error_message = if errors.is_array() {
                errors
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|e| {
                        e.get("message")
                            .and_then(|m| m.as_str())
                            .unwrap_or("Unknown error")
                    })
                    .collect::<Vec<_>>()
                    .join(", ")
            } else {
                errors.to_string()
            };

            if error_message.to_lowercase().contains("rate limit")
                || error_message.to_lowercase().contains("too many requests")
            {
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

impl Default for AniListClient {
    fn default() -> Self {
        Self::new()
    }
}
