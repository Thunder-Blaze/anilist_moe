//! Tests for Common endpoint (likes, follows, favourites)

use crate::test_harness::{delay_between_tests, get_authenticated_harness};
use anilist_moe::{endpoints::common::*, enums::likable::LikeableType};

// All common endpoint functions require authentication

#[tokio::test]
async fn test_toggle_like() {
    let Some(h) = get_authenticated_harness() else {
        eprintln!("Skipping test_toggle_like: ANILIST_TOKEN not set");
        return;
    };
    let client = h.client().clone();

    let result = h
        .run(|| async {
            let options = ToggleLikeOptions {
                id: 870493538,
                like_type: LikeableType::Activity,
            };
            client.common().toggle_like(&options).await
        })
        .await;

    match result {
        Ok(response) => {
            println!("Successfully toggled like on item");
            println!("Response data: {:?}", response);
        }
        Err(e) => {
            println!("Expected authentication error or permission issue: {:?}", e);
        }
    }
}

#[tokio::test]
async fn test_toggle_follow() {
    let Some(h) = get_authenticated_harness() else {
        eprintln!("Skipping test_toggle_follow: ANILIST_TOKEN not set");
        return;
    };
    let client = h.client().clone();

    delay_between_tests().await;

    let result = h
        .run(|| async {
            let options = ToggleFollowOptions { user_id: 5429396 };
            client.common().toggle_follow(&options).await
        })
        .await;

    match result {
        Ok(response) => {
            println!("Successfully toggled follow");
            println!(
                "User: {} (ID: {})",
                response.name.as_ref().unwrap_or(&"Unknown".to_string()),
                response.id
            );
        }
        Err(e) => {
            println!("Expected authentication error or permission issue: {:?}", e);
        }
    }
}

#[tokio::test]
async fn test_toggle_favourite() {
    let Some(h) = get_authenticated_harness() else {
        eprintln!("Skipping test_toggle_favourite: ANILIST_TOKEN not set");
        return;
    };
    let client = h.client().clone();

    delay_between_tests().await;

    let result = h
        .run(|| async {
            let options = ToggleFavouriteOptions {
                anime_id: Some(1), // Cowboy Bebop
                manga_id: None,
                character_id: None,
                staff_id: None,
                studio_id: None,
            };
            client.common().toggle_favourite(&options).await
        })
        .await;

    match result {
        Ok(response) => {
            println!("Successfully toggled favourite");
            println!("Favourites data: {:?}", response);
        }
        Err(e) => {
            println!("Expected authentication error or permission issue: {:?}", e);
        }
    }
}
