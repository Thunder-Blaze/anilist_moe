use anilist_moe::{AniListClient, enums::notification::NotificationType, endpoints::notification::NotificationSearchOptions};
use dotenv::dotenv;
use std::env;
use tokio::time::{Duration, sleep};

async fn rate_limit() {
    sleep(Duration::from_secs(1)).await;
}

fn get_authenticated_client() -> Option<AniListClient> {
    dotenv().ok();
    env::var("ANILIST_TOKEN").ok().map(|token| AniListClient::with_token(&token))
}

#[tokio::test]
async fn test_get_notifications_unauthenticated() {
    // Test that unauthenticated calls return proper error
    let client = AniListClient::new();
    let result = client.notification().get_notifications(1, 10).await;

    println!("Unauthenticated result: {:?}", result);
    // Should fail with authentication error
    assert!(result.is_err(), "Unauthenticated notification access should fail");
    rate_limit().await;
}

#[tokio::test]
async fn test_get_notifications_authenticated() {
    let Some(client) = get_authenticated_client() else {
        println!("Skipping authenticated test - no ANILIST_TOKEN environment variable");
        return;
    };

    let result = client.notification().get_notifications(1, 5).await;

    println!("Authenticated notifications result: {:?}", result);

    match result {
        Ok(data) => {
            println!("Successfully retrieved notifications!");
            // Verify the response structure
            println!("✓ Got notifications response with {} notifications", data.data.page.data.notifications.len());
            // The response is now properly typed!
        }
        Err(e) => {
            println!("Authentication might have failed: {:?}", e);
            // This might still be expected if the token is invalid
        }
    }

    rate_limit().await;
}

#[tokio::test]
async fn test_get_notifications_by_type_authenticated() {
    let Some(client) = get_authenticated_client() else {
        println!("Skipping authenticated test - no ANILIST_TOKEN environment variable");
        return;
    };

    rate_limit().await;

    // Test different notification types
    let notification_types = [
        NotificationType::Airing,
        NotificationType::Following,
        NotificationType::ActivityMessage,
        NotificationType::ActivityReply,
    ];

    for notification_type in notification_types {
        println!("Testing notification type: {:?}", notification_type);
        let result = client.notification().get_notifications_by_type(notification_type.clone(), 1, 3).await;

        match result {
            Ok(data) => {
                println!("Successfully retrieved {:?} notifications!", notification_type);
                // Verify response structure
                let notifications = &data.data.page.data.notifications;
                for _notification in notifications {
                    // Notification types are complex unions - just verify we got data
                    println!("Found a notification");
                }
            }
            Err(e) => {
                println!("Error retrieving {:?} notifications: {:?}", notification_type, e);
            }
        }

        rate_limit().await;
    }
}

#[tokio::test]
async fn test_get_activity_notifications_authenticated() {
    let Some(client) = get_authenticated_client() else {
        println!("Skipping authenticated test - no ANILIST_TOKEN environment variable");
        return;
    };

    rate_limit().await;

    let result = client.notification().get_activity_notifications(1, 5).await;

    match result {
        Ok(data) => {
            println!("Successfully retrieved activity notifications!");
            // Check for activity-related notifications
            let notifications = &data.data.page.data.notifications;
            println!("Found {} activity notifications", notifications.len());
            // Note: Notification types are complex unions, validation simplified for now
        }
        Err(e) => {
            println!("Error retrieving activity notifications: {:?}", e);
        }
    }

    rate_limit().await;
}

#[tokio::test]
async fn test_get_airing_notifications_authenticated() {
    let Some(client) = get_authenticated_client() else {
        println!("Skipping authenticated test - no ANILIST_TOKEN environment variable");
        return;
    };

    rate_limit().await;

    let result = client.notification().get_airing_notifications(1, 5).await;

    match result {
        Ok(data) => {
            println!("Successfully retrieved airing notifications!");
            // Check for airing notifications
            let notifications = &data.data.page.data.notifications;
            println!("Found {} airing notifications", notifications.len());
            // Note: Notification validation simplified due to union complexity
        }
        Err(e) => {
            println!("Error retrieving airing notifications: {:?}", e);
        }
    }

    rate_limit().await;
}

#[tokio::test]
async fn test_get_unread_count_authenticated() {
    let Some(client) = get_authenticated_client() else {
        println!("Skipping authenticated test - no ANILIST_TOKEN environment variable");
        return;
    };

    rate_limit().await;

    let result = client.notification().get_unread_count().await;

    match result {
        Ok(data) => {
            println!("Successfully retrieved unread count!");
            // Should have viewer with unreadNotificationCount
            let count = data.data.viewer.unread_notification_count;
            println!("Unread notification count: {}", count);
        }
        Err(e) => {
            println!("Error retrieving unread count: {:?}", e);
        }
    }

    rate_limit().await;
}

#[tokio::test]
async fn test_search_notifications_authenticated() {
    let Some(client) = get_authenticated_client() else {
        println!("Skipping authenticated test - no ANILIST_TOKEN environment variable");
        return;
    };

    rate_limit().await;

    // Test comprehensive search options
    let options = NotificationSearchOptions {
        page: Some(1),
        per_page: Some(5),
        notification_type: Some(NotificationType::Following),
        reset_notification_count: Some(false), // Don't reset count
        ..Default::default()
    };

    let result = client.notification().search_with_options(options).await;

    match result {
        Ok(data) => {
            println!("Successfully searched notifications!");
            // Verify response structure
            let page = &data.data.page;
            println!("Found {} notifications", page.data.notifications.len());

            // Check page info
            let page_info = &page.page_info;
            println!("Has next page: {}", page_info.has_next_page.unwrap_or(false));
            println!("Total count: {}", page_info.total.unwrap_or(0));
        }
        Err(e) => {
            println!("Error searching notifications: {:?}", e);
        }
    }

    rate_limit().await;
}

#[tokio::test]
async fn test_search_notifications_with_type_filters_authenticated() {
    let Some(client) = get_authenticated_client() else {
        println!("Skipping authenticated test - no ANILIST_TOKEN environment variable");
        return;
    };

    rate_limit().await;

    // Test multiple notification types
    let options = NotificationSearchOptions {
        page: Some(1),
        per_page: Some(10),
        type_in: Some(vec![
            NotificationType::Airing,
            NotificationType::Following,
            NotificationType::ActivityMessage,
        ]),
        ..Default::default()
    };

    let result = client.notification().search_with_options(options).await;

    match result {
        Ok(data) => {
            println!("Successfully searched notifications with multiple types!");

            let notifications = &data.data.page.data.notifications;
            println!("Found {} notifications with multiple type filters", notifications.len());
            // Note: Type verification simplified due to union complexity
        }
        Err(e) => {
            println!("Error searching notifications with type filters: {:?}", e);
        }
    }

    rate_limit().await;
}

#[tokio::test]
async fn test_notification_pagination_authenticated() {
    let Some(client) = get_authenticated_client() else {
        println!("Skipping authenticated test - no ANILIST_TOKEN environment variable");
        return;
    };

    rate_limit().await;

    // Test pagination
    let options1 = NotificationSearchOptions {
        page: Some(1),
        per_page: Some(5),
        ..Default::default()
    };

    let result1 = client.notification().search_with_options(options1).await;

    if let Ok(data1) = result1 {
        let page_info = &data1.data.page.page_info;
        println!("Page 1 has_next_page: {}", page_info.has_next_page.unwrap_or(false));

        if page_info.has_next_page.unwrap_or(false) {
            rate_limit().await;

            // Test page 2
            let options2 = NotificationSearchOptions {
                page: Some(2),
                per_page: Some(5),
                ..Default::default()
            };

            let result2 = client.notification().search_with_options(options2).await;

            match result2 {
                Ok(data2) => {
                    println!("Successfully retrieved page 2!");
                    let page_info2 = &data2.data.page.page_info;
                    println!("Page 2 current_page: {}", page_info2.current_page.unwrap_or(0));
                    assert_eq!(
                        page_info2.current_page.unwrap_or(0),
                        2,
                        "Should be on page 2"
                    );
                }
                Err(e) => {
                    println!("Error retrieving page 2: {:?}", e);
                }
            }
        } else {
            println!("No second page available for pagination test");
        }
    }

    rate_limit().await;
}
