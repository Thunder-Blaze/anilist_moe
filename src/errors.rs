//! Error types for the AniList API client.
//!
//! This module defines all possible errors that can occur when using the API,
//! including network errors, rate limiting, authentication failures, and more.

use thiserror::Error;

/// Errors that can occur when interacting with the AniList API.
#[derive(Error, Debug)]
pub enum AniListError {
    /// Network-level error (connection failed, timeout, etc.)
    #[error("Network error: {0}")]
    Network(#[from] reqwest::Error),

    /// JSON deserialization error
    #[error("JSON parsing error: {0}")]
    Json(#[from] serde_json::Error),

    /// GraphQL API returned an error
    #[error("GraphQL error: {message}")]
    GraphQL { message: String },

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
