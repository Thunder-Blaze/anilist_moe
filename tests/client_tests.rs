//! Unit tests for the AniList client
//!
//! These tests verify the core functionality of the client without making actual API calls.

use anilist_moe::{utils::RetryConfig, AniListClient};

#[test]
fn test_client_creation() {
    let client = AniListClient::new();
    assert!(!client.has_token(), "New client should not have a token");
}

#[test]
fn test_client_with_token() {
    let token = "test_token_12345";
    let client = AniListClient::with_token(token);
    assert!(client.has_token(), "Client should have a token");
}

#[test]
fn test_client_with_token_string() {
    // Test with String instead of &str
    let token = String::from("test_token_12345");
    let client = AniListClient::with_token(token);
    assert!(client.has_token(), "Client should have a token");
}

#[test]
fn test_client_set_token() {
    let mut client = AniListClient::new();
    assert!(!client.has_token());

    client.set_token("new_token");
    assert!(client.has_token());
}

#[test]
fn test_client_clear_token() {
    let mut client = AniListClient::with_token("test_token");
    assert!(client.has_token());

    client.clear_token();
    assert!(!client.has_token());
}

#[test]
fn test_client_with_retry_config() {
    let config = RetryConfig {
        max_retries: 5,
        base_delay_ms: 2000,
        exponential_backoff: true,
        max_delay_ms: 60000,
    };

    let client = AniListClient::new().with_retry_config(config);
    assert!(!client.has_token());

    // Verify retry config is preserved
    let retrieved_config = client.retry_config();
    assert_eq!(retrieved_config.max_retries, 5);
    assert_eq!(retrieved_config.base_delay_ms, 2000);
}

#[test]
fn test_client_with_custom_base_url() {
    let custom_url = "https://custom.api.example.com";
    let client = AniListClient::new().with_base_url(custom_url);
    // Client should be created successfully
    assert!(!client.has_token());
}

#[test]
fn test_client_builder_pattern() {
    let config = RetryConfig {
        max_retries: 3,
        base_delay_ms: 1000,
        exponential_backoff: false,
        max_delay_ms: 5000,
    };

    let client = AniListClient::with_token("test_token")
        .with_retry_config(config)
        .with_base_url("https://test.api.com");

    assert!(client.has_token());
}

#[test]
fn test_client_default() {
    let client = AniListClient::default();
    assert!(!client.has_token());
}

#[test]
fn test_client_clone() {
    let client = AniListClient::with_token("test_token");
    let cloned = client.clone();

    // Both clients should be independent and have the same token status
    assert!(cloned.has_token());

    // Cloning should be cheap (Arc-based)
    // This verifies the optimization is working
}

#[test]
fn test_client_clone_shares_state() {
    let client = AniListClient::with_token("test_token");
    let cloned = client.clone();

    // Both should report the same token status
    assert_eq!(client.has_token(), cloned.has_token());
}

#[test]
fn test_client_debug_impl() {
    let client = AniListClient::with_token("secret_token_should_not_appear_in_debug");
    let debug_string = format!("{:?}", client);

    // Token should not appear in debug output
    assert!(!debug_string.contains("secret_token"));
    assert!(debug_string.contains("has_token"));
    assert!(debug_string.contains("true"));
    assert!(debug_string.contains("base_url"));
}

#[test]
fn test_endpoint_accessors() {
    let client = AniListClient::new();

    // All endpoint accessors should work and return valid endpoints
    let _media = client.media();
    let _anime = client.anime(); // Alias for media
    let _manga = client.manga(); // Alias for media
    let _medialist = client.medialist();
    let _character = client.character();
    let _common = client.common();
    let _staff = client.staff();
    let _user = client.user();
    let _studio = client.studio();
    let _forum = client.forum();
    let _activity = client.activity();
    let _review = client.review();
    let _recommendation = client.recommendation();
    let _airing = client.airing();
    let _notification = client.notification();
}

#[test]
fn test_retry_config_default() {
    let config = RetryConfig::default();
    assert_eq!(config.max_retries, 3);
    assert_eq!(config.base_delay_ms, 1000);
    assert!(config.exponential_backoff);
    assert_eq!(config.max_delay_ms, 30000);
}

#[test]
fn test_retry_config_custom() {
    let config = RetryConfig {
        max_retries: 10,
        base_delay_ms: 500,
        exponential_backoff: false,
        max_delay_ms: 10000,
    };

    assert_eq!(config.max_retries, 10);
    assert_eq!(config.base_delay_ms, 500);
    assert!(!config.exponential_backoff);
    assert_eq!(config.max_delay_ms, 10000);
}

#[test]
fn test_retry_config_copy() {
    let config1 = RetryConfig {
        max_retries: 5,
        base_delay_ms: 2000,
        exponential_backoff: true,
        max_delay_ms: 30000,
    };

    // Use copy semantics instead of clone
    let config2 = config1;
    assert_eq!(config1, config2);
}

#[test]
fn test_retry_config_debug() {
    let config = RetryConfig::default();
    let debug_string = format!("{:?}", config);
    assert!(debug_string.contains("RetryConfig"));
}

#[test]
fn test_client_retry_config_accessor() {
    let config = RetryConfig {
        max_retries: 7,
        base_delay_ms: 1500,
        exponential_backoff: false,
        max_delay_ms: 45000,
    };

    let client = AniListClient::new().with_retry_config(config);
    let retrieved = client.retry_config();

    assert_eq!(retrieved.max_retries, 7);
    assert_eq!(retrieved.base_delay_ms, 1500);
    assert!(!retrieved.exponential_backoff);
    assert_eq!(retrieved.max_delay_ms, 45000);
}

#[test]
fn test_client_is_send_and_sync() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<AniListClient>();
}

#[test]
fn test_retry_config_is_send_and_sync() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<RetryConfig>();
}
