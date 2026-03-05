//! Tests for Airing endpoint

use crate::test_harness::{delay_between_tests, TestHarness};
use anilist_moe::endpoints::airing::*;

fn harness() -> TestHarness {
    TestHarness::new()
}

#[tokio::test]
async fn test_fetch_airing_schedules() {
    let h = harness();
    let client = h.client();

    let result = h
        .run(|| async {
            let options = FetchAiringOptions {
                per_page: Some(10),
                ..Default::default()
            };
            client.airing().fetch(&options).await
        })
        .await;

    assert!(
        result.is_ok(),
        "Should successfully fetch airing schedules: {:?}",
        result.err()
    );

    let response = result.unwrap();
    let schedules = &response.data;
    assert!(!schedules.is_empty(), "Should return airing schedules");

    let first_schedule = &schedules[0];
    assert!(
        first_schedule.id.is_some() && first_schedule.id.unwrap() > 0,
        "Schedule should have a positive ID"
    );
    assert!(
        first_schedule.airing_at > Some(0),
        "Should have airing time"
    );
    assert!(
        first_schedule.episode > Some(0),
        "Should have episode number"
    );
}

#[tokio::test]
async fn test_fetch_airing_pagination() {
    delay_between_tests().await;
    let h = harness();
    let client = h.client();

    // Fetch page 1
    let result1 = h
        .run(|| async {
            let options = FetchAiringOptions {
                per_page: Some(5),
                page: Some(1),
                ..Default::default()
            };
            client.airing().fetch(&options).await
        })
        .await;

    assert!(
        result1.is_ok(),
        "Should successfully fetch page 1: {:?}",
        result1.err()
    );

    delay_between_tests().await;

    // Fetch page 2
    let result2 = h
        .run(|| async {
            let options = FetchAiringOptions {
                per_page: Some(5),
                page: Some(2),
                ..Default::default()
            };
            client.airing().fetch(&options).await
        })
        .await;

    assert!(
        result2.is_ok(),
        "Should successfully fetch page 2: {:?}",
        result2.err()
    );

    let response1 = result1.unwrap();
    let response2 = result2.unwrap();

    let schedules1 = &response1.data;
    let schedules2 = &response2.data;

    // Verify we got results
    assert!(!schedules1.is_empty(), "Page 1 should have results");
    assert!(!schedules2.is_empty(), "Page 2 should have results");

    // Verify different pages have different results
    let ids1: Vec<Option<i32>> = schedules1.iter().map(|s| s.id).collect();
    let ids2: Vec<Option<i32>> = schedules2.iter().map(|s| s.id).collect();
    assert_ne!(ids1, ids2, "Different pages should have different results");
}

#[tokio::test]
async fn test_airing_data_types() {
    delay_between_tests().await;
    let h = harness();
    let client = h.client();

    let result = h
        .run(|| async {
            let options = FetchAiringOptions {
                per_page: Some(1),
                ..Default::default()
            };
            client.airing().fetch(&options).await
        })
        .await;

    assert!(
        result.is_ok(),
        "Should successfully fetch airing schedule: {:?}",
        result.err()
    );

    let response = result.unwrap();
    if let Some(schedule) = response.data.first() {
        assert!(
            schedule.id.is_some() && schedule.id.unwrap() > 0,
            "ID should be positive"
        );
        assert!(
            schedule.airing_at > Some(0),
            "Airing time should be positive timestamp"
        );
        assert!(
            schedule.episode > Some(0),
            "Episode number should be positive"
        );
        assert!(
            schedule.media_id.is_some() && schedule.media_id.unwrap() > 0,
            "Media ID should be positive"
        );
    }
}
