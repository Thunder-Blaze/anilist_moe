//! AniList API endpoint modules.
//!
//! Each module provides a specialized endpoint for interacting with different
//! parts of the AniList API, such as anime, manga, characters, users, etc.

/// Activity feed operations (text posts, list updates, replies)
pub mod activity;
/// Airing schedule operations (upcoming episodes, recently aired)
pub mod airing;
/// Character operations (search, popular, birthdays)
pub mod character;
/// Common operations (likes, follows, favorites)
pub mod common;
/// Forum operations (threads, comments, categories)
pub mod forum;
/// Media operations (anime, manga, search)
pub mod media;
/// Media list operations (user anime/manga lists)
pub mod medialist;
/// Notification operations (read, manage notifications)
pub mod notification;
/// Recommendation operations (browse, manage recommendations)
pub mod recommendation;
/// Review operations (create, read, update, delete reviews)
pub mod review;
/// Staff operations (search, popular, birthdays)
pub mod staff;
/// Studio operations (search, productions)
pub mod studio;
/// User operations (profiles, statistics, favorites)
pub mod user;

pub use activity::ActivityEndpoint;
pub use airing::AiringEndpoint;
pub use character::CharacterEndpoint;
pub use common::CommonEndpoint;
pub use forum::ForumEndpoint;
pub use media::MediaEndpoint;
pub use medialist::MediaListEndpoint;
pub use notification::NotificationEndpoint;
pub use recommendation::RecommendationEndpoint;
pub use review::ReviewEndpoint;
pub use staff::StaffEndpoint;
pub use studio::StudioEndpoint;
pub use user::UserEndpoint;
