//! Tests for Review endpoint

use anilist_moe::{AniListClient, endpoints::review::*};
use dotenv::dotenv;
use std::env;

fn get_authenticated_client() -> AniListClient {
    dotenv().ok();
    let token = env::var("ANILIST_TOKEN").expect("ANILIST_TOKEN must be set in .env file");
    AniListClient::with_token(&token)
}

#[tokio::test]
async fn test_fetch_reviews() {
    let client = AniListClient::new();
    let options = FetchReviewOptions {
        per_page: Some(5),
        ..Default::default()
    };

    let result = client.review().fetch(options).await;
    if let Err(ref e) = result {
        eprintln!("Error fetching reviews: {:?}", e);
    }
    assert!(result.is_ok(), "Should successfully fetch reviews: {:?}", result.err());

    let response = result.unwrap();
    let reviews = &response.data.page.data.reviews;
    assert!(!reviews.is_empty(), "Should return at least one review");
}

#[tokio::test]
async fn test_fetch_reviews_by_media() {
    let client = AniListClient::new();
    let options = FetchReviewOptions {
        media_id: Some(1), // Cowboy Bebop
        per_page: Some(5),
        ..Default::default()
    };

    let result = client.review().fetch(options).await;
    assert!(result.is_ok(), "Should successfully fetch reviews by media");

    let response = result.unwrap();
    let reviews = &response.data.page.data.reviews;
    if !reviews.is_empty() {
        let first_review = &reviews[0];
        assert!(first_review.id > 0, "Review should have a positive ID");
    }
}

#[tokio::test]
async fn test_review_data_types() {
    let client = AniListClient::new();
    let options = FetchReviewOptions {
        per_page: Some(1),
        ..Default::default()
    };

    let result = client.review().fetch(options).await;
    assert!(result.is_ok(), "Should successfully fetch reviews");

    let response = result.unwrap();
    let reviews = &response.data.page.data.reviews;

    if !reviews.is_empty() {
        let review = &reviews[0];
        assert!(review.id > 0, "Review ID should be positive");
        assert!(review.user_id > 0, "User ID should be positive");
        assert!(review.media_id > 0, "Media ID should be positive");
    }
}
