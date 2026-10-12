//! Tests for Common endpoint (likes, follows, favourites)

use crate::support::allow_auth_rejection;
use crate::test_harness::{delay_between_tests, get_authenticated_harness};
use anilist_moe::options;
use anilist_moe::{
    endpoints::common::*,
    enums::{
        external_link::{ExternalLinkMediaType, ExternalLinkType},
        likable::LikeableType,
    },
};

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

#[tokio::test]
async fn test_markdown() {
    let h = crate::test_harness::TestHarness::new();
    let client = h.client();

    let result = h
        .run(|| async { client.common().markdown("# Hello").await })
        .await;

    assert!(result.is_ok(), "Should render markdown: {:?}", result.err());
    let html = result.unwrap().html.unwrap_or_default();
    assert!(
        html.contains("<h1>Hello</h1>"),
        "the rendered html should carry the heading, got {html:?}"
    );
}

#[tokio::test]
async fn test_external_link_sources() {
    let h = crate::test_harness::TestHarness::new();
    let client = h.client();

    let result = h
        .run(|| async {
            let options = ExternalLinkSourceOptions {
                id: None,
                link_type: Some(ExternalLinkType::Streaming),
                media_type: Some(ExternalLinkMediaType::Anime),
            };
            client.common().external_link_sources(&options).await
        })
        .await;

    assert!(
        result.is_ok(),
        "Should list the streaming sources: {:?}",
        result.err()
    );

    let sources = result.unwrap();
    assert!(!sources.is_empty(), "there should be streaming sources");
    assert!(
        sources.iter().all(|source| source.site.is_some()),
        "every source should name its site"
    );
}

#[tokio::test]
async fn test_site_statistics() {
    let h = crate::test_harness::TestHarness::new();
    let client = h.client();

    let result = h
        .run(|| async { client.common().site_statistics().await })
        .await;
    assert!(
        result.is_ok(),
        "Should fetch the site statistics: {:?}",
        result.err()
    );

    let stats = result.unwrap();
    let anime = stats.anime.expect("anime statistics");
    let nodes = anime.nodes.expect("anime trend nodes");
    assert!(!nodes.is_empty(), "the anime series should have points");
    assert!(
        nodes.iter().all(|node| node.count > 0),
        "every point should count something"
    );
}
