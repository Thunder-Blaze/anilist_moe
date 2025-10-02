//! Tests for Airing endpoint

use anilist_moe::{AniListClient, endpoints::airing::*};
use log::info;

#[tokio::test]
async fn test_fetch_airing_schedules() {
    let client = AniListClient::new();
    let options = FetchAiringOptions {
        per_page: Some(10),
        ..Default::default()
    };

    let result = client.airing().fetch(options).await;
    if let Err(ref e) = result {
        eprintln!("Error fetching airing schedules: {:?}", e);
    }
    assert!(result.is_ok(), "Should successfully fetch airing schedules: {:?}", result.err());

    let response = result.unwrap();
    info!("Response: {:?}", response);
    let schedules = &response.data.page.data.airing_schedules;
    assert!(!schedules.is_empty(), "Should return airing schedules");

    let first_schedule = &schedules[0];
    assert!(first_schedule.id.is_some() && first_schedule.id.unwrap() > 0, "Schedule should have a positive ID");
    assert!(first_schedule.airing_at > 0, "Should have airing time");
    assert!(first_schedule.episode > 0, "Should have episode number");
}

#[tokio::test]
async fn test_fetch_airing_pagination() {
    let client = AniListClient::new();

    let options_page1 = FetchAiringOptions {
        per_page: Some(5),
        page: Some(1),
        ..Default::default()
    };

    let result1 = client.airing().fetch(options_page1).await;
    if let Err(ref e) = result1 {
        eprintln!("Error fetching airing page 1: {:?}", e);
    }
    assert!(result1.is_ok(), "Should successfully fetch page 1: {:?}", result1.as_ref().err());

    let options_page2 = FetchAiringOptions {
        per_page: Some(5),
        page: Some(2),
        ..Default::default()
    };

    let result2 = client.airing().fetch(options_page2).await;
    assert!(result2.is_ok(), "Should successfully fetch page 2");

    let response1 = result1.unwrap();
    let response2 = result2.unwrap();

    let schedules1 = &response1.data.page.data.airing_schedules;
    let schedules2 = &response2.data.page.data.airing_schedules;

    let ids1: Vec<Option<i32>> = schedules1.iter().map(|s| s.id).collect();
    let ids2: Vec<Option<i32>> = schedules2.iter().map(|s| s.id).collect();
    assert_ne!(ids1, ids2, "Different pages should have different results");
}

#[tokio::test]
async fn test_airing_data_types() {
    let client = AniListClient::new();
    let options = FetchAiringOptions {
        per_page: Some(1),
        ..Default::default()
    };

    let result = client.airing().fetch(options).await;
    if let Err(ref e) = result {
        eprintln!("Error fetching airing schedule: {:?}", e);
    }
    assert!(result.is_ok(), "Should successfully fetch airing schedule: {:?}", result.err());

    let response = result.unwrap();
    info!("Response: {:?}", response);
    if let Some(schedule) = response.data.page.data.airing_schedules.first() {
        assert!(schedule.id.is_some() && schedule.id.unwrap() > 0, "ID should be positive");
        assert!(schedule.airing_at > 0, "Airing time should be positive timestamp");
        assert!(schedule.episode > 0, "Episode number should be positive");
        assert!(schedule.media_id.is_some() && schedule.media_id.unwrap() > 0, "Media ID should be positive");
    }
}
