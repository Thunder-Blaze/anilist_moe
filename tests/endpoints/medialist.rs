//! Tests for MediaList endpoint

use crate::support::allow_auth_rejection;
use crate::test_harness::{delay_between_tests, get_authenticated_harness, TestHarness};
use anilist_moe::endpoints::medialist::*;
use anilist_moe::enums::media::MediaType;
use anilist_moe::enums::media_list::MediaListStatus;
use anilist_moe::options;

fn harness() -> TestHarness {
    TestHarness::new()
}

#[tokio::test]
async fn test_fetch_media_list() {
    let h = harness();
    let client = h.client();

    let result = h
        .run(|| async {
            let options = options!(
                FetchMediaListOptions,
                user_id: Some(3225),
                per_page: Some(5),
            );
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
            let options = options!(
                FetchMediaListOptions,
                media_id: Some(1), // Cowboy Bebop
                per_page: Some(5),
                is_following: Some(true),
            );
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
            let options = options!(
                FetchMediaListOptions,
                user_id: Some(3225),
                per_page: Some(1),
            );
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
            let options = options!(
                SaveMediaListOptions,
                media_id: Some(1), // Cowboy Bebop
                status: Some(MediaListStatus::Planning),
            );
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
            allow_auth_rejection("test_save_media_list", &e);
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
            let options = options!(
                SaveMediaListOptions,
                media_id: Some(20),
                status: Some(MediaListStatus::Planning),
            );
            client.medialist().save(&options).await
        })
        .await;

    if let Ok(response) = save_result {
        let entry_id = response.id;

        delay_between_tests().await;

        let delete_result = h
            .run(|| async {
                let options = options!(
                    DeleteMediaListOptions,
                    id => entry_id,
                );
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
                allow_auth_rejection("test_delete_media_list", &e);
            }
        }
    }
}

#[tokio::test]
async fn test_fetch_collection() {
    let h = TestHarness::new();
    let client = h.client();

    let result = h
        .run(|| async {
            let options = options!(
                FetchMediaListCollectionOptions,
                user_id: Some(3225),
                media_type: MediaType::Anime,
            );
            client.medialist().fetch_collection(&options).await
        })
        .await;

    assert!(
        result.is_ok(),
        "Should fetch a grouped media list: {:?}",
        result.err()
    );

    let collection = result.unwrap();
    let lists = collection.lists.expect("a collection has lists");
    assert!(!lists.is_empty(), "the user should have lists");
    assert!(
        lists
            .iter()
            .any(|list| list.name.as_deref() == Some("Completed")),
        "the completed list should be among them: {:?}",
        lists
            .iter()
            .map(|list| list.name.clone())
            .collect::<Vec<_>>()
    );
    let entries = lists
        .iter()
        .flat_map(|list| list.entries.clone().unwrap_or_default())
        .count();
    assert!(entries > 0, "the lists should carry entries");
}

#[tokio::test]
async fn test_fetch_one_media_list_entry() {
    let h = TestHarness::new();
    let client = h.client();

    let result = h
        .run(|| async {
            let options = options!(
                FetchMediaListOptions,
                user_id: Some(3225),
            );
            client.medialist().fetch_one(&options).await
        })
        .await;

    assert!(
        result.is_ok(),
        "Should fetch a single media list entry: {:?}",
        result.err()
    );

    let entry = result.unwrap();
    assert!(entry.id > 0, "the entry should have an id");
    assert_eq!(
        entry.user_id,
        Some(3225),
        "and belong to the user asked for"
    );
}
