//! Tests for Notification endpoint

use crate::test_harness::{delay_between_tests, get_authenticated_harness};
use anilist_moe::endpoints::notification::*;

#[tokio::test]
async fn test_fetch_notifications() {
    let Some(h) = get_authenticated_harness() else {
        eprintln!("Skipping test_fetch_notifications: ANILIST_TOKEN not set");
        return;
    };
    let client = h.client().clone();

    let result = h
        .run(|| async {
            let options = NotificationSearchOptions {
                per_page: Some(5),
                ..Default::default()
            };
            client.notification().fetch(&options).await
        })
        .await;

    match result {
        Ok(response) => {
            println!("Successfully fetched notifications");
            let notifications = &response.data;
            println!("Number of notifications: {}", notifications.len());
        }
        Err(e) => {
            eprintln!(
                "Error fetching notifications (expected if not authenticated): {:?}",
                e
            );
        }
    }
}

#[tokio::test]
async fn test_notification_data_types() {
    let Some(h) = get_authenticated_harness() else {
        eprintln!("Skipping test_notification_data_types: ANILIST_TOKEN not set");
        return;
    };
    let client = h.client().clone();

    delay_between_tests().await;

    let result = h
        .run(|| async {
            let options = NotificationSearchOptions {
                per_page: Some(1),
                ..Default::default()
            };
            client.notification().fetch(&options).await
        })
        .await;

    if let Ok(response) = result {
        let notifications = &response.data;
        if !notifications.is_empty() {
            let notification_id = match &notifications[0] {
                anilist_moe::unions::notification::NotificationUnion::Airing(n) => n.id,
                anilist_moe::unions::notification::NotificationUnion::Following(n) => n.id,
                anilist_moe::unions::notification::NotificationUnion::ActivityMessage(n) => n.id,
                anilist_moe::unions::notification::NotificationUnion::ActivityMention(n) => n.id,
                anilist_moe::unions::notification::NotificationUnion::ActivityReply(n) => n.id,
                anilist_moe::unions::notification::NotificationUnion::ActivityReplySubscribed(
                    n,
                ) => n.id,
                anilist_moe::unions::notification::NotificationUnion::ActivityLike(n) => n.id,
                anilist_moe::unions::notification::NotificationUnion::ActivityReplyLike(n) => n.id,
                anilist_moe::unions::notification::NotificationUnion::ThreadCommentMention(n) => {
                    n.id
                }
                anilist_moe::unions::notification::NotificationUnion::ThreadCommentReply(n) => n.id,
                anilist_moe::unions::notification::NotificationUnion::ThreadSubscribed(n) => n.id,
                anilist_moe::unions::notification::NotificationUnion::ThreadCommentLike(n) => n.id,
                anilist_moe::unions::notification::NotificationUnion::ThreadLike(n) => n.id,
                anilist_moe::unions::notification::NotificationUnion::RelatedMediaAddition(n) => {
                    n.id
                }
                anilist_moe::unions::notification::NotificationUnion::MediaDataChange(n) => n.id,
                anilist_moe::unions::notification::NotificationUnion::MediaMerge(n) => n.id,
                anilist_moe::unions::notification::NotificationUnion::MediaDeletion(n) => n.id,
            };
            assert!(notification_id > 0, "Notification ID should be positive");
        }
    }
}
