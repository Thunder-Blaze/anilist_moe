# Contributing to AniList.moe

Thank you for considering contributing to anilist_moe! This document provides guidelines and instructions for contributing.

## Code of Conduct

Be respectful and constructive in all interactions. We aim to maintain a welcoming and inclusive environment for all contributors.

## Getting Started

### Prerequisites

- Rust 1.70 or higher (with edition 2021 support)
- Git
- A GitHub account

### Development Setup

1. Fork the repository on GitHub
2. Clone your fork locally:
   ```bash
   git clone https://github.com/YOUR_USERNAME/anilist_moe.git
   cd anilist_moe
   ```

3. Add the upstream repository:
   ```bash
   git remote add upstream https://github.com/Thunder-Blaze/anilist_moe.git
   ```

4. Create a new branch for your feature:
   ```bash
   git checkout -b feature/your-feature-name
   ```

5. Install dependencies and verify the build:
   ```bash
   cargo build
   cargo test
   ```

## Development Guidelines

### Code Style

- Follow Rust standard naming conventions
- Use `cargo fmt` to format your code before committing
- Run `cargo clippy -- -D warnings` to catch common mistakes and ensure no warnings
- Write clear, descriptive commit messages
- Pre-commit hooks (husky-rs) will automatically run formatting and linting checks for contributors with a local clone

### Type Safety

- All public APIs must return strongly typed responses
- Avoid using `serde_json::Value` in public interfaces
- Use `Option<T>` for optional fields
- Use `Result<T, AniListError>` for fallible operations

### Documentation

- Add doc comments for all public functions and types
- Prefer concise, meaningful comments; avoid AI-generated fluff
- Do not narrate obvious behavior (e.g., asserts, simple control flow)
- Include examples in doc comments where they add value
- Update README.md if adding new features
- Add entries to EXAMPLES.md for new endpoints

### Testing

- Write tests for new features
- Ensure all tests pass before submitting PR
- Add integration tests for new endpoints
- Test error handling paths

Run tests:
```bash
# Run all tests
cargo test

# Run specific test
cargo test test_name

# Run with output
cargo test -- --nocapture

# Run specific endpoint tests
cargo test --test endpoint_tests

# Run doctests
cargo test --doc
```

## Making Changes

### Adding a New Endpoint

1. Define the endpoint struct in `src/endpoints/`
2. Create option structs for parameters
3. Add response types to `src/objects/responses.rs`
4. Implement convenience functions
5. Add GraphQL queries to `src/queries/`
6. Write tests in `tests/endpoints/`
7. Update documentation

Example structure:
```rust
// src/endpoints/new_endpoint.rs
use crate::client::AniListClient;
use crate::errors::AniListError;
use crate::objects::responses::*;

pub struct NewEndpoint {
    client: AniListClient,
}

impl NewEndpoint {
    pub fn new(client: AniListClient) -> Self {
        Self { client }
    }

    /// Get items with options
    /// Returns Page<Vec<Item>>
    pub async fn fetch(&self, options: FetchOptions) -> Result<Page<Vec<Item>>, AniListError> {
        let query = queries::new_endpoint::FETCH;
        let variables = json!(options);
        let variables_map = crate::utils::json_to_hashmap(variables);
        self.client.query_typed(query, Some(&variables_map)).await
    }

    /// Convenience function - get popular items
    /// Returns Page<Vec<Item>>
    pub async fn get_popular(&self, page: Option<i32>, per_page: Option<i32>)
        -> Result<Page<Vec<Item>>, AniListError> {
        self.fetch(FetchOptions {
            sort: Some(vec![Sort::Popularity]),
            page,
            per_page,
            ..Default::default()
        }).await
    }

    /// Get a single item by ID
    /// Returns Item directly (not wrapped)
    pub async fn get_by_id(&self, id: i32) -> Result<Item, AniListError> {
        let query = queries::new_endpoint::FETCH_ONE;
        let variables = json!({ "id": id });
        let variables_map = crate::utils::json_to_hashmap(variables);
        self.client.query_typed(query, Some(&variables_map)).await
    }
}
```

### Adding Response Types

Response types are defined in `src/objects/responses.rs`:
```rust
/// GraphQL response wrapper
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphQLResponse<T> {
    pub data: T,
}

/// Pagination wrapper for list responses
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Page<T> {
    pub page_info: Option<PageInfo>,
    pub data: T,
}
```

Notes on response structure:
- Paginated endpoints return `Page<Vec<T>>` - access the list through `response.data`
- Single item endpoints return `T` directly - no extra wrapper
- The `GraphQLResponse<T>` wrapper is handled internally by the client
- All endpoint methods are already properly typed

### Writing Tests

Add tests to `tests/endpoints/`:
```rust
#[tokio::test]
async fn test_get_popular() {
    let client = get_test_client();
    let result = client.new_endpoint().get_popular(Some(1), Some(5)).await;

    assert!(result.is_ok());
    let response = result.unwrap();

    // Response is Page<Vec<Item>>
    // Access the Vec<Item> through response.data
    assert!(!response.data.is_empty());

    // Access pagination info
    if let Some(page_info) = &response.page_info {
        assert!(page_info.current_page.is_some());
    }
}

#[tokio::test]
async fn test_get_by_id() {
    let client = get_test_client();
    let result = client.new_endpoint().get_by_id(1).await;

    assert!(result.is_ok());
    let item = result.unwrap();

    // Response is Item directly (not wrapped)
    assert_eq!(item.id, Some(1));
}
```

## Pull Request Process

1. Ensure your code follows the style guidelines
2. Update documentation as needed
3. Add tests for new functionality
4. Run all tests and ensure they pass:
   ```bash
   cargo test
   cargo clippy -- -D warning
   cargo fmt --check
   ```

5. Commit your changes with clear messages:
   ```bash
   git add .
   git commit -m "Add feature: description of feature"
   ```
   
   Note: If you have a local clone, pre-commit hooks will automatically run `cargo fmt --check` and `cargo clippy -- -D warnings` before allowing the commit.

6. Push to your fork:
   ```bash
   git push origin feature/your-feature-name
   ```

7. Create a Pull Request on GitHub with:
   - Clear description of changes
   - Reference to any related issues
   - Screenshots/examples if applicable

### Pull Request Template

```markdown
## Description
Brief description of changes

## Type of Change
- [ ] Bug fix
- [ ] New feature
- [ ] Breaking change
- [ ] Documentation update

## Testing
- [ ] Tests added/updated
- [ ] All tests pass
- [ ] Manual testing completed

## Checklist
- [ ] Code follows style guidelines
- [ ] Documentation updated
- [ ] No breaking changes (or documented)
- [ ] Tests pass locally
```

## Project Structure

```
anilist_moe/
├── src/
│   ├── client.rs              # Main client
│   ├── errors.rs              # Error types
│   ├── lib.rs                 # Library entry point
│   ├── utils.rs               # Utility functions
│   ├── endpoints/             # API endpoints
│   │   ├── mod.rs
│   │   ├── anime.rs
│   │   ├── manga.rs
│   │   ├── character.rs
│   │   └── ...
│   ├── objects/               # Data models
│   │   ├── mod.rs
│   │   ├── responses.rs       # Response wrappers
│   │   ├── media.rs
│   │   ├── character.rs
│   │   └── ...
│   ├── enums/                 # Enumerations
│   │   ├── mod.rs
│   │   ├── media.rs
│   │   └── ...
│   ├── unions/                # Union types
│   │   ├── mod.rs
│   │   ├── activity.rs
│   │   └── ...
│   └── queries/               # GraphQL queries
│       ├── mod.rs
│       └── anime/
│           ├── mod.rs
│           └── fetch.graphql
├── tests/                     # Integration tests
│   ├── client_tests.rs
│   └── endpoints/
│       ├── mod.rs
│       └── anime.rs
├── README.md
├── EXAMPLES.md
├── CONTRIBUTING.md
└── Cargo.toml
```

## Common Tasks

### Running Specific Tests

```bash
# Run all tests
cargo test

# Run anime endpoint tests only
cargo test anime

# Run a specific test
cargo test test_get_popular_anime

# Run with output visible
cargo test -- --nocapture
```

### Checking Code Quality

```bash
# Format code
cargo fmt

# Check formatting without modifying
cargo fmt --check

# Run clippy for linting
cargo clippy

# Check for common issues with strict linting (recommended before committing)
cargo clippy -- -D warnings
```

### Building Documentation

```bash
# Build docs
cargo doc

# Build and open docs
cargo doc --open

# Build docs with private items
cargo doc --document-private-items
```

## Reporting Bugs

When reporting bugs, please include:

1. Rust version: `rustc --version`
2. Crate version
3. Minimal reproducible example
4. Expected behavior
5. Actual behavior
6. Error messages or stack traces

Example bug report:
```markdown
## Bug Description
Brief description of the bug

## Environment
- anilist_moe version: 0.2.0
- Rust version: 1.70.0
- OS: Ubuntu 22.04

## Steps to Reproduce
1. Create client
2. Call specific method
3. See error

## Expected Behavior
What should happen

## Actual Behavior
What actually happens

## Code Example
\```rust
// Minimal reproducible example
\```

## Error Message
\```
Full error message or stack trace
\```
```

## Feature Requests

When requesting features:

1. Check if the feature already exists
2. Explain the use case
3. Provide examples of how it would be used
4. Consider if it fits the project scope

## Questions and Support

- Create an issue with the "question" label
- Check existing issues and documentation first
- Be specific about what you're trying to accomplish

## Release Process

(For maintainers)

1. Update version in `Cargo.toml`
2. Update CHANGELOG.md with all changes
3. Run full test suite: `cargo test --all`
4. Run clippy: `cargo clippy -- -D warnings`
5. Run formatting check: `cargo fmt --check`
6. Create release commit
7. Tag release: `git tag -a v0.X.Y -m "Release v0.X.Y"`
8. Push tags: `git push origin --tags`
9. Publish to crates.io: `cargo publish`

## License

By contributing, you agree that your contributions will be licensed under the MIT License.

## Recognition

Contributors will be acknowledged in:
- GitHub contributors list
- CHANGELOG.md for significant contributions
- README.md for major features

## Getting Help

- Create an issue for bugs or feature requests
- Discussion forum for general questions
- Check EXAMPLES.md for usage examples

Thank you for contributing to anilist_moe!
