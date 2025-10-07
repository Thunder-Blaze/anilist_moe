use anilist_moe::client::AniListClient;
use tokio::time::{Duration, sleep};

/// Helper function to add rate limiting between test requests
async fn rate_limit() {
    sleep(Duration::from_secs(1)).await;
}

#[tokio::test]
async fn test_comprehensive_integration() {
    let client = AniListClient::new();

    // Test anime endpoints
    println!("Testing anime endpoints...");

    let popular_anime = client
        .anime()
        .get_popular(1, 3)
        .await
        .expect("Failed to get popular anime");
    assert!(!popular_anime.is_empty());
    println!("✓ Popular anime: Found {} entries", popular_anime.len());
    rate_limit().await;

    let trending_anime = client
        .anime()
        .get_trending(1, 3)
        .await
        .expect("Failed to get trending anime");
    assert!(!trending_anime.is_empty());
    println!("✓ Trending anime: Found {} entries", trending_anime.len());
    rate_limit().await;

    // Get a specific anime by ID
    if let Some(first_anime) = popular_anime.first() {
        let anime_by_id = client
            .anime()
            .get_by_id(first_anime.id)
            .await
            .expect("Failed to get anime by ID");
        assert_eq!(anime_by_id.id, first_anime.id);
        println!(
            "✓ Anime by ID: Successfully retrieved anime {}",
            first_anime.id
        );
    }

    // Test manga endpoints
    println!("Testing manga endpoints...");

    let popular_manga = client
        .manga()
        .get_popular(1, 3)
        .await
        .expect("Failed to get popular manga");
    assert!(!popular_manga.is_empty());
    println!("✓ Popular manga: Found {} entries", popular_manga.len());

    let trending_manga = client
        .manga()
        .get_trending(1, 3)
        .await
        .expect("Failed to get trending manga");
    assert!(!trending_manga.is_empty());
    println!("✓ Trending manga: Found {} entries", trending_manga.len());

    // Test character endpoints
    println!("Testing character endpoints...");

    let popular_characters = client
        .character()
        .get_popular(1, 3)
        .await
        .expect("Failed to get popular characters");
    assert!(!popular_characters.is_empty());
    println!(
        "✓ Popular characters: Found {} entries",
        popular_characters.len()
    );

    // Test staff endpoints
    println!("Testing staff endpoints...");

    let popular_staff = client
        .staff()
        .get_popular(1, 3)
        .await
        .expect("Failed to get popular staff");
    assert!(!popular_staff.is_empty());
    println!("✓ Popular staff: Found {} entries", popular_staff.len());

    // Test search functionality
    println!("Testing search functionality...");

    let anime_search = client
        .anime()
        .search("Attack on Titan", 1, 2)
        .await
        .expect("Failed to search anime");
    assert!(!anime_search.is_empty());
    println!(
        "✓ Anime search: Found {} entries for 'Attack on Titan'",
        anime_search.len()
    );

    let character_search = client
        .character()
        .search("Eren", 1, 2)
        .await
        .expect("Failed to search characters");
    assert!(!character_search.is_empty());
    println!(
        "✓ Character search: Found {} entries for 'Eren'",
        character_search.len()
    );

    println!("All integration tests passed! 🎉");
}

#[tokio::test]
async fn test_error_handling() {
    let client = AniListClient::new();

    // Test with invalid ID (should not panic, but might return an error)
    let result = client.anime().get_by_id(-1).await;

    // The API might return an error or empty result for invalid IDs
    // We just want to make sure it doesn't panic
    match result {
        Ok(_) => println!("API returned a result for invalid ID (unexpected but not an error)"),
        Err(e) => println!("API correctly returned an error for invalid ID: {:?}", e),
    }
}

#[tokio::test]
async fn test_pagination() {
    let client = AniListClient::new();

    // Test that pagination works correctly
    let page1 = client
        .anime()
        .get_popular(1, 5)
        .await
        .expect("Failed to get page 1");
    let page2 = client
        .anime()
        .get_popular(2, 5)
        .await
        .expect("Failed to get page 2");

    assert!(!page1.is_empty());
    assert!(!page2.is_empty());

    // Check that pages contain different content
    let page1_ids: Vec<i32> = page1.iter().map(|a| a.id).collect();
    let page2_ids: Vec<i32> = page2.iter().map(|a| a.id).collect();

    let has_different_content = page1_ids.iter().any(|id| !page2_ids.contains(id));
    assert!(
        has_different_content,
        "Pages should contain different content"
    );

    println!("✓ Pagination works correctly");
}
