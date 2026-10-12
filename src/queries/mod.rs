//! GraphQL query and mutation strings.
//!
//! This module contains all the GraphQL queries and mutations used by the API client.
//! Each submodule contains queries for a specific endpoint.

/// Media queries and mutations
#[cfg(feature = "media")]
pub mod media;

/// MediaList queries and mutations
#[cfg(feature = "medialist")]
pub mod medialist;

/// Activity queries and mutations
#[cfg(feature = "activity")]
pub mod activity;

/// Common mutations (likes, follows, favorites)
#[cfg(feature = "common")]
pub mod common;

/// User queries and mutations
#[cfg(feature = "user")]
pub mod user;

/// Character queries
#[cfg(feature = "character")]
pub mod character;

/// Staff queries
#[cfg(feature = "staff")]
pub mod staff;

/// Studio queries
#[cfg(feature = "studio")]
pub mod studio;

/// Airing queries
#[cfg(feature = "airing")]
pub mod airing;

/// Forum queries and mutations
#[cfg(feature = "forum")]
pub mod forum;

// Review queries and mutations
#[cfg(feature = "review")]
pub mod review;

// Recommendation mutations
#[cfg(feature = "recommendation")]
pub mod recommendation;

// Notification queries
#[cfg(feature = "notification")]
pub mod notification;
