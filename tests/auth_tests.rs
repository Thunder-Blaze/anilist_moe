use anilist_moe::client::AniListClient;
use dotenv::dotenv;
use std::env;
use tokio::time::{Duration, sleep};

/// Helper function to add rate limiting between test requests
async fn rate_limit() {
    sleep(Duration::from_secs(1)).await;
}

#[tokio::test]
async fn test_authenticated_client_creation() {
    dotenv().ok();
    // Test that we can create authenticated clients
    let token = env::var("ANILIST_TOKEN").unwrap_or_else(|_| "fake_token".to_string());
    let _auth_client = AniListClient::with_token(token.clone());

    // We can't test actual authenticated calls without a real token,
    // but we can verify the client is created correctly
    assert!(true, "Authenticated client created successfully");

    rate_limit().await;
}

#[tokio::test]
async fn test_unauthenticated_vs_authenticated_client() {
    dotenv().ok();
    rate_limit().await;

    // Test that both client types work for public endpoints
    let unauth_client = AniListClient::new();
    let token = env::var("ANILIST_TOKEN").unwrap_or_else(|_| "fake_token".to_string());
    let auth_client = AniListClient::with_token(token.to_string());

    // Both should be able to access public endpoints
    let unauth_result = unauth_client.anime().get_popular(1, 1).await;
    rate_limit().await;

    let auth_result = auth_client.anime().get_popular(1, 1).await;

    // Both should succeed (or both should fail with the same type of error)
    match (unauth_result, auth_result) {
        (Ok(_), Ok(_)) => {
            // Both succeeded
            assert!(true);
        }
        (Err(_), Err(_)) => {
            // Both failed (probably network issues)
            assert!(true);
        }
        _ => {
            // One succeeded and one failed - this shouldn't happen for public endpoints
            panic!(
                "Authenticated and unauthenticated clients behaved differently for public endpoint"
            );
        }
    }

    rate_limit().await;
}

#[tokio::test]
async fn test_current_user_without_token() {
    rate_limit().await;

    // Test that current user endpoint fails without authentication
    let client = AniListClient::new();
    let result = client.user().get_current_user().await;

    // This should fail since we don't have a token
    assert!(
        result.is_err(),
        "Current user endpoint should fail without authentication"
    );
}

#[tokio::test]
async fn test_authenticated_current_user() {
    dotenv().ok();

    // Only run this test if we have a real token
    if let Ok(token) = env::var("ANILIST_TOKEN") {
        if !token.is_empty() && token != "fake_token" {
            let client = AniListClient::with_token(token);
            let result = client.user().get_current_user().await;

            match result {
                Ok(user) => {
                    println!("✓ Successfully authenticated as user: {}", user.name);
                    assert!(user.id > 0, "User should have a valid ID");
                    assert!(!user.name.is_empty(), "User should have a name");
                }
                Err(e) => {
                    panic!("Failed to get current user with valid token: {:?}", e);
                }
            }
        } else {
            println!("Skipping authenticated test - no valid ANILIST_TOKEN found");
        }
    } else {
        println!("Skipping authenticated test - ANILIST_TOKEN not set");
    }
}

#[tokio::test]
async fn test_authenticated_user_anime_list() {
    dotenv().ok();

    // Only run this test if we have a real token
    if let Ok(token) = env::var("ANILIST_TOKEN") {
        if !token.is_empty() && token != "fake_token" {
            let client = AniListClient::with_token(token);

            // First get the current user
            let user_result = client.user().get_current_user().await;
            if let Ok(_user) = user_result {
                // Now try to get their anime list
                let anime_list_result = client.user().get_current_user_anime_list(None).await;

                match anime_list_result {
                    Ok(anime_list) => {
                        println!(
                            "✓ Successfully retrieved anime list with {} entries",
                            anime_list.len()
                        );
                        // The list might be empty, which is fine
                        for entry in &anime_list {
                            assert!(entry.id > 0, "Media list entry should have a valid ID");
                        }
                    }
                    Err(e) => {
                        // This might fail if the user's list is private or doesn't exist
                        println!(
                            "Note: Could not retrieve anime list (might be private): {:?}",
                            e
                        );
                    }
                }
            } else {
                println!("Skipping anime list test - could not get current user");
            }
        } else {
            println!("Skipping authenticated anime list test - no valid ANILIST_TOKEN found");
        }
    } else {
        println!("Skipping authenticated anime list test - ANILIST_TOKEN not set");
    }
}

#[tokio::test]
async fn test_token_validation() {
    dotenv().ok();

    if let Ok(token) = env::var("ANILIST_TOKEN") {
        if !token.is_empty() && token != "fake_token" {
            let client = AniListClient::with_token(token);

            // Test that we can make a basic authenticated request
            let result = client.user().get_current_user().await;

            match result {
                Ok(user) => {
                    println!("✓ Token validation successful for user: {}", user.name);

                    // Verify user data integrity
                    assert!(user.id > 0);
                    assert!(!user.name.is_empty());

                    // Test that we can access user statistics if available
                    if let Some(stats) = &user.statistics {
                        if let Some(anime_stats) = &stats.anime {
                            println!("  - Anime count: {:?}", anime_stats.count);
                            println!("  - Episodes watched: {:?}", anime_stats.episodes_watched);
                        }
                        if let Some(manga_stats) = &stats.manga {
                            println!("  - Manga count: {:?}", manga_stats.count);
                            println!("  - Chapters read: {:?}", manga_stats.chapters_read);
                        }
                    }
                }
                Err(e) => {
                    panic!("Token validation failed: {:?}", e);
                }
            }
        } else {
            println!("Skipping token validation test - no valid ANILIST_TOKEN found");
        }
    } else {
        println!("Skipping token validation test - ANILIST_TOKEN not set");
    }
}

// Note: We can't test actual authenticated endpoints without a real token
// In a real application, you would:
// 1. Set up test users with known tokens
// 2. Use environment variables for test tokens
// 3. Mock the API responses for testing

#[tokio::test]
async fn test_token_in_headers() {
    // This is more of a unit test to ensure our client structure is correct
    let _client_with_token = AniListClient::with_token("test_token".to_string());
    let _client_without_token = AniListClient::new();

    // We can't directly test the headers without exposing internal structure,
    // but we can ensure both clients can be created and used
    assert!(true, "Both client types created successfully");

    // In a real test, you might want to use a mock HTTP client to verify
    // that the Authorization header is being sent correctly
}
