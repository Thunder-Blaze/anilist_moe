# AniList_Moe

[![Crates.io](https://img.shields.io/crates/v/anilist_moe.svg)](https://crates.io/crates/anilist_moe)
[![Documentation](https://docs.rs/anilist_moe/badge.svg)](https://docs.rs/anilist_moe)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A comprehensive, type-safe Rust wrapper for the [AniList GraphQL API](https://anilist.gitbook.io/anilist-apiv2-docs/).

## Features

- **Type Safety**: Fully typed responses with proper error handling
- **Simplified Response Format**: Clean and intuitive API responses (v0.3.0+)
- **Modular Design**: Organized endpoints for all AniList features
- **Authentication Support**: Both authenticated and unauthenticated clients
- **Async/Await**: Built with Tokio for high-performance asynchronous operations
- **Zero Unsafe Code**: Written entirely in safe Rust
- **Comprehensive Coverage**: Full AniList API support including social features
- **Pagination Support**: Built-in helpers for paginated results
- **Well Tested**: Extensive test suite covering all endpoints
- **Convenience Functions**: Easy-to-use helper methods for common queries
- **Code Quality**: Pre-commit hooks with Husky for consistent code style

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
anilist_moe = "0.3.0"
tokio = { version = "1.0", features = ["full"] }
```

## Quick Start

### Example 1: Get Trending Anime

```rust
use anilist_moe::AniListClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a client (no authentication needed for public data)
    let client = AniListClient::new();

    // Get trending anime with proper type safety
    let response = client.anime().get_trending(Some(1), Some(10)).await?;

    // Access the data with clean, simple structure (v0.2.2+)
    for anime in &response.data {
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
use anilist_moe::AniListClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = AniListClient::new();

    // Search for an anime
    let search_results = client.anime().search_anime("Attack on Titan", Some(1), Some(5)).await?;

    // Access the data - search_results is Page<Vec<Media>>
    if let Some(first_result) = search_results.data.first() {
        let anime_id = first_result.id.unwrap_or(0);

        // Get detailed information by ID - returns just Media
        let anime = client.anime().get_by_id(anime_id).await?;

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
use anilist_moe::AniListClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = AniListClient::new();

    // Get popular anime - returns Page<Vec<Media>>
    let popular = client.anime().get_popular_anime(Some(1), Some(5)).await?;
    for anime in &popular.data {
        println!("{}", anime.title.as_ref().unwrap().romaji.as_ref().unwrap());
    }

    // Get trending anime
    let trending = client.anime().get_trending_anime(Some(1), Some(5)).await?;

    // Get anime by ID - returns Media directly
    let anime = client.anime().get_by_id(16498).await?; // Attack on Titan

    // Search anime
    let search_results = client.anime().search_anime("Naruto", Some(1), Some(10)).await?;

    // Get anime by season
    use anilist_moe::enums::media::MediaSeason;
    let fall_2023 = client.anime().get_by_season(MediaSeason::Fall, 2023, Some(1), Some(10)).await?;

    // Get top rated anime
    let top_rated = client.anime().get_top_rated_anime(Some(1), Some(10)).await?;

    // Get currently airing anime
    let airing = client.anime().get_airing_anime(Some(1), Some(10)).await?;

    Ok(())
}
```

### Manga Operations

```rust
use anilist_moe::AniListClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = AniListClient::new();

    // Get popular manga
    let popular = client.manga().get_popular_manga(Some(1), Some(5)).await?;

    // Search manga
    let search_results = client.manga().search_manga("One Piece", Some(1), Some(10)).await?;

    // Get top rated manga
    let top_rated = client.manga().get_top_rated_manga(Some(1), Some(10)).await?;

    // Get currently releasing manga
    let releasing = client.manga().get_releasing_manga(Some(1), Some(10)).await?;

    Ok(())
}
```

### Character Operations

```rust
use anilist_moe::AniListClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = AniListClient::new();

    // Get popular characters - returns Page<Vec<Character>>
    let popular = client.character().get_popular_characters(Some(1), Some(10)).await?;

    // Get character by ID - returns Character directly
    let character = client.character().get_by_id(40).await?; // Luffy

    // Search characters - returns Page<Vec<Character>>
    let search_results = client.character().search_character("Luffy", Some(1), Some(10)).await?;

    // Get characters with birthday today - returns Page<Vec<Character>>
    let birthday_chars = client.character().get_by_birthday(5, 5, Some(1), Some(10)).await?;

    // Get most favorited characters - returns Page<Vec<Character>>
    let most_favorited = client.character().get_most_favorited_characters(Some(1), Some(10)).await?;

    Ok(())
}
```

### Forum Operations

```rust
use anilist_moe::AniListClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = AniListClient::new();

    // Get recent threads - returns Page<Vec<Thread>>
    let recent = client.forum().get_recent_threads(Some(1), Some(10)).await?;

    for thread in &recent.data {
        println!("Thread: {}", thread.title.as_ref().unwrap_or(&"Untitled".to_string()));
    }

    // Search threads - returns Page<Vec<Thread>>
    let search = client.forum().search_thread("recommendation", Some(1), Some(10)).await?;

    // Get thread by ID - returns Thread directly
    let thread = client.forum().get_thread_by_id(12345).await?;

    // Get thread comments - returns Page<Vec<ThreadComment>>
    let comments = client.forum().get_thread_comments(12345, Some(1), Some(20)).await?;

    Ok(())
}
```

### Authentication

For endpoints requiring authentication:

```rust
use anilist_moe::AniListClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create an authenticated client
    let token = std::env::var("ANILIST_TOKEN")?;
    let client = AniListClient::with_token(&token);

    // Get current user information - returns User directly
    let current_user = client.user().get_current_user().await?;
    println!("Logged in as: {}", current_user.name);

    // Get current user's anime list - returns Page<Vec<MediaList>>
    let anime_list = client.medialist()
        .get_user_anime_list(&current_user.name, None, Some(1), Some(50))
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
use anilist_moe::{AniListClient, errors::AniListError};

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
// Fully typed response - Page<Vec<Media>>
let response = client.anime().get_trending_anime(Some(1), Some(10)).await?;

// Access the Vec<Media> directly through response.data
let anime = &response.data[0];
let title = anime.title.as_ref().unwrap();
let romaji_title = &title.romaji;
let score = anime.average_score.unwrap_or(0);
let genres = anime.genres.as_ref().unwrap();

// Access pagination info
if let Some(page_info) = &response.page_info {
    println!("Current page: {:?}", page_info.current_page);
    println!("Has next page: {:?}", page_info.has_next_page);
}
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
use anilist_moe::{AniListClient, errors::AniListError};
use std::time::Duration;
use tokio::time::sleep;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = AniListClient::new();

    for page in 1..=10 {
        match client.anime().get_popular_anime(Some(page), Some(50)).await {
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
use anilist_moe::AniListClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = AniListClient::new();

    let mut page = 1;
    let per_page = 50;

    loop {
        let response = client.anime().get_popular_anime(Some(page), Some(per_page)).await?;

        // Process current page - response.data is Vec<Media>
        for anime in &response.data {
            println!("{:?}", anime.title);
        }

        // Check pagination info
        if let Some(page_info) = &response.page_info {
            if !page_info.has_next_page.unwrap_or(false) {
                break;
            }
        } else {
            break;
        }

        page += 1;
    }

    Ok(())
}
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request. See [CONTRIBUTING.md](CONTRIBUTING.md) for detailed guidelines.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Resources

- [AniList API Documentation](https://docs.anilist.co/)
- [Apollo GraphQL Explorer](https://studio.apollographql.com/sandbox/explorer)
- [Crates.io Package](https://crates.io/crates/anilist_moe)
- [Documentation](https://docs.rs/anilist_moe)

## Changelog

See [CHANGELOG.md](CHANGELOG.md) for detailed version history.
