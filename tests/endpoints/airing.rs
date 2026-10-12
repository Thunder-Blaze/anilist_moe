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
            let options = FetchAiringOptions::builder().per_page(Some(10)).build();
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
            let options = FetchAiringOptions::builder()
                .per_page(Some(5))
                .page(Some(1))
                .build();
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
            let options = FetchAiringOptions::builder()
                .per_page(Some(5))
                .page(Some(2))
                .build();
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
    let ids1: Vec<Option<u32>> = schedules1.iter().map(|s| s.id).collect();
    let ids2: Vec<Option<u32>> = schedules2.iter().map(|s| s.id).collect();
    assert_ne!(ids1, ids2, "Different pages should have different results");
}

#[tokio::test]
async fn test_airing_data_types() {
    delay_between_tests().await;
    let h = harness();
    let client = h.client();

    let result = h
        .run(|| async {
            let options = FetchAiringOptions::builder().per_page(Some(1)).build();
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

#[tokio::test]
async fn test_fetch_one_airing_schedule() {
    let h = harness();
    let client = h.client();

    // Which media has a schedule is the API's business, so ask it for one and
    // then fetch that single schedule by id.
    let result = h
        .run(|| async {
            let page = client
                .airing()
                .fetch(
                    &FetchAiringOptions::builder()
                        .not_yet_aired(Some(true))
                        .per_page(Some(1))
                        .build(),
                )
                .await?;
            let listed = page.data.first().expect("an upcoming schedule");
            let id = listed.id.expect("a schedule id");
            let one = client
                .airing()
                .fetch_one(&FetchAiringOptions::builder().id(Some(id)).build())
                .await?;
            Ok::<_, anilist_moe::AniListError>((id, listed.media_id, one))
        })
        .await;

    assert!(
        result.is_ok(),
        "Should fetch one airing schedule: {:?}",
        result.err()
    );

    let (id, media_id, one) = result.unwrap();
    assert_eq!(
        one.id,
        Some(id),
        "the single fetch should answer with the id asked for"
    );
    assert_eq!(
        one.media_id, media_id,
        "and for the same media the list gave"
    );
}
