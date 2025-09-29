use anilist_moe::AniListClient;
use tokio::time::{Duration, sleep};

async fn rate_limit() {
    sleep(Duration::from_secs(1)).await;
}

#[tokio::test]
async fn test_get_upcoming_episodes() {
    let client = AniListClient::new();
    let result = client.airing().get_upcoming_episodes(1, 5).await;

    println!("Result: {:?}", result);
    assert!(result.is_ok());

    rate_limit().await;
}

#[tokio::test]
async fn test_get_today_episodes() {
    rate_limit().await;

    let client = AniListClient::new();
    let result = client.airing().get_today_episodes(1, 5).await;

    println!("Result: {:?}", result);
    assert!(result.is_ok());

    rate_limit().await;
}

#[tokio::test]
async fn test_get_recently_aired() {
    rate_limit().await;

    let client = AniListClient::new();
    let result = client.airing().get_recently_aired(1, 5).await;

    println!("Result: {:?}", result);
    assert!(result.is_ok());

    rate_limit().await;
}

#[tokio::test]
async fn test_get_schedule_for_media() {
    rate_limit().await;

    let client = AniListClient::new();
    // Using Attack on Titan anime ID
    let result = client.airing().get_schedule_for_media(16498, 1, 3).await;

    println!("Result: {:?}", result);
    assert!(result.is_ok());

    rate_limit().await;
}

#[tokio::test]
async fn test_get_schedule_by_id() {
    rate_limit().await;

    let client = AniListClient::new();
    let result = client.airing().get_schedule_by_id(1).await;

    println!("Result: {:?}", result);
    assert!(result.is_ok());

    rate_limit().await;
}
