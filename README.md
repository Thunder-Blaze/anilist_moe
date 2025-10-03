# AniList.moe

[![Crates.io](https://img.shields.io/crates/v/anilist_moe.svg)](https://crates.io/crates/anilist_moe)
[![Documentation](https://docs.rs/anilist_moe/badge.svg)](https://docs.rs/anilist_moe)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A comprehensive, type-safe Rust wrapper for the [AniList GraphQL API](https://anilist.gitbook.io/anilist-apiv2-docs/).

## Features

- **Type Safety**: Fully typed responses with proper error handling
- **Modular Design**: Organized endpoints for all AniList features
- **Authentication Support**: Both authenticated and unauthenticated clients
- **Async/Await**: Built with Tokio for high-performance asynchronous operations
- **Zero Unsafe Code**: Written entirely in safe Rust
- **Comprehensive Coverage**: Full AniList API support including social features
- **Pagination Support**: Built-in helpers for paginated results
- **Well Tested**: Extensive test suite covering all endpoints
- **Convenience Functions**: Easy-to-use helper methods for common queries

## Supported Endpoints

### Core Content
- **Anime**: Popular, trending, search, seasonal, top-rated, airing, upcoming
- **Manga**: Popular, trending, search, top-rated, releasing, completed
- **Characters**: Popular, search, by ID, birthdays, most favorited
- **Staff**: Popular, search, by ID, birthdays, most favorited
- **Studios**: Search, by ID, with media productions

### Social & Community
- **Users**: Profiles, statistics, favorites, lists, followers, following
- **Forums**: Threads, comments, categories, search, subscriptions
- **Activities**: Text activities, message activities, list updates, replies
- **Reviews**: Browse, create, update, delete user reviews
- **Recommendations**: Browse and manage anime/manga recommendations

### Scheduling & Discovery
- **Airing Schedules**: Upcoming episodes, recently aired, date ranges
- **Notifications**: Read, manage, and mark notifications as read
- **Media Lists**: User anime/manga lists with status tracking

### Interactions
- **Common**: Toggle likes, toggle follows, toggle favorites on various content types

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
anilist_moe = "0.2.0"
tokio = { version = "1.0", features = ["full"] }
```

## Quick Start

### Example 1: Get Trending Anime

```rust
use anilist_moe::client::AniListClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a client (no authentication needed for public data)
    let client = AniListClient::new();

    // Get trending anime with proper type safety
    let response = client.anime().get_trending(Some(1), Some(10)).await?;

    // Access the data with full type information
    let anime_list = &response.data.page.data.media;

    for anime in anime_list {
        println!(
            "Title: {} - Score: {}/100",
            anime.title.as_ref().and_then(|t| t.romaji.as_ref()).unwrap_or(&"Unknown".to_string()),
            anime.average_score.unwrap_or(0)
        );
    }

    Ok(())
}
```

### Example 2: Search and Get Detailed Information

```rust
use anilist_moe::client::AniListClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = AniListClient::new();

    // Search for an anime
    let search_results = client.anime().search("Attack on Titan", Some(1), Some(5)).await?;

    if let Some(first_result) = search_results.data.page.data.media.first() {
        // Get detailed information by ID
        let anime_id = first_result.id;
        let detailed = client.anime().get_by_id(anime_id).await?;

        let anime = &detailed.data.media;

        println!("Title: {}", anime.title.as_ref()
            .and_then(|t| t.romaji.as_ref())
            .unwrap_or(&"Unknown".to_string()));

        println!("Synopsis: {}", anime.description.as_ref()
            .unwrap_or(&"No description".to_string()));

        println!("Score: {}/100", anime.average_score.unwrap_or(0));

        println!("Episodes: {}", anime.episodes.unwrap_or(0));

        if let Some(genres) = &anime.genres {
            println!("Genres: {}", genres.join(", "));
        }
    }

    Ok(())
}
```

## Detailed Usage

### Anime Operations

```rust
use anilist_moe::client::AniListClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = AniListClient::new();

    // Get popular anime
    let popular = client.anime().get_popular(Some(1), Some(5)).await?;

    // Get trending anime
    let trending = client.anime().get_trending(Some(1), Some(5)).await?;

    // Get anime by ID
    let anime = client.anime().get_by_id(16498).await?; // Attack on Titan

    // Search anime
    let search_results = client.anime().search("Naruto", Some(1), Some(10)).await?;

    // Get anime by season
    let fall_2023 = client.anime().get_by_season("FALL", 2023, Some(1), Some(10)).await?;

    // Get top rated anime
    let top_rated = client.anime().get_top_rated(Some(1), Some(10)).await?;

    // Get currently airing anime
    let airing = client.anime().get_airing(Some(1), Some(10)).await?;

    Ok(())
}
```

### Manga Operations

```rust
use anilist_moe::client::AniListClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = AniListClient::new();

    // Get popular manga
    let popular = client.manga().get_popular(Some(1), Some(5)).await?;

    // Search manga
    let search_results = client.manga().search("One Piece", Some(1), Some(10)).await?;

    // Get top rated manga
    let top_rated = client.manga().get_top_rated(Some(1), Some(10)).await?;

    // Get currently releasing manga
    let releasing = client.manga().get_releasing(Some(1), Some(10)).await?;

    Ok(())
}
```

### Character Operations

```rust
use anilist_moe::client::AniListClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = AniListClient::new();

    // Get popular characters
    let popular = client.character().get_popular(Some(1), Some(10)).await?;

    // Get character by ID
    let character = client.character().get_by_id(40).await?; // Luffy

    // Search characters
    let search_results = client.character().search("Luffy", Some(1), Some(10)).await?;

    // Get characters with birthday today
    let birthday_chars = client.character().get_by_birthday(5, 5, Some(1), Some(10)).await?;

    // Get most favorited characters
    let most_favorited = client.character().get_most_favorited(Some(1), Some(10)).await?;

    Ok(())
}
```

### Forum Operations

```rust
use anilist_moe::client::AniListClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = AniListClient::new();

    // Get recent threads
    let recent = client.forum().get_recent(Some(1), Some(10)).await?;

    for thread in &recent.data.page.data.threads {
        println!("Thread: {}", thread.title.as_ref().unwrap_or(&"Untitled".to_string()));
    }

    // Search threads
    let search = client.forum().search("recommendation", Some(1), Some(10)).await?;

    // Get thread by ID
    let thread = client.forum().get_by_id(12345).await?;

    // Get thread comments
    let comments = client.forum().get_thread_comments(12345, Some(1), Some(20)).await?;

    Ok(())
}
```

### Authentication

For endpoints requiring authentication:

```rust
use anilist_moe::client::AniListClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create an authenticated client
    let token = std::env::var("ANILIST_TOKEN")?;
    let client = AniListClient::with_token(token);

    // Get current user information
    let current_user = client.user().get_current_user().await?;
    println!("Logged in as: {}", current_user.data.viewer.name);

    // Get current user's anime list
    let anime_list = client.medialist()
        .get_user_anime_list("username", None, Some(1), Some(50))
        .await?;

    Ok(())
}
```

#### Getting an Access Token

To get an access token for authentication:

1. Register your application at [AniList Developer Console](https://anilist.co/settings/developer)
2. Implement OAuth2 flow to get user authorization
3. Exchange authorization code for access token
4. Use the access token with `AniListClient::with_token()`

## Error Handling

The library provides comprehensive error handling:

```rust
use anilist_moe::{client::AniListClient, errors::AniListError};

#[tokio::main]
async fn main() {
    let client = AniListClient::new();

    match client.anime().get_by_id(999999).await {
        Ok(anime) => println!("Found anime: {:?}", anime),
        Err(AniListError::Network(e)) => eprintln!("Network error: {}", e),
        Err(AniListError::GraphQL { message, .. }) => eprintln!("GraphQL error: {}", message),
        Err(AniListError::Json(e)) => eprintln!("JSON parsing error: {}", e),
        Err(AniListError::RateLimit) => eprintln!("Rate limited"),
        Err(AniListError::NotFound) => eprintln!("Not found"),
    }
}
```

## Type Safety

All responses are fully typed. No more dealing with `serde_json::Value`:

```rust
// Fully typed response
let response = client.anime().get_trending(Some(1), Some(10)).await?;

// Access nested data with type safety
let anime = &response.data.page.data.media[0];
let title = anime.title.as_ref().unwrap();
let romaji_title = &title.romaji;
let score = anime.average_score.unwrap_or(0);
let genres = anime.genres.as_ref().unwrap();
```

## Data Models

The library includes comprehensive data models for all AniList entities:

- **Media**: Complete anime/manga information including titles, descriptions, episodes, genres
- **Character**: Character details including names, images, descriptions, birthdays
- **Staff**: Staff information including names, roles, occupations
- **User**: User profiles including statistics, favorites, and preferences
- **Thread**: Forum threads with comments, categories, and user information
- **Activity**: User activities including text posts, list updates, and replies
- **Review**: User reviews with ratings and detailed content
- **Recommendation**: Anime/manga recommendations with ratings

## Testing

Run the test suite:

```bash
cargo test
```

Run specific endpoint tests:

```bash
cargo test --test endpoint_tests
```

The library includes comprehensive tests for:
- All endpoint methods
- Error handling scenarios
- Pagination functionality
- Type safety verification

## Rate Limiting

The AniList API has rate limiting (90 requests per minute). The client handles basic error responses, but you should implement your own rate limiting logic for production applications:

```rust
use std::time::Duration;
use tokio::time::sleep;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = AniListClient::new();

    for page in 1..=10 {
        match client.anime().get_popular(Some(page), Some(50)).await {
            Ok(response) => {
                // Process response
            }
            Err(AniListError::RateLimit) => {
                // Wait and retry
                sleep(Duration::from_secs(60)).await;
                continue;
            }
            Err(e) => return Err(e.into()),
        }

        // Be nice to the API
        sleep(Duration::from_millis(700)).await;
    }

    Ok(())
}
```

## Pagination

All list endpoints support pagination:

```rust
use anilist_moe::client::AniListClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = AniListClient::new();

    let mut page = 1;
    let per_page = 50;

    loop {
        let response = client.anime().get_popular(Some(page), Some(per_page)).await?;
        let page_info = &response.data.page.page_info;

        // Process current page
        for anime in &response.data.page.data.media {
            println!("{:?}", anime.title);
        }

        // Check if there are more pages
        if !page_info.has_next_page.unwrap_or(false) {
            break;
        }

        page += 1;
    }

    Ok(())
}
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

### Development Setup

```bash
# Clone the repository
git clone https://github.com/Thunder-Blaze/anilist_moe.git
cd anilist_moe

# Run tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Check code formatting
cargo fmt --check

# Run clippy
cargo clippy
```

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Resources

- [AniList API Documentation](https://anilist.gitbook.io/anilist-apiv2-docs/)
- [AniList GraphQL Explorer](https://anilist.co/graphiql)
- [Crates.io Package](https://crates.io/crates/anilist_moe)
- [Documentation](https://docs.rs/anilist_moe)

## Changelog

### Version 0.2.0

- **Type Safety Improvements**: All endpoints now return properly typed responses instead of `serde_json::Value`
- **Forum Endpoints**: Added full type safety for forum/thread operations
- **Response Types**: Added comprehensive response wrapper types for all endpoints
- **Convenience Functions**: Added 80+ convenience functions across all endpoints for easier API usage
- **Bug Fixes**: Fixed ActivityUnion deserialization with tagged union approach
- **Breaking Changes**: All return types have changed from `Value` to strongly typed responses

### Version 0.1.0

- Initial release with basic endpoint support
