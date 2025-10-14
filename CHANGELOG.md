# Changelog

All notable changes to this project will be documented in this file.

## Versioning

This project uses [Semantic Versioning](https://semver.org/):
- **MAJOR** version for incompatible API changes
- **MINOR** version for new functionality in a backwards compatible manner
- **PATCH** version for backwards compatible bug fixes

## [Unreleased]

### Added

### Changed

### Fixed

### Breaking Changes

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
