//! Tests for Activity endpoint

use anilist_moe::{AniListClient, endpoints::activity::*};
use dotenv::dotenv;
use std::env;
use log::info;

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
    info!("Response: {:?}", response);
    let activities = &response.data.page.data.activities;
    println!("Fetched {} activities", activities.len());
}

#[tokio::test]
async fn test_fetch_activities_by_user() {
    let client = AniListClient::new();
    let options = FetchActivityOptions {
        user_id: Some(5429396),
        per_page: Some(5),
        ..Default::default()
    };

    let result = client.activity().fetch(options).await;
    if let Err(ref e) = result {
        eprintln!("Error fetching activities by user: {:?}", e);
    }
    if result.is_ok() {
        let response = result.unwrap();
        info!("Response: {:?}", response);
        let activities = &response.data.page.data.activities;
        println!("Fetched {} activities for user", activities.len());
    }
}

#[tokio::test]
async fn test_activity_full_lifecycle() {
    let client = get_authenticated_client();

    println!("\n=== Testing Activity Full Lifecycle ===");

    // Step 1: Create a text activity
    println!("Step 1: Creating text activity...");
    let save_options = SaveTextActivityOptions {
        text: "Test activity from anilist_moe library - full lifecycle test".to_string(),
        id: None,
        locked: None,
    };

    let create_result = client.activity().save_text_activity(save_options).await;
    if let Err(ref e) = create_result {
        println!("Failed to create activity: {:?}", e);
        return;
    }

    let create_response = create_result.unwrap();
    info!("Create Response: {:?}", create_response);
    let activity_id = match &create_response.data.activity {
        anilist_moe::unions::activity::ActivityUnion::TextActivity(a) => {
            println!("✓ Created activity with ID: {}", a.id);
            a.id
        }
        _ => {
            println!("✗ Unexpected activity type");
            return;
        }
    };

    // Step 2: Modify the activity
    println!("Step 2: Modifying activity...");
    let modify_options = SaveTextActivityOptions {
        id: Some(activity_id),
        text: "Modified test activity from anilist_moe library".to_string(),
        locked: None,
    };

    let modify_result = client.activity().save_text_activity(modify_options).await;
    match modify_result {
        Ok(response) => {
            info!("Modify Response: {:?}", response);
            println!("✓ Modified activity successfully");
        }
        Err(e) => println!("✗ Failed to modify activity: {:?}", e),
    }

    // Step 3: Fetch the activity
    println!("Step 3: Fetching activity...");
    let fetch_options = FetchActivityOneOptions {
        id: activity_id,
    };

    let fetch_result = client.activity().fetch_one(fetch_options).await;
    match fetch_result {
        Ok(response) => {
            info!("Fetch Response: {:?}", response);
            match &response.data.activity {
                anilist_moe::unions::activity::ActivityUnion::TextActivity(a) => {
                    println!("✓ Fetched activity: {}", a.text.as_ref().unwrap_or(&"".to_string()));
                }
                _ => println!("✗ Unexpected activity type"),
            }
        }
        Err(e) => println!("✗ Failed to fetch activity: {:?}", e),
    }

    // Step 4: Delete the activity
    println!("Step 4: Deleting activity...");
    let delete_options = DeleteActivityOptions {
        id: activity_id,
    };

    let delete_result = client.activity().delete(delete_options).await;
    match delete_result {
        Ok(response) => {
            info!("Delete Response: {:?}", response);
            println!("✓ Deleted activity successfully");
        }
        Err(e) => println!("✗ Failed to delete activity: {:?}", e),
    }

    println!("=== Activity Lifecycle Test Complete ===\n");
}
