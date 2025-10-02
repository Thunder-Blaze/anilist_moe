//! Tests for Media endpoint
//!
//! These tests verify that the media endpoint correctly fetches, parses,
//! and returns data with proper types from the AniList API.
use log::info;

use anilist_moe::enums::media::{MediaSort, MediaType};
use anilist_moe::{AniListClient, endpoints::media::*};

#[tokio::test]
async fn test_fetch_media_with_search() {
    let client = AniListClient::new();
    let options = FetchMediaOptions {
        search: Some("Naruto".to_string()),
        media_type: Some(MediaType::Anime),
        per_page: Some(5),
        ..Default::default()
    };

    let result = client.media().fetch(options).await;

    assert!(result.is_ok(), "Should successfully fetch media");

    let response = result.unwrap();
    info!("Response: {:?}", response);
    assert!(response.data.page.page_info.current_page.is_some());

    let media_list = &response.data.page.data.media;
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
}

#[tokio::test]
async fn test_fetch_media_by_id() {
    let client = AniListClient::new();
    let options = FetchMediaOptions {
        id: Some(1), // Cowboy Bebop
        ..Default::default()
    };

    let result = client.media().fetch(options).await;
    if let Err(ref e) = result {
        eprintln!("Error fetching media: {:?}", e);
    }
    assert!(
        result.is_ok(),
        "Should successfully fetch media by ID: {:?}",
        result.err()
    );

    let response = result.unwrap();
    info!("Response: {:?}", response);
    let media_list = &response.data.page.data.media;
    assert_eq!(media_list.len(), 1, "Should return exactly one media");

    let media = &media_list[0];
    assert_eq!(media.id, Some(1), "Should return correct media ID");
    assert!(media.title.is_some(), "Media should have a title");
}

#[tokio::test]
async fn test_fetch_one_media() {
    let client = AniListClient::new();
    let options = FetchMediaOneOptions {
        id: Some(1),
        ..Default::default()
    };

    let result = client.media().fetch_one(options).await;
    if let Err(ref e) = result {
        eprintln!("Error fetching one media: {:?}", e);
    }
    assert!(
        result.is_ok(),
        "Should successfully fetch one media: {:?}",
        result.err()
    );

    let response = result.unwrap();
    info!("Response: {:?}", response);
    let media = &response.data.media;
    assert_eq!(media.id, Some(1), "Should return media with ID 1");
    assert!(media.title.is_some(), "Media should have a title");
}

#[tokio::test]
async fn test_media_data_types() {
    let client = AniListClient::new();
    let options = FetchMediaOptions {
        id: Some(1),
        ..Default::default()
    };

    let result = client.media().fetch(options).await;
    assert!(result.is_ok(), "Should successfully fetch media");

    let response = result.unwrap();
    info!("Response: {:?}", response);
    let media = &response.data.page.data.media[0];

    // Verify ID is required and positive
    assert!(media.id.unwrap_or(0) > 0, "ID should be positive");

    // Verify optional numeric fields
    if let Some(popularity) = media.popularity {
        assert!(popularity >= 0, "Popularity should be non-negative");
    }

    if let Some(episodes) = media.episodes {
        assert!(episodes > 0, "Episodes should be positive if present");
    }

    // Verify title structure
    if let Some(ref title) = media.title {
        assert!(
            title.romaji.is_some() || title.english.is_some() || title.native.is_some(),
            "Title should have at least one language variant"
        );
    }
}

#[tokio::test]
async fn test_fetch_media_pagination() {
    let client = AniListClient::new();

    let options_page1 = FetchMediaOptions {
        media_type: Some(MediaType::Anime),
        per_page: Some(5),
        page: Some(1),
        sort: Some(vec![MediaSort::IdDesc]),
        ..Default::default()
    };

    let result1 = client.media().fetch(options_page1).await;
    assert!(result1.is_ok(), "Should successfully fetch page 1");

    let options_page2 = FetchMediaOptions {
        media_type: Some(MediaType::Anime),
        per_page: Some(5),
        page: Some(2),
        sort: Some(vec![MediaSort::IdDesc]),
        ..Default::default()
    };

    let result2 = client.media().fetch(options_page2).await;
    assert!(result2.is_ok(), "Should successfully fetch page 2");

    let response1 = result1.unwrap();
    let response2 = result2.unwrap();

    let media_list1 = &response1.data.page.data.media;
    let media_list2 = &response2.data.page.data.media;

    assert_eq!(media_list1.len(), 5, "Page 1 should have 5 results");
    assert_eq!(media_list2.len(), 5, "Page 2 should have 5 results");

    let ids1: Vec<Option<i32>> = media_list1.iter().map(|m| m.id).collect();
    let ids2: Vec<Option<i32>> = media_list2.iter().map(|m| m.id).collect();
    assert_ne!(ids1, ids2, "Different pages should have different results");
}
