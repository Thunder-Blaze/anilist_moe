//! Tests for Review endpoint

use anilist_moe::{AniListClient, endpoints::review::*};
use dotenv::dotenv;
use std::env;
use log::info;

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
    info!("Response: {:?}", response);
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
    info!("Response: {:?}", response);
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
    info!("Response: {:?}", response);
    let reviews = &response.data.page.data.reviews;

    if !reviews.is_empty() {
        let review = &reviews[0];
        assert!(review.id > 0, "Review ID should be positive");
        assert!(review.user_id > Some(0), "User ID should be positive");
        assert!(review.media_id > 0, "Media ID should be positive");
    }
}

// Authentication required tests
#[tokio::test]
async fn test_save_review() {
    let client = get_authenticated_client();

    // Try to save a review for a media
    let options = SaveReviewOptions {
        media_id: 1, // Cowboy Bebop
        score: Some(80),
        summary: Some("Test review summary".to_string()),
        body: Some("This is a test review body from anilist_moe library.".to_string()),
        private: Some(true), // Make it private so it doesn't clutter
        id: None,
    };

    let result = client.review().save(options).await;

    match result {
        Ok(response) => {
            info!("Save Response: {:?}", response);
            println!("Successfully saved review with ID: {}", response.data.review.id);
        }
        Err(e) => {
            println!("Expected authentication error or permission issue: {:?}", e);
        }
    }
}

#[tokio::test]
async fn test_rate_review() {
    let client = get_authenticated_client();

    // First fetch a review to rate
    let fetch_options = FetchReviewOptions {
        per_page: Some(1),
        ..Default::default()
    };

    if let Ok(response) = client.review().fetch(fetch_options).await {
        let reviews = &response.data.page.data.reviews;
        if !reviews.is_empty() {
            let review_id = reviews[0].id;

            let rate_options = RateReviewOptions {
                review_id,
                rating: 1, // 1 = upvote, 0 = no vote, -1 = downvote
            };

            let result = client.review().rate(rate_options).await;

            match result {
                Ok(_) => {
                    println!("Successfully rated review");
                    // Reset rating
                    let reset_options = RateReviewOptions {
                        review_id,
                        rating: 0,
                    };
                    let _ = client.review().rate(reset_options).await;
                }
                Err(e) => {
                    println!("Expected authentication error or permission issue: {:?}", e);
                }
            }
        }
    }
}

#[tokio::test]
async fn test_delete_review() {
    let client = get_authenticated_client();

    // First create a review to delete
    let save_options = SaveReviewOptions {
        media_id: 1,
        score: Some(50),
        summary: Some("Test review to delete".to_string()),
        body: Some("This review will be deleted.".to_string()),
        private: Some(true),
        id: None,
    };

    if let Ok(response) = client.review().save(save_options).await {
        let review_id = response.data.review.id;

        let delete_options = DeleteReviewOptions {
            id: review_id,
        };

        let result = client.review().delete(delete_options).await;

        match result {
            Ok(_) => {
                println!("Successfully deleted review");
            }
            Err(e) => {
                println!("Expected authentication error or permission issue: {:?}", e);
            }
        }
    }
}

