use anilist_moe::AniListClient;
use tokio::time::{Duration, sleep};

async fn rate_limit() {
    sleep(Duration::from_secs(1)).await;
}

#[tokio::test]
async fn test_get_recent_recommendations() {
    let client = AniListClient::new();
    let result = client.recommendation().get_recent(1, 5).await;

    println!("Result: {:?}", result);
    assert!(result.is_ok());

    rate_limit().await;
}

#[tokio::test]
async fn test_get_for_media() {
    rate_limit().await;

    let client = AniListClient::new();
    let result = client.recommendation().get_for_media(1, 1, 5).await;

    println!("Result: {:?}", result);
    assert!(result.is_ok());

    rate_limit().await;
}

#[tokio::test]
async fn test_get_highly_rated_recommendations() {
    rate_limit().await;

    let client = AniListClient::new();
    let result = client.recommendation().get_highly_rated(5, 1, 5).await;

    println!("Result: {:?}", result);
    assert!(result.is_ok());

    rate_limit().await;
}

#[tokio::test]
async fn test_get_recommendation_by_id() {
    rate_limit().await;

    let client = AniListClient::new();
    let result = client.recommendation().get_by_id(1).await;

    println!("Result: {:?}", result);
    assert!(result.is_ok());

    rate_limit().await;
}

#[tokio::test]
async fn test_get_by_user() {
    rate_limit().await;

    let client = AniListClient::new();
    let result = client.recommendation().get_by_user(1, 1, 5).await;

    println!("Result: {:?}", result);
    assert!(result.is_ok());

    rate_limit().await;
}
