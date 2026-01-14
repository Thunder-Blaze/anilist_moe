//! # AniList.moe
//!
//! A comprehensive, type-safe Rust wrapper for the AniList GraphQL API that provides
//! complete coverage of AniList's features including anime, manga, characters, staff, users,
//! social features, forums, activities, reviews, recommendations, and more.
//!
//! This crate provides a fully typed, async interface to interact with the AniList API,
//! supporting both authenticated and unauthenticated requests with comprehensive error
//! management and proper type safety throughout.
//!
//! ## Features
//!
//! - **Type Safety**: Fully typed responses - no more `serde_json::Value`
//! - **Complete API Coverage**: All major AniList endpoints including social features
//! - **Async/Await Support**: Built with tokio for high-performance async operations
//! - **Modular Design**: Separate endpoint modules for clean code organization
//! - **Authentication**: Full support for authenticated requests with Bearer tokens
//! - **Error Handling**: Comprehensive error types with detailed error messages
//! - **GraphQL Integration**: Direct GraphQL query execution with proper field mapping
//! - **Pagination**: Built-in support for paginated results across all endpoints
//! - **Convenience Functions**: 80+ helper functions for common queries
//!
//! ## Supported Endpoints
//!
//! ### Core Content
//! - **Anime**: Search, trending, popular, seasonal, airing schedules
//! - **Manga**: Search, trending, popular, releasing status
//! - **Characters**: Search, popular, detailed character information
//! - **Staff**: Search, popular, detailed staff information
//! - **Studios**: Search, studio information and productions
//!
//! ### Social & Community
//! - **Users**: Profiles, statistics, favorites, media lists, following
//! - **Forums**: Threads, comments, categories, moderation
//! - **Activities**: Text posts, message activities, list updates, replies
//! - **Reviews**: Create, read, update, delete user reviews with ratings
//! - **Recommendations**: Browse and manage anime/manga recommendations
//! - **Notifications**: Read, filter, and manage user notifications
//!
//! ### Scheduling & Discovery
//! - **Airing Schedules**: Upcoming episodes, recently aired, date-based filtering
//! - **Media Lists**: User anime/manga lists with status tracking
//!
//! ## Authentication
//!
//! Many endpoints require authentication. Get your access token from the
//! [AniList Developer Settings](https://anilist.co/settings/developer).
//!
//! ```bash
//! export ANILIST_TOKEN="your_access_token_here"
//! ```
//!
//! ## Rate Limiting
//!
//! AniList enforces a rate limit of 90 requests per minute. Be respectful of the API
//! and implement appropriate delays between requests in production applications.
//!
//! ## Examples
//!
//! ### Get Trending Anime
//!
//! ```rust,no_run
//! use anilist_moe::client::AniListClient;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = AniListClient::new();
//!
//!     // Get trending anime with full type safety
//!     let response = client.media().get_trending_anime(Some(1), Some(10)).await?;
//!
//!     // Access typed data (Page<Vec<Media>>)
//!     for anime in &response.data {
//!         if let Some(title) = &anime.title {
//!             if let Some(romaji) = &title.romaji {
//!                 println!("Anime: {}", romaji);
//!             }
//!         }
//!     }
//!
//!     Ok(())
//! }
//! ```
//!
//! ### Search and Get Details
//!
//! ```rust,no_run
//! use anilist_moe::client::AniListClient;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = AniListClient::new();
//!
//!     // Search for anime
//!     let search = client.media().search_anime("Steins Gate", Some(1), Some(5)).await?;
//!
//!     if let Some(first) = search.data.first() {
//!         // Get detailed information
//!         let details = client.media().get_anime_by_id(first.id.unwrap()).await?;
//!         let anime = &details;
//!
//!         println!("Score: {}/100", anime.average_score.unwrap_or(0));
//!         println!("Episodes: {}", anime.episodes.unwrap_or(0));
//!     }
//!
//!     Ok(())
//! }
//! ```
//!
//! ### Authenticated Usage
//!
//! ```rust,no_run
//! use anilist_moe::client::AniListClient;
//! use std::env;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let token = env::var("ANILIST_TOKEN")?;
//!     let client = AniListClient::with_token(token);
//!
//!     // Get current user
//!     let user = client.user().get_current_user().await?;
//!     println!("Logged in as: {}", user.name.unwrap_or_default());
//!
//!     Ok(())
//! }
//! ```
//!
//! ### Error Handling
//!
//! ```rust,no_run
//! use anilist_moe::client::AniListClient;
//! use anilist_moe::errors::AniListError;
//!
//! #[tokio::main]
//! async fn main() {
//!     let client = AniListClient::new();
//!
//!     match client.media().get_anime_by_id(999999).await {
//!         Ok(anime) => println!("Found: {:?}", anime),
//!         Err(AniListError::Network(e)) => eprintln!("Network error: {}", e),
//!         Err(AniListError::GraphQL { message, .. }) => eprintln!("API error: {}", message),
//!         Err(AniListError::NotFound) => eprintln!("Not found"),
//!         Err(AniListError::RateLimit { .. }) => eprintln!("Rate limited"),
//!         Err(AniListError::RateLimitSimple) => eprintln!("Rate limited"),
//!         Err(AniListError::BurstLimit) => eprintln!("Too many requests"),
//!         Err(e) => eprintln!("Error: {:?}", e),
//!     }
//! }
//! ```

pub mod client;
pub mod endpoints;
pub mod enums;
pub mod errors;
pub mod objects;
pub mod queries;
pub mod unions;
pub mod utils;

pub use client::AniListClient;
pub use errors::AniListError;
