//! Unit tests for utility functions

use anilist_moe::utils::{RetryConfig, calculate_delay, json_to_hashmap};
use serde_json::{Value, json};
use std::time::Duration;

#[test]
fn test_json_to_hashmap_with_object() {
    let json_value = json!({
        "key1": "value1",
        "key2": 42,
        "key3": true
    });

    let map = json_to_hashmap(json_value);
    assert_eq!(map.len(), 3);
    assert!(map.contains_key("key1"));
    assert!(map.contains_key("key2"));
    assert!(map.contains_key("key3"));
}

#[test]
fn test_json_to_hashmap_with_non_object() {
    let json_value = json!("just a string");
    let map = json_to_hashmap(json_value);
    assert_eq!(map.len(), 0, "Non-object JSON should return empty hashmap");
}

#[test]
fn test_json_to_hashmap_with_array() {
    let json_value = json!([1, 2, 3]);
    let map = json_to_hashmap(json_value);
    assert_eq!(map.len(), 0, "Array JSON should return empty hashmap");
}

#[test]
fn test_json_to_hashmap_with_null() {
    let json_value = Value::Null;
    let map = json_to_hashmap(json_value);
    assert_eq!(map.len(), 0, "Null JSON should return empty hashmap");
}

#[test]
fn test_json_to_hashmap_with_nested_object() {
    let json_value = json!({
        "outer": {
            "inner": "value"
        }
    });

    let map = json_to_hashmap(json_value);
    assert_eq!(map.len(), 1);
    assert!(map.contains_key("outer"));

    if let Some(Value::Object(inner)) = map.get("outer") {
        assert!(inner.contains_key("inner"));
    } else {
        panic!("Expected nested object");
    }
}

#[test]
fn test_calculate_delay_with_zero_remaining() {
    let delay = calculate_delay(0, 60);
    assert_eq!(
        delay,
        Duration::from_secs(60),
        "Should wait full reset time when no requests remaining"
    );
}

#[test]
fn test_calculate_delay_with_low_remaining() {
    let delay = calculate_delay(5, 60);
    assert_eq!(
        delay,
        Duration::from_millis(2000),
        "Should wait 2 seconds when less than 10 remaining"
    );
}

#[test]
fn test_calculate_delay_with_moderate_remaining() {
    let delay = calculate_delay(20, 60);
    assert_eq!(
        delay,
        Duration::from_millis(1000),
        "Should wait 1 second when 10-30 remaining"
    );
}

#[test]
fn test_calculate_delay_with_plenty_remaining() {
    let delay = calculate_delay(50, 60);
    assert_eq!(
        delay,
        Duration::from_millis(500),
        "Should wait 500ms when plenty remaining"
    );
}

#[test]
fn test_calculate_delay_boundary_conditions() {
    // Test exact boundaries
    assert_eq!(calculate_delay(10, 60), Duration::from_millis(1000));
    assert_eq!(calculate_delay(9, 60), Duration::from_millis(2000));
    assert_eq!(calculate_delay(30, 60), Duration::from_millis(500));
    assert_eq!(calculate_delay(29, 60), Duration::from_millis(1000));
}

#[test]
fn test_retry_config_equality() {
    let config1 = RetryConfig {
        max_retries: 5,
        base_delay_ms: 1000,
        exponential_backoff: true,
        max_delay_ms: 30000,
    };

    let config2 = RetryConfig {
        max_retries: 5,
        base_delay_ms: 1000,
        exponential_backoff: true,
        max_delay_ms: 30000,
    };

    assert_eq!(config1, config2);
}

#[test]
fn test_retry_config_inequality() {
    let config1 = RetryConfig {
        max_retries: 5,
        base_delay_ms: 1000,
        exponential_backoff: true,
        max_delay_ms: 30000,
    };

    let config2 = RetryConfig {
        max_retries: 3,
        base_delay_ms: 1000,
        exponential_backoff: true,
        max_delay_ms: 30000,
    };

    assert_ne!(config1, config2);
}
