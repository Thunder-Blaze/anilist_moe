use anilist_moe::AniListClient;
use tokio::time::{Duration, sleep};

async fn rate_limit() {
    sleep(Duration::from_secs(1)).await;
}

#[tokio::test]
async fn test_get_recent_reviews() {
    let client = AniListClient::new();
    let result = client.review().get_recent_reviews(1, 5).await;

    println!("Result: {:?}", result);
    assert!(result.is_ok());

    rate_limit().await;
}

#[tokio::test]
async fn test_get_reviews_for_media() {
    rate_limit().await;

    let client = AniListClient::new();
    let result = client.review().get_reviews_for_media(1, 1, 5).await;

    println!("Result: {:?}", result);
    assert!(result.is_ok());

    rate_limit().await;
}

#[tokio::test]
async fn test_get_top_rated_reviews() {
    rate_limit().await;

    let client = AniListClient::new();
    let result = client.review().get_top_rated_reviews(1, 5).await;

    println!("Result: {:?}", result);
    assert!(result.is_ok());

    rate_limit().await;
}

#[tokio::test]
async fn test_get_review_by_id() {
    rate_limit().await;

    let client = AniListClient::new();
    let result = client.review().get_review_by_id(1).await;

    println!("Result: {:?}", result);
    assert!(result.is_ok());

    rate_limit().await;
}

#[tokio::test]
async fn test_get_high_scored_reviews() {
    rate_limit().await;

    let client = AniListClient::new();
    let result = client.review().get_high_scored_reviews(80, 1, 5).await;

    println!("Result: {:?}", result);
    assert!(result.is_ok());

    rate_limit().await;
}

#[tokio::test]
async fn test_get_reviews_by_user() {
    rate_limit().await;

    let client = AniListClient::new();
    let result = client.review().get_reviews_by_user(1, 1, 5).await;

    println!("Result: {:?}", result);
    assert!(result.is_ok());

    rate_limit().await;
}