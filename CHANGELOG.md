# Changelog

All notable changes to this project will be documented in this file.

## Versioning

This project uses [Semantic Versioning](https://semver.org/):
- **MAJOR** version for incompatible API changes
- **MINOR** version for new functionality in a backwards compatible manner
- **PATCH** version for backwards compatible bug fixes

## [0.6.0] - 2026-10-14

Endpoint options are built with the typed builder instead of the `options!` macro, which
makes the fields the API requires a compile-time error rather than something the server
rejects. This release also fixes six endpoints that could never have worked — `favorites`,
`media_list`, `pin`, `subscribe`, `toggle_thread_subscription` and `update_comment` — each
checked against the API itself, since production turns introspection off.

### Added
- A typed builder on every options struct (`FetchMediaOptions::builder()…build()`), which
  replaces the `options!` macro. Setters take the inner value and the `_opt` fallbacks take
  the `Option` (`.per_page(5)` / `.per_page_opt(maybe_page)`), the string and list fields
  take `impl Into<Cow<..>>` so a literal, a `&[T]` or a `Vec` all work, and a field the API
  requires is a compile-time error to leave out
- `#[builder(!default)]` marks the 25 fields the API insists on, and the structs that have
  one no longer derive `Default`: `PinActivityOptions::default()` used to compile and send
  `id: 0, pinned: false`
- The `asMod` argument on `SaveActivityReply`, which the [mutation
  reference](https://docs.anilist.co/reference/mutation) lists alongside the reply text
- `llms.txt`, a single page describing the crate, its conventions and the quirks of the API
  for coding assistants, linked from the README

### Changed
- **Breaking**: `options!` is gone, every options struct is `#[non_exhaustive]` with `pub`
  fields, and struct literals no longer compile outside the crate — a new GraphQL variable
  is a minor change now, while a built value can still be read and adjusted (a paging loop
  keeps setting `options.page` instead of rebuilding)
- **Breaking**: `UserEndpoint::favorites` returns `User`, not `Page<Vec<User>>` (the
  response has no `Page`), and `UserEndpoint::media_list` returns `Page<Vec<MediaList>>`,
  because the rows are media list entries. `FetchUserFavoritesOptions` lost `anime_sort`,
  `manga_sort`, `characters_sort`, `staff_sort` and `studios_sort` — and with them its
  lifetime parameter — since `Favourites` takes no sort
- **Breaking**: `ToggleThreadSubscriptionOptions::subscribe` is a `bool`, not an
  `Option<bool>`. The API's validator rejects the request without it, so the type no longer
  pretends it can be left out, and `toggle_thread_subscription` reads the thread's current
  state and sends the opposite rather than sending nothing
- `SaveRecommendationOptions` no longer derives `Deserialize`: an options struct is a
  request input
- `ReqwestTransport` maps request and body failures directly onto `AniListError::Network`,
  preserving the underlying `reqwest::Error` cause chain (timeouts, DNS failures, connection
  resets) instead of flattening them into a string message
- Statuses `401`, `403` and `404` preserve structured `GraphQL` errors returned by the
  server instead of discarding the response body, allowing callers to inspect server error
  messages via `graphql_errors()` and `graphql_message_contains()` while falling back to
  unit error variants when no structured errors are present
- The test harness rate-limiter introduces an explicit safety cap
  (`ANILIST_TEST_MAX_RATE_LIMIT_RETRIES`, defaulting to 5) preventing unbounded retry loops
  and test runner hangs on persistent 429 errors
- CI runs `clippy --all-targets --all-features`, so the `pagination` module and the test
  suite are linted, and a new job builds every feature on the declared MSRV

### Fixed
- `UserEndpoint::favorites` never worked: its document asked `Favourites.anime`, `.manga`,
  `.characters`, `.staff` and `.studios` to `sort` (the type takes only `page` and
  `perPage`, which the API rejects as "Unknown argument `sort`"), and it selected `isMain`
  on a `MediaEdge` (the field there is `isMainStudio`). It validates now
- `UserEndpoint::media_list` sent `MediaSort` values to a `[MediaListSort]` variable, so
  every call failed with `Variable "$sort" got invalid value ["ID"]`, and it returned
  `Page<Vec<User>>` for rows that are media list entries — which deserialised into `User`
  with every field but `id` left `None`. It now sends `MediaListSort`
- `ActivityEndpoint::pin` never worked: its document selected `id` and `type` directly on
  `ActivityUnion`, which has no such fields, and neither it nor `subscribe` selected
  `__typename`, which `ActivityUnion`'s tagged deserializer requires
- `ForumEndpoint::toggle_thread_subscription` sent no `subscribe` value while its document
  declared `Boolean!`, so the API refused it before doing anything. AniList's validator
  requires the state to reach it — leaving the argument out and passing `null` both come
  back as "The subscribe field is required", and only a supplied value (or a variable
  default) gets through
- `ForumEndpoint::update_comment` sent no `threadId` while its document declared `Int!`, so
  it failed with `Variable "$threadId" of required type "Int!" was not provided`. The
  reference types the argument `Int`, an update identifies the comment by `id`, and the API
  accepts the request without it
- `tests/pagination_test.rs` still used the removed `options!` macro, which is why the
  offline job was red: the target only compiles behind `--all-features`
- A `403` whose body says the API is temporarily disabled is an outage, not a permission
  problem: [the docs](https://docs.anilist.co/guide/considerations) describe exactly that
  response as the signal that AniList has been taken down, and none of its words matched
  `detect_maintenance`, so it came back as `AccessDenied`
- `MediaTagsOptions` had an `Option` field without `#[skip_serializing_none]`, so it sent
  `status: null` — the same class of bug as the `FetchStudioOptions` one

### Documentation
- `README.md`, `EXAMPLES.md`, `llms.txt` and `CONTRIBUTING.md` document the builder, the
  `_opt` fallbacks and the `#[non_exhaustive]` + `pub` field rule, and `CONTRIBUTING.md`
  has the attribute stack to copy for a new options struct
- The options the reference marks "(Mod Only)" say so (`locked`, `sticky`, `asMod`,
  `MediaTagCollection`'s `status`), and `compareWithAuthList` notes the user id or name the
  API requires with it
- The docs and the README say what the [rate limiting
  page](https://docs.anilist.co/guide/rate-limiting) says: 90 requests per minute, currently
  30 while the API recovers

### Dependencies
- `typed-builder` 0.22, held below 0.23 on purpose: 0.23 is edition 2024, which needs Rust
  1.85, and this crate supports 1.82. CI now builds every feature on 1.82, so a dependency
  that raises the floor fails there rather than for a consumer

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
- `CommonEndpoint::genres` and `CommonEndpoint::media_tags`, the lists the filters take their
  genre and tag names from, and `CommonEndpoint::likes`, the users who liked something
- `MediaListEndpoint::fetch_collection` returns a whole list grouped the way the site shows it
  (`MediaListCollection`), and `MediaListEndpoint::fetch_one` fetches a single entry
- `MediaListEndpoint::delete_custom_list`, `ActivityEndpoint::save_list_activity` and
  `CommonEndpoint::update_favourite_order` cover the last of the list and activity mutations.
  The API documents `SaveListActivity` as a moderator mutation and requires a custom list's
  name and type, so those options say so
- `AiringEndpoint::fetch_one` and `ActivityEndpoint::fetch_reply` fetch a single row where the
  API offers one
- `CommonEndpoint::markdown` renders markdown the way AniList does,
  `CommonEndpoint::external_link_sources` lists the sites a `MediaExternalLink` can point at,
  `CommonEndpoint::site_statistics` returns the site-wide count series, and
  `CommonEndpoint::ani_chart_user` with `update_ani_chart_settings` /
  `update_ani_chart_highlights` cover the AniChart tool
- `MediaEndpoint::fetch_trends` and `fetch_trend_one` read the daily trend history AniList keeps
  for a title
- The three submission update notifications (`MediaSubmissionUpdateNotification`,
  `StaffSubmissionUpdateNotification`, `CharacterSubmissionUpdateNotification`) and their
  `NotificationUnion` variants, so those notifications no longer land in `Unknown`
- Every enum that a response can carry has an `Unknown` catch-all variant
  (`#[serde(other)]`), and the three response unions (`ActivityUnion`,
  `LikeableUnion`, `NotificationUnion`) catch an unknown `__typename` the same
  way: a value AniList adds before this crate knows about it no longer fails the
  whole response

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
- Every option struct is `#[serde(rename_all = "camelCase")]` and no field carries a
  `rename` any more: a GraphQL variable name is the client's to choose, only the argument
  name is validated, so the serialised names now follow the Rust fields and the queries
  declare those names. `FetchMediaOptions` alone lost 99 hand written renames
- `MediaRelation` gained `SAME_UNIVERSE`, `SubmissionStatus::Approved` became
  `SubmissionStatus::Accepted` (the API never sent `APPROVED`), and `RevisionHistoryAction`
  stopped renaming `Edit` to a value the API does not use (`EDIT`)
- `ReviewSort` and `ThreadSort` keep `CREATED_AT` and `CREATED_AT_DESC` but mark them
  deprecated, as the API does; the four convenience methods that sorted by them use `IdDesc`
- Every AniList ID is `u32`, in the option structs and in the response objects
  alike (`id`, `user_id`, `media_id`, `id_mal`, `site_id`, …), including
  `media_ids` and `character_ids` as `Vec<u32>` and `id_in`/`id_not_in` as
  `Cow<[u32]>`. Counters, timestamps and sort orders stay `i32`, which is what the
  API's signed `Int` maps to, and `FuzzyDate.year` is `u32` like its month and day
- Every enum is `#[non_exhaustive]` and derives `PartialEq`, `Eq` and `Hash`
  (before, only some did): a match on one outside this crate needs a trailing `_`
  arm, which is what makes a new AniList value a non-event rather than a breaking
  change
- The `mods` modules are `moderators` (`anilist_moe::enums::moderators`,
  `anilist_moe::objects::moderators`), because that is what they hold, and the two
  now agree with each other
- The endpoint accessors (`client.media()`, `client.anime()`, …) are `#[must_use]`:
  dropping the handle without calling it is a warning instead of silence

### Dependencies
- Floors are the current release of every dependency: thiserror 2.0.21, serde
  1.0.229, serde_json 1.0.154, tokio 1.53.2, log 0.4.34, tracing 0.1.44
- Two are held at the newest release that keeps the MSRV at 1.82, and the reason is
  written in `Cargo.toml`: `reqwest` stays on 0.12.28, because 0.13 needs Rust 1.85
  and its `rustls` feature pulls `aws-lc-rs`, which makes every consumer build C
  code with CMake; `serde_with` stays on 3.17, because 3.18 needs Rust 1.88
- `chrono` is gone (nothing in the crate used it) and `dotenv` moved to
  dev-dependencies: a library has no business loading a `.env` into a consumer's
  process
- `tokio` asks for its `time` feature alone — retry backoff is the only thing this
  crate sleeps for — instead of `full`, so a consumer no longer compiles io, net,
  fs, process and signal support on this crate's behalf
- `husky-rs` is gone: nothing used it, and the pre-commit hook is wired through
  `core.hooksPath` instead

### Fixed
- Filter options that never reached the API: the serialised name of a field has to match the
  variable its query declares, and 14 media filters (`id_not`, `status_not_in`, `genre_in`, …),
  three character filters, four media list filters and `activityId` on a subscription did not.
  `ActivityEndpoint::toggle_subscription` could not work at all, because the API requires
  `activityId` and it was sent as `id`
- `FetchMediaListOptions` sent a `FuzzyDate` object for `startedAt`/`completedAt`, which the API
  types as `FuzzyDateInt` (`YYYYMMDD`) and rejects
- `AiringProgression::progress` was the API's `score`, `ListActivityOption`'s type was the wrong
  field and the wrong enum, and `MediaTagCollection`'s `type`, `MessageActivity::isPinned` and
  `ThreadLikeNotification::comment` were missing
- Options that promised something the API cannot do: `FetchCharacterOneOptions` had `page` and
  `per_page` on a field that takes neither, and `FetchThreadOneOptions` had three comment
  options for a `Thread` field that has no comments
- `MediaList::as_array` is gone: the API has no such field, it has an `asArray`
  argument on `customLists`, so the field could only ever be `None`
- `Query.Like` is not wrapped either: the API answers it with "Page parent query
  required", so a lookup by `likeableId` goes through `Page.likes`
  (`CommonEndpoint::likes`), which is the paginated form and works
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
- The live tests no longer pass when the crate is the problem: an error that is
  not the environment's fault — a rejected query, an unparseable response, a
  missing field — fails the test instead of being printed, and an unauthenticated
  test only tolerates an auth rejection. See `tests/support/mod.rs`
- The demo binary is `examples/media.rs`, so the published crate no longer carries
  an `anilist_moe` binary whose only job was printing one response; the scratch
  file `examples/tmp_serde.rs` went with it

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
