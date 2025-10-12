//! Tests for Recommendation endpoint

use anilist_moe::{AniListClient, endpoints::recommendation::*};
use dotenv::dotenv;
use log::info;
use std::env;

fn get_authenticated_client() -> AniListClient {
    dotenv().ok();
    let token = env::var("ANILIST_TOKEN").expect("ANILIST_TOKEN must be set in .env file");
    AniListClient::with_token(&token)
}

#[tokio::test]
async fn test_fetch_recommendations() {
    let client = AniListClient::new();
    let options = FetchRecommendationOptions {
        per_page: Some(5),
        ..Default::default()
    };

    let result = client.recommendation().fetch(&options).await;
    if let Err(ref e) = result {
        eprintln!("Error fetching recommendations: {:?}", e);
    }
    assert!(
        result.is_ok(),
        "Should successfully fetch recommendations: {:?}",
        result.err()
    );

    let response = result.unwrap();
    info!("Response: {:?}", response);
    let recommendations = &response.data;
    assert!(
        !recommendations.is_empty(),
        "Should return at least one recommendation"
    );
}

#[tokio::test]
async fn test_fetch_recommendations_by_media() {
    let client = AniListClient::new();
    let options = FetchRecommendationOptions {
        media_id: Some(1), // Cowboy Bebop
        per_page: Some(5),
        ..Default::default()
    };

    let result = client.recommendation().fetch(&options).await;
    assert!(
        result.is_ok(),
        "Should successfully fetch recommendations by media"
    );

    let response = result.unwrap();
    info!("Response: {:?}", response);
    let recommendations = &response.data;
    if !recommendations.is_empty() {
        let first_rec = &recommendations[0];
        assert!(first_rec.id > 0, "Recommendation should have a positive ID");
    }
}

#[tokio::test]
async fn test_recommendation_data_types() {
    let client = AniListClient::new();
    let options = FetchRecommendationOptions {
        per_page: Some(1),
        ..Default::default()
    };

    let result = client.recommendation().fetch(&options).await;
    assert!(result.is_ok(), "Should successfully fetch recommendations");

    let response = result.unwrap();
    info!("Response: {:?}", response);
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
    let client = get_authenticated_client();

    // Try to save a recommendation rating
    // Recommending Cowboy Bebop (1) -> Samurai Champloo (205)
    let options = SaveRecommendationOptions {
        media_id: 1,
        media_recommendation_id: 205,
        rating: 1, // 1 = upvote, -1 = downvote
    };

    let result = client.recommendation().save(&options).await;

    match result {
        Ok(response) => {
            info!("Save Response: {:?}", response);
            println!("Successfully saved recommendation");
            println!("Recommendation ID: {}", response.id);
        }
        Err(e) => {
            println!("Expected authentication error or permission issue: {:?}", e);
        }
    }
}
