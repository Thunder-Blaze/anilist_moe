//! Endpoint integration tests.
//!
//! These tests hit the real AniList API.
//!
//! Run with rate limit handling:
//! ```bash
//! cargo test --test endpoint_tests -- --test-threads=1
//! ```

mod endpoints;
mod test_harness;

pub use test_harness::*;
