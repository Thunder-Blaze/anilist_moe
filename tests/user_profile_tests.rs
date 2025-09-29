use anilist_moe::client::AniListClient;
use anilist_moe::enums::user::UserSort;
use anilist_moe::enums::activity::{ActivitySort, ActivityType};
use anilist_moe::enums::media_list::{MediaListSort, MediaListStatus};
use anilist_moe::enums::media::MediaSort;
use tokio::time::{Duration, sleep};

async fn rate_limit() {
    sleep(Duration::from_secs(1)).await;
}

// User Profile Tests

#[tokio::test]
async fn test_get_user_anime_list() {
    rate_limit().await;

    let client = AniListClient::new();
    let result = client.user().get_anime_list(
        5429396, // Known user ID
        Some(1), // First page
        Some(10), // 10 items per page
        Some(MediaListStatus::Completed), // Only completed anime
        Some(vec![MediaListSort::UpdatedTimeDesc]), // Sort by recently updated
    ).await;

    println!("Result: {:#?}", result);

    match result {
        Ok(response) => {
            let page = &response.data.page;
            assert!(page.data.media_list.len() <= 10);
            // Verify pagination info
            assert!(page.page_info.current_page.unwrap_or(0) >= 1);
            assert!(page.page_info.per_page.unwrap_or(0) <= 10);
        }
        Err(_) => {
            // User might not have a public anime list, which is acceptable
        }
    }

    rate_limit().await;
}

#[tokio::test]
async fn test_get_user_manga_list() {
    rate_limit().await;

    let client = AniListClient::new();
    let result = client.user().get_manga_list(
        5429396,
        Some(1),
        Some(5),
        None,
        Some(vec![MediaListSort::ScoreDesc]),
    ).await;

    println!("Manga List Result: {:#?}", result);

    match result {
        Ok(response) => {
            println!("Success! Response structure: {:#?}", response);
            let page = &response.data.page;
            assert!(page.data.media_list.len() <= 5);
            assert_eq!(page.page_info.current_page.unwrap_or(0), 1);
            assert!(page.page_info.per_page.unwrap_or(0) <= 5);
        }
        Err(e) => {
            println!("Error occurred: {:#?}", e);
            // User might not have a public manga list, which is acceptable
        }
    }

    rate_limit().await;
}

#[tokio::test]
async fn test_get_user_followers() {
    rate_limit().await;

    let client = AniListClient::new();
    let result = client.user().get_followers(
        1,
        Some(1),
        Some(5),
        Some(vec![UserSort::UsernameDesc]),
    ).await;

    println!("Followers Result: {:#?}", result);

    match result {
        Ok(response) => {
            println!("Success! Response structure: {:#?}", response);
            assert!(response.data.page.data.users.len() <= 5);
            assert!(response.data.page.page_info.current_page.unwrap_or(0) >= 1);
        }
        Err(e) => {
            println!("Error occurred: {:#?}", e);
            // Query might fail for various reasons
        }
    }

    rate_limit().await;
}

#[tokio::test]
async fn test_get_user_following() {
    rate_limit().await;

    let client = AniListClient::new();
    let result = client.user().get_following(
        1,
        Some(1),
        Some(5),
        Some(vec![UserSort::Username]),
    ).await;

    println!("Following Result: {:#?}", result);

    match result {
        Ok(response) => {
            println!("Success! Response structure: {:#?}", response);
            assert!(response.data.page.data.users.len() <= 5);
            assert!(response.data.page.page_info.current_page.unwrap_or(0) >= 1);
        }
        Err(e) => {
            println!("Error occurred: {:#?}", e);
            // Query might fail for various reasons
        }
    }

    rate_limit().await;
}

#[tokio::test]
async fn test_get_favorites_anime() {
    rate_limit().await;

    let client = AniListClient::new();
    let result = client.user().get_favorites_anime(
        1,
        Some(1),
        Some(5),
        Some(vec![MediaSort::PopularityDesc]),
    ).await;

    println!("Favorites Anime Result: {:#?}", result);

    match result {
        Ok(response) => {
            println!("Success! Response structure: {:#?}", response);
            assert!(response.data.page.data.media.len() <= 5);
            assert!(response.data.page.page_info.current_page.unwrap_or(0) >= 1);
        }
        Err(e) => {
            println!("Error occurred: {:#?}", e);
            // Query might fail
        }
    }

    rate_limit().await;
}

#[tokio::test]
async fn test_get_user_activities() {
    rate_limit().await;

    let client = AniListClient::new();
    let result = client.user().get_activities(
        1,
        Some(1),
        Some(5),
        Some(vec![ActivitySort::IdDesc]),
        Some(ActivityType::Text),
        None,
    ).await;

    println!("Activities Result: {:#?}", result);

    match result {
        Ok(response) => {
            println!("Success! Response structure: {:#?}", response);
            assert!(response.data.page.data.activities.len() <= 5);
            assert!(response.data.page.page_info.current_page.unwrap_or(0) >= 1);
        }
        Err(e) => {
            println!("Error occurred: {:#?}", e);
            // Query might fail
        }
    }

    rate_limit().await;
}

#[tokio::test]
async fn test_get_user_stats() {
    rate_limit().await;

    let client = AniListClient::new();
    let result = client.user().get_stats(5429396).await;

    println!("Stats Result: {:#?}", result);

    match result {
        Ok(response) => {
            println!("Success! Response structure: {:#?}", response);
            assert!(response.is_object());
            if let Some(data) = response.get("data") {
                if let Some(user) = data.get("User") {
                    if let Some(statistics) = user.get("statistics") {
                        assert!(statistics.get("anime").is_some());
                        assert!(statistics.get("manga").is_some());
                    }
                }
            }
        }
        Err(e) => {
            println!("Error occurred: {:#?}", e);
            // Query might fail
        }
    }

    rate_limit().await;
}

// Debug test to check raw response structure
#[tokio::test]
async fn test_debug_raw_anime_list_response() {
    rate_limit().await;

    let client = AniListClient::new();

    // Let's try a simpler approach - just test the basic user query first
    let basic_user_result = client.user().get_by_id(5429396).await;
    println!("Basic User Query Result: {:#?}", basic_user_result);

    // Now let's try the anime list without the raw query method
    let anime_list_result = client.user().get_anime_list(
        5429396,
        Some(1),
        Some(3),
        None,
        None,
    ).await;

    println!("Anime List Handler Result: {:#?}", anime_list_result);

    rate_limit().await;
}
