use anilist_moe::client::AniListClient;
use anilist_moe::error::AniListError;
use anilist_moe::utils::{retry_with_backoff, RetryConfig, rate_limit_delay};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== AniList API Wrapper Demo ===");
    println!("This demo showcases proper error handling and rate limiting.\n");

    // Example of using unauthenticated client with error handling
    println!("🔓 Unauthenticated Client Example");
    println!("=================================");
    let client = AniListClient::new();
    
    // Demonstrate proper error handling
    match client.anime().get_popular(1, 3).await {
        Ok(popular_anime) => {
            println!("✅ Popular anime (first 3):");
            for anime in popular_anime {
                if let Some(title) = &anime.title {
                    println!("  - {} (ID: {})", 
                        title.user_preferred.as_ref()
                            .or(title.english.as_ref())
                            .or(title.romaji.as_ref())
                            .unwrap_or(&"Unknown".to_string()), 
                        anime.id
                    );
                }
            }
        }
        Err(e) => {
            println!("❌ Failed to get popular anime: {}", e);
            handle_api_error(&e);
        }
    }

    // Add delay to respect rate limits
    rate_limit_delay(1000).await;

    // Example with retry logic
    println!("\n🔄 Example with Retry Logic");
    println!("===========================");
    
    let retry_config = RetryConfig {
        max_retries: 3,
        base_delay_ms: 1000,
        exponential_backoff: true,
        max_delay_ms: 10000,
    };

    let search_result = retry_with_backoff(
        || async {
            client.anime().search("Attack on Titan", 1, 2).await
                .map_err(|e| e)
                .map(|results| {
                    if results.is_empty() {
                        Err(AniListError::GraphQL { 
                            message: "No search results found".to_string() 
                        })
                    } else {
                        Ok(results)
                    }
                })
                .unwrap_or_else(Err)
        },
        retry_config,
    ).await;

    match search_result {
        Ok(search_results) => {
            println!("✅ Search results for 'Attack on Titan':");
            for anime in search_results {
                if let Some(title) = &anime.title {
                    println!("  - {} (ID: {})", 
                        title.user_preferred.as_ref()
                            .or(title.english.as_ref())
                            .or(title.romaji.as_ref())
                            .unwrap_or(&"Unknown".to_string()), 
                        anime.id
                    );
                }
            }
        }
        Err(e) => {
            println!("❌ Search failed after retries: {}", e);
            handle_api_error(&e);
        }
    }

    rate_limit_delay(1000).await;

    // Example of using authenticated client
    println!("\n🔐 Authenticated Client Example");
    println!("================================");
    
    // Check if we have a token
    if let Ok(token) = std::env::var("ANILIST_TOKEN") {
        let authenticated_client = AniListClient::with_token(token);
        
        match authenticated_client.user().get_current_user().await {
            Ok(user) => {
                println!("✅ Current user: {}", user.name);
                if let Some(stats) = user.statistics {
                    if let Some(anime_stats) = stats.anime {
                        println!("   📺 Anime watched: {} episodes", 
                            anime_stats.episodes_watched.unwrap_or(0));
                    }
                    if let Some(manga_stats) = stats.manga {
                        println!("   📚 Manga read: {} chapters", 
                            manga_stats.chapters_read.unwrap_or(0));
                    }
                }
            }
            Err(e) => {
                println!("❌ Failed to get current user: {}", e);
                handle_api_error(&e);
            }
        }
    } else {
        println!("⚠️  No ANILIST_TOKEN environment variable found.");
        println!("   To test authenticated features:");
        println!("   1. Get an access token from https://anilist.co/settings/developer");
        println!("   2. Set it as an environment variable: export ANILIST_TOKEN='your_token'");
        println!("   3. Run this demo again");
    }

    rate_limit_delay(1000).await;

    // Demonstrate other client capabilities
    println!("\n🎭 Character Example");
    println!("====================");
    match client.character().get_popular(1, 3).await {
        Ok(popular_characters) => {
            println!("✅ Popular characters:");
            for character in popular_characters {
                if let Some(name) = &character.name {
                    println!("  - {} (ID: {})", 
                        name.user_preferred.as_ref()
                            .or(name.full.as_ref())
                            .unwrap_or(&"Unknown".to_string()), 
                        character.id
                    );
                }
            }
        }
        Err(e) => {
            println!("❌ Failed to get popular characters: {}", e);
            handle_api_error(&e);
        }
    }

    println!("\n🎊 Demo completed!");
    println!("This demo showed:");
    println!("  ✅ Basic API calls with error handling");
    println!("  ✅ Rate limiting respect");
    println!("  ✅ Retry logic for failed requests");
    println!("  ✅ Authenticated vs unauthenticated endpoints");
    println!("  ✅ Proper error message interpretation");

    Ok(())
}

fn handle_api_error(error: &AniListError) {
    match error {
        AniListError::RateLimit { limit, remaining, reset_at, retry_after } => {
            println!("   ⏰ Rate limit details:");
            println!("      - Limit: {} requests/minute", limit);
            println!("      - Remaining: {}", remaining);
            println!("      - Reset at: {} (unix timestamp)", reset_at);
            println!("      - Retry after: {} seconds", retry_after);
            println!("   💡 Tip: Add delays between requests to avoid this");
        }
        AniListError::RateLimitSimple => {
            println!("   ⏰ Rate limit exceeded");
            println!("   💡 Tip: Slow down your requests and try again in a minute");
        }
        AniListError::BurstLimit => {
            println!("   💨 Burst limit exceeded - requests sent too quickly");
            println!("   💡 Tip: Add meaningful delays between requests");
        }
        AniListError::AuthenticationRequired => {
            println!("   🔐 This endpoint requires authentication");
            println!("   💡 Tip: Set ANILIST_TOKEN environment variable");
        }
        AniListError::AccessDenied => {
            println!("   🚫 Access denied - check your token permissions");
            println!("   💡 Tip: Verify your token is valid and has required scope");
        }
        AniListError::NotFound => {
            println!("   🔍 Resource not found");
            println!("   💡 Tip: Check if the ID or search query is valid");
        }
        AniListError::BadRequest { message } => {
            println!("   ❌ Bad request: {}", message);
            println!("   💡 Tip: Check your query parameters");
        }
        AniListError::ServerError { status, message } => {
            println!("   🖥️  Server error ({}): {}", status, message);
            println!("   💡 Tip: Try again later, this is usually temporary");
        }
        AniListError::GraphQL { message } => {
            println!("   📊 GraphQL error: {}", message);
            println!("   💡 Tip: Check your query syntax and variables");
        }
        AniListError::Network(e) => {
            println!("   🌐 Network error: {}", e);
            println!("   💡 Tip: Check your internet connection");
        }
        AniListError::Json(e) => {
            println!("   📄 JSON parsing error: {}", e);
            println!("   💡 Tip: This might indicate an API response format change");
        }
    }
}
