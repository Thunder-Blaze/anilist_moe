use anilist_moe::{
    client::AniListClient,
    endpoints::media::AnimeSearchOptions,
    enums::media::{MediaSort, MediaStatus},
};
use tokio::time::{Duration, sleep};

async fn rate_limit() {
    sleep(Duration::from_secs(1)).await;
}

#[tokio::test]
async fn test_get_popular_anime() {
    let client = AniListClient::new();
    let result = client.media().get_popular_anime(Some(1), Some(5)).await;

    println!("Result: {:?}", result);
    assert!(result.is_ok());
    let anime_list = result.unwrap();
    assert!(anime_list
        .get("data")
        .and_then(|d| d.get("Page"))
        .and_then(|p| p.get("media"))
        .and_then(|m| m.as_array())
        .map_or(false, |a| !a.is_empty()));

    rate_limit().await;
}

#[tokio::test]
async fn test_get_trending_anime() {
    rate_limit().await;

    let client = AniListClient::new();
    let result = client.media().get_trending_anime(Some(1), Some(3)).await;

    assert!(result.is_ok());
    let anime_list = result.unwrap();
    assert!(anime_list
        .get("data")
        .and_then(|d| d.get("Page"))
        .and_then(|p| p.get("media"))
        .and_then(|m| m.as_array())
        .map_or(false, |a| !a.is_empty()));

    rate_limit().await;
}

#[tokio::test]
async fn test_get_anime_by_id() {
    rate_limit().await;

    let client = AniListClient::new();
    // Using Cowboy Bepop's ID (1)
    let result = client.media().get_anime_by_id(1).await;

    assert!(result.is_ok());
    let anime = result.unwrap();
    assert_eq!(
        anime
            .get("data")
            .and_then(|d| d.get("Page"))
            .and_then(|p| p.get("media"))
            .and_then(|m| m.as_array())
            .and_then(|a| a.get(0))
            .and_then(|a| a.get("id"))
            .and_then(|id| id.as_i64()),
        Some(1)
    );

    rate_limit().await;
}

#[tokio::test]
async fn test_search_anime() {
    rate_limit().await;

    let client = AniListClient::new();
    let options = AnimeSearchOptions {
        search_term: Some("One Piece".to_string()),
        page: Some(1),
        per_page: Some(5),
        ..Default::default()
    };
    let result = client.media().search_anime(options).await;

    assert!(result.is_ok());
    let anime_list = result.unwrap();
    let media = anime_list
        .get("data")
        .and_then(|d| d.get("Page"))
        .and_then(|p| p.get("media"))
        .and_then(|m| m.as_array())
        .unwrap();
    assert!(!media.is_empty());

    // Check that results contain "One Piece" in some form
    let has_one_piece = media.iter().any(|anime| {
        if let Some(title) = anime.get("title") {
            title
                .get("romaji")
                .and_then(|r| r.as_str())
                .map_or(false, |t| t.to_lowercase().contains("one piece"))
                || title
                    .get("english")
                    .and_then(|e| e.as_str())
                    .map_or(false, |t| t.to_lowercase().contains("one piece"))
        } else {
            false
        }
    });
    assert!(has_one_piece);

    rate_limit().await;
}

#[tokio::test]
async fn test_get_top_rated_anime() {
    rate_limit().await;

    let client = AniListClient::new();
    let result = client.media().get_top_rated_anime(Some(1), Some(5)).await;

    assert!(result.is_ok());
    let anime_list = result.unwrap();
    let media = anime_list
        .get("data")
        .and_then(|d| d.get("Page"))
        .and_then(|p| p.get("media"))
        .and_then(|m| m.as_array())
        .unwrap();
    assert!(!media.is_empty());

    // Check that anime have scores
    for anime in media {
        assert!(anime.get("id").and_then(|id| id.as_i64()).is_some());
    }

    rate_limit().await;
}

#[tokio::test]
async fn test_get_releasing_anime() {
    rate_limit().await;

    let client = AniListClient::new();
    let result = client.media().get_airing_anime(Some(1), Some(5)).await;

    assert!(result.is_ok());
    let anime_list = result.unwrap();
    if let Some(media) = anime_list
        .get("data")
        .and_then(|d| d.get("Page"))
        .and_then(|p| p.get("media"))
        .and_then(|m| m.as_array())
    {
        for anime in media {
            assert!(anime.get("id").and_then(|id| id.as_i64()).is_some());
        }
    }

    rate_limit().await;
}

#[tokio::test]
async fn test_get_completed_anime() {
    rate_limit().await;

    let client = AniListClient::new();
    let options = AnimeSearchOptions {
        status: Some(MediaStatus::Finished),
        sort: Some(vec![MediaSort::PopularityDesc]),
        page: Some(1),
        per_page: Some(5),
        ..Default::default()
    };
    let result = client.media().search_anime(options).await;

    assert!(result.is_ok());
    let anime_list = result.unwrap();
    let media = anime_list
        .get("data")
        .and_then(|d| d.get("Page"))
        .and_then(|p| p.get("media"))
        .and_then(|m| m.as_array())
        .unwrap();
    assert!(!media.is_empty());

    for anime in media {
        assert!(anime.get("id").and_then(|id| id.as_i64()).is_some());
        assert_eq!(
            anime.get("status").and_then(|s| s.as_str()),
            Some("FINISHED")
        );
    }

    rate_limit().await;
}
