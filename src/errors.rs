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
#[must_use]
pub fn format_messages(errors: &[GraphQLError]) -> String {
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
///
/// The enum is `#[non_exhaustive]`, so a new variant is not a breaking change for
/// callers: match with a trailing `_ =>` arm.
#[derive(Error, Debug)]
#[non_exhaustive]
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

    /// AniList is down for maintenance, or said so in a response.
    ///
    /// AniList answers with a `503`, or with a body that reads like a maintenance
    /// notice; both end up here so a caller can tell "come back later" apart from
    /// a query that needs fixing. See [`detect_maintenance`].
    #[error("AniList is unavailable: {message}")]
    Maintenance { message: String },

    /// A status code this crate does not map to anything more specific.
    #[error("Unexpected status {status}: {message}")]
    UnexpectedStatus { status: u16, message: String },

    /// Failed to parse API response
    #[error("Parse error: {message}")]
    ParseError { message: String },
}

/// Whether a message reads like AniList saying it is down.
///
/// Used on a `503`, and on the messages of a GraphQL error, so that an outage is
/// reported as [`AniListError::Maintenance`] rather than as a bad query.
///
/// ```rust
/// use anilist_moe::errors::detect_maintenance;
///
/// assert!(detect_maintenance("AniList is down due to maintenance"));
/// assert!(!detect_maintenance("Cannot query field \"nope\"."));
/// ```
#[must_use]
pub fn detect_maintenance(message: &str) -> bool {
    let lower = message.to_lowercase();

    [
        "api is down",
        "maintenance",
        "will be back",
        "temporarily unavailable",
        "service unavailable",
        "down due to",
        "shut down",
        // The docs describe the outage signal as a `403` reading "The AniList API
        // has been temporarily disabled due to severe stability issues": without
        // these it would be reported as a permission problem.
        "temporarily disabled",
        "severe stability",
    ]
    .iter()
    .any(|phrase| lower.contains(phrase))
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

    /// A stable, low-cardinality label for this error's variant.
    ///
    /// Useful as a metric or tracing dimension, where the full message would be
    /// too high-cardinality to group by.
    ///
    /// ```rust
    /// use anilist_moe::errors::AniListError;
    ///
    /// assert_eq!(AniListError::NotFound.kind(), "not_found");
    /// ```
    #[must_use]
    pub fn kind(&self) -> &'static str {
        match self {
            Self::Network(_) => "network",
            Self::Transport { .. } => "transport",
            Self::Json(_) => "json",
            Self::GraphQL { .. } => "graphql",
            Self::RateLimit { .. } => "rate_limit",
            Self::RateLimitSimple => "rate_limit_simple",
            Self::BurstLimit => "burst_limit",
            Self::NotFound => "not_found",
            Self::AuthenticationRequired => "authentication_required",
            Self::AccessDenied => "access_denied",
            Self::BadRequest { .. } => "bad_request",
            Self::ServerError { .. } => "server_error",
            Self::Maintenance { .. } => "maintenance",
            Self::UnexpectedStatus { .. } => "unexpected_status",
            Self::ParseError { .. } => "parse_error",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_error_variant_has_a_distinct_kind() {
        let errors = [
            AniListError::Transport {
                message: "boom".to_string(),
            },
            AniListError::Json(serde_json::from_str::<serde_json::Value>("").unwrap_err()),
            AniListError::GraphQL {
                errors: vec![GraphQLError::new("nope")],
                data: None,
            },
            AniListError::RateLimit {
                limit: 90,
                remaining: 0,
                reset_at: 0,
                retry_after: 1,
            },
            AniListError::RateLimitSimple,
            AniListError::BurstLimit,
            AniListError::NotFound,
            AniListError::AuthenticationRequired,
            AniListError::AccessDenied,
            AniListError::BadRequest {
                message: "bad".to_string(),
            },
            AniListError::ServerError {
                status: 500,
                message: "boom".to_string(),
            },
            AniListError::ParseError {
                message: "boom".to_string(),
            },
        ];

        let kinds: Vec<&str> = errors.iter().map(AniListError::kind).collect();
        for (label, error) in kinds.iter().zip(&errors) {
            assert!(!label.is_empty(), "{error:?} has an empty kind");
            assert!(label.chars().all(|c| c.is_ascii_lowercase() || c == '_'));
        }
        assert_eq!(AniListError::NotFound.kind(), "not_found");
    }
}
