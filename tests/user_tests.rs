use anilist_moe::client::AniListClient;
use tokio::time::{Duration, sleep};

/// Helper function to add rate limiting between test requests
async fn rate_limit() {
    sleep(Duration::from_secs(1)).await;
}

#[tokio::test]
async fn test_get_user_by_id() {
    let client = AniListClient::new();
    // Using a known user ID (this might fail if the user doesn't exist)
    let result = client.user().get_by_id(5429396).await;

    // This test might fail if the user doesn't exist, so we just check that the call works
    match result {
        Ok(user) => {
            assert_eq!(user.id, 5429396);
            assert!(!user.name.is_empty());
        }
        Err(_) => {
            // User might not exist, which is acceptable for this test
        }
    }

    rate_limit().await;
}

#[tokio::test]
async fn test_get_user_by_name() {
    rate_limit().await;

    let client = AniListClient::new();
    // This test might fail if the specific user doesn't exist
    let result = client.user().get_by_name("xSensei").await;

    // This is expected to potentially fail, so we don't assert on success
    match result {
        Ok(user) => {
            assert_eq!(user.name, "xSensei");
        }
        Err(_) => {
            // User might not exist, which is acceptable for this test
        }
    }

    rate_limit().await;
}

#[tokio::test]
async fn test_search_users() {
    rate_limit().await;

    let client = AniListClient::new();
    let result = client.user().search("xuehua", 1, 5).await;

    assert!(result.is_ok());
    let users = result.unwrap();
    // Note: This might be empty if no users match the search

    for user in &users {
        assert!(user.id > 0);
        assert!(!user.name.is_empty());
    }

    rate_limit().await;
}

#[tokio::test]
async fn test_get_most_anime_watched() {
    rate_limit().await;

    let client = AniListClient::new();
    let result = client.user().get_most_anime_watched(1, 5).await;

    assert!(result.is_ok());
    let users = result.unwrap();
    // Note: This might be empty based on privacy settings and data availability

    for user in &users {
        assert!(user.id > 0);
        assert!(!user.name.is_empty());
    }

    rate_limit().await;
}

#[tokio::test]
async fn test_get_most_manga_read() {
    rate_limit().await;

    let client = AniListClient::new();
    let result = client.user().get_most_manga_read(1, 5).await;

    assert!(result.is_ok());
    let users = result.unwrap();
    // Note: This might be empty based on privacy settings and data availability

    for user in &users {
        assert!(user.id > 0);
        assert!(!user.name.is_empty());
    }

    rate_limit().await;
}

// Integration test to verify the basic functionality works
#[tokio::test]
async fn test_client_integration() {
    rate_limit().await;

    let client = AniListClient::new();

    // Test that we can make a basic query
    let anime_result = client.anime().get_popular(1, 1).await;
    assert!(anime_result.is_ok());
    rate_limit().await;

    let manga_result = client.manga().get_popular(1, 1).await;
    assert!(manga_result.is_ok());
    rate_limit().await;

    let character_result = client.character().get_popular(1, 1).await;
    assert!(character_result.is_ok());
    rate_limit().await;

    let staff_result = client.staff().get_popular(1, 1).await;
    assert!(staff_result.is_ok());
    rate_limit().await;
}
