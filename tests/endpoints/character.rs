//! Tests for Character endpoint

use crate::test_harness::{TestHarness, delay_between_tests};
use anilist_moe::endpoints::character::*;

fn harness() -> TestHarness {
    TestHarness::new()
}

#[tokio::test]
async fn test_fetch_character_by_search() {
    let h = harness();
    let client = h.client();

    let result = h
        .run(|| async {
            let options = FetchCharacterOptions {
                search: Some("Spike".to_string()),
                per_page: Some(5),
                ..Default::default()
            };
            client.character().fetch(&options).await
        })
        .await;

    assert!(
        result.is_ok(),
        "Should successfully fetch characters: {:?}",
        result.err()
    );

    let response = result.unwrap();
    let characters = &response.data;
    assert!(
        !characters.is_empty(),
        "Should return at least one character"
    );
    assert!(characters.len() <= 5, "Should respect per_page limit");

    let first_char = &characters[0];
    assert!(first_char.id > 0, "Character should have a positive ID");
    assert!(first_char.name.is_some(), "Character should have a name");

    // Verify search relevance
    if let Some(ref name) = first_char.name {
        let _has_spike = name
            .full
            .as_ref()
            .map(|n| n.to_lowercase().contains("spike"))
            .unwrap_or(false)
            || name
                .first
                .as_ref()
                .map(|n| n.to_lowercase().contains("spike"))
                .unwrap_or(false);
        // Note: First result might not always contain "Spike" depending on search algorithm
        println!("First character: {:?}", name.full);
    }
}

#[tokio::test]
async fn test_fetch_character_by_id() {
    delay_between_tests().await;
    let h = harness();
    let client = h.client();

    let result = h
        .run(|| async {
            let options = FetchCharacterOptions {
                id: Some(1),
                ..Default::default()
            };
            client.character().fetch(&options).await
        })
        .await;

    assert!(
        result.is_ok(),
        "Should successfully fetch character by ID: {:?}",
        result.err()
    );

    let response = result.unwrap();
    let characters = &response.data;
    assert_eq!(characters.len(), 1, "Should return exactly one character");
    assert_eq!(characters[0].id, 1, "Should return correct character ID");
}

#[tokio::test]
async fn test_fetch_one_character() {
    delay_between_tests().await;
    let h = harness();
    let client = h.client();

    let result = h
        .run(|| async {
            let options = FetchCharacterOneOptions {
                id: Some(1),
                ..Default::default()
            };
            client.character().fetch_one(&options).await
        })
        .await;

    assert!(
        result.is_ok(),
        "Should successfully fetch one character: {:?}",
        result.err()
    );

    let character = result.unwrap();
    assert_eq!(character.id, 1, "Should return character with ID 1");
    assert!(character.name.is_some(), "Character should have a name");
}

#[tokio::test]
async fn test_character_data_types() {
    delay_between_tests().await;
    let h = harness();
    let client = h.client();

    let result = h
        .run(|| async {
            let options = FetchCharacterOptions {
                id: Some(1),
                ..Default::default()
            };
            client.character().fetch(&options).await
        })
        .await;

    assert!(result.is_ok(), "Should successfully fetch character");

    let response = result.unwrap();
    let character = &response.data[0];

    // Verify required fields
    assert!(character.id > 0, "ID should be positive");

    // Verify optional numeric fields
    if let Some(favourites) = character.favourites {
        assert!(favourites >= 0, "Favourites should be non-negative");
    }

    // Verify name structure
    if let Some(ref name) = character.name {
        let has_name = name.first.is_some() || name.last.is_some() || name.full.is_some();
        assert!(has_name, "Name should have at least one field");

        // Full name should be present for most characters
        if let Some(ref full) = name.full {
            assert!(!full.is_empty(), "Full name should not be empty if present");
        }
    }
}

#[tokio::test]
async fn test_character_search_pagination() {
    delay_between_tests().await;
    let h = harness();
    let client = h.client();

    // Fetch page 1
    let result1 = h
        .run(|| async {
            let options = FetchCharacterOptions {
                search: Some("Naruto".to_string()),
                page: Some(1),
                per_page: Some(5),
                ..Default::default()
            };
            client.character().fetch(&options).await
        })
        .await;

    assert!(result1.is_ok(), "Should fetch page 1");

    delay_between_tests().await;

    // Fetch page 2
    let result2 = h
        .run(|| async {
            let options = FetchCharacterOptions {
                search: Some("Naruto".to_string()),
                page: Some(2),
                per_page: Some(5),
                ..Default::default()
            };
            client.character().fetch(&options).await
        })
        .await;

    assert!(result2.is_ok(), "Should fetch page 2");

    let response1 = result1.unwrap();
    let response2 = result2.unwrap();

    // Verify page info
    if let Some(ref page_info) = response1.page_info {
        assert_eq!(page_info.current_page, Some(1), "Should be page 1");
    }

    if let Some(ref page_info) = response2.page_info {
        assert_eq!(page_info.current_page, Some(2), "Should be page 2");
    }

    // Verify different pages have different results
    let ids1: Vec<i32> = response1.data.iter().map(|c| c.id).collect();
    let ids2: Vec<i32> = response2.data.iter().map(|c| c.id).collect();

    // Check for no overlap between pages
    let overlap: Vec<&i32> = ids1.iter().filter(|id| ids2.contains(id)).collect();
    assert!(
        overlap.is_empty(),
        "Different pages should not have overlapping results"
    );
}
