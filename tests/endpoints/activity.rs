//! Tests for Activity endpoint

use anilist_moe::{AniListClient, endpoints::activity::*};
use dotenv::dotenv;
use log::info;
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
    assert!(
        result.is_ok(),
        "Should successfully fetch activities: {:?}",
        result.err()
    );

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
async fn test_text_activity_full_lifecycle() {
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
    let activity_id = match &create_response.data.save_text_activity {
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
    let fetch_options = FetchActivityOneOptions { id: activity_id };

    let fetch_result = client.activity().fetch_one(fetch_options).await;
    match fetch_result {
        Ok(response) => {
            info!("Fetch Response: {:?}", response);
            match &response.data.activity {
                anilist_moe::unions::activity::ActivityUnion::TextActivity(a) => {
                    println!(
                        "✓ Fetched activity: {}",
                        a.text.as_ref().unwrap_or(&"".to_string())
                    );
                }
                _ => println!("✗ Unexpected activity type"),
            }
        }
        Err(e) => println!("✗ Failed to fetch activity: {:?}", e),
    }

    // Step 4: Delete the activity
    println!("Step 4: Deleting activity...");
    let delete_options = DeleteActivityOptions { id: activity_id };

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

#[tokio::test]
async fn test_message_activity_full_lifecycle() {
    let client = get_authenticated_client();

    println!("\n=== Testing Message Activity Full Lifecycle ===");

    // Get the authenticated user's ID first (using a simple fetch to get our own user)
    // For this test, we'll send a message to ourselves
    let user_id = 5429396; // Your user ID - you can change this to send to another user

    // Step 1: Create a message activity
    println!("Step 1: Creating message activity...");
    let save_options = SaveMessageActivityOptions {
        message: "Heya".to_string(),
        recipient_id: user_id,
        id: None,
        private: Some(true),
        locked: None,
        as_mod: None,
    };

    let create_result = client.activity().save_message_activity(save_options).await;
    if let Err(ref e) = create_result {
        println!("Failed to create message activity: {:?}", e);
        return;
    }

    let create_response = create_result.unwrap();
    info!("Create Response: {:?}", create_response);
    let activity_id = match &create_response.data.save_message_activity {
        a => {
            println!("✓ Created message activity with ID: {}", a.id);
            a.id
        }
    };

    // Step 2: Modify the message activity
    println!("Step 2: Modifying message activity...");
    let modify_options = SaveMessageActivityOptions {
        id: Some(activity_id),
        message: "Heya ThunderBlaze".to_string(),
        recipient_id: user_id,
        private: Some(true),
        locked: None,
        as_mod: None,
    };

    let modify_result = client.activity().save_message_activity(modify_options).await;
    match modify_result {
        Ok(response) => {
            info!("Modify Response: {:?}", response);
            println!("✓ Modified message activity successfully");
        }
        Err(e) => println!("✗ Failed to modify message activity: {:?}", e),
    }

    // Step 3: Fetch the message activity
    println!("Step 3: Fetching message activity...");
    let fetch_options = FetchActivityOneOptions { id: activity_id };

    let fetch_result = client.activity().fetch_one(fetch_options).await;
    match fetch_result {
        Ok(response) => {
            info!("Fetch Response: {:?}", response);
            match &response.data.activity {
                anilist_moe::unions::activity::ActivityUnion::MessageActivity(a) => {
                    println!(
                        "✓ Fetched message activity: {}",
                        a.message.as_ref().unwrap_or(&"".to_string())
                    );
                }
                anilist_moe::unions::activity::ActivityUnion::TextActivity(a) => {
                    println!("✗ Got TextActivity instead of MessageActivity: id={}", a.id);
                }
                anilist_moe::unions::activity::ActivityUnion::ListActivity(a) => {
                    println!("✗ Got ListActivity instead of MessageActivity: id={}", a.id);
                }
            }
        }
        Err(e) => println!("✗ Failed to fetch message activity: {:?}", e),
    }

    // Step 4: Delete the message activity
    println!("Step 4: Deleting message activity...");
    let delete_options = DeleteActivityOptions { id: activity_id };

    let delete_result = client.activity().delete(delete_options).await;
    match delete_result {
        Ok(response) => {
            info!("Delete Response: {:?}", response);
            println!("✓ Deleted message activity successfully");
        }
        Err(e) => println!("✗ Failed to delete message activity: {:?}", e),
    }

    println!("=== Message Activity Lifecycle Test Complete ===\n");
}

#[tokio::test]
async fn test_activity_reply() {
    let client = get_authenticated_client();

    println!("\n=== Testing Activity Reply ===");

    let activity_id = 964502724; // The activity ID to reply to

    // Step 1: Create a reply
    println!("Step 1: Creating reply...");
    let save_reply_options = SaveActivityReplyOptions {
        text: "Test reply from anilist_moe library".to_string(),
        activity_id,
        id: None,
    };

    let create_result = client.activity().save_reply(save_reply_options).await;
    if let Err(ref e) = create_result {
        println!("Failed to create reply: {:?}", e);
        return;
    }

    let create_response = create_result.unwrap();
    info!("Create Reply Response: {:?}", create_response);
    let reply_id = create_response.data.save_activity_reply.id;
    println!("✓ Created reply with ID: {}", reply_id);

    // Step 2: Modify the reply
    println!("Step 2: Modifying reply...");
    let modify_reply_options = SaveActivityReplyOptions {
        id: Some(reply_id),
        text: "Modified test reply from anilist_moe library".to_string(),
        activity_id,
    };

    let modify_result = client.activity().save_reply(modify_reply_options).await;
    match modify_result {
        Ok(response) => {
            info!("Modify Reply Response: {:?}", response);
            println!("✓ Modified reply successfully");
        }
        Err(e) => println!("✗ Failed to modify reply: {:?}", e),
    }

    // Step 3: Fetch replies to verify
    println!("Step 3: Fetching replies...");
    let fetch_replies_options = FetchActivityRepliesOptions {
        activity_id,
        page: Some(1),
        per_page: Some(10),
        id: None,
    };

    let fetch_result = client.activity().fetch_replies(fetch_replies_options).await;
    match fetch_result {
        Ok(response) => {
            info!("Fetch Replies Response: {:?}", response);
            let replies = &response.data.page.data.activity_replies;
            let found = replies.iter().find(|r| r.id == reply_id);
            if let Some(reply) = found {
                println!(
                    "✓ Found reply: {}",
                    reply.text.as_ref().unwrap_or(&"".to_string())
                );
            } else {
                println!("✗ Reply not found in activity");
            }
        }
        Err(e) => println!("✗ Failed to fetch replies: {:?}", e),
    }

    // Step 4: Delete the reply
    println!("Step 4: Deleting reply...");
    let delete_reply_options = DeleteActivityReplyOptions { id: reply_id };

    let delete_result = client.activity().delete_reply(delete_reply_options).await;
    match delete_result {
        Ok(response) => {
            info!("Delete Reply Response: {:?}", response);
            println!("✓ Deleted reply successfully");
        }
        Err(e) => println!("✗ Failed to delete reply: {:?}", e),
    }

    println!("=== Activity Reply Test Complete ===\n");
}
