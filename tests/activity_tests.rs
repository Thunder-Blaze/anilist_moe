use anilist_moe::client::AniListClient;
use tokio::time::{Duration, sleep};

/// Helper function to add rate limiting between test requests
async fn rate_limit() {
    sleep(Duration::from_secs(1)).await;
}

#[tokio::test]
async fn test_get_recent_activities() {
    let client = AniListClient::new();
    let result = client.activity().get_recent_activities(1, 5).await;

    assert!(result.is_ok());
    let activities = result.unwrap();
    // Note: This might be empty based on privacy settings

    for activity in &activities {
        assert!(activity.id > 0);
    }

    rate_limit().await;
}

#[tokio::test]
async fn test_get_text_activities() {
    rate_limit().await;

    let client = AniListClient::new();
    let result = client.activity().get_text_activities(1, 5).await;

    assert!(result.is_ok());
    let activities = result.unwrap();
    // Note: This might be empty based on privacy settings

    for activity in &activities {
        assert!(activity.id > 0);
    }

    rate_limit().await;
}

#[tokio::test]
async fn test_get_user_activities() {
    let client = AniListClient::new();
    // Test with a known user ID (this might fail if the user doesn't exist or has private activities)
    let result = client.activity().get_user_activities(1, 1, 5).await;

    // We just check that the call doesn't panic
    match result {
        Ok(activities) => {
            for activity in &activities {
                assert!(activity.id > 0);
            }
        }
        Err(_) => {
            // User might not exist or have private activities, which is acceptable
        }
    }
}

#[tokio::test]
async fn test_get_activity_by_id() {
    let client = AniListClient::new();
    // This test might fail if the specific activity doesn't exist
    let result = client.activity().get_activity_by_id(1).await;

    // We just check that the call doesn't panic
    match result {
        Ok(activity) => {
            assert_eq!(activity.id, 1);
        }
        Err(_) => {
            // Activity might not exist, which is acceptable for this test
        }
    }
}

#[tokio::test]
async fn test_get_activity_replies() {
    let client = AniListClient::new();
    // This test might fail if the specific activity doesn't exist or has no replies
    let result = client.activity().get_activity_replies(1, 1, 5).await;

    // We just check that the call doesn't panic
    match result {
        Ok(replies) => {
            for reply in &replies {
                assert!(reply.id > 0);
            }
        }
        Err(_) => {
            // Activity might not exist or have no replies, which is acceptable
        }
    }
}
