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
