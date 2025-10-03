//! Union types for AniList API.
//!
//! This module contains union types that can represent multiple possible entity types,
//! such as different activity types or notification types.

/// Activity union (TextActivity, MessageActivity, ListActivity)
pub mod activity;
/// Likeable entity union
pub mod likeable;
/// Notification union (various notification types)
pub mod notification;
