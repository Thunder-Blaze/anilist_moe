//! Tests for Studio endpoint

use crate::test_harness::{delay_between_tests, TestHarness};
use anilist_moe::endpoints::studio::*;

fn harness() -> TestHarness {
    TestHarness::new()
}

#[tokio::test]
async fn test_fetch_studio_by_search() {
    let h = harness();
    let client = h.client();

    let result = h
        .run(|| async {
            let options = FetchStudioOptions {
                search: Some("Kyoto".to_string()),
                per_page: Some(5),
                ..Default::default()
            };
            client.studio().fetch(&options).await
        })
        .await;

    assert!(
        result.is_ok(),
        "Should successfully fetch studios: {:?}",
        result.err()
    );

    let response = result.unwrap();
    let studios = &response.data;
    assert!(!studios.is_empty(), "Should return at least one studio");

    let first_studio = &studios[0];
    assert!(first_studio.id.is_some(), "Studio should have an ID");
    if let Some(id) = first_studio.id {
        assert!(id > 0, "Studio ID should be positive");
    }
    assert!(first_studio.name.is_some(), "Studio should have a name");
}

#[tokio::test]
async fn test_fetch_studio_by_id() {
    delay_between_tests().await;
    let h = harness();
    let client = h.client();

    let result = h
        .run(|| async {
            let options = FetchStudioOptions {
                id: Some(2), // Kyoto Animation
                ..Default::default()
            };
            client.studio().fetch(&options).await
        })
        .await;

    assert!(
        result.is_ok(),
        "Should successfully fetch studio by ID: {:?}",
        result.err()
    );

    let response = result.unwrap();
    let studios = &response.data;
    assert_eq!(studios.len(), 1, "Should return exactly one studio");
    assert_eq!(studios[0].id, Some(2), "Should return correct studio ID");
}

#[tokio::test]
async fn test_fetch_one_studio() {
    delay_between_tests().await;
    let h = harness();
    let client = h.client();

    let result = h
        .run(|| async {
            let options = FetchStudioOneOptions {
                id: Some(2),
                ..Default::default()
            };
            client.studio().fetch_one(&options).await
        })
        .await;

    assert!(
        result.is_ok(),
        "Should successfully fetch one studio: {:?}",
        result.err()
    );

    let studio = result.unwrap();
    assert_eq!(studio.id, Some(2), "Should return studio with ID 2");
}

#[tokio::test]
async fn test_studio_data_types() {
    delay_between_tests().await;
    let h = harness();
    let client = h.client();

    let result = h
        .run(|| async {
            let options = FetchStudioOptions {
                id: Some(2),
                ..Default::default()
            };
            client.studio().fetch(&options).await
        })
        .await;

    assert!(
        result.is_ok(),
        "Should successfully fetch studio: {:?}",
        result.err()
    );

    let response = result.unwrap();
    let studio = &response.data[0];

    if let Some(id) = studio.id {
        assert!(id > 0, "ID should be positive");
    }

    if let Some(favourites) = studio.favourites {
        assert!(favourites >= 0, "Favourites should be non-negative");
    }

    if let Some(ref name) = studio.name {
        assert!(!name.is_empty(), "Name should not be empty");
    }
}
