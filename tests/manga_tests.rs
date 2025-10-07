use anilist_moe::client::AniListClient;
use tokio::time::{Duration, sleep};

/// Helper function to add rate limiting between test requests
async fn rate_limit() {
    sleep(Duration::from_secs(1)).await;
}

#[tokio::test]
async fn test_get_popular_manga() {
    let client = AniListClient::new();
    let result = client.manga().get_popular(1, 5).await;

    assert!(result.is_ok());
    let manga_list = result.unwrap();
    assert!(!manga_list.is_empty());
    assert!(manga_list.len() <= 5);

    // Check that all manga have required fields
    for manga in &manga_list {
        assert!(manga.id > 0);
        assert!(manga.title.is_some());
    }

    rate_limit().await;
}

#[tokio::test]
async fn test_get_trending_manga() {
    rate_limit().await;

    let client = AniListClient::new();
    let result = client.manga().get_trending(1, 3).await;

    assert!(result.is_ok());
    let manga_list = result.unwrap();
    assert!(!manga_list.is_empty());
    assert!(manga_list.len() <= 3);

    rate_limit().await;
}

#[tokio::test]
async fn test_get_manga_by_id() {
    rate_limit().await;

    let client = AniListClient::new();
    // Using One Piece's ID (30013)
    let result = client.manga().get_by_id(30013).await;

    assert!(result.is_ok());
    let manga = result.unwrap();
    assert_eq!(manga.id, 30013);
    assert!(manga.title.is_some());

    rate_limit().await;
}

#[tokio::test]
async fn test_search_manga() {
    rate_limit().await;

    let client = AniListClient::new();
    let result = client.manga().search("One Piece", 1, 5).await;

    assert!(result.is_ok());
    let manga_list = result.unwrap();
    assert!(!manga_list.is_empty());

    // Check that results contain "One Piece" in some form
    let has_one_piece = manga_list.iter().any(|manga| {
        if let Some(title) = &manga.title {
            title
                .romaji
                .as_ref()
                .map_or(false, |t| t.to_lowercase().contains("one piece"))
                || title
                    .english
                    .as_ref()
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
    let result = client.manga().get_top_rated(1, 5).await;

    assert!(result.is_ok());
    let manga_list = result.unwrap();
    assert!(!manga_list.is_empty());

    // Check that manga have scores
    for manga in &manga_list {
        assert!(manga.id > 0);
        // Note: Not all manga may have scores, so we don't assert on that
    }

    rate_limit().await;
}

#[tokio::test]
async fn test_get_releasing_manga() {
    rate_limit().await;

    let client = AniListClient::new();
    let result = client.manga().get_releasing(1, 5).await;

    assert!(result.is_ok());
    let manga_list = result.unwrap();
    // Note: This might be empty if no manga are currently releasing

    for manga in &manga_list {
        assert!(manga.id > 0);
    }

    rate_limit().await;
}

#[tokio::test]
async fn test_get_completed_manga() {
    rate_limit().await;

    let client = AniListClient::new();
    let result = client.manga().get_completed(1, 5).await;

    assert!(result.is_ok());
    let manga_list = result.unwrap();
    assert!(!manga_list.is_empty());

    for manga in &manga_list {
        assert!(manga.id > 0);
        assert!(manga.title.is_some());
    }

    rate_limit().await;
}
