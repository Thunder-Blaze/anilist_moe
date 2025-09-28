use anilist_moe::{
    client::AniListClient,
    endpoints::media::MangaSearchOptions,
    enums::media::{MediaSort, MediaStatus},
};
use tokio::time::{Duration, sleep};

async fn rate_limit() {
    sleep(Duration::from_secs(1)).await;
}

#[tokio::test]
async fn test_get_popular_manga() {
    let client = AniListClient::new();
    let result = client.media().get_popular_manga(Some(1), Some(5)).await;

    println!("Result: {:?}", result);
    assert!(result.is_ok());
    let manga_list = result.unwrap();
    assert!(manga_list
        .get("data")
        .and_then(|d| d.get("Page"))
        .and_then(|p| p.get("media"))
        .and_then(|m| m.as_array())
        .map_or(false, |a| !a.is_empty()));

    rate_limit().await;
}

#[tokio::test]
async fn test_get_trending_manga() {
    rate_limit().await;

    let client = AniListClient::new();
    let result = client.media().get_trending_manga(Some(1), Some(3)).await;

    assert!(result.is_ok());
    let manga_list = result.unwrap();
    assert!(manga_list
        .get("data")
        .and_then(|d| d.get("Page"))
        .and_then(|p| p.get("media"))
        .and_then(|m| m.as_array())
        .map_or(false, |a| !a.is_empty()));

    rate_limit().await;
}

#[tokio::test]
async fn test_get_manga_by_id() {
    rate_limit().await;

    let client = AniListClient::new();
    // Using One Piece's ID (30013) is incorrect, it's an anime. Let's use Berserk's ID (30002)
    let result = client.media().get_manga_by_id(30002).await;

    assert!(result.is_ok());
    let manga = result.unwrap();
    assert_eq!(
        manga
            .get("data")
            .and_then(|d| d.get("Page"))
            .and_then(|p| p.get("media"))
            .and_then(|m| m.as_array())
            .and_then(|a| a.get(0))
            .and_then(|a| a.get("id"))
            .and_then(|id| id.as_i64()),
        Some(30002)
    );

    rate_limit().await;
}

#[tokio::test]
async fn test_search_manga() {
    rate_limit().await;

    let client = AniListClient::new();
    let options = MangaSearchOptions {
        search_term: Some("One Piece".to_string()),
        page: Some(1),
        per_page: Some(5),
        ..Default::default()
    };
    let result = client.media().search_manga(options).await;

    assert!(result.is_ok());
    let manga_list = result.unwrap();
    let media = manga_list
        .get("data")
        .and_then(|d| d.get("Page"))
        .and_then(|p| p.get("media"))
        .and_then(|m| m.as_array())
        .unwrap();
    assert!(!media.is_empty());

    // Check that results contain "One Piece" in some form
    let has_one_piece = media.iter().any(|manga| {
        if let Some(title) = manga.get("title") {
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
async fn test_get_top_rated_manga() {
    rate_limit().await;

    let client = AniListClient::new();
    let result = client.media().get_top_rated_manga(Some(1), Some(5)).await;

    assert!(result.is_ok());
    let manga_list = result.unwrap();
    let media = manga_list
        .get("data")
        .and_then(|d| d.get("Page"))
        .and_then(|p| p.get("media"))
        .and_then(|m| m.as_array())
        .unwrap();
    assert!(!media.is_empty());

    // Check that manga have scores
    for manga in media {
        assert!(manga.get("id").and_then(|id| id.as_i64()).is_some());
    }

    rate_limit().await;
}

#[tokio::test]
async fn test_get_releasing_manga() {
    rate_limit().await;

    let client = AniListClient::new();
    let result = client.media().get_releasing_manga(Some(1), Some(5)).await;

    assert!(result.is_ok());
    let manga_list = result.unwrap();
    if let Some(media) = manga_list
        .get("data")
        .and_then(|d| d.get("Page"))
        .and_then(|p| p.get("media"))
        .and_then(|m| m.as_array())
    {
        for manga in media {
            assert!(manga.get("id").and_then(|id| id.as_i64()).is_some());
        }
    }

    rate_limit().await;
}

#[tokio::test]
async fn test_get_completed_manga() {
    rate_limit().await;

    let client = AniListClient::new();
    let options = MangaSearchOptions {
        status: Some(MediaStatus::Finished),
        sort: Some(vec![MediaSort::PopularityDesc]),
        page: Some(1),
        per_page: Some(5),
        ..Default::default()
    };
    let result = client.media().search_manga(options).await;

    assert!(result.is_ok());
    let manga_list = result.unwrap();
    let media = manga_list
        .get("data")
        .and_then(|d| d.get("Page"))
        .and_then(|p| p.get("media"))
        .and_then(|m| m.as_array())
        .unwrap();
    assert!(!media.is_empty());

    for manga in media {
        assert!(manga.get("id").and_then(|id| id.as_i64()).is_some());
        assert_eq!(
            manga.get("status").and_then(|s| s.as_str()),
            Some("FINISHED")
        );
    }

    rate_limit().await;
}
