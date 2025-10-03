//! GraphQL query and mutation strings.
//!
//! This module contains all the GraphQL queries and mutations used by the API client.
//! Each submodule contains queries for a specific endpoint.

/// Media queries and mutations
pub mod media;

/// MediaList queries and mutations
pub mod medialist;

/// Activity queries and mutations
pub mod activity;

/// Common mutations (likes, follows, favorites)
pub mod common;

/// User queries and mutations
pub mod user;

/// Character queries
pub mod character;

/// Staff queries
pub mod staff;

/// Studio queries
pub mod studio;

/// Airing queries
pub mod airing;

/// Forum queries and mutations
pub mod forum;

// Review queries and mutations
pub mod review;

// Recommendation mutations
pub mod recommendation;

// Notification queries
pub mod notification;
