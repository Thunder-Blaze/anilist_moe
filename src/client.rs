use crate::endpoints::{
    MediaEndpoint,
    StaffEndpoint,
    UserEndpoint,
    AiringEndpoint,
    ActivityEndpoint,
    NotificationEndpoint,
    ForumEndpoint,
    CharacterEndpoint,
    RecommendationEndpoint,
    ReviewEndpoint,
    StudioEndpoint,
};
use crate::errors::AniListError;
use reqwest::Client;
use serde_json::Value;
use std::collections::HashMap;

const ANILIST_API_URL: &str = "https://graphql.anilist.co";

#[derive(Clone)]
pub struct AniListClient {
    client: Client,
    token: Option<String>,
}

impl AniListClient {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
            token: None,
        }
    }

    pub fn with_token(token: &str) -> Self {
        Self {
            client: Client::new(),
            token: Some(token.to_string()),
        }
    }

    pub fn media(&self) -> MediaEndpoint {
        MediaEndpoint::new(self.clone())
    }

    pub fn character(&self) -> CharacterEndpoint {
        CharacterEndpoint::new(self.clone())
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
        query: &str,
        variables: Option<&HashMap<String, Value>>,
    ) -> Result<Value, AniListError> {
        let response_data = self.raw_query(query, variables).await?;
        Ok(response_data)
    }

    pub(crate) async fn query_typed<T>(
        &self,
        query: &str,
        variables: Option<&HashMap<String, Value>>,
    ) -> Result<T, AniListError>
    where
        T: serde::de::DeserializeOwned,
    {
        let response_data = self.raw_query(query, variables).await?;
        serde_json::from_value(response_data).map_err(|e| AniListError::ParseError {
            message: format!("Failed to deserialize response: {}", e),
        })
    }

    async fn raw_query(
        &self,
        query: &str,
        variables: Option<&HashMap<String, Value>>,
    ) -> Result<Value, AniListError> {
        let mut body = HashMap::new();
        body.insert("query", Value::String(query.to_string()));

        if let Some(vars) = variables {
            body.insert("variables", Value::Object(vars.clone().into_iter().collect()));
        }

        let mut request = self
            .client
            .post(ANILIST_API_URL)
            .header("Content-Type", "application/json");

        // Add authorization header if token is present
        if let Some(token) = &self.token {
            request = request.header("Authorization", format!("Bearer {}", token));
        }

        let response = request.json(&body).send().await?;

        // Handle HTTP status codes
        let status = response.status();
        match status.as_u16() {
            200..=299 => {
                // Success, continue processing
            }
            400 => {
                let error_text = response
                    .text()
                    .await
                    .unwrap_or_else(|_| "Bad Request".to_string());
                return Err(AniListError::BadRequest {
                    message: error_text,
                });
            }
            401 => {
                return Err(AniListError::AuthenticationRequired);
            }
            403 => {
                return Err(AniListError::AccessDenied);
            }
            404 => {
                return Err(AniListError::NotFound);
            }
            429 => {
                // Rate limit exceeded - extract rate limit headers
                let headers = response.headers();

                // Try to get detailed rate limit information
                if let (
                    Some(limit_header),
                    Some(remaining_header),
                    Some(reset_header),
                    Some(retry_after_header),
                ) = (
                    headers.get("X-RateLimit-Limit"),
                    headers.get("X-RateLimit-Remaining"),
                    headers.get("X-RateLimit-Reset"),
                    headers.get("Retry-After"),
                ) {
                    let limit = limit_header.to_str().unwrap_or("90").parse().unwrap_or(90);
                    let remaining = remaining_header
                        .to_str()
                        .unwrap_or("0")
                        .parse()
                        .unwrap_or(0);
                    let reset_at = reset_header.to_str().unwrap_or("0").parse().unwrap_or(0);
                    let retry_after = retry_after_header
                        .to_str()
                        .unwrap_or("60")
                        .parse()
                        .unwrap_or(60);

                    return Err(AniListError::RateLimit {
                        limit,
                        remaining,
                        reset_at,
                        retry_after,
                    });
                } else {
                    // Fallback if headers are not available
                    return Err(AniListError::RateLimitSimple);
                }
            }
            500..=599 => {
                let error_text = response
                    .text()
                    .await
                    .unwrap_or_else(|_| "Server Error".to_string());
                return Err(AniListError::ServerError {
                    status: status.as_u16(),
                    message: error_text,
                });
            }
            _ => {
                let error_text = response
                    .text()
                    .await
                    .unwrap_or_else(|_| "Unknown Error".to_string());
                return Err(AniListError::ServerError {
                    status: status.as_u16(),
                    message: error_text,
                });
            }
        }

        let json: Value = response.json().await?;

        // Check for GraphQL errors
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

            // Check if it's a rate limit error in GraphQL response
            if error_message.to_lowercase().contains("rate limit")
                || error_message.to_lowercase().contains("too many requests")
            {
                return Err(AniListError::BurstLimit);
            }

            return Err(AniListError::GraphQL {
                message: error_message,
            });
        }

        Ok(json)
    }
}

impl Default for AniListClient {
    fn default() -> Self {
        Self::new()
    }
}
