use anilist_moe::client::AniListClient;
use tokio::time::{Duration, sleep};

/// Helper function to add rate limiting between test requests
async fn rate_limit() {
    sleep(Duration::from_secs(1)).await;
}

#[tokio::test]
async fn test_get_popular_anime() {
    let client = AniListClient::new();
    let result = client.anime().get_popular(1, 5).await;

    assert!(result.is_ok());
    let anime_list = result.unwrap();
    assert!(!anime_list.is_empty());
    assert!(anime_list.len() <= 5);

    // Check that all anime have required fields
    for anime in &anime_list {
        assert!(anime.id > 0);
        assert!(anime.title.is_some());
    }

    rate_limit().await;
}

#[tokio::test]
async fn test_get_trending_anime() {
    rate_limit().await;

    let client = AniListClient::new();
    let result = client.anime().get_trending(1, 3).await;

    assert!(result.is_ok());
    let anime_list = result.unwrap();
    assert!(!anime_list.is_empty());
    assert!(anime_list.len() <= 3);

    rate_limit().await;
}

#[tokio::test]
async fn test_get_anime_by_id() {
    rate_limit().await;

    let client = AniListClient::new();
    // Using Attack on Titan's ID (16498)
    let result = client.anime().get_by_id(16498).await;

    assert!(result.is_ok());
    let anime = result.unwrap();
    assert_eq!(anime.id, 16498);
    assert!(anime.title.is_some());

    rate_limit().await;
}

#[tokio::test]
async fn test_search_anime() {
    rate_limit().await;

    let client = AniListClient::new();
    let result = client.anime().search("Naruto", 1, 5).await;

    assert!(result.is_ok());
    let anime_list = result.unwrap();
    assert!(!anime_list.is_empty());

    // Check that results contain "Naruto" in some form
    let has_naruto = anime_list.iter().any(|anime| {
        if let Some(title) = &anime.title {
            title
                .romaji
                .as_ref()
                .map_or(false, |t| t.to_lowercase().contains("naruto"))
                || title
                    .english
                    .as_ref()
                    .map_or(false, |t| t.to_lowercase().contains("naruto"))
        } else {
            false
        }
    });
    assert!(has_naruto);

    rate_limit().await;
}

#[tokio::test]
async fn test_get_anime_by_season() {
    rate_limit().await;

    let client = AniListClient::new();
    let result = client.anime().get_by_season("FALL", 2023, 1, 5).await;

    assert!(result.is_ok());
    let anime_list = result.unwrap();
    assert!(!anime_list.is_empty());

    // Check that anime have the correct season and year
    for anime in &anime_list {
        if let Some(season_year) = anime.season_year {
            assert_eq!(season_year, 2023);
        }
    }

    rate_limit().await;
}

#[tokio::test]
async fn test_get_top_rated_anime() {
    rate_limit().await;

    let client = AniListClient::new();
    let result = client.anime().get_top_rated(1, 5).await;

    assert!(result.is_ok());
    let anime_list = result.unwrap();
    assert!(!anime_list.is_empty());

    // Check that anime have scores and are sorted by score
    let mut prev_score = 100;
    for anime in &anime_list {
        if let Some(score) = anime.average_score {
            assert!(score <= prev_score);
            prev_score = score;
        }
    }

    rate_limit().await;
}

#[tokio::test]
async fn test_get_airing_anime() {
    rate_limit().await;

    let client = AniListClient::new();
    let result = client.anime().get_airing(1, 5).await;

    assert!(result.is_ok());
    let anime_list = result.unwrap();
    // Note: This might be empty if no anime are currently airing

    for anime in &anime_list {
        assert!(anime.id > 0);
        // Airing anime should have status RELEASING (though this might not always be set)
    }

    rate_limit().await;
}
