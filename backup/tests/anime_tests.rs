use anilist_moe::{AniListClient, enums::media::MediaSeason, endpoints::media::AnimeSearchOptions};
use serde_json::Value;
use tokio::time::{Duration, sleep};

async fn rate_limit() {
    sleep(Duration::from_secs(1)).await;
}

#[tokio::test]
async fn test_get_popular_anime() {
    let client = AniListClient::new();
    let result = client.anime().get_popular(1, 5).await;

    assert!(result.is_ok());
    let anime_list = result.unwrap();
    assert!(anime_list.get("data").and_then(|d| d.get("Page")).and_then(|p| p.get("media")).and_then(|m| m.as_array()).map_or(false, |a| !a.is_empty()));

    rate_limit().await;
}

#[tokio::test]
async fn test_get_trending_anime() {
    rate_limit().await;

    let client = AniListClient::new();
    let result = client.anime().get_trending(1, 3).await;

    assert!(result.is_ok());
    let anime_list = result.unwrap();
    assert!(anime_list.get("data").and_then(|d| d.get("Page")).and_then(|p| p.get("media")).and_then(|m| m.as_array()).map_or(false, |a| !a.is_empty()));

    rate_limit().await;
}

#[tokio::test]
async fn test_get_anime_by_id() {
    rate_limit().await;

    let client = AniListClient::new();
    let result = client.anime().get_by_id(16498).await;

    assert!(result.is_ok());
    let anime = result.unwrap();
    assert_eq!(anime.get("data").and_then(|d| d.get("Page")).and_then(|p| p.get("media")).and_then(|m| m.as_array()).and_then(|a| a.get(0)).and_then(|a| a.get("id")).and_then(|id| id.as_i64()), Some(16498));

    rate_limit().await;
}

#[tokio::test]
async fn test_search_anime() {
    rate_limit().await;

    let client = AniListClient::new();
    let options = AnimeSearchOptions {
        search_term: Some("Naruto".to_string()),
        page: Some(1),
        per_page: Some(5),
        ..Default::default()
    };
    let result = client.anime().search_with_options(options).await;

    assert!(result.is_ok());
    let anime_list = result.unwrap();
    let media = anime_list.get("data").and_then(|d| d.get("Page")).and_then(|p| p.get("media")).and_then(|m| m.as_array()).unwrap();
    assert!(!media.is_empty());

    // Check that results contain "Naruto" in some form
    let has_naruto = media.iter().any(|anime| {
        if let Some(title) = anime.get("title") {
            title.get("romaji").and_then(|r| r.as_str()).map_or(false, |t| t.to_lowercase().contains("naruto")) ||
            title.get("english").and_then(|e| e.as_str()).map_or(false, |t| t.to_lowercase().contains("naruto"))
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
    let media = anime_list.get("data").and_then(|d| d.get("Page")).and_then(|p| p.get("media")).and_then(|m| m.as_array()).unwrap();
    assert!(!media.is_empty());

    // Check that anime have the correct season and year
    for anime in media {
        if let Some(season_year) = anime.get("seasonYear").and_then(|y| y.as_i64()) {
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
    let media = anime_list.get("data").and_then(|d| d.get("Page")).and_then(|p| p.get("media")).and_then(|m| m.as_array()).unwrap();
    assert!(!media.is_empty());

    // Check that anime have scores and are sorted by score
    let mut prev_score = 101;
    for anime in media {
        if let Some(score) = anime.get("averageScore").and_then(|s| s.as_i64()) {
            assert!(score <= prev_score as i64);
            prev_score = score as i32;
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
    let media = anime_list.get("data").and_then(|d| d.get("Page")).and_then(|p| p.get("media")).and_then(|m| m.as_array());
    // Note: This might be empty if no anime are currently airing, so we don't assert !is_empty()

    if let Some(media) = media {
        for anime in media {
            assert!(anime.get("id").and_then(|id| id.as_i64()).is_some());
            // Airing anime should have status RELEASING
            assert_eq!(anime.get("status").and_then(|s| s.as_str()), Some("RELEASING"));
        }
    }

    rate_limit().await;
}
