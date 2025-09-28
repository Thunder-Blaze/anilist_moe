# AniList API Wrapper

A comprehensive, modular Rust wrapper for the AniList GraphQL API.

## Features

- **Modular Design**: Separate endpoints for all AniList features
- **Authentication Support**: Both authenticated and unauthenticated clients
- **Async/Await Support**: Built with Tokio for asynchronous operations
- **Type Safety**: Strongly typed responses with Serde serialization
- **Comprehensive Coverage**: Full AniList API coverage including social features
- **Error Handling**: Proper error types for different failure scenarios
- **Pagination**: Built-in support for paginated results
- **Tested**: Comprehensive test suite covering all endpoints

## Supported Endpoints

### Core Content
- **Anime**: Popular, trending, search, seasonal, top-rated, airing
- **Manga**: Popular, trending, search, top-rated, releasing, completed
- **Characters**: Popular, search, by ID, with media roles
- **Staff**: Popular, search, by ID, with media roles
- **Studios**: Search, by ID, with media productions

### Social & Community
- **Users**: Profiles, statistics, favorites, lists, followers
- **Forums**: Threads, comments, categories, search
- **Activities**: Text activities, list updates, replies, likes
- **Reviews**: Create, read, update, delete user reviews
- **Recommendations**: Browse and manage anime/manga recommendations
- **Notifications**: Read, manage, and mark as read

### Scheduling & Discovery
- **Airing Schedules**: Upcoming episodes, recently aired, date ranges
- **Trending & Popular**: Real-time trending content across all types

## Quick Start

Add this to your `Cargo.toml`:

```toml
[dependencies]
anilist_moe = "0.1.1"
tokio = { version = "1.0", features = ["full"] }
```

## Authentication

For endpoints requiring authentication, set up your environment:

```bash
# Get your token from https://anilist.co/settings/developer
export ANILIST_TOKEN="your_access_token_here"
```

## API Endpoints

### Public Endpoints (No Authentication Required)

All clients (authenticated and unauthenticated) can access these endpoints:

#### Anime
- `get_popular(page, per_page)` - Get popular anime
- `get_trending(page, per_page)` - Get trending anime
- `get_by_id(id)` - Get anime by ID
- `search(query, page, per_page)` - Search anime by title
- `get_by_season(season, year, page, per_page)` - Get anime by season/year
- `get_top_rated(page, per_page)` - Get highest rated anime
- `get_airing(page, per_page)` - Get currently airing anime

#### Manga
- `get_popular(page, per_page)` - Get popular manga
- `get_trending(page, per_page)` - Get trending manga
- `get_by_id(id)` - Get manga by ID
- `search(query, page, per_page)` - Search manga by title
- `get_top_rated(page, per_page)` - Get highest rated manga
- `get_releasing(page, per_page)` - Get currently releasing manga
- `get_completed(page, per_page)` - Get completed manga

#### Characters
- `get_popular(page, per_page)` - Get popular characters
- `get_by_id(id)` - Get character by ID
- `search(query, page, per_page)` - Search characters by name
- `get_by_birthday(month, day, page, per_page)` - Get characters by birthday
- `get_most_favorited(page, per_page)` - Get most favorited characters

#### Staff
- `get_popular(page, per_page)` - Get popular staff
- `get_by_id(id)` - Get staff by ID
- `search(query, page, per_page)` - Search staff by name
- `get_by_birthday(month, day, page, per_page)` - Get staff by birthday
- `get_most_favorited(page, per_page)` - Get most favorited staff

#### Users (Public Data)
- `get_by_id(id)` - Get user by ID
- `get_by_username(name)` - Get user by username
- `search(query, page, per_page)` - Search users
- `get_most_anime_watched(page, per_page)` - Get users with most anime watched
- `get_most_manga_read(page, per_page)` - Get users with most manga read

### Authenticated Endpoints (Requires Access Token)

These endpoints require an authenticated client created with `AniListClient::with_token()`:

#### User (Private Data)
- `get_current_user()` - Get current authenticated user's profile
- `get_current_user_anime_list(status)` - Get current user's anime list

*Note: More authenticated endpoints will be added in future versions for list management, favorites, etc.*

## Usage Examples

### Basic Usage

```rust
use anilist_moe::client::AniListClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create an unauthenticated client for public data
    let client = AniListClient::new();

    // Get popular anime
    let popular_anime = client.anime().get_popular(1, 10).await?;
    println!("Popular anime: {:#?}", popular_anime);

    Ok(())
}
```

### Authenticated Client

For accessing user-specific data, you'll need an authenticated client:

```rust
use anilist_moe::client::AniListClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create an authenticated client (requires AniList access token)
    let token = "your_access_token_here".to_string();
    let client = AniListClient::with_token(token);

    // Get current user information
    let current_user = client.user().get_current_user().await?;
    println!("Current user: {}", current_user.name);

    // Get current user's anime list
    let anime_list = client.user().get_current_user_anime_list(Some("CURRENT")).await?;
    println!("Currently watching {} anime", anime_list.len());

    Ok(())
}
```

#### Getting an Access Token

To get an access token for authentication:

1. Register your application at [AniList Developer Console](https://anilist.co/settings/developer)
2. Implement OAuth2 flow to get user authorization
3. Exchange authorization code for access token
4. Use the access token with `AniListClient::with_token()`

### Anime Operations

```rust
use anilist_moe::client::AniListClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = AniListClient::new();

    // Get popular anime
    let popular = client.anime().get_popular(1, 5).await?;

    // Get trending anime
    let trending = client.anime().get_trending(1, 5).await?;

    // Get anime by ID
    let anime = client.anime().get_by_id(16498).await?; // Attack on Titan

    // Search anime
    let search_results = client.anime().search("Naruto", 1, 10).await?;

    // Get anime by season
    let fall_2023 = client.anime().get_by_season("FALL", 2023, 1, 10).await?;

    // Get top rated anime
    let top_rated = client.anime().get_top_rated(1, 10).await?;

    // Get currently airing anime
    let airing = client.anime().get_airing(1, 10).await?;

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
    let popular = client.manga().get_popular(1, 5).await?;

    // Get trending manga
    let trending = client.manga().get_trending(1, 5).await?;

    // Get manga by ID
    let manga = client.manga().get_by_id(30013).await?; // One Piece

    // Search manga
    let search_results = client.manga().search("One Piece", 1, 10).await?;

    // Get top rated manga
    let top_rated = client.manga().get_top_rated(1, 10).await?;

    // Get currently releasing manga
    let releasing = client.manga().get_releasing(1, 10).await?;

    // Get completed manga
    let completed = client.manga().get_completed(1, 10).await?;

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
    let popular = client.character().get_popular(1, 10).await?;

    // Get character by ID
    let character = client.character().get_by_id(417).await?; // Lelouch

    // Search characters
    let search_results = client.character().search("Luffy", 1, 10).await?;

    // Get characters by birthday
    let birthday_chars = client.character().get_by_birthday(3, 15, 1, 10).await?;

    // Get most favorited characters
    let most_favorited = client.character().get_most_favorited(1, 10).await?;

    Ok(())
}
```

### Staff Operations

```rust
use anilist_moe::client::AniListClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = AniListClient::new();

    // Get popular staff
    let popular = client.staff().get_popular(1, 10).await?;

    // Get staff by ID
    let staff = client.staff().get_by_id(1870).await?; // Hayao Miyazaki

    // Search staff
    let search_results = client.staff().search("Miyazaki", 1, 10).await?;

    // Get staff by birthday
    let birthday_staff = client.staff().get_by_birthday(1, 5, 1, 10).await?;

    // Get most favorited staff
    let most_favorited = client.staff().get_most_favorited(1, 10).await?;

    Ok(())
}
```

### User Operations

```rust
use anilist_moe::client::AniListClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = AniListClient::new();

    // Get user by ID
    let user = client.user().get_by_id(1).await?;

    // Get user by name
    let user = client.user().get_by_username("username").await?;

    // Search users
    let search_results = client.user().search("test", 1, 10).await?;

    // Get users with most anime watched
    let anime_watchers = client.user().get_most_anime_watched(1, 10).await?;

    // Get users with most manga read
    let manga_readers = client.user().get_most_manga_read(1, 10).await?;

    Ok(())
}
```

## Error Handling

The library provides comprehensive error handling:

```rust
use anilist_moe::{client::AniListClient, error::AniListError};

#[tokio::main]
async fn main() {
    let client = AniListClient::new();

    match client.anime().get_by_id(999999).await {
        Ok(anime) => println!("Found anime: {:?}", anime),
        Err(AniListError::Network(e)) => println!("Network error: {}", e),
        Err(AniListError::GraphQL { message }) => println!("GraphQL error: {}", message),
        Err(AniListError::Json(e)) => println!("JSON parsing error: {}", e),
        Err(AniListError::RateLimit) => println!("Rate limited"),
        Err(AniListError::NotFound) => println!("Not found"),
    }
}
```

## Data Models

The library includes comprehensive data models for all AniList entities:

- **Anime**: Complete anime information including titles, descriptions, episodes, genres, etc.
- **Manga**: Complete manga information including chapters, volumes, status, etc.
- **Character**: Character details including names, images, descriptions, birthdays, etc.
- **Staff**: Staff information including names, roles, occupations, etc.
- **User**: User profiles including statistics, favorites, and preferences.

## Testing

Run the test suite:

```bash
cargo test
```

The library includes comprehensive tests for all endpoints:

- Unit tests for each endpoint method
- Integration tests for the complete API workflow
- Error handling tests
- Pagination tests

## Rate Limiting

The AniList API has rate limiting. The client handles basic error responses, but you should implement your own rate limiting logic for production applications.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License.
