use anilist_moe::AniListClient;
use tokio::time::{Duration, sleep};

async fn rate_limit() {
    sleep(Duration::from_secs(1)).await;
}

#[tokio::test]
async fn test_get_popular_characters() {
    let client = AniListClient::new();
    let result = client.character().get_popular(1, 5).await;

    println!("Result: {:?}", result);
    assert!(result.is_ok());

    rate_limit().await;
}

#[tokio::test]
async fn test_search_characters() {
    rate_limit().await;

    let client = AniListClient::new();
    let result = client.character().search("Goku", 1, 3).await;

    println!("Result: {:?}", result);
    assert!(result.is_ok());

    rate_limit().await;
}

#[tokio::test]
async fn test_get_trending_characters() {
    rate_limit().await;

    let client = AniListClient::new();
    let result = client.character().get_trending(1, 5).await;

    println!("Result: {:?}", result);
    assert!(result.is_ok());

    rate_limit().await;
}

#[tokio::test]
async fn test_get_character_by_id() {
    rate_limit().await;

    let client = AniListClient::new();
    let result = client.character().get_by_id(1).await;

    println!("Result: {:?}", result);
    assert!(result.is_ok());

    rate_limit().await;
}

#[tokio::test]
async fn test_get_birthday_characters() {
    rate_limit().await;

    let client = AniListClient::new();
    let result = client.character().get_birthday_characters(1, 3).await;

    println!("Result: {:?}", result);
    assert!(result.is_ok());

    rate_limit().await;
}

#[tokio::test]
async fn test_get_most_favorited_characters() {
    rate_limit().await;

    let client = AniListClient::new();
    let result = client.character().get_most_favorited(1, 5).await;

    println!("Result: {:?}", result);
    assert!(result.is_ok());

    rate_limit().await;
}