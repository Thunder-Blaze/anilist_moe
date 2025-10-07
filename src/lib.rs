//! # AniList API Wrapper
//!
//! A comprehensive, production-ready Rust wrapper for the AniList GraphQL API that provides
//! complete coverage of AniList's features including anime, manga, characters, staff, users,
//! social features, forums, activities, reviews, recommendations, and more.
//!
//! This crate provides a type-safe, async interface to interact with the AniList API,
//! supporting both authenticated and unauthenticated requests with automatic rate limiting
//! handling and comprehensive error management.
//!
//! ## Features
//!
//! - **Complete API Coverage**: All major AniList endpoints including social features
//! - **Async/Await Support**: Built with tokio for high-performance async operations
//! - **Type Safety**: Strongly typed responses with serde serialization/deserialization
//! - **Modular Design**: Separate endpoint modules for clean code organization
//! - **Authentication**: Full support for authenticated requests with Bearer tokens
//! - **Rate Limiting**: Automatic handling of AniList's 90 requests/minute rate limit
//! - **Error Handling**: Comprehensive error types with detailed error messages
//! - **Retry Logic**: Built-in retry mechanisms for transient failures
//! - **GraphQL Integration**: Direct GraphQL query execution with proper field mapping
//! - **Pagination**: Built-in support for paginated results across all endpoints
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
//! - **Activities**: Text posts, list updates, replies, likes, following feed
//! - **Reviews**: Create, read, update, delete user reviews with ratings
//! - **Recommendations**: Browse and manage anime/manga recommendations
//! - **Notifications**: Read, filter, and manage user notifications
//!
//! ### Scheduling & Discovery
//! - **Airing Schedules**: Upcoming episodes, recently aired, date-based filtering
//! - **Trending Data**: Real-time trending content across all media types
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
//! AniList enforces a rate limit of 90 requests per minute. This wrapper automatically
//! handles rate limiting with:
//! - Automatic detection of rate limit headers
//! - Proper 429 error handling with retry-after support
//! - Built-in retry logic with exponential backoff
//! - Burst request protection
//!
//! ## Examples
//!
//! ### Basic Usage (No Authentication)
//!
//! ```rust
//! use anilist_moe::AniListClient;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = AniListClient::new();
//!     
//!     // Search for anime
//!     let anime_results = client.anime().search("Attack on Titan", 1, 5).await?;
//!     for anime in anime_results {
//!         println!("Found: {} ({})", anime.title.romaji, anime.id);
//!     }
//!     
//!     // Get trending anime
//!     let trending = client.anime().get_trending(1, 10).await?;
//!     println!("Found {} trending anime", trending.len());
//!     
//!     Ok(())
//! }
//! ```
//!
//! ### Authenticated Usage
//!
//! ```rust
//! use anilist_moe::AniListClient;
//! use std::env;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let token = env::var("ANILIST_TOKEN")?;
//!     let client = AniListClient::with_token(token);
//!     
//!     // Get user profile
//!     let user = client.user().get_current_user().await?;
//!     println!("Logged in as: {}", user.name);
//!     
//!     // Get user's notifications
//!     let notifications = client.notification().get_notifications(1, 10).await?;
//!     println!("You have {} notifications", notifications.len());
//!     
//!     // Post a text activity
//!     let activity = client.activity().post_text_activity("Hello from Rust!".to_string()).await?;
//!     println!("Posted activity: {}", activity.id);
//!     
//!     Ok(())
//! }
//! ```
//!
//! ### Error Handling
//!
//! ```rust
//! use anilist_moe::{AniListClient, AniListError};
//!
//! #[tokio::main]
//! async fn main() {
//!     let client = AniListClient::new();
//!     
//!     match client.anime().get_by_id(999999).await {
//!         Ok(anime) => println!("Found anime: {}", anime.title.romaji),
//!         Err(AniListError::RateLimit { retry_after, .. }) => {
//!             println!("Rate limited! Retry after {} seconds", retry_after);
//!         },
//!         Err(AniListError::NotFound) => {
//!             println!("Anime not found");
//!         },
//!         Err(e) => println!("Error: {}", e),
//!     }
//! }
//! ```

pub mod client;
pub mod endpoints;
pub mod error;
pub mod models;
pub mod queries;
pub mod utils;

pub use client::AniListClient;
pub use error::AniListError;
