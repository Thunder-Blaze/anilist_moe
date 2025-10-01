//! Tests for Character endpoint

use anilist_moe::{AniListClient, endpoints::character::*};

#[tokio::test]
async fn test_fetch_character_by_search() {
    let client = AniListClient::new();
    let options = FetchCharacterOptions {
        search: Some("Spike".to_string()),
        per_page: Some(5),
        ..Default::default()
    };

    let result = client.character().fetch(options).await;
    assert!(result.is_ok(), "Should successfully fetch characters");
    
    let response = result.unwrap();
    let characters = &response.data.page.data.characters;
    assert!(!characters.is_empty(), "Should return at least one character");
    
    let first_char = &characters[0];
    assert!(first_char.id > 0, "Character should have a positive ID");
    assert!(first_char.name.is_some(), "Character should have a name");
}

#[tokio::test]
async fn test_fetch_character_by_id() {
    let client = AniListClient::new();
    let options = FetchCharacterOptions {
        id: Some(1),
        ..Default::default()
    };

    let result = client.character().fetch(options).await;
    assert!(result.is_ok(), "Should successfully fetch character by ID");
    
    let response = result.unwrap();
    let characters = &response.data.page.data.characters;
    assert_eq!(characters.len(), 1, "Should return exactly one character");
    assert_eq!(characters[0].id, 1, "Should return correct character ID");
}

#[tokio::test]
async fn test_fetch_one_character() {
    let client = AniListClient::new();
    let options = FetchCharacterOneOptions {
        id: Some(1),
        ..Default::default()
    };

    let result = client.character().fetch_one(options).await;
    assert!(result.is_ok(), "Should successfully fetch one character");
    
    let response = result.unwrap();
    let characters = &response.data.page.data.characters;
    assert!(!characters.is_empty(), "Should return character");
    assert_eq!(characters[0].id, 1, "Should return character with ID 1");
}

#[tokio::test]
async fn test_character_data_types() {
    let client = AniListClient::new();
    let options = FetchCharacterOptions {
        id: Some(1),
        ..Default::default()
    };

    let result = client.character().fetch(options).await;
    assert!(result.is_ok(), "Should successfully fetch character");
    
    let response = result.unwrap();
    let character = &response.data.page.data.characters[0];
    
    assert!(character.id > 0, "ID should be positive");
    
    if let Some(favourites) = character.favourites {
        assert!(favourites >= 0, "Favourites should be non-negative");
    }
    
    if let Some(ref name) = character.name {
        let has_name = name.first.is_some() || name.last.is_some() || name.full.is_some();
        assert!(has_name, "Name should have at least one field");
    }
}
