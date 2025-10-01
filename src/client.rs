use crate::endpoints::{
    ActivityEndpoint, AiringEndpoint, CharacterEndpoint, CommonEndpoint, ForumEndpoint, MediaEndpoint, MediaListEndpoint, NotificationEndpoint, RecommendationEndpoint, ReviewEndpoint, StaffEndpoint, StudioEndpoint, UserEndpoint
};
use crate::errors::AniListError;
use crate::utils::{retry_with_backoff, RetryConfig};
use reqwest::{Client, Response, StatusCode};
use serde_json::{Value, from_value};
use std::collections::HashMap;

const ANILIST_API_URL: &str = "https://graphql.anilist.co";

#[derive(Clone)]
pub struct AniListClient {
    client: Client,
    token: Option<String>,
    retry_config: RetryConfig,
    base_url: String,
}

impl AniListClient {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
            token: None,
            retry_config: RetryConfig::default(),
            base_url: ANILIST_API_URL.to_string(),
        }
    }

    pub fn with_token(token: &str) -> Self {
        Self {
            client: Client::new(),
            token: Some(token.to_string()),
            retry_config: RetryConfig::default(),
            base_url: ANILIST_API_URL.to_string(),
        }
    }

    pub fn with_retry_config(mut self, config: RetryConfig) -> Self {
        self.retry_config = config;
        self
    }

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
            || async {
                self.raw_query(query, variables).await
            },
            self.retry_config.clone(),
        ).await
    }

    async fn raw_query(
        &self,
        query: &'static str,
        variables: Option<&HashMap<String, Value>>,
    ) -> Result<Value, AniListError> {
        let body = self.build_request_body(query, variables);

        let mut request = self.client.post(&self.base_url).header("Content-Type", "application/json");

        if let Some(token) = &self.token {
            request = request.header("Authorization", format!("Bearer {}", token));
        }

        let response = request.json(&body).send().await?;

        self.handle_response(response).await
    }

    fn build_request_body(&self, query: &'static str, variables: Option<&HashMap<String, Value>>) -> HashMap<&'static str, Value> {
        let mut body = HashMap::new();
        body.insert("query", Value::String(query.to_string()));

        if let Some(vars) = variables {
            body.insert("variables", Value::Object(vars.clone().into_iter().collect()));
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
            400 => AniListError::BadRequest { message: response.text().await.unwrap_or_else(|_| "Bad Request".to_string()) },
            401 => AniListError::AuthenticationRequired,
            403 => AniListError::AccessDenied,
            404 => AniListError::NotFound,
            429 => self.parse_rate_limit_error(response),
            500..=599 => AniListError::ServerError { status: status.as_u16(), message: response.text().await.unwrap_or_else(|_| "Server Error".to_string()) },
            _ => AniListError::ServerError { status: status.as_u16(), message: response.text().await.unwrap_or_else(|_| "Unknown Error".to_string()) },
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
            AniListError::RateLimit { limit, remaining, reset_at: reset, retry_after }
        } else {
            AniListError::RateLimitSimple
        }
    }

    fn handle_graphql_errors(&self, json: Value) -> Result<Value, AniListError> {
        if let Some(errors) = json.get("errors") {
            let error_message = if errors.is_array() {
                errors.as_array().unwrap().iter()
                    .map(|e| e.get("message").and_then(|m| m.as_str()).unwrap_or("Unknown error"))
                    .collect::<Vec<_>>().join(", ")
            } else {
                errors.to_string()
            };

            if error_message.to_lowercase().contains("rate limit") || error_message.to_lowercase().contains("too many requests") {
                return Err(AniListError::BurstLimit);
            }

            Err(AniListError::GraphQL { message: error_message })
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
