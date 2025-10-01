//! Tests for error types and error handling

use anilist_moe::errors::AniListError;

#[test]
fn test_error_display_graphql() {
    let error = AniListError::GraphQL {
        message: "Field 'test' not found".to_string(),
    };
    let display = format!("{}", error);
    assert!(display.contains("GraphQL error"));
    assert!(display.contains("Field 'test' not found"));
}

#[test]
fn test_error_display_rate_limit() {
    let error = AniListError::RateLimit {
        limit: 90,
        remaining: 0,
        reset_at: 1234567890,
        retry_after: 60,
    };
    let display = format!("{}", error);
    assert!(display.contains("Rate limit exceeded"));
    assert!(display.contains("90"));
    assert!(display.contains("60"));
}

#[test]
fn test_error_display_rate_limit_simple() {
    let error = AniListError::RateLimitSimple;
    let display = format!("{}", error);
    assert!(display.contains("Rate limit exceeded"));
    assert!(display.contains("Try again in a few moments"));
}

#[test]
fn test_error_display_burst_limit() {
    let error = AniListError::BurstLimit;
    let display = format!("{}", error);
    assert!(display.contains("Burst limit exceeded"));
}

#[test]
fn test_error_display_not_found() {
    let error = AniListError::NotFound;
    let display = format!("{}", error);
    assert_eq!(display, "Not found");
}

#[test]
fn test_error_display_authentication_required() {
    let error = AniListError::AuthenticationRequired;
    let display = format!("{}", error);
    assert!(display.contains("Authentication required"));
    assert!(display.contains("access token"));
}

#[test]
fn test_error_display_access_denied() {
    let error = AniListError::AccessDenied;
    let display = format!("{}", error);
    assert!(display.contains("Access denied"));
}

#[test]
fn test_error_display_bad_request() {
    let error = AniListError::BadRequest {
        message: "Invalid parameter 'page'".to_string(),
    };
    let display = format!("{}", error);
    assert!(display.contains("Bad request"));
    assert!(display.contains("Invalid parameter 'page'"));
}

#[test]
fn test_error_display_server_error() {
    let error = AniListError::ServerError {
        status: 500,
        message: "Internal server error".to_string(),
    };
    let display = format!("{}", error);
    assert!(display.contains("Server error"));
    assert!(display.contains("500"));
    assert!(display.contains("Internal server error"));
}

#[test]
fn test_error_display_parse_error() {
    let error = AniListError::ParseError {
        message: "Failed to parse JSON response".to_string(),
    };
    let display = format!("{}", error);
    assert!(display.contains("Parse error"));
    assert!(display.contains("Failed to parse JSON response"));
}

#[test]
fn test_error_debug() {
    let error = AniListError::NotFound;
    let debug_str = format!("{:?}", error);
    assert!(debug_str.contains("NotFound"));
}

#[test]
fn test_error_from_serde_json() {
    let json_error = serde_json::from_str::<serde_json::Value>("{invalid json")
        .err()
        .unwrap();
    let anilist_error: AniListError = json_error.into();

    match anilist_error {
        AniListError::Json(_) => {}, // Expected
        _ => panic!("Expected Json error variant"),
    }
}

#[test]
fn test_rate_limit_error_values() {
    let error = AniListError::RateLimit {
        limit: 90,
        remaining: 5,
        reset_at: 1234567890,
        retry_after: 45,
    };

    if let AniListError::RateLimit { limit, remaining, reset_at, retry_after } = error {
        assert_eq!(limit, 90);
        assert_eq!(remaining, 5);
        assert_eq!(reset_at, 1234567890);
        assert_eq!(retry_after, 45);
    } else {
        panic!("Expected RateLimit error variant");
    }
}

#[test]
fn test_error_is_send_and_sync() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<AniListError>();
}
