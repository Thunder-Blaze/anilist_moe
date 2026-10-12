# Changelog

All notable changes to this project will be documented in this file.

## Versioning

This project uses [Semantic Versioning](https://semver.org/):
- **MAJOR** version for incompatible API changes
- **MINOR** version for new functionality in a backwards compatible manner
- **PATCH** version for backwards compatible bug fixes

## [0.5.0] - 2026-10-12

This release comes from the `critical` line, which implemented several of the
features 0.4.0 shipped differently. Consumers upgrading from 0.4.0 should read the
`Changed` list: `use_jitter` became `jitter`, `retry_on_server_error` became
`retry_transient_errors`, `GraphQLErrorItem` became `GraphQLError`, sort lists went
from `Vec` to `Cow`, `Media.id` became required, and retries are now off unless
asked for.

### Added
- `options!` macro and the `options` module, for building endpoint options without
  spelling out every `Some(..)` wrapper
- `Transport` trait and `ReqwestTransport`, so requests can go through another
  HTTP stack or be answered without a network
- `AniListClientBuilder`, and `AniListClient::builder()`, for token, base URL,
  timeout, user agent, retry configuration and transport
- `GraphQLError` with the message, path, locations and extensions of each error
  the server returned, plus `AniListError::graphql_errors` and
  `AniListError::graphql_message_contains`
- `RetryConfig::jitter` (spreads delays by up to 50%) and
  `RetryConfig::retry_transient_errors` (5xx and transport failures)
- per-endpoint cargo features, for builds that only need some of the API
- optional `tracing` feature
- optional `pagination` feature: `AniListClient::paginate(options)` returns a
  cursor that walks the pages of a list query, yielding pages or single entries,
  capped so a query that never says "no more" cannot spin
- `AniListError::kind`, a stable label for logs and spans
- The `tracing` query span names the operation and the options struct it sent
  (`operation=GetMedia variables=FetchMediaOptions`), and `RUST_LOG=anilist_moe=trace`
  shows the request body
- `AniListClientBuilder::header` / `headers`, for extra headers on every request,
  handed to whichever transport is in use
- `AniListClient::try_new` and `try_with_token`, so a client can be built without
  the panicking shorthand
- `AniListError::Maintenance` and `detect_maintenance`, so an outage is told apart
  from a query that needs fixing (a `503`, or a body that reads like a notice)
- `AniListError::UnexpectedStatus` for a status this crate does not map to
  anything more specific (a `405`, a `418`)

### Changed
- Endpoint option fields take `Cow` instead of a borrowed reference, so the same
  call can pass literals (no allocation) or owned data (a `'static` value that can
  be moved into a spawned task)
- `Media.id` is required (`u32`), `id_mal` is `Option<u32>` and `FuzzyDate` months
  and days are `u32`
- `*Options` structs no longer implement `Deserialize` (they are request inputs)
- `ReviewEndpoint::get_by_id` returns `AniListError::NotFound` instead of panicking
  when the review does not exist
- `AniListError::GraphQL` carries `errors` and any partial `data` instead of a
  single joined message
- Retries are off by default: `RetryConfig::default()` retries nothing, and
  `RetryConfig::for_rate_limits()` / `aggressive()` are how a client opts in
- `RetryConfig` gained `jitter` and `retry_transient_errors`, so its literals need
  the new values or `..Default::default()`
- CI runs the offline tests (formatting, clippy, unit, scripted-transport and doc
  tests) on every push and pull request, and the live API tests only on a
  schedule, on `workflow_dispatch`, or on a pull request labelled `run-live-tests`
- A `400` whose body carries a GraphQL `errors` array is reported as
  `AniListError::GraphQL` with its structure, instead of as a raw body in
  `BadRequest`
- `AniListError` is `#[non_exhaustive]`, so a future variant is not a breaking
  change: match with a trailing `_ =>` arm
- The MSRV is declared as Rust 1.82, and docs.rs builds every feature so the
  optional APIs are documented

### Dependencies
- Floors are the current releases: reqwest 0.13, thiserror 2, serde 1.0.229,
  serde_json 1.0.154, serde_with 3.24, tokio 1.53, chrono 0.4.45, log 0.4.34,
  tracing 0.1.44, parking_lot 0.12.5, tracing-subscriber 0.3.23, husky-rs 0.4
- Those floors move the MSRV to **1.88** (`serde_with` needs it, `reqwest` needs
  1.85), and reqwest 0.13 renamed `rustls-tls` to `rustls`, which now pulls
  `aws-lc-rs`: building the crate needs a C toolchain and CMake, as reqwest's own
  default does
- reqwest 0.13 also verifies certificates against the platform trust store
  (`rustls-platform-verifier`) instead of the bundled roots it used before

### Fixed
- Every sort ends in the unique key of the row: a sort you pass is sent as
  `[yours…, ID]` (or `MEDIA_ID` for media list entries, which have no `ID`), and
  an endpoint you leave unsorted is sent as `[ID]`. Sorting on a non-unique column
  alone let paging repeat or skip rows (issue #19)
- `FetchStudioOptions` sent `null` for unset fields, which made every studio fetch
  fail with a server error
- The airing convenience methods read the clock with `unwrap`, which panicked on a
  system clock set before 1970
- `Page<T>` refused a response with a second unknown top-level key, so a field
  added to AniList's `Page` type would have broken every paginated call

## [0.4.0] - 2026-06-04

### Added
- **AniListClientBuilder**: Introduced a new builder pattern for configuring client properties (e.g., token, base URL, retry config).
- **Retry & Backoff Mechanism**: Added configurable `RetryConfig` supporting max retries, exponential backoff, jitter, and rate-limit awareness.
- **Tracing Support**: Integrated optional `tracing` feature flag along with structured logging.
- **CI/CD Infrastructure**: Added workflows for PR checks and reviews, and introduced a `rust-toolchain.toml` file.
- **License**: Added MIT License.
- **HTML Option**: Added manual option to opt for HTML content.

### Changed
- **Trace Formatting**: Enhanced request tracing to output request URL, options struct name (e.g., `FetchMediaOptions`), and variables payload, while omitting raw GraphQL query text.
- **Retry config initialization**: Updated tests to support new fields `use_jitter` and `retry_on_server_error` in `RetryConfig`.

### Fixed
- **Tracing Concurrency Bug**: Fixed a compilation bug where `!Send` entered span guards were held across `.await` points in async code by leveraging `.instrument(span)`.
- **GraphQL Error Parsing on HTTP 400**: Enhanced `handle_http_error` to parse structured GraphQL validation errors from HTTP 400 responses instead of returning raw JSON string messages.
- **Time Panic Safety**: Avoided panic risks in airing convenience methods when the system clock is set to before 1970.
- **Enums & Tests**: Fixed `NotificationType` enum variants and activity/forum integration tests.
- **Clippy**: Resolved collapsible match linter warning in `src/utils.rs`.

## [0.3.3] - 2026-03-05

### Fixed
- **Documentation**: Fixed major mistakes in documentation.
- **Typename fix**: Typename fix for notifications and activities.

## [0.3.2] - 2024-01-22

### Fixed
- **Critical**: Fixed invalid Rust edition in `Cargo.toml` from "2024" to "2021"
- **Documentation**: Updated README.md to correct method names and examples
  - Fixed `client.anime().get_trending()` → `client.media().get_trending_anime()`
  - Fixed `client.anime().get_by_id()` → `client.media().get_anime_by_id()`
  - Corrected version number in installation instructions to "0.3"
  - Updated error handling examples to show all error variants
  - Fixed response access patterns in all examples
- **Documentation**: Updated CONTRIBUTING.md with correct clippy command syntax
- **Documentation**: Clarified that pre-commit hooks are for developers only

## [0.3.1] - 2024-01-XX

### Added
- **Custom Query Support**: Added a generic `AniListClient::fetch` helper for executing user-defined GraphQL queries with typed responses.
- **GraphQL Query Modules**: Introduced dedicated `.graphql` documents for media, character, staff, studio, and related fetch operations.
- **Testing Infrastructure**: Added a rate-limit-aware test harness plus new conditional fetching and custom query integration tests.

### Changed
- **Endpoint Options**: Fetch option structs now derive `Deserialize`, enabling round-tripping and fixture-driven testing.
- **Media Endpoint Refactor**: Consolidated pagination helpers, expanded include flags, and streamlined media convenience methods.
- **Inline Documentation**: Refined comments across endpoints, utilities, and tests for clearer guidance.

### Fixed
- **Doctests**: Resolved doctest failures triggered by the media endpoint refactor.

## [0.3.0] - 2024-10-14

### Added
- **Husky Pre-commit Hooks**: Added husky-rs for automated code quality checks
  - Runs `cargo fmt --check` before commits
  - Runs `cargo clippy -- -D warnings` before commits
  - Ensures consistent code style and catches common issues

### Changed
- **Response Format Simplification**: Major refactoring of response types for cleaner API
  - Removed redundant wrapper types (`PageResponse`, `ViewerResponse`, etc.)
  - Simplified response access patterns: `response.data` instead of nested wrappers
  - All endpoints now use `GraphQLResponse<T>` and `Page<T>` directly
  - Example: `GraphQLResponse<Page<Vec<Media>>>` instead of `GraphQLResponse<PageResponse<MediaData>>`
- **Common Endpoint**: Simplified return types
  - `toggle_like()` now returns `LikeableUnion` directly instead of `ToggleLikeResponse`
  - `toggle_follow()` now returns `User` directly instead of `ToggleFollowResponse`
  - `toggle_favourite()` now returns `Favourites` directly instead of `ToggleFavouriteResponse`
- **Review Endpoint**: Simplified delete operation
  - `delete()` now returns `bool` directly
  - Uses `Deleted` struct internally for consistency
- **User Endpoint**: Fixed authenticated user fetch
  - `fetch_basic()` now returns `User` directly instead of `ViewerUserData`
  - Simplified viewer query responses

### Fixed
- **MediaExternalLink**: Made all fields public for proper access (#4 by @Nachtalb, fixed #3)
  - Fields `id`, `url`, `site`, `type`, `language`, `color`, `icon`, and `isDisabled` are now public
- **ListActivity**: Made `status` field public for proper access
- **MediaConnection**: Made `edges`, `nodes`, and `page_info` fields public
- **MediaEdge**: Made `node`, `id`, and `relation_type` fields public
- **ThreadComment**: Removed extra blank line for code consistency

### Breaking Changes
- Response structure changed from `response.data.page.data.media` to `response.data` for paginated endpoints
- Response wrapper types removed - direct access to data types
- All endpoints return simplified response types

## [0.2.2] - 2024-10-13

### Fixed
- **Authenticated User Fetch**: Fixed issue with fetching current authenticated user
- Added `color` and `main_studio` fields in media fetch operations

## [0.2.1] - 2024-10-13

### Fixed
- **Critical Hotfix**: Fixed authenticated user fetch functionality
  - Resolved issue where fetching the current authenticated user would fail
  - Updated user query to properly handle viewer context

## [0.2.0] - 2024-10

### Added
- **Type Safety**: All endpoints now return properly typed responses instead of `serde_json::Value`
  - Added typed response structs for all endpoint operations
  - Created consistent response wrapper pattern across all endpoints
- **MediaList Endpoint**: Complete media list management functionality
  - `fetch()` - Fetch media list entries with filtering
  - `save()` - Create or update single media list entry
  - `save_multiple()` - Bulk update multiple entries
  - `delete()` - Delete media list entry
- **Common Endpoint**: Social interaction operations
  - `toggle_like()` - Like/unlike activities, threads, comments
  - `toggle_follow()` - Follow/unfollow users
  - `toggle_favourite()` - Add/remove favorites (anime, manga, characters, staff, studios)
  - Convenience methods: `like_activity()`, `like_thread()`, `like_thread_comment()`, etc.
- **Convenience Functions**: 80+ helper methods across all endpoints
  - Media: `get_popular_anime()`, `get_trending_anime()`, `get_airing_anime()`, `search_anime()`, etc.
  - Character: `get_popular()`, `get_by_birthday()`, `get_most_favorited()`, etc.
  - Staff: `get_popular()`, `get_by_birthday()`, `get_most_favorited()`, etc.
  - User: `get_by_username()`, `get_followers()`, `get_following()`, `get_favorites()`, etc.
  - Activity: `get_recent()`, `get_following_feed()`, `get_by_user()`, etc.
  - Forum: `get_recent()`, `get_popular()`, `search()`, `get_by_category()`, etc.
  - Review: `get_recent()`, `get_by_media()`, `get_by_user()`, etc.
  - Recommendation: `get_recent()`, `get_by_media()`, etc.
  - Airing: `get_upcoming()`, `get_recently_aired()`, etc.
  - Notification: `get_unread()`, etc.
- **Client Enhancements**:
  - Added `anime()` convenience alias for `media()` endpoint
  - Added `manga()` convenience alias for `media()` endpoint
  - Improved authentication token management
  - Better error handling with typed errors
- **Documentation**: Comprehensive inline documentation
  - Added concise doc comments to all 13 endpoint structs
  - Documented all 49 option structs across all endpoints
  - Added module-level documentation for core modules (endpoints, objects, enums, unions, queries)
  - Documented all utility functions in `utils.rs`
  - Added PR template in `.github/PULL_REQUEST_TEMPLATE.md`
- **Documentation Files**:
  - `README.md` - Complete project documentation with examples
  - `EXAMPLES.md` - 25+ working code examples covering all endpoints
  - `CONTRIBUTING.md` - Developer contribution guidelines
  - `DOCUMENTATION.md` - Documentation summary and overview
  - `DOC_COMMENTS_COMPLETE.md` - Summary of inline documentation work

### Changed
- **Breaking**: Forum endpoints now return typed responses
  - `fetch()` returns `ThreadListResponse` instead of `Value`
  - `fetch_one()` returns `ThreadSingleResponse` instead of `Value`
  - `fetch_comments()` returns `ThreadCommentListResponse` instead of `Value`
  - `save()` returns `SaveThreadResponse` instead of `Value`
- **Breaking**: MediaList endpoints now return typed responses
  - `fetch()` returns `MediaListResponse` instead of `Value`
  - `save()` returns `SaveMediaListResponse` instead of `Value`
- **Breaking**: All response access patterns updated
  - Old: `response["data"]["Page"]["media"]`
  - New: `response.data.page.data.media`
- **Improved**: Enum naming conventions
  - Changed from SCREAMING_SNAKE_CASE to PascalCase
  - Example: `MediaType::ANIME` → `MediaType::Anime`
  - Affects all enums: MediaType, MediaFormat, MediaStatus, MediaSeason, etc.
- Improved documentation structure and consistency across all modules

### Fixed
- **ActivityUnion Deserialization**: Fixed parsing issues
  - Changed from `#[serde(untagged)]` to `#[serde(tag = "__typename")]`
  - Now correctly identifies MessageActivity, TextActivity, and ListActivity
- **MediaList Tests**: Fixed all medialist endpoint tests
- **Activity Tests**: Fixed activity endpoint tests
- **Common Module**: Fixed like/follow/favorite operations
- **Type Safety**: Eliminated all `serde_json::Value` returns in public APIs

### Removed
- Removed old backup test files and examples
- Cleaned up deprecated code and unused dependencies

### Migration Guide from 0.1.x to 0.2.0

#### Response Type Changes
```rust
// Before (0.1.x) - untyped JSON
let response: Value = client.forum().fetch(options).await?;
let threads = response["data"]["Page"]["threads"].as_array().unwrap();

// After (0.2.0) - strongly typed
let response: ThreadListResponse = client.forum().fetch(options).await?;
let threads = &response.data.page.data.threads;
```

#### Enum Variant Changes
```rust
// Before (0.1.x)
let options = FetchMediaOptions {
    type_: Some(MediaType::ANIME),
    format: Some(MediaFormat::TV),
    status: Some(MediaStatus::FINISHED),
    ..Default::default()
};

// After (0.2.0)
let options = FetchMediaOptions {
    type_: Some(MediaType::Anime),
    format: Some(MediaFormat::Tv),
    status: Some(MediaStatus::Finished),
    ..Default::default()
};
```

#### Using Convenience Functions
```rust
// Before (0.1.x) - manual option construction
let response = client.media().fetch(FetchMediaOptions {
    sort: Some(vec![MediaSort::Popularity]),
    type_: Some(MediaType::Anime),
    page: Some(1),
    per_page: Some(10),
    ..Default::default()
}).await?;

// After (0.2.0) - convenience function
let response = client.anime().get_popular_anime(Some(1), Some(10)).await?;
```

## [0.1.0] - 2024

### Added
- Initial implementation of AniList GraphQL API wrapper
- Core endpoint support:
  - Media (Anime/Manga) - search, fetch by ID, trending, popular
  - Character - search, fetch by ID
  - Staff - search, fetch by ID
  - User - profiles, statistics, followers, following
  - Studio - search, fetch by ID
  - Review - fetch, create, update, delete
  - Recommendation - fetch, rate
  - Activity - fetch feed, create text/message activities
  - Forum - threads and comments (returned as JSON Value)
  - Airing - schedule information
  - Notification - fetch user notifications
- Authentication support with Bearer tokens
- Error handling with `AniListError` enum
- GraphQL query execution
- Basic pagination support
- Async/await support with tokio
- Added Support for Synonyms (#1 By @Random-Scientist)
- Added Basic Support for Anime Relations (#2 By @Random-Scientist)

---

## Links

- [Repository](https://github.com/Thunder-Blaze/anilist_moe)
- [Crates.io](https://crates.io/crates/anilist_moe)
- [Documentation](https://docs.rs/anilist_moe)
- [Issues](https://github.com/Thunder-Blaze/anilist_moe/issues)
