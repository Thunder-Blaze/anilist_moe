//! Tests for Media endpoint
//!
//! These tests verify that the media endpoint correctly fetches, parses,
//! and returns data with proper types from the AniList API.

use crate::test_harness::{TestHarness, delay_between_tests};
use anilist_moe::enums::media::{MediaSort, MediaType};
use anilist_moe::{AniListError, endpoints::media::*};

/// Helper to create a test harness
fn harness() -> TestHarness {
    TestHarness::new()
}

#[tokio::test]
async fn test_fetch_media_with_search() {
    let h = harness();
    let client = h.client();

    let result = h
        .run(|| async {
            let options = FetchMediaOptions {
                search: Some("Naruto".to_string()),
                media_type: Some(MediaType::Anime),
                per_page: Some(5),
                ..Default::default()
            };
            client.media().fetch(&options).await
        })
        .await;

    assert!(
        result.is_ok(),
        "Should successfully fetch media: {:?}",
        result.err()
    );

    let response = result.unwrap();
    assert!(
        response
            .page_info
            .as_ref()
            .map(|p| p.current_page)
            .is_some(),
        "Should have page info"
    );

    let media_list = &response.data;
    assert!(
        !media_list.is_empty(),
        "Should return at least one media result"
    );
    assert!(media_list.len() <= 5, "Should respect perPage limit");

    let first_media = &media_list[0];
    assert!(
        first_media.id.unwrap_or(0) > 0,
        "Media should have a positive ID"
    );
    assert!(first_media.title.is_some(), "Media should have a title");

    // Verify search relevance
    if let Some(ref title) = first_media.title {
        let has_naruto = title
            .romaji
            .as_ref()
            .map(|s| s.to_lowercase().contains("naruto"))
            .unwrap_or(false)
            || title
                .english
                .as_ref()
                .map(|s| s.to_lowercase().contains("naruto"))
                .unwrap_or(false);
        assert!(has_naruto, "Search result should be relevant to 'Naruto'");
    }
}

#[tokio::test]
async fn test_fetch_media_by_id() {
    delay_between_tests().await;
    let h = harness();
    let client = h.client();

    let result = h
        .run(|| async {
            let options = FetchMediaOptions {
                id: Some(1), // Cowboy Bebop
                ..Default::default()
            };
            client.media().fetch(&options).await
        })
        .await;

    assert!(
        result.is_ok(),
        "Should successfully fetch media by ID: {:?}",
        result.err()
    );

    let response = result.unwrap();
    let media_list = &response.data;
    assert_eq!(media_list.len(), 1, "Should return exactly one media");

    let media = &media_list[0];
    assert_eq!(media.id, Some(1), "Should return correct media ID");
    assert!(media.title.is_some(), "Media should have a title");

    // Verify it's Cowboy Bebop
    if let Some(ref title) = media.title {
        assert!(
            title
                .romaji
                .as_ref()
                .map(|s| s.contains("Cowboy"))
                .unwrap_or(false),
            "ID 1 should be Cowboy Bebop"
        );
    }
}

#[tokio::test]
async fn test_fetch_one_media() {
    delay_between_tests().await;
    let h = harness();
    let client = h.client();

    let result = h
        .run(|| async {
            let options = FetchMediaOneOptions {
                id: Some(1),
                ..Default::default()
            };
            client.media().fetch_one(&options).await
        })
        .await;

    assert!(
        result.is_ok(),
        "Should successfully fetch one media: {:?}",
        result.err()
    );

    let media = result.unwrap();
    assert_eq!(media.id, Some(1), "Should return media with ID 1");
    assert!(media.title.is_some(), "Media should have a title");
}

#[tokio::test]
async fn test_media_data_types() {
    delay_between_tests().await;
    let h = harness();
    let client = h.client();

    let result = h
        .run(|| async {
            let options = FetchMediaOptions {
                id: Some(1),
                ..Default::default()
            };
            client.media().fetch(&options).await
        })
        .await;

    assert!(result.is_ok(), "Should successfully fetch media");

    let response = result.unwrap();
    let media = &response.data[0];

    // Verify ID is required and positive
    assert!(media.id.unwrap_or(0) > 0, "ID should be positive");

    // Verify optional numeric fields have valid ranges
    if let Some(popularity) = media.popularity {
        assert!(popularity >= 0, "Popularity should be non-negative");
    }

    if let Some(episodes) = media.episodes {
        assert!(episodes > 0, "Episodes should be positive if present");
    }

    if let Some(average_score) = media.average_score {
        assert!(
            (0..=100).contains(&average_score),
            "Average score should be 0-100"
        );
    }

    // Verify title structure
    if let Some(ref title) = media.title {
        assert!(
            title.romaji.is_some() || title.english.is_some() || title.native.is_some(),
            "Title should have at least one language variant"
        );
    }

    // Verify media type if present
    assert!(media.media_type.is_some(), "Media type should be present");
}

#[tokio::test]
async fn test_fetch_media_pagination() {
    delay_between_tests().await;
    let h = harness();
    let client = h.client();

    // Fetch page 1
    let result1 = h
        .run(|| async {
            let options = FetchMediaOptions {
                media_type: Some(MediaType::Anime),
                per_page: Some(5),
                page: Some(1),
                sort: Some(vec![MediaSort::IdDesc]),
                ..Default::default()
            };
            client.media().fetch(&options).await
        })
        .await;

    assert!(result1.is_ok(), "Should successfully fetch page 1");

    delay_between_tests().await;

    // Fetch page 2
    let result2 = h
        .run(|| async {
            let options = FetchMediaOptions {
                media_type: Some(MediaType::Anime),
                per_page: Some(5),
                page: Some(2),
                sort: Some(vec![MediaSort::IdDesc]),
                ..Default::default()
            };
            client.media().fetch(&options).await
        })
        .await;

    assert!(result2.is_ok(), "Should successfully fetch page 2");

    let response1 = result1.unwrap();
    let response2 = result2.unwrap();

    let media_list1 = &response1.data;
    let media_list2 = &response2.data;

    assert_eq!(media_list1.len(), 5, "Page 1 should have 5 results");
    assert_eq!(media_list2.len(), 5, "Page 2 should have 5 results");

    // Verify page info
    if let Some(ref page_info) = response1.page_info {
        assert_eq!(
            page_info.current_page,
            Some(1),
            "Page 1 should report current page as 1"
        );
        assert!(
            page_info.has_next_page == Some(true),
            "Should have next page"
        );
    }

    if let Some(ref page_info) = response2.page_info {
        assert_eq!(
            page_info.current_page,
            Some(2),
            "Page 2 should report current page as 2"
        );
    }

    // Verify different pages have different results
    let ids1: Vec<Option<i32>> = media_list1.iter().map(|m| m.id).collect();
    let ids2: Vec<Option<i32>> = media_list2.iter().map(|m| m.id).collect();
    assert_ne!(ids1, ids2, "Different pages should have different results");

    // Verify IDs are in descending order within each page
    for window in ids1.windows(2) {
        if let (Some(a), Some(b)) = (window[0], window[1]) {
            assert!(a > b, "IDs should be in descending order");
        }
    }
}

#[tokio::test]
async fn test_get_trending_anime() {
    delay_between_tests().await;
    let h = harness();
    let client = h.client();

    let result = h
        .run(|| async { client.media().get_trending_anime(Some(1), Some(10)).await })
        .await;

    assert!(result.is_ok(), "Should successfully fetch trending anime");

    let response = result.unwrap();
    assert!(!response.data.is_empty(), "Should return trending anime");
    assert!(response.data.len() <= 10, "Should respect per_page limit");
}

#[tokio::test]
async fn test_get_popular_anime() {
    delay_between_tests().await;
    let h = harness();
    let client = h.client();

    let result = h
        .run(|| async { client.media().get_popular_anime(Some(1), Some(10)).await })
        .await;

    assert!(result.is_ok(), "Should successfully fetch popular anime");

    let response = result.unwrap();
    assert!(!response.data.is_empty(), "Should return popular anime");

    // Verify results are sorted by popularity (descending)
    let popularities: Vec<i32> = response.data.iter().filter_map(|m| m.popularity).collect();

    for window in popularities.windows(2) {
        assert!(
            window[0] >= window[1],
            "Results should be sorted by popularity descending"
        );
    }
}

#[tokio::test]
async fn test_search_anime() {
    delay_between_tests().await;
    let h = harness();
    let client = h.client();

    let result = h
        .run(|| async {
            client
                .media()
                .search_anime("Attack on Titan", Some(1), Some(5))
                .await
        })
        .await;

    assert!(result.is_ok(), "Should successfully search anime");

    let response = result.unwrap();
    assert!(!response.data.is_empty(), "Should return search results");

    // First result should be relevant
    let first = &response.data[0];
    if let Some(ref title) = first.title {
        let is_relevant = title
            .romaji
            .as_ref()
            .map(|s| s.to_lowercase().contains("attack") || s.to_lowercase().contains("titan"))
            .unwrap_or(false)
            || title
                .english
                .as_ref()
                .map(|s| s.to_lowercase().contains("attack") || s.to_lowercase().contains("titan"))
                .unwrap_or(false);
        assert!(is_relevant, "First search result should be relevant");
    }
}

#[tokio::test]
async fn test_get_anime_by_id() {
    delay_between_tests().await;
    let h = harness();
    let client = h.client();

    // ID 16498 is Shingeki no Kyojin (Attack on Titan)
    let result = h
        .run(|| async { client.media().get_anime_by_id(16498).await })
        .await;

    assert!(result.is_ok(), "Should successfully fetch anime by ID");

    let anime = result.unwrap();
    assert_eq!(anime.id, Some(16498), "Should return correct anime");
    assert!(anime.title.is_some(), "Should have title");
    assert!(anime.average_score.is_some(), "Should have average score");
}

#[tokio::test]
async fn test_nonexistent_media() {
    delay_between_tests().await;
    let h = harness();
    let client = h.client();

    let result = h
        .run(|| async {
            let options = FetchMediaOptions {
                id: Some(999999999), // Very unlikely to exist
                ..Default::default()
            };
            client.media().fetch(&options).await
        })
        .await;

    // Should either return empty results or an error
    match result {
        Ok(response) => {
            assert!(
                response.data.is_empty(),
                "Should return empty results for nonexistent ID"
            );
        }
        Err(e) => {
            // GraphQL error is also acceptable
            assert!(
                matches!(e, AniListError::GraphQL { .. } | AniListError::NotFound),
                "Should return appropriate error for nonexistent ID"
            );
        }
    }
}
