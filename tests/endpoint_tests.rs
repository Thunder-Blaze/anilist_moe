//! Integration tests for AniList API endpoints
//!
//! These tests verify that the endpoints correctly fetch, parse, and return
//! data with proper types from the AniList API.
//!
//! **Note**: These tests make real API calls and require internet connection.
//!
//! ## Running Tests
//!
//! To run tests with rate limit handling:
//! ```bash
//! cargo test --test endpoint_tests -- --test-threads=1
//! ```
//!
//! ## Rate Limit Handling
//!
//! The test harness automatically:
//! - Enforces delays between requests to avoid hitting rate limits
//! - Pauses for 1 minute when a rate limit is encountered
//! - Retries the operation after the pause

mod endpoints;
mod test_harness;

pub use test_harness::*;
