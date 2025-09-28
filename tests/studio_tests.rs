use anilist_moe::AniListClient;
use tokio::time::{Duration, sleep};

async fn rate_limit() {
    sleep(Duration::from_secs(1)).await;
}

#[tokio::test]
async fn test_get_popular_studios() {
    let client = AniListClient::new();
    let result = client.studio().get_popular(1, 5).await;

    println!("Result: {:?}", result);
    assert!(result.is_ok());

    rate_limit().await;
}

#[tokio::test]
async fn test_search_studios() {
    rate_limit().await;

    let client = AniListClient::new();
    let result = client.studio().search("Madhouse", 1, 5).await;

    println!("Result: {:?}", result);
    assert!(result.is_ok());

    rate_limit().await;
}

#[tokio::test]
async fn test_get_trending_studios() {
    rate_limit().await;

    let client = AniListClient::new();
    let result = client.studio().get_trending(1, 5).await;

    println!("Result: {:?}", result);
    assert!(result.is_ok());

    rate_limit().await;
}

#[tokio::test]
async fn test_get_studio_by_id() {
    rate_limit().await;

    let client = AniListClient::new();
    let result = client.studio().get_by_id(1).await;

    println!("Result: {:?}", result);
    assert!(result.is_ok());

    rate_limit().await;
}

#[tokio::test]
async fn test_get_most_favorited_studios() {
    rate_limit().await;

    let client = AniListClient::new();
    let result = client.studio().get_most_favorited(1, 5).await;

    println!("Result: {:?}", result);
    assert!(result.is_ok());

    rate_limit().await;
}

#[tokio::test]
async fn test_get_studios_by_name() {
    rate_limit().await;

    let client = AniListClient::new();
    let result = client.studio().get_by_name(1, 5).await;

    println!("Result: {:?}", result);
    assert!(result.is_ok());

    rate_limit().await;
}