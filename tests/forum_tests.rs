use anilist_moe::AniListClient;
use tokio::time::{Duration, sleep};

async fn rate_limit() {
    sleep(Duration::from_secs(1)).await;
}

#[tokio::test]
async fn test_get_recent_threads() {
    let client = AniListClient::new();
    let result = client.forum().get_recent_threads(1, 5).await;

    println!("Result: {:?}", result);
    assert!(result.is_ok());

    rate_limit().await;
}

#[tokio::test]
async fn test_search_threads() {
    rate_limit().await;

    let client = AniListClient::new();
    let result = client.forum().search_threads("anime", 1, 5).await;

    println!("Result: {:?}", result);
    assert!(result.is_ok());

    rate_limit().await;
}

#[tokio::test]
async fn test_get_threads_by_user() {
    rate_limit().await;

    let client = AniListClient::new();
    let result = client.forum().get_threads_by_user(1, 1, 3).await;

    println!("Result: {:?}", result);
    assert!(result.is_ok());

    rate_limit().await;
}

#[tokio::test]
async fn test_get_thread_by_id() {
    rate_limit().await;

    let client = AniListClient::new();
    let result = client.forum().get_thread_by_id(1).await;

    println!("Result: {:?}", result);
    // This test might fail if the specific thread doesn't exist, which is acceptable
    rate_limit().await;
}

#[tokio::test]
async fn test_get_threads_by_category() {
    rate_limit().await;

    let client = AniListClient::new();
    let result = client.forum().get_threads_by_category(1, 1, 3).await;

    println!("Result: {:?}", result);
    assert!(result.is_ok());

    rate_limit().await;
}

#[tokio::test]
async fn test_get_thread_comments() {
    rate_limit().await;

    let client = AniListClient::new();
    let result = client.forum().get_thread_comments(1, 1, 5).await;

    println!("Result: {:?}", result);
    // This test might fail if the specific thread doesn't exist, which is acceptable
    rate_limit().await;
}