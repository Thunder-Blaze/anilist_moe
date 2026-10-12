//! AniList API endpoint modules.
//!
//! Each module provides a specialized endpoint for interacting with different
//! parts of the AniList API, such as anime, manga, characters, users, etc.

/// Activity feed operations (text posts, list updates, replies)
#[cfg(feature = "activity")]
pub mod activity;
/// Airing schedule operations (upcoming episodes, recently aired)
#[cfg(feature = "airing")]
pub mod airing;
/// Character operations (search, popular, birthdays)
#[cfg(feature = "character")]
pub mod character;
/// Common operations (likes, follows, favorites)
#[cfg(feature = "common")]
pub mod common;
/// Forum operations (threads, comments, categories)
#[cfg(feature = "forum")]
pub mod forum;
/// Media operations (anime, manga, search)
#[cfg(feature = "media")]
pub mod media;
/// Media list operations (user anime/manga lists)
#[cfg(feature = "medialist")]
pub mod medialist;
/// Notification operations (read, manage notifications)
#[cfg(feature = "notification")]
pub mod notification;
/// Recommendation operations (browse, manage recommendations)
#[cfg(feature = "recommendation")]
pub mod recommendation;
/// Review operations (create, read, update, delete reviews)
#[cfg(feature = "review")]
pub mod review;
/// Staff operations (search, popular, birthdays)
#[cfg(feature = "staff")]
pub mod staff;
/// Studio operations (search, productions)
#[cfg(feature = "studio")]
pub mod studio;
/// User operations (profiles, statistics, favorites)
#[cfg(feature = "user")]
pub mod user;

#[cfg(feature = "activity")]
pub use activity::ActivityEndpoint;
#[cfg(feature = "airing")]
pub use airing::AiringEndpoint;
#[cfg(feature = "character")]
pub use character::CharacterEndpoint;
#[cfg(feature = "common")]
pub use common::CommonEndpoint;
#[cfg(feature = "forum")]
pub use forum::ForumEndpoint;
#[cfg(feature = "media")]
pub use media::MediaEndpoint;
#[cfg(feature = "medialist")]
pub use medialist::MediaListEndpoint;
#[cfg(feature = "notification")]
pub use notification::NotificationEndpoint;
#[cfg(feature = "recommendation")]
pub use recommendation::RecommendationEndpoint;
#[cfg(feature = "review")]
pub use review::ReviewEndpoint;
#[cfg(feature = "staff")]
pub use staff::StaffEndpoint;
#[cfg(feature = "studio")]
pub use studio::StudioEndpoint;
#[cfg(feature = "user")]
pub use user::UserEndpoint;
