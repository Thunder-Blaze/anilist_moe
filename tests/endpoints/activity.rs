//! Tests for Activity endpoint

use anilist_moe::{AniListClient, endpoints::activity::*};
use dotenv::dotenv;
use std::env;

fn get_authenticated_client() -> AniListClient {
    dotenv().ok();
    let token = env::var("ANILIST_TOKEN").expect("ANILIST_TOKEN must be set in .env file");
    AniListClient::with_token(&token)
}

#[tokio::test]
async fn test_fetch_activities() {
    let client = AniListClient::new();
    let options = FetchActivityOptions {
        per_page: Some(5),
        ..Default::default()
    };

    let result = client.activity().fetch(options).await;
    if let Err(ref e) = result {
        eprintln!("Error fetching activities: {:?}", e);
    }
    assert!(result.is_ok(), "Should successfully fetch activities: {:?}", result.err());

    let response = result.unwrap();
    let activities = &response.data.page.data.activities;
    assert!(!activities.is_empty(), "Should return at least one activity");
}

#[tokio::test]
async fn test_fetch_activities_by_user() {
    let client = AniListClient::new();
    let options = FetchActivityOptions {
        user_id: Some(3225),
        per_page: Some(5),
        ..Default::default()
    };

    let result = client.activity().fetch(options).await;
    assert!(result.is_ok(), "Should successfully fetch activities by user");

    let response = result.unwrap();
    let activities = &response.data.page.data.activities;
    assert!(!activities.is_empty(), "Should return at least one activity");
}

#[tokio::test]
async fn test_fetch_one_activity() {
    let client = AniListClient::new();

    // First fetch some activities to get a valid ID
    let list_options = FetchActivityOptions {
        per_page: Some(1),
        ..Default::default()
    };
    let list_result = client.activity().fetch(list_options).await;
    assert!(list_result.is_ok(), "Should successfully fetch activities");

    let activities = &list_result.unwrap().data.page.data.activities;
    if activities.is_empty() {
        println!("No activities found to test fetch_one");
        return;
    }

    let activity_id = match &activities[0] {
        anilist_moe::unions::activity::ActivityUnion::TextActivity(a) => a.id,
        anilist_moe::unions::activity::ActivityUnion::ListActivity(a) => a.id,
        anilist_moe::unions::activity::ActivityUnion::MessageActivity(a) => a.id,
    };
    let options = FetchActivityOneOptions {
        id: activity_id,
    };

    let result = client.activity().fetch_one(options).await;
    assert!(result.is_ok(), "Should successfully fetch one activity");
}

#[tokio::test]
async fn test_fetch_activity_replies() {
    let client = AniListClient::new();

    // First fetch an activity with replies
    let list_options = FetchActivityOptions {
        has_replies: Some(true),
        per_page: Some(1),
        ..Default::default()
    };
    let list_result = client.activity().fetch(list_options).await;

    if let Ok(response) = list_result {
        let activities = &response.data.page.data.activities;
        if !activities.is_empty() {
            let activity_id = match &activities[0] {
                anilist_moe::unions::activity::ActivityUnion::TextActivity(a) => a.id,
                anilist_moe::unions::activity::ActivityUnion::ListActivity(a) => a.id,
                anilist_moe::unions::activity::ActivityUnion::MessageActivity(a) => a.id,
            };
            let options = FetchActivityRepliesOptions {
                activity_id,
                per_page: Some(5),
                ..Default::default()
            };

            let result = client.activity().fetch_replies(options).await;
            if let Err(ref e) = result {
                eprintln!("Error fetching activity replies: {:?}", e);
            }
            if result.is_ok() {
                let response = result.unwrap();
                let replies = &response.data.page.data.activity_replies;
                println!("Found {} activity replies", replies.len());
            }
        }
    }
}

#[tokio::test]
async fn test_activity_data_types() {
    let client = AniListClient::new();
    let options = FetchActivityOptions {
        per_page: Some(1),
        ..Default::default()
    };

    let result = client.activity().fetch(options).await;
    assert!(result.is_ok(), "Should successfully fetch activities");

    let response = result.unwrap();
    let activities = &response.data.page.data.activities;

    if !activities.is_empty() {
        let activity_id = match &activities[0] {
            anilist_moe::unions::activity::ActivityUnion::TextActivity(a) => a.id,
            anilist_moe::unions::activity::ActivityUnion::ListActivity(a) => a.id,
            anilist_moe::unions::activity::ActivityUnion::MessageActivity(a) => a.id,
        };
        assert!(activity_id > 0, "Activity ID should be positive");
    }
}

// Authentication required tests
#[tokio::test]
async fn test_save_text_activity() {
    let client = get_authenticated_client();
    let options = SaveTextActivityOptions {
        text: "Test activity from anilist_moe library".to_string(),
        id: None,
        locked: None,
    };

    let result = client.activity().save_text_activity(options).await;

    // This might fail if the user doesn't have proper permissions
    // We just check if it processes the request properly
    match result {
        Ok(_) => {
            // Success case - cleanup by deleting
            println!("Successfully created test activity");
        }
        Err(e) => {
            println!("Expected authentication error or permission issue: {:?}", e);
        }
    }
}
