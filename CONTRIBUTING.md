# Contributing to AniList.moe

Thank you for considering contributing to anilist_moe! This document provides guidelines and instructions for contributing.

## Code of Conduct

Be respectful and constructive in all interactions. We aim to maintain a welcoming and inclusive environment for all contributors.

## Getting Started

### Prerequisites

- Rust 1.70 or higher
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
- Run `cargo clippy` to catch common mistakes
- Write clear, descriptive commit messages

### Type Safety

- All public APIs must return strongly typed responses
- Avoid using `serde_json::Value` in public interfaces
- Use `Option<T>` for optional fields
- Use `Result<T, AniListError>` for fallible operations

### Documentation

- Add doc comments for all public functions and types
- Include examples in doc comments where appropriate
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
    pub async fn fetch(&self, options: FetchOptions) -> Result<ItemListResponse, AniListError> {
        let query = queries::new_endpoint::FETCH;
        let variables = json!(options);
        let variables_map = crate::utils::json_to_hashmap(variables);
        self.client.query_typed(query, Some(&variables_map)).await
    }

    /// Convenience function - get popular items
    pub async fn get_popular(&self, page: Option<i32>, per_page: Option<i32>)
        -> Result<ItemListResponse, AniListError> {
        self.fetch(FetchOptions {
            sort: Some(vec![Sort::Popularity]),
            page,
            per_page,
            ..Default::default()
        }).await
    }
}
```

### Adding Response Types

Add to `src/objects/responses.rs`:
```rust
// Data wrapper
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ItemData {
    pub items: Vec<Item>,
}

// Single item wrapper
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ItemResponse {
    #[serde(rename = "Item")]
    pub item: Item,
}

// Type aliases
pub type ItemListResponse = GraphQLResponse<PageResponse<ItemData>>;
pub type ItemSingleResponse = GraphQLResponse<ItemResponse>;
```

### Writing Tests

Add tests to `tests/endpoints/`:
```rust
#[tokio::test]
async fn test_get_popular() {
    let client = get_test_client();
    let result = client.new_endpoint().get_popular(Some(1), Some(5)).await;

    assert!(result.is_ok());
    let response = result.unwrap();
    assert!(!response.data.page.data.items.is_empty());
}
```

## Pull Request Process

1. Ensure your code follows the style guidelines
2. Update documentation as needed
3. Add tests for new functionality
4. Run all tests and ensure they pass:
   ```bash
   cargo test
   cargo clippy
   cargo fmt --check
   ```

5. Commit your changes with clear messages:
   ```bash
   git add .
   git commit -m "Add feature: description of feature"
   ```

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

# Check for common issues with strict linting
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
2. Update CHANGELOG.md
3. Run full test suite
4. Create release commit
5. Tag release: `git tag -a v0.X.Y -m "Release v0.X.Y"`
6. Push tags: `git push origin --tags`
7. Publish to crates.io: `cargo publish`

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
