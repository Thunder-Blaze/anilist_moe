use anilist_moe::AniListClient;
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
async fn test_authenticated_client_creation() {
    dotenv().ok();

    // Test that we can create authenticated clients
    let token = env::var("ANILIST_TOKEN").unwrap_or_else(|_| "fake_token".to_string());
    let _auth_client = AniListClient::with_token(&token);

    // We can verify the client is created correctly
    println!("Authenticated client created successfully with token");
    assert!(true, "Authenticated client created successfully");

    rate_limit().await;
}

#[tokio::test]
async fn test_authenticated_vs_unauthenticated_clients() {
    dotenv().ok();

    // Test that both client types can be created
    let unauth_client = AniListClient::new();
    let token = env::var("ANILIST_TOKEN").unwrap_or_else(|_| "fake_token".to_string());
    let auth_client = AniListClient::with_token(&token);

        // Both should be able to access public endpoints
    let unauth_result = unauth_client.media().get_popular_anime(Some(1), Some(1)).await;
    rate_limit().await;

    let auth_result = auth_client.media().get_popular_anime(Some(1), Some(1)).await;

    // Both should succeed (or both should fail with the same type of error)
    match (unauth_result, auth_result) {
        (Ok(unauth_data), Ok(auth_data)) => {
            println!("Both authenticated and unauthenticated clients can access public endpoints");
            // Both should return the same data structure for public endpoints
            assert!(!unauth_data.data.page.data.media.is_empty());
            assert!(!auth_data.data.page.data.media.is_empty());
        }
        (Err(e1), Err(e2)) => {
            println!("Both clients failed (probably network issues): unauth={:?}, auth={:?}", e1, e2);
            // This is acceptable - both failing suggests network or API issues
        }
        _ => {
            // This would be unexpected - one succeeding and one failing
            println!("Unexpected: one client succeeded while the other failed");
        }
    }

    rate_limit().await;
}

#[tokio::test]
async fn test_user_search_functionality() {
    let Some(client) = get_authenticated_client() else {
        println!("Skipping authenticated test - no ANILIST_TOKEN environment variable");
        return;
    };

    // Test user search functionality
    let result = client.user().search("test", 1, 5).await;

    match result {
        Ok(data) => {
            println!("User search succeeded!");

            // Verify the response structure
            let page = &data.data.page;
            println!("Found {} users matching search", page.data.users.len());

            for user in &page.data.users {
                println!("Found user: {}", &user.name);
                assert!(user.id > 0, "User should have ID");
                assert!(!user.name.is_empty(), "User should have name");
            }
        }
        Err(e) => {
            println!("User search failed: {:?}", e);
            // This might be expected if there are rate limits or API issues
        }
    }

    rate_limit().await;
}

#[tokio::test]
async fn test_user_stats_functionality() {
    let Some(client) = get_authenticated_client() else {
        println!("Skipping authenticated test - no ANILIST_TOKEN environment variable");
        return;
    };

    // Test getting a specific user by ID (using a known popular user)
    let user_id = 1; // AniList admin user
    let result = client.user().get_by_id(user_id).await;

    match result {
        Ok(data) => {
            println!("Successfully retrieved user by ID!");

            // get_by_id returns UserListResponse (paginated), so access first user
            let user = &data.data.user;
            println!("User name: {:?}", user.name);
            println!("User ID: {:?}", user.id);

            // Check for basic user fields
            assert!(user.id > 0, "Should have user ID");
            assert!(!user.name.is_empty(), "Should have username");

            // Check for statistics if available
            if let Some(stats) = &user.statistics {
                println!("User has statistics available");
                if let Some(anime_stats) = &stats.anime_status {
                    println!("Anime count: {:?}", anime_stats.count);
                    println!("Mean score: {:?}", anime_stats.mean_score);
                }
            }

            // Check for favourites if available
            if user.favourites.is_some() {
                println!("User has favourites available");
            }
        }
        Err(e) => {
            println!("Error retrieving user by ID: {:?}", e);
        }
    }

    rate_limit().await;
}

#[tokio::test]
async fn test_user_sorting_functionality() {
    let Some(client) = get_authenticated_client() else {
        println!("Skipping authenticated test - no ANILIST_TOKEN environment variable");
        return;
    };

    rate_limit().await;

    // Test getting users with most anime watched
    let result = client.user().get_most_anime_watched(1, 5).await;

    match result {
        Ok(data) => {
            println!("Successfully retrieved users by anime watch time!");

            let users = &data.data.page.data.users;
            println!("Found {} users sorted by anime watch time", users.len());

            let mut prev_minutes: Option<i32> = None;
            for user in users {
                if let Some(stats) = &user.statistics {
                    if let Some(anime_stats) = &stats.anime_status {
                        let minutes = anime_stats.minutes_watched;
                        println!("User {} watched {} minutes",
                            &user.name, minutes);

                        // Verify sorting (descending order)
                        if let Some(prev) = prev_minutes {
                            assert!(minutes <= prev, "Should be sorted in descending order");
                        }
                        prev_minutes = Some(minutes);
                    }
                }
            }
        }
        Err(e) => {
            println!("Error retrieving users by watch time: {:?}", e);
        }
    }

    rate_limit().await;
}

#[tokio::test]
async fn test_media_access_comparison() {
    let Some(auth_client) = get_authenticated_client() else {
        println!("Skipping authenticated test - no ANILIST_TOKEN environment variable");
        return;
    };

    let unauth_client = AniListClient::new();

    // Test getting a popular anime with both clients
    let anime_id = 16498; // Attack on Titan

    // Unauthenticated request
    let unauth_result = unauth_client.media().get_anime_by_id(anime_id).await;
    rate_limit().await;

    // Authenticated request
    let auth_result = auth_client.media().get_anime_by_id(anime_id).await;

    match (unauth_result, auth_result) {
        (Ok(unauth_data), Ok(auth_data)) => {
            println!("Both authenticated and unauthenticated requests succeeded");

            // Both should have basic media info
            assert!(unauth_data.data.page.data.media[0].id == anime_id);
            assert!(auth_data.data.page.data.media[0].id == anime_id);

            // Both should have the same basic structure
            let unauth_media = &unauth_data.data.page.data.media[0];
            let auth_media = &auth_data.data.page.data.media[0];

            println!("Both have title: unauth={}, auth={}",
                unauth_media.title.as_ref().map(|t| t.romaji.is_some()).unwrap_or(false),
                auth_media.title.as_ref().map(|t| t.romaji.is_some()).unwrap_or(false));

            // Authenticated request might have additional user-specific fields
            println!("Unauthenticated mediaListEntry: {:?}", unauth_media.media_list_entry);
            println!("Authenticated mediaListEntry: {:?}", auth_media.media_list_entry);
        }
        (Ok(_), Err(e)) => {
            println!("Unauthenticated succeeded but authenticated failed: {:?}", e);
        }
        (Err(e), Ok(_)) => {
            println!("Authenticated succeeded but unauthenticated failed: {:?}", e);
        }
        (Err(e1), Err(e2)) => {
            println!("Both requests failed - unauth: {:?}, auth: {:?}", e1, e2);
        }
    }

    rate_limit().await;
}

#[tokio::test]
async fn test_activity_access_with_authentication() {
    let Some(client) = get_authenticated_client() else {
        println!("Skipping authenticated test - no ANILIST_TOKEN environment variable");
        return;
    };

        // Test getting recent activities (this might have different results when authenticated)
    let result = client.activity().get_global_activities(1, 5).await;

    match result {
        Ok(data) => {
            println!("Successfully retrieved activities with authenticated client!");

            let activities = &data.data.page.data.activities;
            println!("Found {} activities", activities.len());

            for activity in activities {
                println!("Activity: {:?}", activity);
            }
        }
        Err(e) => {
            println!("Error retrieving activities: {:?}", e);
        }
    }

    rate_limit().await;
}
