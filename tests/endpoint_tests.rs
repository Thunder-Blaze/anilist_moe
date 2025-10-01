//! Integration tests for AniList API endpoints
//!
//! These tests verify that the endpoints correctly fetch, parse, and return
//! data with proper types from the AniList API.
//!
//! **Note**: These tests make real API calls and require internet connection.
//! Run with: `cargo test --test endpoint_tests -- --test-threads=1`
//! to avoid rate limiting.

mod endpoints;
