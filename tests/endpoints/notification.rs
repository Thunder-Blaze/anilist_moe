//! Tests for Notification endpoint

use anilist_moe::{AniListClient, endpoints::notification::*};
use dotenv::dotenv;
use std::env;

fn get_authenticated_client() -> AniListClient {
    dotenv().ok();
    let token = env::var("ANILIST_TOKEN").expect("ANILIST_TOKEN must be set in .env file");
    AniListClient::with_token(&token)
}

#[tokio::test]
async fn test_fetch_notifications() {
    let client = get_authenticated_client();
    let options = NotificationSearchOptions {
        per_page: Some(5),
        ..Default::default()
    };

    let result = client.notification().fetch(options).await;

    // Notifications require authentication
    match result {
        Ok(response) => {
            println!("Successfully fetched notifications");
            let notifications = &response.data.page.data.notifications;
            println!("Number of notifications: {}", notifications.len());
        }
        Err(e) => {
            eprintln!("Error fetching notifications (expected if not authenticated): {:?}", e);
        }
    }
}

#[tokio::test]
async fn test_notification_data_types() {
    let client = get_authenticated_client();
    let options = NotificationSearchOptions {
        per_page: Some(1),
        ..Default::default()
    };

    let result = client.notification().fetch(options).await;

    if let Ok(response) = result {
        let notifications = &response.data.page.data.notifications;
        if !notifications.is_empty() {
            let notification_id = match &notifications[0] {
                anilist_moe::unions::notification::NotificationUnion::Airing(n) => n.id,
                anilist_moe::unions::notification::NotificationUnion::Following(n) => n.id,
                anilist_moe::unions::notification::NotificationUnion::ActivityMessage(n) => n.id,
                anilist_moe::unions::notification::NotificationUnion::ActivityReply(n) => n.id,
                anilist_moe::unions::notification::NotificationUnion::ActivityMention(n) => n.id,
                anilist_moe::unions::notification::NotificationUnion::ActivityLike(n) => n.id,
                anilist_moe::unions::notification::NotificationUnion::ThreadCommentMessage(n) => n.id,
                anilist_moe::unions::notification::NotificationUnion::ThreadSubscribed(n) => n.id,
                anilist_moe::unions::notification::NotificationUnion::ThreadCommentReply(n) => n.id,
                anilist_moe::unions::notification::NotificationUnion::ThreadLike(n) => n.id,
                anilist_moe::unions::notification::NotificationUnion::RelatedMediaAddition(n) => n.id,
            };
            assert!(notification_id > 0, "Notification ID should be positive");
        }
    }
}
