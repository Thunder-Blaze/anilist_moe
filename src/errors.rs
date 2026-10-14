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
/// let error = GraphQLError::new("Cannot query field \"nope\" on type \"Media\".")
///     .with_path(vec![serde_json::json!("Media"), serde_json::json!("nope")]);
/// assert!(error.path.unwrap().len() == 2);
/// ```
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[non_exhaustive]
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
    /// Sets the error path.
    #[must_use]
    pub fn with_path(mut self, path: Vec<serde_json::Value>) -> Self {
        self.path = Some(path);
        self
    }

    /// Sets the error locations.
    #[must_use]
    pub fn with_locations(mut self, locations: Vec<GraphQLLocation>) -> Self {
        self.locations = Some(locations);
        self
    }

    /// Sets server specific extensions metadata.
    #[must_use]
    pub fn with_extensions(
        mut self,
        extensions: serde_json::Map<String, serde_json::Value>,
    ) -> Self {
        self.extensions = Some(extensions);
        self
    }
}

/// A position inside the query document, as reported by GraphQL errors.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[non_exhaustive]
pub struct GraphQLLocation {
    /// One-based line number.
    pub line: u32,
    /// One-based column number.
    pub column: u32,
}

impl GraphQLLocation {
    /// Creates a new location with 1-based line and column numbers.
    #[must_use]
    pub const fn new(line: u32, column: u32) -> Self {
        Self { line, column }
    }
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
/// High-level categorization of an [`AniListError`].
///
/// Allows callers to pattern-match on failure classes exhaustively or map them
/// to metrics dimensions without parsing error strings.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum ErrorKind {
    /// Underlying connection failure, timeout, or TLS issue.
    Network,
    /// Custom or external transport failure.
    Transport,
    /// JSON decoding error.
    Json,
    /// GraphQL response contained one or more validation or execution errors.
    GraphQL,
    /// Rate limit reached with header information.
    RateLimit,
    /// Rate limit reached without detailed headers.
    RateLimitSimple,
    /// Too many requests sent within a burst window.
    BurstLimit,
    /// The requested entity was not found (HTTP 404).
    NotFound,
    /// Authentication token required but missing (HTTP 401).
    AuthenticationRequired,
    /// Authentication token lacks required permissions (HTTP 403).
    AccessDenied,
    /// Malformed request or invalid arguments (HTTP 400).
    BadRequest,
    /// Server-side error (HTTP 5xx).
    ServerError,
    /// AniList is down for maintenance (HTTP 503 or outage message).
    Maintenance,
    /// Unexpected HTTP status code.
    UnexpectedStatus,
    /// Failed to parse or serialize response/request payload.
    ParseError,
}

impl ErrorKind {
    /// Returns a stable, lowercase string label suitable for metrics dimensions.
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Network => "network",
            Self::Transport => "transport",
            Self::Json => "json",
            Self::GraphQL => "graphql",
            Self::RateLimit => "rate_limit",
            Self::RateLimitSimple => "rate_limit_simple",
            Self::BurstLimit => "burst_limit",
            Self::NotFound => "not_found",
            Self::AuthenticationRequired => "authentication_required",
            Self::AccessDenied => "access_denied",
            Self::BadRequest => "bad_request",
            Self::ServerError => "server_error",
            Self::Maintenance => "maintenance",
            Self::UnexpectedStatus => "unexpected_status",
            Self::ParseError => "parse_error",
        }
    }
}

impl std::fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
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
    Transport {
        message: String,
        #[source]
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    },

    /// JSON deserialization error
    #[error("JSON parsing error: {0}")]
    Json(#[from] serde_json::Error),

    /// GraphQL API returned one or more errors
    #[error("GraphQL error{}: {}", match .operation { Some(op) => format!(" on {op}"), None => String::new() }, format_messages(.errors))]
    GraphQL {
        /// The operation name declared in the GraphQL document, if any.
        operation: Option<String>,
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

    /// Failed to parse API response or serialize request payload.
    #[error("Parse error: {message}")]
    ParseError {
        message: String,
        #[source]
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    },
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
    ///     operation: None,
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
        self.error_kind().as_str()
    }

    /// A typed, high-level [`ErrorKind`] for matching or error classification.
    ///
    /// ```rust
    /// use anilist_moe::errors::{AniListError, ErrorKind};
    ///
    /// assert_eq!(AniListError::NotFound.error_kind(), ErrorKind::NotFound);
    /// ```
    #[must_use]
    pub fn error_kind(&self) -> ErrorKind {
        match self {
            Self::Network(_) => ErrorKind::Network,
            Self::Transport { .. } => ErrorKind::Transport,
            Self::Json(_) => ErrorKind::Json,
            Self::GraphQL { .. } => ErrorKind::GraphQL,
            Self::RateLimit { .. } => ErrorKind::RateLimit,
            Self::RateLimitSimple => ErrorKind::RateLimitSimple,
            Self::BurstLimit => ErrorKind::BurstLimit,
            Self::NotFound => ErrorKind::NotFound,
            Self::AuthenticationRequired => ErrorKind::AuthenticationRequired,
            Self::AccessDenied => ErrorKind::AccessDenied,
            Self::BadRequest { .. } => ErrorKind::BadRequest,
            Self::ServerError { .. } => ErrorKind::ServerError,
            Self::Maintenance { .. } => ErrorKind::Maintenance,
            Self::UnexpectedStatus { .. } => ErrorKind::UnexpectedStatus,
            Self::ParseError { .. } => ErrorKind::ParseError,
        }
    }

    /// The HTTP status code returned by the server, if an HTTP response was produced.
    ///
    /// Returns `None` for network or client-side transport/serialization errors.
    ///
    /// ```rust
    /// use anilist_moe::errors::AniListError;
    ///
    /// assert_eq!(AniListError::NotFound.status(), Some(404));
    /// assert_eq!(AniListError::AuthenticationRequired.status(), Some(401));
    /// ```
    #[must_use]
    pub fn status(&self) -> Option<u16> {
        match self {
            Self::RateLimit { .. } | Self::RateLimitSimple => Some(429),
            Self::BurstLimit => Some(200),
            Self::NotFound => Some(404),
            Self::AuthenticationRequired => Some(401),
            Self::AccessDenied => Some(403),
            Self::BadRequest { .. } => Some(400),
            Self::ServerError { status, .. } | Self::UnexpectedStatus { status, .. } => {
                Some(*status)
            }
            Self::Maintenance { .. } => Some(503),
            _ => None,
        }
    }

    /// Whether this error represents a rate limit condition.
    #[must_use]
    pub fn is_rate_limit(&self) -> bool {
        matches!(
            self,
            Self::RateLimit { .. } | Self::RateLimitSimple | Self::BurstLimit
        )
    }

    /// Whether this error represents a network or transport-level failure.
    #[must_use]
    pub fn is_network(&self) -> bool {
        matches!(self, Self::Network(_) | Self::Transport { .. })
    }

    /// How many seconds to wait before retrying, if the error conveyed a delay.
    #[must_use]
    pub fn retry_after_secs(&self) -> Option<u64> {
        match self {
            Self::RateLimit { retry_after, .. } => Some(u64::from(*retry_after)),
            _ => None,
        }
    }

    /// Whether this error was caused by a transient network failure, rate limit, or server outage
    /// that is eligible for a retry.
    #[must_use]
    pub fn is_retryable(&self) -> bool {
        match self {
            Self::RateLimit { .. }
            | Self::RateLimitSimple
            | Self::BurstLimit
            | Self::Network(_)
            | Self::Transport { .. }
            | Self::Maintenance { .. } => true,
            Self::ServerError { status, .. } => (500..=599).contains(status),
            _ => false,
        }
    }

    /// Returns the name of the operation that produced this error, when attached.
    #[must_use]
    pub fn operation(&self) -> Option<&str> {
        match self {
            Self::GraphQL { operation, .. } => operation.as_deref(),
            _ => None,
        }
    }

    /// Attaches an operation name to this error, if applicable.
    #[must_use]
    pub fn with_operation(mut self, op: impl Into<String>) -> Self {
        if let Self::GraphQL { operation, .. } = &mut self {
            *operation = Some(op.into());
        }
        self
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
                source: None,
            },
            AniListError::Json(serde_json::from_str::<serde_json::Value>("").unwrap_err()),
            AniListError::GraphQL {
                operation: None,
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
                source: None,
            },
        ];

        let kinds: Vec<&str> = errors.iter().map(AniListError::kind).collect();
        for (label, error) in kinds.iter().zip(&errors) {
            assert!(!label.is_empty(), "{error:?} has an empty kind");
            assert!(label.chars().all(|c| c.is_ascii_lowercase() || c == '_'));
        }
        assert_eq!(AniListError::NotFound.kind(), "not_found");
        assert_eq!(AniListError::NotFound.error_kind(), ErrorKind::NotFound);
        assert_eq!(AniListError::NotFound.status(), Some(404));
        assert_eq!(AniListError::RateLimitSimple.status(), Some(429));
        assert!(AniListError::RateLimitSimple.is_rate_limit());
        assert!(AniListError::RateLimitSimple.is_retryable());
        assert!(!AniListError::NotFound.is_retryable());
    }

    #[test]
    fn operation_name_can_be_attached_to_graphql_error() {
        let error = AniListError::GraphQL {
            operation: None,
            errors: vec![GraphQLError::new("nope")],
            data: None,
        };
        assert_eq!(error.operation(), None);
        let with_op = error.with_operation("GetUser");
        assert_eq!(with_op.operation(), Some("GetUser"));
        assert!(with_op.to_string().contains("on GetUser"));
    }
}
