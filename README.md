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
- **Custom Query Support**: Execute tailored GraphQL queries via the generic `AniListClient::fetch` helper
- **Conditional Field Toggles**: Fine-grained `include_*` flags let you opt into heavy nested data only when you need it
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
- **Airing Schedules**: Upcoming episodes, recently aired, date ranges, or a single schedule by id
- **Notifications**: Read, manage, and mark notifications as read
- **Media Lists**: User anime/manga lists with status tracking, a whole list grouped the way the
  site shows it (`fetch_collection`), a single entry, and custom list management

### Interactions
- **Common**: Toggle likes, toggle follows, toggle favourites on various content types, the users
  who liked something, the genre and tag lists, and favourite ordering

## Installation

Rust 1.82 or newer.

Add this to your `Cargo.toml`:

```toml
[dependencies]
anilist_moe = "0.5"
# This crate asks tokio only for its `time` feature (retry backoff); the runtime
# is yours to choose, and `full` is what a typical application wants.
tokio = { version = "1.0", features = ["full"] }
```

### Feature Flags

Every endpoint group is a cargo feature, and `default` enables `full`, so the
default build has everything. A leaner build opts out of the groups it does not
need, or adds `tracing` for request spans:

```toml
# Only the media and user endpoints.
anilist_moe = { version = "0.5", default-features = false, features = ["media", "user"] }

# Everything, plus tracing.
anilist_moe = { version = "0.5", features = ["tracing"] }
```

The available groups are `media`, `medialist`, `user`, `character`, `staff`,
`studio`, `forum`, `activity`, `review`, `recommendation`, `airing`,
`notification` and `common`; `tracing` and `pagination` are off by default.

## Quick Start

### Example 1: Get Trending Anime

```rust
use anilist_moe::{AniListClient, AniListError};

#[tokio::main]
async fn main() -> Result<(), AniListError> {
    // Create a client (no authentication needed for public data)
    let client = AniListClient::new();

    // Get trending anime with proper type safety
    let response = client.media().get_trending_anime(Some(1), Some(10)).await?;

    // Access the data with clean, simple structure (v0.3.0+)
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
use anilist_moe::{AniListClient, AniListError};

#[tokio::main]
async fn main() -> Result<(), AniListError> {
    let client = AniListClient::new();

    // Search for an anime
    let search_results = client.media().search_anime("Attack on Titan", Some(1), Some(5)).await?;

    // Access the data - search_results is Page<Vec<Media>>
    if let Some(first_result) = search_results.data.first() {
        let anime_id = first_result.id.unwrap_or(0);

        // Get detailed information by ID - returns just Media
        let anime = client.media().get_anime_by_id(anime_id).await?;

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

## Advanced Usage

### Custom GraphQL Query

```rust
use anilist_moe::{
    AniListClient,
    AniListError,
    objects::{media::Media, responses::Page},
};
use serde_json::json;
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), AniListError> {
    let client = AniListClient::new();

    let mut variables = HashMap::new();
    variables.insert("page".to_string(), json!(1));
    variables.insert("perPage".to_string(), json!(5));

    let query = r#"
        query ($page: Int, $perPage: Int) {
            Page(page: $page, perPage: $perPage) {
                media {
                    id
                    title {
                        romaji
                        english
                        native
                    }
                }
            }
        }
    "#;

    let page: Page<Vec<Media>> = client.fetch(query, Some(&variables)).await?;
    for media in page.data {
        if let Some(title) = media.title.as_ref() {
            println!("{} - {:?}", media.id, title.romaji);
        }
    }

    Ok(())
}
```

### Client Configuration

`AniListClient::new()` and `with_token(..)` cover the common cases. Anything else
goes through the builder:

```rust
use anilist_moe::{AniListClient, AniListError};
use anilist_moe::utils::RetryConfig;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), AniListError> {
    let client = AniListClient::builder()
        .token("your_access_token")
        .timeout(Duration::from_secs(60))
        .user_agent("my-app/1.0")
        .header("X-Client", "my-app")
        .retry_config(RetryConfig {
            max_retries: 5,
            jitter: true,
            ..RetryConfig::default()
        })
        .build()?;

    Ok(())
}
```

Requests go through a `Transport`. The default one uses `reqwest`; supply your own
to route them through another HTTP stack, or to answer them from memory in tests:

```rust
use anilist_moe::transport::{HttpRequest, HttpResponse, Transport};
use anilist_moe::{AniListClient, AniListError};
use std::future::Future;
use std::pin::Pin;

#[derive(Debug)]
struct Scripted;

impl Transport for Scripted {
    fn execute(
        &self,
        _request: HttpRequest,
    ) -> Pin<Box<dyn Future<Output = Result<HttpResponse, AniListError>> + Send + '_>> {
        Box::pin(async { Ok(HttpResponse::new(200, r#"{"data":{"Page":{"media":[],"pageInfo":{}}}}"#)) })
    }
}

let client = AniListClient::builder().transport(Scripted).build().unwrap();
```

### Conditional Fetching Controls

```rust
use anilist_moe::{options, AniListClient, AniListError};
use anilist_moe::endpoints::media::FetchMediaOptions;
use anilist_moe::enums::media::{MediaSort, MediaType};

#[tokio::main]
async fn main() -> Result<(), AniListError> {
    let client = AniListClient::new();

    let options = options!(
        FetchMediaOptions,
        media_type: MediaType::Anime,
        sort: &[MediaSort::PopularityDesc],
        page: 1,
        per_page: 3,
        include_characters: true,
        include_staff: true,
        include_reviews: true,
        include_recommendations: true,
    );

    let response = client.media().fetch(&options).await?;
    for media in response.data {
        println!(
            "{} has {} character edges",
            media.title
                .as_ref()
                .and_then(|t| t.romaji.as_ref())
                .unwrap_or(&"Unknown".to_string()),
            media
                .characters
                .as_ref()
                .and_then(|c| c.edges.as_ref())
                .map(|edges| edges.len())
                .unwrap_or(0)
        );
    }

    Ok(())
}
```

## Detailed Usage

### Anime Operations

```rust
use anilist_moe::{AniListClient, AniListError};

#[tokio::main]
async fn main() -> Result<(), AniListError> {
    let client = AniListClient::new();

    // Get popular anime - returns Page<Vec<Media>>
    let popular = client.media().get_popular_anime(Some(1), Some(5)).await?;
    for anime in &popular.data {
        println!("{}", anime.title.as_ref().unwrap().romaji.as_ref().unwrap());
    }

    // Get trending anime
    let trending = client.media().get_trending_anime(Some(1), Some(5)).await?;

    // Get anime by ID - returns Media directly
    let anime = client.media().get_anime_by_id(16498).await?; // Attack on Titan

    // Search anime
    let search_results = client.media().search_anime("Naruto", Some(1), Some(10)).await?;

    // Get anime by season
    use anilist_moe::enums::media::MediaSeason;
    let fall_2023 = client.media().get_by_season(MediaSeason::Fall, 2023, Some(1), Some(10)).await?;

    // Get top rated anime
    let top_rated = client.media().get_top_rated_anime(Some(1), Some(10)).await?;

    // Get currently airing anime
    let airing = client.media().get_airing_anime(Some(1), Some(10)).await?;

    Ok(())
}
```

### Manga Operations

```rust
use anilist_moe::{AniListClient, AniListError};

#[tokio::main]
async fn main() -> Result<(), AniListError> {
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
use anilist_moe::{AniListClient, AniListError};

#[tokio::main]
async fn main() -> Result<(), AniListError> {
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
use anilist_moe::{AniListClient, AniListError};

#[tokio::main]
async fn main() -> Result<(), AniListError> {
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
use anilist_moe::{AniListClient, AniListError};

#[tokio::main]
async fn main() -> Result<(), AniListError> {
    // Create an authenticated client
    let token = std::env::var("ANILIST_TOKEN")?;
    let client = AniListClient::with_token(&token);

    // Get current user information - returns User directly
    let current_user = client.user().get_current_user().await?;
    println!("Logged in as: {}", current_user.name.unwrap_or_default());

    // Get current user's anime list - returns Page<Vec<MediaList>>
    let anime_list = client.medialist()
        .get_user_anime_list(&current_user.name.unwrap_or_default(), None, Some(1), Some(50))
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

    match client.media().get_anime_by_id(999999).await {
        Ok(anime) => println!("Found anime: {:?}", anime),
        Err(AniListError::Network(e)) => eprintln!("Network error: {}", e),
        Err(AniListError::GraphQL { errors, data }) => {
            eprintln!("GraphQL error: {}", errors[0].message());
            if let Some(partial) = data {
                eprintln!("Partial data: {:?}", partial);
            }
        }
        Err(AniListError::Json(e)) => eprintln!("JSON parsing error: {}", e),
        Err(AniListError::RateLimit { limit, remaining, reset_at, retry_after }) => {
            eprintln!("Rate limited - retry after {} seconds", retry_after);
        }
        Err(AniListError::RateLimitSimple) => eprintln!("Rate limited"),
        Err(AniListError::BurstLimit) => eprintln!("Too many requests too quickly"),
        Err(AniListError::NotFound) => eprintln!("Not found"),
        Err(AniListError::AuthenticationRequired) => eprintln!("Authentication required"),
        Err(AniListError::AccessDenied) => eprintln!("Access denied"),
        Err(e) => eprintln!("Error: {:?}", e),
    }
}
```

An outage is its own variant, so an application can say "AniList is down" instead
of showing a raw failure: a `503`, or a body that reads like a maintenance notice,
becomes `AniListError::Maintenance`. A status this crate does not map to anything
more specific becomes `AniListError::UnexpectedStatus` with the code in it.

Each GraphQL error keeps the details the server sent, so callers can branch on a
field path or an extension code instead of matching on a string:

```rust
use anilist_moe::{AniListClient, errors::AniListError};

#[tokio::main]
async fn main() {
    let client = AniListClient::new();

    if let Err(error) = client.media().fetch_one(&anilist_moe::options!(
        anilist_moe::endpoints::media::FetchMediaOneOptions,
        id: 999_999_999,
    )).await {
        if let Some(errors) = error.graphql_errors() {
            for entry in errors {
                eprintln!("{} at {:?}", entry.message(), entry.path);
            }
        }
        if error.graphql_message_contains("cannot query field") {
            eprintln!("the query asked for a field that does not exist");
        }
    }
}
```

## Type Safety

All responses are fully typed. No more dealing with `serde_json::Value`:

```rust
// Fully typed response - Page<Vec<Media>>
let response = client.media().get_trending_anime(Some(1), Some(10)).await?;

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

### Forward compatibility

AniList grows. Every enum is `#[non_exhaustive]` and carries an `Unknown`
catch-all that deserialization falls back to, so a `MediaFormat`, `MediaStatus` or
`__typename` this crate has not heard of yet is a value you can see rather than a
response that fails to parse:

```rust
match media.format {
    Some(MediaFormat::Movie) => println!("a film"),
    Some(MediaFormat::Unknown) => println!("a format this version does not know"),
    Some(other) => println!("{other:?}"),
    None => println!("no format"),
}
```

`Unknown` is only ever produced by deserialization — never send it as input. The
same holds for the response unions: match `ActivityUnion`, `LikeableUnion` or
`NotificationUnion` with a trailing `_` arm.

## Testing

Run the test suite:

```bash
cargo test
```

Run specific endpoint tests:

```bash
cargo test --test endpoint_tests
```

The offline tests (formatting, clippy, unit tests, scripted transport tests, and
doctests) run on every push and pull request. The live endpoint tests call the
real AniList API and are rate limited, so CI only runs them on a schedule, on a
manual `workflow_dispatch`, or on a pull request labelled `run-live-tests`.

The library includes comprehensive tests for:
- All endpoint methods
- Error handling scenarios
- Pagination functionality
- Type safety verification

## Rate Limiting

The AniList API has rate limiting (90 requests per minute). The client does not
retry by default: a `429`, a `5xx` or a dropped connection comes back to you as an
error, and your application decides what to do with it. That keeps the client's
policy out of the request path, where it cannot know your budget or your
cancellation story.

```rust
use anilist_moe::{AniListClient, errors::AniListError};
use std::time::Duration;
use tokio::time::sleep;

#[tokio::main]
async fn main() -> Result<(), AniListError> {
    let client = AniListClient::new();

    for page in 1..=10 {
        match client.media().get_popular_anime(Some(page), Some(50)).await {
            Ok(response) => {
                // Process response
            }
            Err(AniListError::RateLimit { retry_after, .. }) => {
                // Wait and retry
                sleep(Duration::from_secs(retry_after as u64)).await;
                continue;
            }
            Err(AniListError::RateLimitSimple) => {
                // Wait and retry
                sleep(Duration::from_secs(60)).await;
                continue;
            }
            Err(e) => return Err(e),
        }

        // Be nice to the API
        sleep(Duration::from_millis(700)).await;
    }

    Ok(())
}
```

If you would rather have the client retry for you, opt in with a `RetryConfig`:

```rust
use anilist_moe::{AniListClient, utils::RetryConfig};

// Retries rate limits with a one minute base delay.
let client = AniListClient::new().with_retry_config(RetryConfig::for_rate_limits());

// Retries rate limits, 5xx responses and transport errors, up to 5 times.
let client = AniListClient::new().with_retry_config(RetryConfig::aggressive());
```

## Pagination

All list endpoints support pagination:

```rust
use anilist_moe::{AniListClient, AniListError};

#[tokio::main]
async fn main() -> Result<(), AniListError> {
    let client = AniListClient::new();

    let mut page = 1;
    let per_page = 50;

    loop {
        let response = client.media().get_popular_anime(Some(page), Some(per_page)).await?;

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

### Stable paging

AniList sorts on its database columns as they are, so two rows that share the
sorted value have no defined order between them — and an order that changes
between requests makes a walk repeat some rows and miss others. The client
therefore appends the row's unique key to every sort:

| You pass | Sent to AniList |
|---|---|
| nothing | `[ID]` |
| `&[MediaSort::ScoreDesc]` | `[SCORE_DESC, ID]` |
| `&[MediaSort::ScoreDesc, MediaSort::StartDateDesc]` | `[SCORE_DESC, START_DATE_DESC, ID]` |
| `&[MediaSort::ScoreDesc, MediaSort::Id]` | `[SCORE_DESC, ID]` (no duplicate) |

Media list entries have no `ID` of their own, so they are tied by `MEDIA_ID`. Two
queries declare their own sort default and keep it, because it is already total:
the activity feed (`[ID_DESC]`, newest first) and the airing schedule
(`[TIME, ID]`). `import anilist_moe::options::SortSpec` if you need the tiebreaker
key for a sort type of your own.

### Walking every page

With the `pagination` feature, `AniListClient::paginate` walks a list for you. It
asks for one page at a time — nothing is prefetched — and stops when the server
says there is no next page, when a page comes back empty, or at a cap of 1000
pages:

```rust
use anilist_moe::{options, AniListClient, AniListError};
use anilist_moe::endpoints::media::FetchMediaOptions;
use anilist_moe::enums::media::MediaType;

#[tokio::main]
async fn main() -> Result<(), AniListError> {
    let client = AniListClient::new();

    // Entry by entry.
    let mut items = client.paginate(options!(
        FetchMediaOptions,
        media_type: MediaType::Anime,
        per_page: 50,
    ));

    while let Some(anime) = items.next_item().await {
        let anime = anime?;
        println!("{:?}", anime.title);
    }

    // Or page by page, which keeps the page number next to the entries.
    let mut pages = client.paginate(options!(
        FetchMediaOptions,
        media_type: MediaType::Anime,
        per_page: 50,
    ));

    while let Some(page) = pages.next_page().await {
        let page = page?;
        println!("page of {} entries", page.data.len());
    }

    Ok(())
}
```

Nothing is retried and nothing sleeps between pages, as everywhere else in the
crate: if a `429` is worth waiting out, that is your loop's decision. The walk is
cancel-safe — dropping the cursor stops the requests.

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