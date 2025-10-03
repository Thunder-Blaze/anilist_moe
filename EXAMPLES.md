# AniList.moe Examples

Comprehensive examples for using the anilist_moe crate.

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

```rust
use anilist_moe::client::AniListClient;

// Unauthenticated client for public data
let client = AniListClient::new();

// Authenticated client for user-specific operations
let token = std::env::var("ANILIST_TOKEN").expect("Token required");
let auth_client = AniListClient::with_token(token);
```

## Anime Examples

### Get Trending Anime

```rust
use anilist_moe::client::AniListClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = AniListClient::new();

    let response = client.anime().get_trending(Some(1), Some(10)).await?;

    for anime in &response.data.page.data.media {
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

```rust
use anilist_moe::client::AniListClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = AniListClient::new();

    let search_query = "Steins Gate";
    let response = client.anime().search(search_query, Some(1), Some(10)).await?;

    for anime in &response.data.page.data.media {
        if let Some(title) = &anime.title {
            println!("Found: {} (ID: {})",
                title.romaji.as_ref().unwrap_or(&"Unknown".to_string()),
                anime.id
            );
        }
    }

    Ok(())
}
```

### Get Anime by Season

```rust
use anilist_moe::client::AniListClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = AniListClient::new();

    // Get Fall 2024 anime
    let response = client.anime()
        .get_by_season("FALL", 2024, Some(1), Some(20))
        .await?;

    println!("Fall 2024 Anime:");
    for anime in &response.data.page.data.media {
        if let Some(title) = &anime.title {
            println!("  - {}", title.romaji.as_ref().unwrap_or(&"Unknown".to_string()));
        }
    }

    Ok(())
}
```

### Get Detailed Anime Information

```rust
use anilist_moe::client::AniListClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = AniListClient::new();

    // Attack on Titan ID: 16498
    let response = client.anime().get_by_id(16498).await?;
    let anime = &response.data.media;

    println!("=== Anime Details ===");

    if let Some(title) = &anime.title {
        println!("Title (Romaji): {}", title.romaji.as_ref().unwrap_or(&"N/A".to_string()));
        println!("Title (English): {}", title.english.as_ref().unwrap_or(&"N/A".to_string()));
        println!("Title (Native): {}", title.native.as_ref().unwrap_or(&"N/A".to_string()));
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

```rust
use anilist_moe::client::AniListClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = AniListClient::new();

    let response = client.manga().get_top_rated(Some(1), Some(10)).await?;

    println!("Top Rated Manga:");
    for (i, manga) in response.data.page.data.media.iter().enumerate() {
        if let Some(title) = &manga.title {
            println!("{}. {} - Score: {}/100",
                i + 1,
                title.romaji.as_ref().unwrap_or(&"Unknown".to_string()),
                manga.average_score.unwrap_or(0)
            );
        }
    }

    Ok(())
}
```

### Get Currently Releasing Manga

```rust
use anilist_moe::client::AniListClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = AniListClient::new();

    let response = client.manga().get_releasing(Some(1), Some(20)).await?;

    println!("Currently Releasing Manga:");
    for manga in &response.data.page.data.media {
        if let Some(title) = &manga.title {
            println!("  - {}", title.romaji.as_ref().unwrap_or(&"Unknown".to_string()));
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

```rust
use anilist_moe::client::AniListClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = AniListClient::new();

    let response = client.character().search("Luffy", Some(1), Some(5)).await?;

    for character in &response.data.page.data.characters {
        if let Some(name) = &character.name {
            println!("Character: {}", name.full.as_ref().unwrap_or(&"Unknown".to_string()));
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

```rust
use anilist_moe::client::AniListClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = AniListClient::new();

    // Get characters with birthday on May 5th
    let response = client.character().get_by_birthday(5, 5, Some(1), Some(10)).await?;

    println!("Characters born on May 5th:");
    for character in &response.data.page.data.characters {
        if let Some(name) = &character.name {
            println!("  - {}", name.full.as_ref().unwrap_or(&"Unknown".to_string()));
        }
    }

    Ok(())
}
```

## Staff Examples

### Get Popular Staff

```rust
use anilist_moe::client::AniListClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = AniListClient::new();

    let response = client.staff().get_popular(Some(1), Some(10)).await?;

    println!("Popular Staff:");
    for staff in &response.data.page.data.staff {
        if let Some(name) = &staff.name {
            println!("  - {}", name.full.as_ref().unwrap_or(&"Unknown".to_string()));
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

```rust
use anilist_moe::client::AniListClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = AniListClient::new();

    let response = client.user().get_by_username("username").await?;
    let user = &response.data.user;

    println!("User: {}", user.name);
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

```rust
use anilist_moe::client::AniListClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let token = std::env::var("ANILIST_TOKEN")?;
    let client = AniListClient::with_token(token);

    let response = client.user().get_current_user().await?;
    let user = &response.data.viewer;

    println!("Logged in as: {}", user.name);
    println!("User ID: {}", user.id);

    if let Some(unread) = user.unread_notification_count {
        println!("Unread notifications: {}", unread);
    }

    Ok(())
}
```

## Forum Examples

### Browse Forum Threads

```rust
use anilist_moe::client::AniListClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = AniListClient::new();

    // Get recent threads
    let response = client.forum().get_recent(Some(1), Some(10)).await?;

    println!("Recent Forum Threads:");
    for thread in &response.data.page.data.threads {
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

```rust
use anilist_moe::client::AniListClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = AniListClient::new();

    let response = client.forum().search("recommendation", Some(1), Some(10)).await?;

    println!("Search Results:");
    for thread in &response.data.page.data.threads {
        if let Some(title) = &thread.title {
            println!("  - {}", title);
        }
    }

    Ok(())
}
```

### Get Thread Comments

```rust
use anilist_moe::client::AniListClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = AniListClient::new();

    let thread_id = 12345;
    let response = client.forum().get_thread_comments(thread_id, Some(1), Some(20)).await?;

    println!("Comments:");
    for comment in &response.data.page.data.thread_comments {
        if let Some(user) = &comment.user {
            println!("\n  User: {}", user.name);
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

```rust
use anilist_moe::client::AniListClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = AniListClient::new();

    let response = client.activity().get_recent(Some(1), Some(20)).await?;

    for activity in &response.data.page.data.activities {
        match activity {
            crate::unions::activity::ActivityUnion::TextActivity(text) => {
                if let Some(user) = &text.user {
                    println!("Text Activity by {}", user.name);
                }
                if let Some(text_content) = &text.text {
                    println!("  {}", text_content);
                }
            },
            crate::unions::activity::ActivityUnion::MessageActivity(msg) => {
                if let Some(messenger) = &msg.messenger {
                    println!("Message by {}", messenger.name);
                }
            },
            crate::unions::activity::ActivityUnion::ListActivity(list) => {
                if let Some(user) = &list.user {
                    println!("List Activity by {}", user.name);
                }
            },
        }
    }

    Ok(())
}
```

## Advanced Examples

### Pagination with Error Handling

```rust
use anilist_moe::client::AniListClient;
use anilist_moe::errors::AniListError;
use std::time::Duration;
use tokio::time::sleep;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = AniListClient::new();
    let mut page = 1;
    let per_page = 50;
    let max_pages = 5;

    while page <= max_pages {
        match client.anime().get_popular(Some(page), Some(per_page)).await {
            Ok(response) => {
                let page_info = &response.data.page.page_info;

                // Process anime
                for anime in &response.data.page.data.media {
                    if let Some(title) = &anime.title {
                        println!("Anime: {}", title.romaji.as_ref().unwrap_or(&"Unknown".to_string()));
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
            }
            Err(AniListError::RateLimit) => {
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

```rust
use anilist_moe::client::AniListClient;
use tokio::try_join;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = AniListClient::new();

    // Fetch multiple things concurrently
    let (anime, manga, characters) = try_join!(
        client.anime().get_popular(Some(1), Some(5)),
        client.manga().get_popular(Some(1), Some(5)),
        client.character().get_popular(Some(1), Some(5))
    )?;

    println!("Fetched:");
    println!("  {} anime", anime.data.page.data.media.len());
    println!("  {} manga", manga.data.page.data.media.len());
    println!("  {} characters", characters.data.page.data.characters.len());

    Ok(())
}
```

### Building a Simple Anime Tracker

```rust
use anilist_moe::client::AniListClient;
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = AniListClient::new();

    // Get user's anime list (requires authentication)
    let token = std::env::var("ANILIST_TOKEN")?;
    let auth_client = AniListClient::with_token(token);

    let current_user = auth_client.user().get_current_user().await?;
    let username = &current_user.data.viewer.name;

    // Get watching list
    let watching = auth_client.medialist()
        .get_user_anime_list(username, Some("CURRENT"), Some(1), Some(50))
        .await?;

    println!("Currently Watching:");
    for entry in &watching.data.page.data.media_list {
        if let Some(media) = &entry.media {
            if let Some(title) = &media.title {
                println!("  - {}", title.romaji.as_ref().unwrap_or(&"Unknown".to_string()));
                println!("    Progress: {}/{}",
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

```rust
use anilist_moe::client::AniListClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = AniListClient::new();

    // All responses are fully typed
    let response = client.anime().get_by_id(1).await?;

    // Type-safe access to nested data
    let anime = &response.data.media;

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

```rust
use anilist_moe::client::AniListClient;
use anilist_moe::errors::AniListError;

#[tokio::main]
async fn main() {
    let client = AniListClient::new();

    match client.anime().get_by_id(999999).await {
        Ok(response) => {
            println!("Success: {:?}", response);
        }
        Err(AniListError::Network(e)) => {
            eprintln!("Network error: {}", e);
        }
        Err(AniListError::GraphQL { message, status, errors }) => {
            eprintln!("GraphQL error: {}", message);
            if let Some(code) = status {
                eprintln!("Status code: {}", code);
            }
            if let Some(errs) = errors {
                eprintln!("Errors: {:?}", errs);
            }
        }
        Err(AniListError::Json(e)) => {
            eprintln!("JSON parsing error: {}", e);
        }
        Err(AniListError::RateLimit) => {
            eprintln!("Rate limited - please wait before making more requests");
        }
        Err(AniListError::NotFound) => {
            eprintln!("Resource not found");
        }
    }
}
```

## Testing Examples

### Unit Test Example

```rust
#[cfg(test)]
mod tests {
    use anilist_moe::client::AniListClient;

    #[tokio::test]
    async fn test_get_anime() {
        let client = AniListClient::new();
        let result = client.anime().get_by_id(1).await;
        assert!(result.is_ok());

        let response = result.unwrap();
        assert_eq!(response.data.media.id, 1);
    }

    #[tokio::test]
    async fn test_search_anime() {
        let client = AniListClient::new();
        let result = client.anime().search("Cowboy Bebop", Some(1), Some(5)).await;
        assert!(result.is_ok());

        let response = result.unwrap();
        assert!(!response.data.page.data.media.is_empty());
    }
}
```
