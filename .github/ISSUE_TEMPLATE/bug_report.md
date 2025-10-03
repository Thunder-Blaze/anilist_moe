---
name: Bug Report
about: Report a bug or issue with the library
title: '[BUG] '
labels: bug
assignees: ''
---

## Bug Description
A clear and concise description of what the bug is.

## Environment
- **anilist_moe version**: (e.g., 0.2.0)
- **Rust version**: (run `rustc --version`)
- **Operating System**: (e.g., Ubuntu 22.04, Windows 11, macOS 14)

## Steps to Reproduce
1. Create client with...
2. Call specific method...
3. See error...

## Expected Behavior
A clear and concise description of what you expected to happen.

## Actual Behavior
A clear and concise description of what actually happened.

## Code Example
```rust
// Minimal reproducible example
use anilist_moe::AniListClient;

#[tokio::main]
async fn main() {
    let client = AniListClient::new();
    // Your code that reproduces the bug
}
```

## Error Message
```
Paste the full error message or stack trace here
```

## Additional Context
Add any other context about the problem here (screenshots, logs, etc.).

## Possible Solution
(Optional) If you have suggestions on how to fix the bug, please describe them here.
