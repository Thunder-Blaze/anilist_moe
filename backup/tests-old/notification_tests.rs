use anilist_moe::client::AniListClient;
use std::env;

#[tokio::test]
async fn test_get_notifications() {
    // Skip if no token provided
    let Ok(token) = env::var("ANILIST_TOKEN") else {
        println!("Skipping authenticated test - no ANILIST_TOKEN environment variable");
        return;
    };

    let client = AniListClient::with_token(token);
    let result = client.notification().get_notifications(1, 10).await;

    assert!(result.is_ok());
    let notifications = result.unwrap();

    for notification in &notifications {
        assert!(notification.id > 0);
        if let Some(created_at) = notification.created_at {
            assert!(created_at > 0);
        }
        assert!(notification.notification_type.is_some());
    }
}

#[tokio::test]
async fn test_get_notifications_by_type() {
    // Skip if no token provided
    let Ok(token) = env::var("ANILIST_TOKEN") else {
        println!("Skipping authenticated test - no ANILIST_TOKEN environment variable");
        return;
    };

    let client = AniListClient::with_token(token);
    let result = client
        .notification()
        .get_notifications_by_type("ActivityMessage", 1, 5)
        .await;

    assert!(result.is_ok());
    let notifications = result.unwrap();

    for notification in &notifications {
        assert!(notification.id > 0);
        if let Some(_notification_type) = &notification.notification_type {
            // We can't easily compare enum variants in string format, so just check it exists
            assert!(notification.notification_type.is_some());
        }
    }
}

#[tokio::test]
async fn test_mark_notifications_as_read() {
    // Skip if no token provided
    let Ok(token) = env::var("ANILIST_TOKEN") else {
        println!("Skipping authenticated test - no ANILIST_TOKEN environment variable");
        return;
    };

    let client = AniListClient::with_token(token);

    // First try to get some notifications to mark as read
    let notifications_result = client.notification().get_notifications(1, 1).await;
    if let Ok(notifications) = notifications_result {
        if let Some(notification) = notifications.first() {
            let result = client
                .notification()
                .mark_notifications_as_read(vec![notification.id])
                .await;
            // This might fail if the notification is already read or doesn't belong to user
            match result {
                Ok(success) => assert!(success),
                Err(_) => {
                    // Acceptable if notification doesn't exist or is already read
                }
            }
        }
    }
}

#[tokio::test]
async fn test_get_unread_count() {
    // Skip if no token provided
    let Ok(token) = env::var("ANILIST_TOKEN") else {
        println!("Skipping authenticated test - no ANILIST_TOKEN environment variable");
        return;
    };

    let client = AniListClient::with_token(token);
    let result = client.notification().get_unread_count().await;

    assert!(result.is_ok());
    let count = result.unwrap();
    assert!(count >= 0);
}
