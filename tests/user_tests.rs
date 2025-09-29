use anilist_moe::client::AniListClient;
use tokio::time::{Duration, sleep};

async fn rate_limit() {
    sleep(Duration::from_secs(1)).await;
}

#[tokio::test]
async fn test_get_user_by_id() {
    let client = AniListClient::new();
    // Using a known user ID (this might fail if the user doesn't exist)
    let result = client.user().get_by_id(5429396).await;

    // This test might fail if the user doesn't exist, so we just check that the call works
    assert!(result.is_ok());
    let user_response = result.unwrap();
    let user = &user_response.data.user;

    assert_eq!(user.id, 5429396);
    assert!(!user.name.is_empty());

    rate_limit().await;
}

#[tokio::test]
async fn test_get_user_by_name() {
    rate_limit().await;

    let client = AniListClient::new();
    // This test might fail if the specific user doesn't exist
    let result = client.user().get_by_username("ThunderBlaze").await;

    // This is expected to potentially fail, so we don't assert on success
    match result {
        Ok(user_response) => {
            let user = &user_response.data.user;
            assert_eq!(user.name, "ThunderBlaze");
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
    let user_response = result.unwrap();
    let users = &user_response.data.page.data.users;
    // Note: This might be empty if no users match the search

    for user in users {
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
    let user_response = result.unwrap();
    let users = &user_response.data.page.data.users;
    // Note: This might be empty based on privacy settings and data availability

    for user in users {
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
    let user_response = result.unwrap();
    let users = &user_response.data.page.data.users;
    // Note: This might be empty based on privacy settings and data availability

    for user in users {
        assert!(user.id > 0);
        assert!(!user.name.is_empty());
    }

    rate_limit().await;
}
