//! Data models for AniList API entities.
//!
//! This module contains all the struct definitions for data returned by the API,
//! including anime, manga, characters, users, and more.

/// Activity-related data models
pub mod activity;
/// Airing schedule data models
pub mod airing;
/// AniChart data models
pub mod anichart;
/// Character data models
pub mod character;
/// Common data models shared across entities
pub mod common;
/// User favourites data models
pub mod favourites;
/// Media (anime/manga) data models
pub mod media;
/// Media list data models
pub mod media_list;
/// Moderator-related data models
pub mod mods;
/// Notification data models
pub mod notification;
/// Recommendation data models
pub mod recommendation;
/// GraphQL response wrappers
pub mod responses;
/// Review data models
pub mod review;
/// Staff data models
pub mod staff;
/// Statistics data models
pub mod stats;
/// Studio data models
pub mod studio;
/// Submission data models
pub mod submission;
/// Forum thread data models
pub mod thread;
/// User data models
pub mod user;
