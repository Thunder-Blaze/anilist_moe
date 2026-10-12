# AniList API Endpoint Tests

## 📋 Overview

This directory contains **24 comprehensive integration tests** for all AniList API endpoints. These tests verify that:

✅ Data is successfully fetched from the API
✅ JSON responses are correctly parsed into Rust structs
✅ Field types match expectations (i32, String, Option<T>, etc.)
✅ Pagination works correctly
✅ Search and filtering options work

## 🧪 Test Coverage

| Endpoint | Tests | Coverage |
|----------|-------|----------|
| **Media** | 5 tests | Search, ID lookup, fetch_one, pagination, data types |
| **Character** | 4 tests | Search, ID lookup, fetch_one, data types |
| **User** | 4 tests | Search, ID lookup, fetch_one, data types |
| **Staff** | 4 tests | Search, ID lookup, fetch_one, data types |
| **Studio** | 4 tests | Search, ID lookup, fetch_one, data types |
| **Airing** | 3 tests | Fetch schedules, pagination, data types |
| **Total** | **24 tests** | Full endpoint integration coverage |

## 🚀 Running Tests

These are the **live tests**: they call the real AniList API, are rate limited,
and must run one thread at a time. CI skips them on ordinary pushes and only
runs them on a schedule, on a manual `workflow_dispatch`, or on a pull request
labelled `run-live-tests`.

### Run All Endpoint Tests
```bash
cargo test --test endpoint_tests
```

### Run Specific Endpoint Tests
```bash
# Media endpoint
cargo test --test endpoint_tests media

# Character endpoint
cargo test --test endpoint_tests character
```

### Run with Single Thread (Avoid Rate Limiting)
```bash
cargo test --test endpoint_tests -- --test-threads=1
```

### Tuning the Rate Limit

The harness sends at most 80 requests per 60s window by default, which stays
under AniList's 90 requests per minute. Override any of it for a run:

| Variable | Default | Meaning |
|---|---|---|
| `ANILIST_TEST_WINDOW_REQUESTS` | 80 | requests allowed per window |
| `ANILIST_TEST_WINDOW_SECS` | 60 | length of the window |
| `ANILIST_TEST_PAUSE_SECS` | 61 | pause after a 429 without `Retry-After` |
| `ANILIST_TEST_MIN_DELAY_MS` | 700 | minimum delay between two requests |
| `ANILIST_TEST_MAX_RETRIES` | 3 | transient network failures retried per request |
| `ANILIST_TEST_RETRY_DELAY_MS` | 2000 | delay between those retries |

```bash
# Slower run that is kind to the API
ANILIST_TEST_MIN_DELAY_MS=1500 cargo test --test endpoint_tests -- --test-threads=1

# Quick smoke run
ANILIST_TEST_WINDOW_REQUESTS=20 ANILIST_TEST_MIN_DELAY_MS=100 cargo test --test endpoint_tests
```

A 429 pauses every thread for the `Retry-After` the API sent, or
`ANILIST_TEST_PAUSE_SECS` when the response carried no such header.

## 📝 What Each Test Verifies

### Data Fetching
- Tests make real API calls to AniList's public GraphQL API
- No authentication required (read-only public data)
- Internet connection required

### Type Correctness
Tests verify correct Rust types for:
- **Required fields**: `id: i32`, `name: String`
- **Optional fields**: `Option<i32>`, `Option<String>`
- **Arrays**: `Vec<T>`
- **Nested structures**: `Option<MediaTitle>`, `Option<CharacterName>`

### Pagination
- Multiple pages return different results
- Page info is correct
- Per-page limits are respected

## 📊 Summary

**Total: 24 integration tests across 6 endpoints** 🎉

All tests compile successfully and are ready to run against the AniList API.
