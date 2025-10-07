//! # Error Types
//!
//! Comprehensive error handling for the AniList API wrapper.
//!
//! This module defines all possible error conditions that can occur when
//! interacting with the AniList API, from network issues to rate limiting
//! and authentication problems.

use thiserror::Error;

/// Comprehensive error type for all AniList API interactions.
///
/// This enum covers all possible error conditions that can occur when using
/// the AniList API wrapper, providing detailed error information and context
/// for proper error handling and user feedback.
///
/// # Error Categories
///
/// ## Network Errors
/// - [`AniListError::Network`] - HTTP request failures, connection issues
///
/// ## Parsing Errors  
/// - [`AniListError::Json`] - JSON deserialization failures
///
/// ## API Errors
/// - [`AniListError::GraphQL`] - GraphQL query errors from the API
/// - [`AniListError::NotFound`] - Resource not found (404)
/// - [`AniListError::BadRequest`] - Invalid request parameters (400)
///
/// ## Authentication Errors
/// - [`AniListError::AuthenticationRequired`] - Missing or invalid token (401)
/// - [`AniListError::AccessDenied`] - Insufficient permissions (403)
///
/// ## Rate Limiting Errors
/// - [`AniListError::RateLimit`] - Rate limit exceeded with detailed info
/// - [`AniListError::RateLimitSimple`] - Simple rate limit without details
/// - [`AniListError::BurstLimit`] - Burst request limit exceeded
///
/// # Examples
///
/// ```rust
/// use anilist_moe::{AniListClient, AniListError};
///
/// match client.anime().get_by_id(999999).await {
///     Ok(anime) => println!("Found: {}", anime.title.romaji),
///     Err(AniListError::RateLimit { retry_after, .. }) => {
///         println!("Rate limited! Wait {} seconds", retry_after);
///     },
///     Err(AniListError::NotFound) => {
///         println!("Anime not found");
///     },
///     Err(AniListError::AuthenticationRequired) => {
///         println!("This operation requires authentication");
///     },
///     Err(e) => println!("Other error: {}", e),
/// }
/// ```
#[derive(Error, Debug)]
pub enum AniListError {
    /// Network-related errors such as connection failures, timeouts, or DNS issues.
    ///
    /// This error wraps underlying [`reqwest::Error`] types and indicates problems
    /// with the HTTP request itself rather than the API response.
    ///
    /// # Common Causes
    /// - No internet connection
    /// - DNS resolution failures  
    /// - Connection timeouts
    /// - SSL/TLS certificate issues
    /// - Server unreachable
    ///
    /// # Handling
    ///
    /// Network errors are typically transient and may resolve with retry attempts.
    /// Consider implementing exponential backoff for automatic retry logic.
    #[error("Network error: {0}")]
    Network(#[from] reqwest::Error),

    /// JSON parsing errors when deserializing API responses.
    ///
    /// This error occurs when the API returns a response that cannot be parsed
    /// into the expected data structures, usually indicating API schema changes
    /// or unexpected response formats.
    ///
    /// # Common Causes
    /// - API schema changes not reflected in the wrapper
    /// - Malformed JSON responses from the API
    /// - Missing required fields in the response
    /// - Type mismatches between expected and actual data
    ///
    /// # Handling
    ///
    /// JSON errors typically indicate either API changes or bugs in the wrapper.
    /// These should be reported as issues for investigation.
    #[error("JSON parsing error: {0}")]
    Json(#[from] serde_json::Error),

    /// GraphQL-specific errors returned by the AniList API.
    ///
    /// These errors originate from the GraphQL API itself and typically indicate
    /// issues with query syntax, invalid parameters, or business logic violations.
    ///
    /// # Common Causes
    /// - Invalid GraphQL query syntax
    /// - Requesting non-existent fields
    /// - Invalid parameter values
    /// - Business logic violations (e.g., invalid media types)
    /// - API-level validation failures
    ///
    /// # Handling
    ///
    /// GraphQL errors usually require fixing the query or parameters being sent.
    /// The error message provides specific details about what went wrong.
    #[error("GraphQL error: {message}")]
    GraphQL {
        /// Detailed error message from the GraphQL API
        message: String,
    },

    /// Detailed rate limit error with comprehensive rate limiting information.
    ///
    /// AniList enforces a rate limit of 90 requests per minute. This error provides
    /// detailed information about the current rate limit state, allowing for
    /// intelligent retry logic and user feedback.
    ///
    /// # Rate Limit Details
    ///
    /// AniList's rate limiting works as follows:
    /// - 90 requests per minute per IP address
    /// - Additional burst limiting for rapid consecutive requests
    /// - Rate limit headers provided in all responses
    /// - 429 status code when limit is exceeded
    ///
    /// # Handling
    ///
    /// Use the `retry_after` field to determine when to retry the request.
    /// Consider implementing exponential backoff with jitter for production use.
    ///
    /// # Example
    ///
    /// ```rust
    /// if let Err(AniListError::RateLimit { retry_after, remaining, .. }) = result {
    ///     if remaining == 0 {
    ///         println!("Rate limit exceeded, waiting {} seconds", retry_after);
    ///         tokio::time::sleep(Duration::from_secs(retry_after as u64)).await;
    ///         // Retry the request
    ///     }
    /// }
    /// ```
    #[error(
        "Rate limit exceeded. Limit: {limit}, Remaining: {remaining}, Reset at: {reset_at}, Retry after: {retry_after} seconds"
    )]
    RateLimit {
        /// The total rate limit (typically 90 requests per minute)
        limit: u32,
        /// Number of requests remaining in the current window
        remaining: u32,
        /// Unix timestamp when the rate limit window resets
        reset_at: u64,
        /// Number of seconds to wait before retrying
        retry_after: u32,
    },

    /// Simple rate limit error without detailed information.
    ///
    /// This error is used when rate limiting is detected but detailed
    /// rate limit headers are not available in the response.
    ///
    /// # Handling
    ///
    /// Since detailed timing information is not available, implement
    /// a conservative retry strategy with exponential backoff.
    #[error("Rate limit exceeded (simple). Try again in a few moments.")]
    RateLimitSimple,

    /// Burst limit exceeded for rapid consecutive requests.
    ///
    /// AniList implements burst limiting to prevent rapid-fire requests
    /// that could impact API performance, separate from the per-minute
    /// rate limiting.
    ///
    /// # Common Causes
    /// - Making many requests in rapid succession
    /// - Parallel requests without proper throttling
    /// - Retry logic without delays
    ///
    /// # Handling
    ///
    /// Implement request throttling and avoid making rapid consecutive
    /// requests. Add delays between requests, especially in loops or
    /// batch operations.
    #[error("Burst limit exceeded. Please slow down your requests.")]
    BurstLimit,

    /// Resource not found (HTTP 404).
    ///
    /// This error indicates that the requested resource (anime, manga, user, etc.)
    /// does not exist or is not accessible.
    ///
    /// # Common Causes
    /// - Invalid ID provided to get_by_id methods
    /// - Deleted or private resources
    /// - Mistyped resource identifiers
    /// - Resources that have been removed from AniList
    ///
    /// # Handling
    ///
    /// Verify the resource ID or search parameters. For user-facing applications,
    /// provide helpful error messages suggesting alternative searches.
    #[error("Not found")]
    NotFound,

    /// Authentication required but not provided (HTTP 401).
    ///
    /// This error indicates that the requested operation requires authentication
    /// but no valid access token was provided with the request.
    ///
    /// # Common Causes
    /// - Using unauthenticated client for protected endpoints
    /// - Expired or invalid access tokens
    /// - Missing authentication headers
    /// - Revoked API access
    ///
    /// # Handling
    ///
    /// Ensure you're using [`crate::AniListClient::with_token`] with a valid access token
    /// for operations that require authentication. Check that your token is still
    /// valid and hasn't expired.
    ///
    /// # Example
    ///
    /// ```rust
    /// // This will fail with AuthenticationRequired
    /// let client = AniListClient::new();
    /// let result = client.user().get_current_user().await;
    ///
    /// // This should work if token is valid
    /// let client = AniListClient::with_token(token);
    /// let result = client.user().get_current_user().await;
    /// ```
    #[error("Authentication required. Please provide a valid access token.")]
    AuthenticationRequired,

    /// Access denied due to insufficient permissions (HTTP 403).
    ///
    /// This error indicates that while authentication was provided, the current
    /// user or token does not have sufficient permissions to perform the requested
    /// operation.
    ///
    /// # Common Causes
    /// - Attempting to modify resources owned by other users
    /// - Missing required permissions for moderation actions
    /// - Token with limited scope trying to access restricted operations
    /// - Attempting to access private resources without permission
    ///
    /// # Handling
    ///
    /// Verify that the authenticated user has the necessary permissions for the
    /// operation. Some operations may require special permissions or ownership
    /// of the resource being modified.
    #[error("Access denied. Check your token permissions.")]
    AccessDenied,

    /// Bad request with detailed error information (HTTP 400).
    ///
    /// This error indicates that the request was malformed or contained invalid
    /// parameters that prevented the API from processing it.
    ///
    /// # Common Causes
    /// - Invalid parameter values (e.g., negative page numbers)
    /// - Missing required parameters
    /// - Parameters outside valid ranges
    /// - Malformed GraphQL queries
    /// - Invalid data formats
    ///
    /// # Handling
    ///
    /// Check the error message for specific details about what parameter or
    /// aspect of the request was invalid. Validate input parameters before
    /// making API calls.
    #[error("Bad request: {message}")]
    BadRequest {
        /// Detailed error message explaining what was wrong with the request
        message: String,
    },

    /// Server-side errors from the AniList API (HTTP 5xx).
    ///
    /// These errors indicate problems on the AniList server side rather than
    /// issues with the client request. They are typically temporary and may
    /// resolve with retry attempts.
    ///
    /// # Common Causes
    /// - AniList server maintenance or downtime
    /// - Database connectivity issues on AniList's side
    /// - Internal server errors
    /// - Service overload or high traffic
    ///
    /// # Handling
    ///
    /// Server errors are typically transient. Implement retry logic with
    /// exponential backoff. If errors persist, check AniList's status page
    /// or social media for service announcements.
    #[error("Server error: {status} - {message}")]
    ServerError {
        /// HTTP status code of the server error
        status: u16,
        /// Error message from the server
        message: String,
    },
}
