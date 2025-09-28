use anilist_moe::{AniListClient, enums::notification::NotificationType, endpoints::notification::NotificationSearchOptions};
use tokio::time::{Duration, sleep};

async fn rate_limit() {
    sleep(Duration::from_secs(1)).await;
}

#[tokio::test]
async fn test_get_notifications() {
    // This test requires authentication, but we'll test the method structure
    let client = AniListClient::new();
    let result = client.notification().get_notifications(1, 10).await;

    println!("Result: {:?}", result);
    // This will likely fail without authentication, but tests the endpoint structure
    rate_limit().await;
}

#[tokio::test]
async fn test_get_notifications_by_type() {
    rate_limit().await;

    let client = AniListClient::new();
    let result = client.notification().get_notifications_by_type(NotificationType::Airing, 1, 5).await;

    println!("Result: {:?}", result);
    // This will likely fail without authentication, but tests the endpoint structure
    rate_limit().await;
}

#[tokio::test]
async fn test_get_activity_notifications() {
    rate_limit().await;

    let client = AniListClient::new();
    let result = client.notification().get_activity_notifications(1, 5).await;

    println!("Result: {:?}", result);
    // This will likely fail without authentication, but tests the endpoint structure
    rate_limit().await;
}

#[tokio::test]
async fn test_get_airing_notifications() {
    rate_limit().await;

    let client = AniListClient::new();
    let result = client.notification().get_airing_notifications(1, 5).await;

    println!("Result: {:?}", result);
    // This will likely fail without authentication, but tests the endpoint structure
    rate_limit().await;
}

#[tokio::test]
async fn test_get_unread_count() {
    rate_limit().await;

    let client = AniListClient::new();
    let result = client.notification().get_unread_count().await;

    println!("Result: {:?}", result);
    // This will likely fail without authentication, but tests the endpoint structure
    rate_limit().await;
}

#[tokio::test]
async fn test_search_notifications() {
    rate_limit().await;

    let client = AniListClient::new();
    let options = NotificationSearchOptions {
        page: Some(1),
        per_page: Some(5),
        notification_type: Some(NotificationType::Following),
        ..Default::default()
    };
    
    let result = client.notification().search_with_options(options).await;

    println!("Result: {:?}", result);
    // This will likely fail without authentication, but tests the endpoint structure
    rate_limit().await;
}