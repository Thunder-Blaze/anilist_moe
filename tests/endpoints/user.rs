//! Tests for User endpoint

use crate::test_harness::{TestHarness, delay_between_tests};
use anilist_moe::endpoints::user::*;

fn harness() -> TestHarness {
    TestHarness::new()
}

#[tokio::test]
async fn test_fetch_user_by_search() {
    let h = harness();
    let client = h.client();

    let result = h
        .run(|| async {
            let options = FetchUserOptions {
                search: Some("ThunderBlaze".to_string()),
                per_page: Some(5),
                ..Default::default()
            };
            client.user().fetch(options).await
        })
        .await;

    assert!(
        result.is_ok(),
        "Should successfully fetch users: {:?}",
        result.err()
    );

    let response = result.unwrap();
    let users = &response.data;
    assert!(!users.is_empty(), "Should return at least one user");

    let first_user = &users[0];
    assert!(first_user.id > 0, "User should have a positive ID");
    assert!(
        first_user
            .name
            .as_ref()
            .map(|n| !n.is_empty())
            .unwrap_or(false),
        "User should have a name"
    );
}

#[tokio::test]
async fn test_fetch_user_by_id() {
    delay_between_tests().await;
    let h = harness();
    let client = h.client();

    let result = h
        .run(|| async {
            let options = FetchUserOptions {
                id: Some(5429396),
                ..Default::default()
            };
            client.user().fetch(options).await
        })
        .await;

    assert!(result.is_ok(), "Should successfully fetch user by ID");

    let response = result.unwrap();
    let users = &response.data;
    assert_eq!(users.len(), 1, "Should return exactly one user");
    assert_eq!(users[0].id, 5429396, "Should return correct user ID");
}

#[tokio::test]
async fn test_fetch_one_user() {
    delay_between_tests().await;
    let h = harness();
    let client = h.client();

    let result = h
        .run(|| async {
            let options = FetchUserOneOptions {
                id: Some(5429396),
                ..Default::default()
            };
            client.user().fetch_one(&options).await
        })
        .await;

    assert!(result.is_ok(), "Should successfully fetch one user");

    let user = result.unwrap();
    assert!(user.id > 0, "User should have positive ID");
    assert!(
        user.name.as_ref().map(|n| !n.is_empty()).unwrap_or(false),
        "User should have name"
    );
}

#[tokio::test]
async fn test_user_data_types() {
    delay_between_tests().await;
    let h = harness();
    let client = h.client();

    let result = h
        .run(|| async {
            let options = FetchUserOptions {
                id: Some(3225), // A well-known user with complete profile
                ..Default::default()
            };
            client.user().fetch(options).await
        })
        .await;

    assert!(result.is_ok(), "Should successfully fetch user");

    let response = result.unwrap();
    let user = &response.data[0];

    assert!(
        user.name.as_ref().map(|n| !n.is_empty()).unwrap_or(false),
        "Name should not be empty"
    );

    // Check avatar structure
    if let Some(ref avatar) = user.avatar {
        assert!(
            avatar.large.is_some() || avatar.medium.is_some(),
            "Avatar should have at least one size"
        );
    }

    // Verify ID is positive
    assert!(user.id > 0, "User ID should be positive");
}

#[tokio::test]
async fn test_user_search_relevance() {
    delay_between_tests().await;
    let h = harness();
    let client = h.client();

    let result = h
        .run(|| async {
            let options = FetchUserOptions {
                search: Some("Josh".to_string()),
                per_page: Some(10),
                ..Default::default()
            };
            client.user().fetch(options).await
        })
        .await;

    assert!(result.is_ok(), "Should successfully search users");

    let response = result.unwrap();
    assert!(!response.data.is_empty(), "Should return search results");

    // At least some results should contain the search term
    let relevant_count = response
        .data
        .iter()
        .filter(|u| {
            u.name
                .as_ref()
                .map(|n| n.to_lowercase().contains("josh"))
                .unwrap_or(false)
        })
        .count();

    assert!(
        relevant_count > 0,
        "At least some results should be relevant to search term"
    );
}
