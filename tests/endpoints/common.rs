//! Tests for Common endpoint (likes, follows, favourites)

use crate::support::allow_auth_rejection;
use crate::test_harness::{delay_between_tests, get_authenticated_harness};
use anilist_moe::options;
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
            allow_auth_rejection("test_toggle_like", &e);
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
            let options = options!(
                ToggleFollowOptions,
                user_id => 5429396,
            );
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
            allow_auth_rejection("test_toggle_follow", &e);
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
            let options = options!(
                ToggleFavouriteOptions,
                anime_id: Some(1), // Cowboy Bebop
            );
            client.common().toggle_favourite(&options).await
        })
        .await;

    match result {
        Ok(response) => {
            println!("Successfully toggled favourite");
            println!("Favourites data: {:?}", response);
        }
        Err(e) => {
            allow_auth_rejection("test_toggle_favourite", &e);
        }
    }
}

#[tokio::test]
async fn test_genres() {
    let h = crate::test_harness::TestHarness::new();
    let client = h.client();

    let result = h.run(|| async { client.common().genres().await }).await;
    assert!(
        result.is_ok(),
        "Should fetch the genre list: {:?}",
        result.err()
    );

    let genres = result.unwrap();
    assert!(
        genres.iter().any(|genre| genre == "Action"),
        "Action should be one of the genres, got {:?}",
        genres
    );
}

#[tokio::test]
async fn test_media_tags() {
    let h = crate::test_harness::TestHarness::new();
    let client = h.client();

    let result = h
        .run(|| async { client.common().media_tags(None).await })
        .await;
    assert!(
        result.is_ok(),
        "Should fetch the media tags: {:?}",
        result.err()
    );

    let tags = result.unwrap();
    assert!(!tags.is_empty(), "There should be media tags");
    assert!(
        tags.iter()
            .any(|tag| tag.name.as_deref() == Some("Female Harem")),
        "a known tag should be in the list"
    );
}

#[tokio::test]
async fn test_likes() {
    let h = crate::test_harness::TestHarness::new();
    let client = h.client();

    let result = h
        .run(|| async {
            let options = LikesOptions {
                likeable_id: 870493538,
                like_type: LikeableType::Activity,
                page: Some(1),
                per_page: Some(5),
            };
            client.common().likes(&options).await
        })
        .await;

    assert!(
        result.is_ok(),
        "Should list the users who liked an activity: {:?}",
        result.err()
    );
    for user in &result.unwrap().data {
        assert!(user.id > 0, "every user should have an id");
    }
}
