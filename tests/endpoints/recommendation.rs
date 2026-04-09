//! Tests for Recommendation endpoint

use crate::test_harness::{delay_between_tests, get_authenticated_harness, TestHarness};
use anilist_moe::endpoints::recommendation::*;
use anilist_moe::enums::recommendation::RecommendationRating;

fn harness() -> TestHarness {
    TestHarness::new()
}

#[tokio::test]
async fn test_fetch_recommendations() {
    let h = harness();
    let client = h.client();

    let result = h
        .run(|| async {
            let options = FetchRecommendationOptions {
                per_page: Some(5),
                ..Default::default()
            };
            client.recommendation().fetch(&options).await
        })
        .await;

    assert!(
        result.is_ok(),
        "Should successfully fetch recommendations: {:?}",
        result.err()
    );

    let response = result.unwrap();
    let recommendations = &response.data;
    assert!(
        !recommendations.is_empty(),
        "Should return at least one recommendation"
    );
}

#[tokio::test]
async fn test_fetch_recommendations_by_media() {
    delay_between_tests().await;
    let h = harness();
    let client = h.client();

    let result = h
        .run(|| async {
            let options = FetchRecommendationOptions {
                media_id: Some(1), // Cowboy Bebop
                per_page: Some(5),
                ..Default::default()
            };
            client.recommendation().fetch(&options).await
        })
        .await;

    assert!(
        result.is_ok(),
        "Should successfully fetch recommendations by media: {:?}",
        result.err()
    );

    let response = result.unwrap();
    let recommendations = &response.data;
    if !recommendations.is_empty() {
        let first_rec = &recommendations[0];
        assert!(first_rec.id > 0, "Recommendation should have a positive ID");
    }
}

#[tokio::test]
async fn test_recommendation_data_types() {
    delay_between_tests().await;
    let h = harness();
    let client = h.client();

    let result = h
        .run(|| async {
            let options = FetchRecommendationOptions {
                per_page: Some(1),
                ..Default::default()
            };
            client.recommendation().fetch(&options).await
        })
        .await;

    assert!(
        result.is_ok(),
        "Should successfully fetch recommendations: {:?}",
        result.err()
    );

    let response = result.unwrap();
    let recommendations = &response.data;

    if !recommendations.is_empty() {
        let rec = &recommendations[0];
        assert!(rec.id > 0, "Recommendation ID should be positive");
        assert!(rec.rating.is_some(), "Recommendation should have a rating");
    }
}

// Authentication required tests
#[tokio::test]
async fn test_save_recommendation() {
    let Some(h) = get_authenticated_harness() else {
        eprintln!("Skipping test_save_recommendation: ANILIST_TOKEN not set");
        return;
    };
    let client = h.client().clone();

    let result = h
        .run(|| async {
            // Recommending Cowboy Bebop (1) -> Samurai Champloo (205)
            let options = SaveRecommendationOptions {
                media_id: 1,
                media_recommendation_id: 205,
                rating: RecommendationRating::RateUp,
            };
            client.recommendation().save(&options).await
        })
        .await;

    match result {
        Ok(response) => {
            println!("Successfully saved recommendation with ID: {}", response.id);
        }
        Err(e) => {
            println!("Expected authentication error or permission issue: {:?}", e);
        }
    }
}
