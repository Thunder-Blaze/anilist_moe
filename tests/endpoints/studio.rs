//! Tests for Studio endpoint

use anilist_moe::{AniListClient, endpoints::studio::*};
use log::info;

#[tokio::test]
async fn test_fetch_studio_by_search() {
    let client = AniListClient::new();
    let options = FetchStudioOptions {
        search: Some("Kyoto".to_string()),
        per_page: Some(5),
        ..Default::default()
    };

    let result = client.studio().fetch(options).await;
    assert!(result.is_ok(), "Should successfully fetch studios");

    let response = result.unwrap();
    info!("Response: {:?}", response);
    let studios = &response.data.page.data.studios;
    assert!(!studios.is_empty(), "Should return at least one studio");

    let first_studio = &studios[0];
    assert!(first_studio.id.is_some(), "Studio should have an ID");
    if let Some(id) = first_studio.id {
        assert!(id > 0, "Studio ID should be positive");
    }
    assert!(first_studio.name.is_some(), "Studio should have a name");
}

#[tokio::test]
async fn test_fetch_studio_by_id() {
    let client = AniListClient::new();
    let options = FetchStudioOptions {
        id: Some(2), // Kyoto Animation
        ..Default::default()
    };

    let result = client.studio().fetch(options).await;
    assert!(result.is_ok(), "Should successfully fetch studio by ID");

    let response = result.unwrap();
    info!("Response: {:?}", response);
    let studios = &response.data.page.data.studios;
    assert_eq!(studios.len(), 1, "Should return exactly one studio");
    assert_eq!(studios[0].id, Some(2), "Should return correct studio ID");
}

#[tokio::test]
async fn test_fetch_one_studio() {
    let client = AniListClient::new();
    let options = FetchStudioOneOptions {
        id: Some(2),
        ..Default::default()
    };

    let result = client.studio().fetch_one(options).await;
    if let Err(ref e) = result {
        eprintln!("Error fetching one studio: {:?}", e);
    }
    assert!(
        result.is_ok(),
        "Should successfully fetch one studio: {:?}",
        result.err()
    );

    let response = result.unwrap();
    info!("Response: {:?}", response);
    let studio = &response.data.studio;
    assert_eq!(studio.id, Some(2), "Should return studio with ID 2");
}

#[tokio::test]
async fn test_studio_data_types() {
    let client = AniListClient::new();
    let options = FetchStudioOptions {
        id: Some(2),
        ..Default::default()
    };

    let result = client.studio().fetch(options).await;
    assert!(result.is_ok(), "Should successfully fetch studio");

    let response = result.unwrap();
    info!("Response: {:?}", response);
    let studio = &response.data.page.data.studios[0];

    if let Some(id) = studio.id {
        assert!(id > 0, "ID should be positive");
    }

    if let Some(favourites) = studio.favourites {
        assert!(favourites >= 0, "Favourites should be non-negative");
    }

    if let Some(ref name) = studio.name {
        assert!(!name.is_empty(), "Name should not be empty");
    }
}
