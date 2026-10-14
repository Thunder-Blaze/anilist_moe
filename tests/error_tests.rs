//! Tests for error types and error handling

use anilist_moe::errors::{AniListError, GraphQLError};

#[test]
fn test_error_display_graphql() {
    let error = AniListError::GraphQL {
        operation: None,
        errors: vec![GraphQLError::new("Field 'test' not found")],
        data: None,
    };
    let display = format!("{}", error);
    assert!(display.contains("GraphQL error"));
    assert!(display.contains("Field 'test' not found"));
}

#[test]
fn test_graphql_errors_expose_the_payload() {
    let error = AniListError::GraphQL {
        operation: None,
        errors: vec![
            GraphQLError::new("Cannot query field \"nope\".")
                .with_path(vec![serde_json::json!("Media")])
                .with_locations(vec![anilist_moe::errors::GraphQLLocation::new(2, 3)]),
            GraphQLError::new("and another"),
        ],
        data: Some(serde_json::json!({"Media": {"id": 1}})),
    };

    let errors = error.graphql_errors().expect("structured errors");
    assert_eq!(errors.len(), 2);
    assert_eq!(errors[0].path.as_ref().unwrap()[0], "Media");
    assert_eq!(errors[0].locations.as_ref().unwrap()[0].line, 2);
    assert!(error.graphql_message_contains("cannot query"));
    assert!(!error.graphql_message_contains("absent"));

    // The display string stays readable for logging.
    assert_eq!(
        error.to_string(),
        "GraphQL error: Cannot query field \"nope\"., and another"
    );

    assert!(!AniListError::NotFound.graphql_message_contains("anything"));
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
        source: None,
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
        AniListError::Json(_) => {} // Expected
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

    if let AniListError::RateLimit {
        limit,
        remaining,
        reset_at,
        retry_after,
    } = error
    {
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

#[test]
fn test_detect_maintenance() {
    use anilist_moe::errors::detect_maintenance;

    assert!(detect_maintenance("AniList is down due to maintenance"));
    assert!(detect_maintenance("SERVICE UNAVAILABLE"));
    assert!(detect_maintenance("we will be back soon"));
    assert!(!detect_maintenance("Cannot query field \"nope\"."));
    assert!(!detect_maintenance(""));
}

#[test]
fn test_error_display_maintenance_and_unexpected_status() {
    let maintenance = AniListError::Maintenance {
        message: "down for maintenance".to_string(),
    };
    assert!(maintenance.to_string().contains("unavailable"));
    assert_eq!(maintenance.kind(), "maintenance");

    let unexpected = AniListError::UnexpectedStatus {
        status: 418,
        message: "teapot".to_string(),
    };
    assert!(unexpected.to_string().contains("418"));
    assert_eq!(unexpected.kind(), "unexpected_status");
}

/// The reference documents the outage signal as a `403` whose body says the API is
/// temporarily disabled. Without the phrases below it reads as a permission
/// problem, which sends a caller looking at the wrong thing.
#[test]
fn the_documented_outage_message_reads_as_maintenance() {
    let documented = "The AniList API has been temporarily disabled due to severe stability issues. Please check the official AniList Discord for more information.";
    assert!(anilist_moe::errors::detect_maintenance(documented));

    // the older phrasings stay recognised
    assert!(anilist_moe::errors::detect_maintenance(
        "AniList is down due to maintenance"
    ));
    assert!(anilist_moe::errors::detect_maintenance(
        "The service is temporarily unavailable"
    ));

    // and a plain rejection is still not an outage
    assert!(!anilist_moe::errors::detect_maintenance(
        "You are not authorized"
    ));
    assert!(!anilist_moe::errors::detect_maintenance(
        "Cannot query field \"nope\" on type \"Media\"."
    ));
}

/// A `403` carrying that body has to come back as `Maintenance`, whatever the
/// status code says.
#[test]
fn a_403_with_the_outage_body_is_maintenance() {
    use anilist_moe::errors::AniListError;
    use anilist_moe::transport::{HttpResponse, Transport};

    #[derive(Debug)]
    struct Outage;
    impl Transport for Outage {
        fn execute(
            &self,
            _request: anilist_moe::transport::HttpRequest,
        ) -> std::pin::Pin<
            Box<dyn std::future::Future<Output = Result<HttpResponse, AniListError>> + Send + '_>,
        > {
            Box::pin(async move {
                Ok(HttpResponse::new(
                    403,
                    br#"{"errors":[{"message":"The AniList API has been temporarily disabled due to severe stability issues. Please check the official AniList Discord for more information.","status":403}],"data":null}"#.to_vec(),
                ))
            })
        }
    }

    let client = anilist_moe::AniListClient::builder()
        .transport(Outage)
        .build()
        .expect("a client with a scripted transport");

    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("a runtime");
    let error = runtime
        .block_on(client.user().fetch_basic())
        .expect_err("the outage should not look like a success");

    assert!(
        matches!(error, AniListError::Maintenance { .. }),
        "a 403 saying the API is temporarily disabled is an outage, not a permission problem: {error:?}"
    );
}
