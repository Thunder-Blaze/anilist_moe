//! Tests for Review endpoint

use crate::support::allow_auth_rejection;
use crate::test_harness::{delay_between_tests, get_authenticated_harness, TestHarness};
use anilist_moe::endpoints::review::*;
use anilist_moe::enums::review::ReviewRating;

fn harness() -> TestHarness {
    TestHarness::new()
}

#[tokio::test]
async fn test_fetch_reviews() {
    let h = harness();
    let client = h.client();

    let result = h
        .run(|| async {
            let options = FetchReviewOptions::builder().per_page(5).build();
            client.review().fetch(&options).await
        })
        .await;

    assert!(
        result.is_ok(),
        "Should successfully fetch reviews: {:?}",
        result.err()
    );

    let response = result.unwrap();
    let reviews = &response.data;
    assert!(!reviews.is_empty(), "Should return at least one review");
}

#[tokio::test]
async fn test_fetch_reviews_by_media() {
    delay_between_tests().await;
    let h = harness();
    let client = h.client();

    let result = h
        .run(|| async {
            let options = FetchReviewOptions::builder()
                .media_id(1) // Cowboy Bebop
                .per_page(5)
                .build();
            client.review().fetch(&options).await
        })
        .await;

    assert!(
        result.is_ok(),
        "Should successfully fetch reviews by media: {:?}",
        result.err()
    );

    let response = result.unwrap();
    let reviews = &response.data;
    if !reviews.is_empty() {
        let first_review = &reviews[0];
        assert!(first_review.id > 0, "Review should have a positive ID");
    }
}

#[tokio::test]
async fn test_review_data_types() {
    delay_between_tests().await;
    let h = harness();
    let client = h.client();

    let result = h
        .run(|| async {
            let options = FetchReviewOptions::builder().per_page(1).build();
            client.review().fetch(&options).await
        })
        .await;

    assert!(result.is_ok(), "Should successfully fetch reviews");

    let response = result.unwrap();
    let reviews = &response.data;

    if !reviews.is_empty() {
        let review = &reviews[0];
        assert!(review.id > 0, "Review ID should be positive");
        assert!(review.user_id > Some(0), "User ID should be positive");
        assert!(review.media_id > Some(0), "Media ID should be positive");
    }
}

// Authentication required tests
#[tokio::test]
async fn test_rate_review() {
    let Some(h) = get_authenticated_harness() else {
        eprintln!("Skipping test_rate_review: ANILIST_TOKEN not set");
        return;
    };
    let client = h.client().clone();

    // First fetch a review to rate
    let fetch_result = h
        .run(|| async {
            let options = FetchReviewOptions::builder().per_page(1).build();
            client.review().fetch(&options).await
        })
        .await;

    if let Ok(response) = fetch_result {
        let reviews = &response.data;
        if !reviews.is_empty() {
            let review_id = reviews[0].id;

            delay_between_tests().await;

            let result = h
                .run(|| async {
                    let rate_options = RateReviewOptions::builder()
                        .review_id(review_id)
                        .rating(ReviewRating::UpVote)
                        .build();
                    client.review().rate(&rate_options).await
                })
                .await;

            match result {
                Ok(_) => {
                    println!("Successfully rated review");
                    // Reset rating
                    delay_between_tests().await;
                    let _ = h
                        .run(|| async {
                            let reset_options = RateReviewOptions::builder()
                                .review_id(review_id)
                                .rating(ReviewRating::NoVote)
                                .build();
                            client.review().rate(&reset_options).await
                        })
                        .await;
                }
                Err(e) => {
                    allow_auth_rejection("test_rate_review", &e);
                }
            }
        }
    }
}
