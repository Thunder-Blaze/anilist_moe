use thiserror::Error;

#[derive(Error, Debug)]
pub enum AniListError {
    #[error("Network error: {0}")]
    Network(#[from] reqwest::Error),

    #[error("JSON parsing error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("GraphQL error: {message}")]
    GraphQL { message: String },

    #[error(
        "Rate limit exceeded. Limit: {limit}, Remaining: {remaining}, Reset at: {reset_at}, Retry after: {retry_after} seconds"
    )]
    RateLimit {
        limit: u32,
        remaining: u32,
        reset_at: u64,
        retry_after: u32,
    },

    #[error("Rate limit exceeded (simple). Try again in a few moments.")]
    RateLimitSimple,

    #[error("Burst limit exceeded. Please slow down your requests.")]
    BurstLimit,

    #[error("Not found")]
    NotFound,

    #[error("Authentication required. Please provide a valid access token.")]
    AuthenticationRequired,

    #[error("Access denied. Check your token permissions.")]
    AccessDenied,

    #[error("Bad request: {message}")]
    BadRequest { message: String },

    #[error("Server error: {status} - {message}")]
    ServerError { status: u16, message: String },

    #[error("Parse error: {message}")]
    ParseError { message: String },
}
