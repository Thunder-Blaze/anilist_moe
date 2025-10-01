//! Tests for MediaList endpoint

use anilist_moe::{AniListClient, endpoints::medialist::*};
use anilist_moe::enums::media_list::MediaListStatus;
use dotenv::dotenv;
use std::env;

fn get_authenticated_client() -> AniListClient {
    dotenv().ok();
    let token = env::var("ANILIST_TOKEN").expect("ANILIST_TOKEN must be set in .env file");
    AniListClient::with_token(&token)
}

#[tokio::test]
async fn test_fetch_media_list() {
    let client = AniListClient::new();
    let options = FetchMediaListOptions {
        user_id: Some(3225),
        per_page: Some(5),
        ..Default::default()
    };

    let result = client.medialist().fetch(options).await;
    if let Err(ref e) = result {
        eprintln!("Error fetching media list: {:?}", e);
    }
    assert!(result.is_ok(), "Should successfully fetch media list: {:?}", result.err());

    let response = result.unwrap();
    let lists = &response.data.page.data.media_list;
    assert!(!lists.is_empty(), "Should return at least one media list entry");
}

#[tokio::test]
async fn test_fetch_media_list_by_media() {
    let client = AniListClient::new();
    let options = FetchMediaListOptions {
        media_id: Some(1), // Cowboy Bebop
        per_page: Some(5),
        ..Default::default()
    };

    let result = client.medialist().fetch(options).await;
    if let Err(ref e) = result {
        eprintln!("Error fetching media list by media: {:?}", e);
    }
    assert!(result.is_ok(), "Should successfully fetch media list by media: {:?}", result.err());

    let response = result.unwrap();
    let lists = &response.data.page.data.media_list;
    if !lists.is_empty() {
        let first_entry = &lists[0];
        assert!(first_entry.id > 0, "Media list entry should have a positive ID");
    }
}

#[tokio::test]
async fn test_media_list_data_types() {
    let client = AniListClient::new();
    let options = FetchMediaListOptions {
        user_id: Some(3225),
        per_page: Some(1),
        ..Default::default()
    };

    let result = client.medialist().fetch(options).await;
    assert!(result.is_ok(), "Should successfully fetch media list");

    let response = result.unwrap();
    let lists = &response.data.page.data.media_list;

    if !lists.is_empty() {
        let entry = &lists[0];
        assert!(entry.id > 0, "Media list ID should be positive");
        assert!(entry.user_id > 0, "User ID should be positive");
    }
}

// Authentication required test
#[tokio::test]
async fn test_save_media_list() {
    let client = get_authenticated_client();

    // This test will attempt to save a media list entry
    // It might fail if the user doesn't have proper permissions
    let options = SaveMediaListOptions {
        media_id: Some(1), // Cowboy Bebop
        status: Some(MediaListStatus::Planning),
        ..Default::default()
    };

    let result = client.medialist().save(options).await;

    match result {
        Ok(_) => {
            println!("Successfully saved media list entry");
        }
        Err(e) => {
            println!("Expected authentication error or permission issue: {:?}", e);
        }
    }
}
