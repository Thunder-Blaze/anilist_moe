//! Tests for User endpoint

use crate::test_harness::{delay_between_tests, TestHarness};
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
            let options = FetchUserOptions::builder()
                .search("ThunderBlaze")
                .per_page(5)
                .build();
            client.user().fetch(&options).await
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
            let options = FetchUserOptions::builder().id(5429396).build();
            client.user().fetch(&options).await
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
            let options = FetchUserOneOptions::builder().id(5429396).build();
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
async fn test_get_user_by_name() {
    // The document used to declare `$id: Int!`, so a lookup by name alone was
    // rejected before it ran.
    delay_between_tests().await;
    let h = harness();
    let client = h.client();

    let result = h
        .run(|| async { client.user().get_by_name("ThunderBlaze").await })
        .await;

    let user = result.expect("a user can be fetched by name alone");
    assert_eq!(user.id, 5429396, "the name resolves to the right user");
}

#[tokio::test]
async fn test_fetch_user_stats() {
    // The stats document used to carry stray text and failed to parse.
    delay_between_tests().await;
    let h = harness();
    let client = h.client();

    let result = h
        .run(|| async {
            let options = FetchUserStatsOptions::builder().user_id(5429396).build();
            client.user().stats(&options).await
        })
        .await;

    let user = result.expect("user statistics can be fetched");
    assert_eq!(user.id, 5429396);
    assert!(user.statistics.is_some(), "the statistics are returned");
}

#[tokio::test]
async fn test_user_data_types() {
    delay_between_tests().await;
    let h = harness();
    let client = h.client();

    let result = h
        .run(|| async {
            let options = FetchUserOptions::builder().id(3225).build();
            client.user().fetch(&options).await
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
            let options = FetchUserOptions::builder()
                .search("Josh")
                .per_page(10)
                .build();
            client.user().fetch(&options).await
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
