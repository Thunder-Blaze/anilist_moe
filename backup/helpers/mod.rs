//! Helper functions and utilities for building GraphQL queries and handling responses
//!
//! This module provides:
//! - Query builders with fluent API for all AniList operations
//! - Automatic null/None value filtering
//! - Type-safe variable building
//! - Common query presets and utilities

pub mod query_builders;
pub mod query_utils;

pub use query_builders::*;
pub use query_utils::*;
