# AniList.moe Examples

Comprehensive examples for using the anilist_moe crate.

Every Rust block below is compiled as a doctest. The blocks that talk to the
AniList API are marked `no_run`: they are type-checked but never executed, so no
request ever leaves the machine.

## Table of Contents

- [Basic Usage](#basic-usage)
- [Anime Examples](#anime-examples)
- [Manga Examples](#manga-examples)
- [Character Examples](#character-examples)
- [Staff Examples](#staff-examples)
- [User Examples](#user-examples)
- [Forum Examples](#forum-examples)
- [Activity Examples](#activity-examples)
- [Advanced Examples](#advanced-examples)

## Basic Usage

### Creating a Client

```rust,no_run
use anilist_moe::AniListClient;

// Unauthenticated client for public data.
let client = AniListClient::new();

// Authenticated client for user-specific operations.
let token = std::env::var("ANILIST_TOKEN").expect("Token required");
let auth_client = AniListClient::with_token(token);

// Keep the bindings alive for the example.
let _ = (client, auth_client);
```

## Anime Examples

### Get Trending Anime

```rust,no_run
use anilist_moe::{AniListClient, AniListError};

#[tokio::main]
async fn main() -> Result<(), AniListError> {
    let client = AniListClient::new();

    // Returns Page<Vec<Media>>
    let response = client.media().get_trending_anime(Some(1), Some(10)).await?;

    // Access the Vec<Media> through response.data
    for anime in &response.data {
        if let Some(title) = &anime.title {
            if let Some(romaji) = &title.romaji {
                println!("Anime: {}", romaji);
                println!("  Score: {}/100", anime.average_score.unwrap_or(0));
                println!("  Popularity: {}", anime.popularity.unwrap_or(0));
            }
        }
    }

    Ok(())
}
```

### Search Anime by Title

```rust,no_run
use anilist_moe::{AniListClient, AniListError};

#[tokio::main]
async fn main() -> Result<(), AniListError> {
    let client = AniListClient::new();

    let search_query = "Steins Gate";
    // Returns Page<Vec<Media>>
    let response = client.media().search_anime(search_query, Some(1), Some(10)).await?;

    // Access pagination info
    if let Some(page_info) = &response.page_info {
        println!("Total results: {:?}", page_info.total);
    }

    // Iterate through media list
    for anime in &response.data {
        if let Some(title) = &anime.title {
            println!(
                "Found: {} (ID: {})",
                title.romaji.as_deref().unwrap_or("Unknown"),
                anime.id
            );
        }
    }

    Ok(())
}
```

### Get Anime by Season

```rust,no_run
use anilist_moe::{enums::media::MediaSeason, AniListClient, AniListError};

#[tokio::main]
async fn main() -> Result<(), AniListError> {
    let client = AniListClient::new();

    // Get Fall 2024 anime
    let response = client.media()
        .get_by_season(MediaSeason::Fall, 2024, Some(1), Some(20))
        .await?;

    println!("Fall 2024 Anime:");
    for anime in &response.data {
        if let Some(title) = &anime.title {
            println!("  - {}", title.romaji.as_deref().unwrap_or("Unknown"));
        }
    }

    Ok(())
}
```

### Get Detailed Anime Information

```rust,no_run
use anilist_moe::{AniListClient, AniListError};

#[tokio::main]
async fn main() -> Result<(), AniListError> {
    let client = AniListClient::new();

    // Attack on Titan ID: 16498
    // Returns Media directly (not wrapped)
    let anime = client.media().get_anime_by_id(16498).await?;

    println!("=== Anime Details ===");

    if let Some(title) = &anime.title {
        println!("Title (Romaji): {}", title.romaji.as_deref().unwrap_or("N/A"));
        println!("Title (English): {}", title.english.as_deref().unwrap_or("N/A"));
        println!("Title (Native): {}", title.native.as_deref().unwrap_or("N/A"));
    }

    println!("\nScore: {}/100", anime.average_score.unwrap_or(0));
    println!("Popularity: {} users", anime.popularity.unwrap_or(0));
    println!("Episodes: {}", anime.episodes.unwrap_or(0));

    if let Some(genres) = &anime.genres {
        println!("\nGenres: {}", genres.join(", "));
    }

    if let Some(description) = &anime.description {
        println!("\nDescription:\n{}", description);
    }

    Ok(())
}
```

## Manga Examples

### Get Top Rated Manga

```rust,no_run
use anilist_moe::{AniListClient, AniListError};

#[tokio::main]
async fn main() -> Result<(), AniListError> {
    let client = AniListClient::new();

    // Returns Page<Vec<Media>>
    let response = client.media().get_top_rated_manga(Some(1), Some(10)).await?;

    println!("Top Rated Manga:");
    for (i, manga) in response.data.iter().enumerate() {
        if let Some(title) = &manga.title {
            println!(
                "{}. {} - Score: {}/100",
                i + 1,
                title.romaji.as_deref().unwrap_or("Unknown"),
                manga.average_score.unwrap_or(0)
            );
        }
    }

    Ok(())
}
```

### Get Currently Releasing Manga

```rust,no_run
use anilist_moe::{AniListClient, AniListError};

#[tokio::main]
async fn main() -> Result<(), AniListError> {
    let client = AniListClient::new();

    // Returns Page<Vec<Media>>
    let response = client.media().get_releasing_manga(Some(1), Some(10)).await?;

    println!("Currently Releasing Manga:");
    for manga in &response.data {
        if let Some(title) = &manga.title {
            println!("  - {}", title.romaji.as_deref().unwrap_or("Unknown"));
            if let Some(chapters) = manga.chapters {
                println!("    Chapters: {}", chapters);
            }
        }
    }

    Ok(())
}
```

## Character Examples

### Search for Characters

```rust,no_run
use anilist_moe::{AniListClient, AniListError};

#[tokio::main]
async fn main() -> Result<(), AniListError> {
    let client = AniListClient::new();

    // Returns Page<Vec<Character>>
    let response = client.character().search("Luffy", Some(1), Some(5)).await?;

    for character in &response.data {
        if let Some(name) = &character.name {
            println!("Character: {}", name.full.as_deref().unwrap_or("Unknown"));
            println!("  ID: {}", character.id);
            if let Some(favourites) = character.favourites {
                println!("  Favourites: {}", favourites);
            }
        }
    }

    Ok(())
}
```

### Get Characters by Birthday

```rust,no_run
use anilist_moe::{AniListClient, AniListError};

#[tokio::main]
async fn main() -> Result<(), AniListError> {
    let client = AniListClient::new();

    // Characters whose birthday is today, on AniList's clock.
    // Returns Page<Vec<Character>>
    let response = client.character().get_today_birthday(Some(1), Some(10)).await?;

    println!("Characters with a birthday today:");
    for character in &response.data {
        if let Some(name) = &character.name {
            println!("  - {}", name.full.as_deref().unwrap_or("Unknown"));
        }
    }

    Ok(())
}
```

## Staff Examples

### Get Popular Staff

```rust,no_run
use anilist_moe::{AniListClient, AniListError};

#[tokio::main]
async fn main() -> Result<(), AniListError> {
    let client = AniListClient::new();

    // Returns Page<Vec<Staff>>
    let response = client.staff().get_popular(Some(1), Some(10)).await?;

    println!("Popular Staff:");
    for staff in &response.data {
        if let Some(name) = &staff.name {
            println!("  - {}", name.full.as_deref().unwrap_or("Unknown"));
            if let Some(favourites) = staff.favourites {
                println!("    Favourites: {}", favourites);
            }
        }
    }

    Ok(())
}
```

## User Examples

### Get User Information

```rust,no_run
use anilist_moe::{AniListClient, AniListError};

#[tokio::main]
async fn main() -> Result<(), AniListError> {
    let client = AniListClient::new();

    // Returns User directly (not wrapped)
    let user = client.user().get_by_name("username").await?;

    println!("User: {}", user.name.as_deref().unwrap_or("Unknown"));
    println!("ID: {}", user.id);

    if let Some(about) = &user.about {
        println!("About: {}", about);
    }

    if let Some(stats) = &user.statistics {
        if let Some(anime) = &stats.anime {
            println!("\nAnime Statistics:");
            println!("  Count: {}", anime.count.unwrap_or(0));
            println!("  Episodes Watched: {}", anime.episodes_watched.unwrap_or(0));
            println!("  Mean Score: {}", anime.mean_score.unwrap_or(0.0));
        }
    }

    Ok(())
}
```

### Get Current Authenticated User

```rust,no_run
use anilist_moe::{AniListClient, AniListError};

#[tokio::main]
async fn main() -> Result<(), AniListError> {
    let token = std::env::var("ANILIST_TOKEN").expect("ANILIST_TOKEN must be set");
    let client = AniListClient::with_token(token);

    // Returns User directly (not wrapped)
    let user = client.user().get_current_user().await?;

    println!("Logged in as: {}", user.name.as_deref().unwrap_or("Unknown"));
    println!("User ID: {}", user.id);

    if let Some(unread) = user.unread_notification_count {
        println!("Unread notifications: {}", unread);
    }

    Ok(())
}
```

## Forum Examples

### Browse Forum Threads

```rust,no_run
use anilist_moe::{AniListClient, AniListError};

#[tokio::main]
async fn main() -> Result<(), AniListError> {
    let client = AniListClient::new();

    // Get recent threads - Returns Page<Vec<Thread>>
    let response = client.forum().get_recent(Some(1), Some(10)).await?;

    println!("Recent Forum Threads:");
    for thread in &response.data {
        if let Some(title) = &thread.title {
            println!("\n  Title: {}", title);
            println!("  ID: {}", thread.id);
            println!("  Replies: {}", thread.reply_count.unwrap_or(0));
            println!("  Views: {}", thread.view_count.unwrap_or(0));
        }
    }

    Ok(())
}
```

### Search Forum Threads

```rust,no_run
use anilist_moe::{AniListClient, AniListError};

#[tokio::main]
async fn main() -> Result<(), AniListError> {
    let client = AniListClient::new();

    // Returns Page<Vec<Thread>>
    let response = client.forum().search("recommendation", Some(1), Some(10)).await?;

    println!("Search Results:");
    for thread in &response.data {
        if let Some(title) = &thread.title {
            println!("  - {}", title);
        }
    }

    Ok(())
}
```

### Get Thread Comments

```rust,no_run
use anilist_moe::{AniListClient, AniListError};

#[tokio::main]
async fn main() -> Result<(), AniListError> {
    let client = AniListClient::new();

    let thread_id = 12345;
    // Returns Page<Vec<ThreadComment>>
    let response = client.forum().get_thread_comments(thread_id, Some(1), Some(20)).await?;

    println!("Comments:");
    for comment in &response.data {
        if let Some(user) = &comment.user {
            println!("\n  User: {}", user.name.as_deref().unwrap_or("Unknown"));
        }
        if let Some(comment_text) = &comment.comment {
            println!("  Comment: {}", comment_text);
        }
    }

    Ok(())
}
```

## Activity Examples

### Get User Activities

```rust,no_run
use anilist_moe::unions::activity::ActivityUnion;
use anilist_moe::{AniListClient, AniListError};

#[tokio::main]
async fn main() -> Result<(), AniListError> {
    let client = AniListClient::new();

    // Returns Page<Vec<ActivityUnion>>
    let response = client.activity().get_recent(Some(1), Some(20)).await?;

    for activity in &response.data {
        match activity {
            ActivityUnion::TextActivity(text) => {
                if let Some(user) = &text.user {
                    println!("Text Activity by {}", user.name.as_deref().unwrap_or("Unknown"));
                }
                if let Some(text_content) = &text.text {
                    println!("  {}", text_content);
                }
            }
            ActivityUnion::MessageActivity(msg) => {
                if let Some(messenger) = &msg.messenger {
                    println!("Message by {}", messenger.name.as_deref().unwrap_or("Unknown"));
                }
            }
            ActivityUnion::ListActivity(list) => {
                if let Some(user) = &list.user {
                    println!("List Activity by {}", user.name.as_deref().unwrap_or("Unknown"));
                }
            }
            // The unions are `#[non_exhaustive]`: AniList can add an activity type
            // before this crate knows about it, and a `_` arm is what keeps that
            // from being a breaking change.
            other => println!("Other activity: {other:?}"),
        }
    }

    Ok(())
}
```

## Advanced Examples

### Building Filter Options

```rust,no_run
use anilist_moe::endpoints::media::FetchMediaOptions;
use anilist_moe::enums::media::{MediaSeason, MediaSort, MediaType};
use anilist_moe::{AniListClient, AniListError};
use std::borrow::Cow;

#[tokio::main]
async fn main() -> Result<(), AniListError> {
    let client = AniListClient::new();

    let options = FetchMediaOptions::builder()
    	.media_type(MediaType::Anime)
    	.season(MediaSeason::Fall)
    	.season_year(2024)
    	.sort(Cow::Borrowed(&[MediaSort::PopularityDesc]))
    	.per_page(5)
    	.build();

    let page = client.media().fetch(&options).await?;
    for media in &page.data {
        if let Some(title) = &media.title {
            println!("{}", title.romaji.as_deref().unwrap_or("Unknown"));
        }
    }

Ok(())
}
```

### Pagination with Error Handling

```rust,no_run
use anilist_moe::{errors::AniListError, AniListClient};
use std::time::Duration;
use tokio::time::sleep;

#[tokio::main]
async fn main() -> Result<(), AniListError> {
    let client = AniListClient::new();
    let mut page = 1;
    let per_page = 50;
    let max_pages = 5;

    while page <= max_pages {
        match client.anime().get_popular_anime(Some(page), Some(per_page)).await {
            Ok(response) => {
                // Access pagination info
                if let Some(page_info) = &response.page_info {
                    // Process anime
                    for anime in &response.data {
                        if let Some(title) = &anime.title {
                            println!("Anime: {}", title.romaji.as_deref().unwrap_or("Unknown"));
                        }
                    }

                    // Check if there are more pages
                    if !page_info.has_next_page.unwrap_or(false) {
                        println!("No more pages");
                        break;
                    }

                    page += 1;

                    // Rate limiting - be nice to the API
                    sleep(Duration::from_millis(700)).await;
                } else {
                    break;
                }
            }
            Err(AniListError::RateLimit { .. }) => {
                eprintln!("Rate limited, waiting 60 seconds...");
                sleep(Duration::from_secs(60)).await;
                continue;
            }
            Err(e) => {
                eprintln!("Error: {:?}", e);
                break;
            }
        }
    }

    Ok(())
}
```

### Concurrent Requests

```rust,no_run
use anilist_moe::{AniListClient, AniListError};
use tokio::try_join;

#[tokio::main]
async fn main() -> Result<(), AniListError> {
    let client = AniListClient::new();

    // Endpoint values must outlive the futures that borrow them, so bind them
    // to named locals before handing them to `try_join!`.
    let anime_endpoint = client.anime();
    let manga_endpoint = client.manga();
    let character_endpoint = client.character();

    // Fetch multiple things concurrently
    let (anime, manga, characters) = try_join!(
        anime_endpoint.get_popular_anime(Some(1), Some(5)),
        manga_endpoint.get_popular_manga(Some(1), Some(5)),
        character_endpoint.get_popular(Some(1), Some(5))
    )?;

    println!("Fetched:");
    println!("  {} anime", anime.data.len());
    println!("  {} manga", manga.data.len());
    println!("  {} characters", characters.data.len());

    Ok(())
}
```

### Building a Simple Anime Tracker

```rust,no_run
use anilist_moe::enums::media_list::MediaListStatus;
use anilist_moe::{AniListClient, AniListError};

#[tokio::main]
async fn main() -> Result<(), AniListError> {
    let token = std::env::var("ANILIST_TOKEN").expect("ANILIST_TOKEN must be set");
    let auth_client = AniListClient::with_token(token);

    // Returns User directly
    let current_user = auth_client.user().get_current_user().await?;
    let username = current_user.name.clone().unwrap_or_default();

    // Get watching list - Returns Page<Vec<MediaList>>
    let watching = auth_client.medialist()
        .get_user_anime_list(&username, Some(MediaListStatus::Current), Some(1), Some(50))
        .await?;

    println!("Currently Watching:");
    for entry in &watching.data {
        if let Some(media) = &entry.media {
            if let Some(title) = &media.title {
                println!("  - {}", title.romaji.as_deref().unwrap_or("Unknown"));
                println!(
                    "    Progress: {}/{}",
                    entry.progress.unwrap_or(0),
                    media.episodes.unwrap_or(0)
                );
            }
        }
    }

    Ok(())
}
```

### Type-Safe Response Access

```rust,no_run
use anilist_moe::{AniListClient, AniListError};

#[tokio::main]
async fn main() -> Result<(), AniListError> {
    let client = AniListClient::new();

    // Returns Media directly (not wrapped)
    let anime = client.anime().get_anime_by_id(1).await?;

    // Option handling with map/and_then
    let title = anime.title.as_ref()
        .and_then(|t| t.romaji.as_ref())
        .map(|s| s.as_str())
        .unwrap_or("Unknown");

    let score = anime.average_score.unwrap_or(0);
    let episodes = anime.episodes.unwrap_or(0);

    // Safe array access
    let genres = anime.genres.as_ref()
        .map(|g| g.join(", "))
        .unwrap_or_else(|| "No genres".to_string());

    println!("Title: {}", title);
    println!("Score: {}/100", score);
    println!("Episodes: {}", episodes);
    println!("Genres: {}", genres);

    Ok(())
}
```

## Error Handling Patterns

### Comprehensive Error Handling

```rust,no_run
use anilist_moe::errors::AniListError;
use anilist_moe::AniListClient;

#[tokio::main]
async fn main() {
    let client = AniListClient::new();

    match client.media().get_anime_by_id(16498).await {
        Ok(anime) => {
            println!("Success: {:?}", anime);
        }
        Err(AniListError::Network(e)) => {
            eprintln!("Network error: {}", e);
        }
        Err(AniListError::GraphQL { errors, .. }) => {
            eprintln!("GraphQL error: {}", errors[0].message());
            for error in &errors {
                if let Some(path) = &error.path {
                    eprintln!("  at {:?}", path);
                }
            }
        }
        Err(AniListError::Json(e)) => {
            eprintln!("JSON parsing error: {}", e);
        }
        Err(AniListError::RateLimit { retry_after, .. }) => {
            eprintln!("Rate limited - retry after {} seconds", retry_after);
        }
        Err(AniListError::RateLimitSimple) => {
            eprintln!("Rate limited - please wait before making more requests");
        }
        Err(AniListError::NotFound) => {
            eprintln!("Resource not found");
        }
        // `AniListError` is `#[non_exhaustive]`, so a catch-all arm is required.
        Err(other) => {
            eprintln!("Other error: {:?}", other);
        }
    }
}
```

## Testing Examples

### Unit Test Example

```rust,no_run
use anilist_moe::AniListClient;

// Drop these into your own crate's test module.
#[tokio::test]
async fn test_get_anime() {
    let client = AniListClient::new();
    let result = client.anime().get_anime_by_id(1).await;
    assert!(result.is_ok());

    let anime = result.unwrap();
    assert_eq!(anime.id, 1);
}

#[tokio::test]
async fn test_search_anime() {
    let client = AniListClient::new();
    let result = client.anime().search_anime("Cowboy Bebop", Some(1), Some(5)).await;
    assert!(result.is_ok());

    let response = result.unwrap();
    assert!(!response.data.is_empty());
}

fn main() {}
```
