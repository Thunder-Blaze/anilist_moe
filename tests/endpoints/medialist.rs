//! Tests for MediaList endpoint

use crate::test_harness::{TestHarness, delay_between_tests, get_authenticated_harness};
use anilist_moe::endpoints::medialist::*;
use anilist_moe::enums::media_list::MediaListStatus;

fn harness() -> TestHarness {
    TestHarness::new()
}

#[tokio::test]
async fn test_fetch_media_list() {
    let h = harness();
    let client = h.client();

    let result = h
        .run(|| async {
            let options = FetchMediaListOptions {
                user_id: Some(3225),
                per_page: Some(5),
                ..Default::default()
            };
            client.medialist().fetch(&options).await
        })
        .await;

    assert!(
        result.is_ok(),
        "Should successfully fetch media list: {:?}",
        result.err()
    );

    let response = result.unwrap();
    let lists = &response.data;
    assert!(
        !lists.is_empty(),
        "Should return at least one media list entry"
    );
}

#[tokio::test]
async fn test_fetch_media_list_by_media() {
    let Some(h) = get_authenticated_harness() else {
        eprintln!("Skipping test_fetch_media_list_by_media: ANILIST_TOKEN not set");
        return;
    };
    let client = h.client().clone();

    let result = h
        .run(|| async {
            let options = FetchMediaListOptions {
                media_id: Some(1), // Cowboy Bebop
                per_page: Some(5),
                is_following: Some(true),
                ..Default::default()
            };
            client.medialist().fetch(&options).await
        })
        .await;

    assert!(
        result.is_ok(),
        "Should successfully fetch media list by media: {:?}",
        result.err()
    );

    let response = result.unwrap();
    let lists = &response.data;
    if !lists.is_empty() {
        let first_entry = &lists[0];
        assert!(
            first_entry.id > 0,
            "Media list entry should have a positive ID"
        );
    }
}

#[tokio::test]
async fn test_media_list_data_types() {
    delay_between_tests().await;
    let h = harness();
    let client = h.client();

    let result = h
        .run(|| async {
            let options = FetchMediaListOptions {
                user_id: Some(3225),
                per_page: Some(1),
                ..Default::default()
            };
            client.medialist().fetch(&options).await
        })
        .await;

    assert!(
        result.is_ok(),
        "Should successfully fetch media list: {:?}",
        result.err()
    );

    let response = result.unwrap();
    let lists = &response.data;

    if !lists.is_empty() {
        let entry = &lists[0];
        assert!(entry.id > 0, "Media list ID should be positive");
        assert!(entry.user_id > Some(0), "User ID should be positive");
    }
}

// Authentication required tests
#[tokio::test]
async fn test_save_media_list() {
    let Some(h) = get_authenticated_harness() else {
        eprintln!("Skipping test_save_media_list: ANILIST_TOKEN not set");
        return;
    };
    let client = h.client().clone();

    let result = h
        .run(|| async {
            let options = SaveMediaListOptions {
                media_id: Some(1), // Cowboy Bebop
                status: Some(MediaListStatus::Planning),
                ..Default::default()
            };
            client.medialist().save(&options).await
        })
        .await;

    match result {
        Ok(response) => {
            println!(
                "Successfully saved media list entry with ID: {}",
                response.id
            );
        }
        Err(e) => {
            println!("Expected authentication error or permission issue: {:?}", e);
        }
    }
}

#[tokio::test]
async fn test_delete_media_list() {
    let Some(h) = get_authenticated_harness() else {
        eprintln!("Skipping test_delete_media_list: ANILIST_TOKEN not set");
        return;
    };
    let client = h.client().clone();

    // First create a media list entry to delete
    let save_result = h
        .run(|| async {
            let options = SaveMediaListOptions {
                media_id: Some(20),
                status: Some(MediaListStatus::Planning),
                ..Default::default()
            };
            client.medialist().save(&options).await
        })
        .await;

    if let Ok(response) = save_result {
        let entry_id = response.id;

        delay_between_tests().await;

        let delete_result = h
            .run(|| async {
                let options = DeleteMediaListOptions { id: entry_id };
                client.medialist().delete(&options).await
            })
            .await;

        match delete_result {
            Ok(deleted) => {
                if deleted {
                    println!("Successfully deleted media list entry");
                } else {
                    println!("Failed to delete media list entry");
                }
            }
            Err(e) => {
                println!("Expected authentication error or permission issue: {:?}", e);
            }
        }
    }
}
