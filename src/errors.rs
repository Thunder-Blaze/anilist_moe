//! Error types for the AniList API client.
//!
//! This module defines all possible errors that can occur when using the API,
//! including network errors, rate limiting, authentication failures, and more.

use thiserror::Error;

/// One entry of a GraphQL `errors` array.
///
/// The AniList API answers a malformed or rejected query with a list of errors;
/// each carries the message, and usually the field it points at, the position in
/// the query document, and a server specific code.
///
/// ```rust
/// use anilist_moe::errors::GraphQLError;
///
/// let error = GraphQLError {
///     message: "Cannot query field \"nope\" on type \"Media\".".to_string(),
///     path: Some(vec![serde_json::json!("Media"), serde_json::json!("nope")]),
///     locations: None,
///     extensions: None,
/// };
///
/// assert!(error.path.unwrap().len() == 2);
/// ```
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GraphQLError {
    /// Human readable description of what went wrong.
    pub message: String,
    /// The field the error points at, e.g. `["Page", "mediaList", 0, "title"]`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<Vec<serde_json::Value>>,
    /// Where in the query document the error was raised.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub locations: Option<Vec<GraphQLLocation>>,
    /// Server specific metadata, such as an error code or a status.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extensions: Option<serde_json::Map<String, serde_json::Value>>,
}

impl GraphQLError {
    /// An error with only a message, as returned by malformed input tests.
    #[must_use]
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            path: None,
            locations: None,
            extensions: None,
        }
    }

    /// The message the server sent.
    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }
}

/// A position inside the query document, as reported by GraphQL errors.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct GraphQLLocation {
    /// One-based line number.
    pub line: u32,
    /// One-based column number.
    pub column: u32,
}

/// Joins the messages of a GraphQL error list for display.
fn format_messages(errors: &[GraphQLError]) -> String {
    if errors.is_empty() {
        return "unknown error".to_string();
    }

    let mut joined = String::new();
    for (index, error) in errors.iter().enumerate() {
        if index > 0 {
            joined.push_str(", ");
        }
        joined.push_str(&error.message);
    }
    joined
}

/// Errors that can occur when interacting with the AniList API.
#[derive(Error, Debug)]
pub enum AniListError {
    /// Network-level error (connection failed, timeout, etc.)
    #[error("Network error: {0}")]
    Network(#[from] reqwest::Error),

    /// The transport failed before an HTTP response was produced.
    #[error("Transport error: {message}")]
    Transport { message: String },

    /// JSON deserialization error
    #[error("JSON parsing error: {0}")]
    Json(#[from] serde_json::Error),

    /// GraphQL API returned one or more errors
    #[error("GraphQL error: {}", format_messages(.errors))]
    GraphQL {
        /// Every error the server returned, in order.
        errors: Vec<GraphQLError>,
        /// Data the server returned alongside the errors, when it returned any.
        ///
        /// GraphQL allows a partial answer: the queried fields that resolved are
        /// sent in `data` while the failures are listed in `errors`.
        data: Option<serde_json::Value>,
    },

    /// Rate limit exceeded with detailed information
    #[error(
        "Rate limit exceeded. Limit: {limit}, Remaining: {remaining}, Reset at: {reset_at}, Retry after: {retry_after} seconds"
    )]
    RateLimit {
        limit: u32,
        remaining: u32,
        reset_at: u64,
        retry_after: u32,
    },

    /// Rate limit exceeded (simple response without details)
    #[error("Rate limit exceeded (simple). Try again in a few moments.")]
    RateLimitSimple,

    /// Too many requests in a short time period
    #[error("Burst limit exceeded. Please slow down your requests.")]
    BurstLimit,

    /// Requested resource not found
    #[error("Not found")]
    NotFound,

    /// Authentication token required but not provided
    #[error("Authentication required. Please provide a valid access token.")]
    AuthenticationRequired,

    /// Authentication token provided but lacks required permissions
    #[error("Access denied. Check your token permissions.")]
    AccessDenied,

    /// Malformed request
    #[error("Bad request: {message}")]
    BadRequest { message: String },

    /// Server-side error (5xx status codes)
    #[error("Server error: {status} - {message}")]
    ServerError { status: u16, message: String },

    /// Failed to parse API response
    #[error("Parse error: {message}")]
    ParseError { message: String },
}

impl AniListError {
    /// The GraphQL errors behind this error, when it is one.
    ///
    /// ```rust
    /// use anilist_moe::errors::{AniListError, GraphQLError};
    ///
    /// let error = AniListError::GraphQL {
    ///     errors: vec![GraphQLError::new("Cannot query field \"nope\".")],
    ///     data: None,
    /// };
    ///
    /// assert!(error.graphql_errors().unwrap()[0].message().contains("nope"));
    /// ```
    #[must_use]
    pub fn graphql_errors(&self) -> Option<&[GraphQLError]> {
        match self {
            Self::GraphQL { errors, .. } => Some(errors),
            _ => None,
        }
    }

    /// Whether any GraphQL error message mentions `needle`, ignoring case.
    #[must_use]
    pub fn graphql_message_contains(&self, needle: &str) -> bool {
        self.graphql_errors().is_some_and(|errors| {
            errors.iter().any(|error| {
                error
                    .message
                    .to_lowercase()
                    .contains(&needle.to_lowercase())
            })
        })
    }
}
