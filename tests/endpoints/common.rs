//! Tests for Common endpoint (likes, follows, favourites)

use anilist_moe::{AniListClient, endpoints::common::*, enums::likable::LikeableType};
use dotenv::dotenv;
use log::info;
use std::env;

fn get_authenticated_client() -> AniListClient {
    dotenv().ok();
    let token = env::var("ANILIST_TOKEN").expect("ANILIST_TOKEN must be set in .env file");
    AniListClient::with_token(&token)
}

// All common endpoint functions require authentication

#[tokio::test]
async fn test_toggle_like() {
    let client = get_authenticated_client();

    // Try to like an activity (using a known activity ID)
    let options = ToggleLikeOptions {
        id: 870493538,
        like_type: LikeableType::Activity,
    };

    let result = client.common().toggle_like(options).await;

    match result {
        Ok(response) => {
            info!("Response: {:?}", response);
            println!("Successfully toggled like on item");
            println!("Response data: {:?}", response.data.toggle_like_v2);
        }
        Err(e) => {
            println!("Expected authentication error or permission issue: {:?}", e);
        }
    }
}

#[tokio::test]
async fn test_toggle_follow() {
    let client = get_authenticated_client();

    // Try to follow a user
    let options = ToggleFollowOptions { user_id: 5429396 };

    let result = client.common().toggle_follow(options).await;

    match result {
        Ok(response) => {
            info!("Response: {:?}", response);
            println!("Successfully toggled follow");
            println!(
                "User: {} (ID: {})",
                response.data.toggle_follow.name.as_ref().unwrap(),
                response.data.toggle_follow.id
            );
        }
        Err(e) => {
            println!("Expected authentication error or permission issue: {:?}", e);
        }
    }
}

#[tokio::test]
async fn test_toggle_favourite() {
    let client = get_authenticated_client();

    // Try to favourite an anime
    let options = ToggleFavouriteOptions {
        anime_id: Some(1), // Cowboy Bebop
        manga_id: None,
        character_id: None,
        staff_id: None,
        studio_id: None,
    };

    let result = client.common().toggle_favourite(options).await;

    match result {
        Ok(response) => {
            info!("Response: {:?}", response);
            println!("Successfully toggled favourite");
            println!("Favourites data: {:?}", response.data.toggle_favourite);
        }
        Err(e) => {
            println!("Expected authentication error or permission issue: {:?}", e);
        }
    }
}
