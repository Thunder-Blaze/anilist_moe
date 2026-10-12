//! Tests for Activity endpoint

use crate::support::allow_environmental_failure;
use crate::test_harness::{delay_between_tests, TestHarness};
use anilist_moe::{endpoints::activity::*, unions::activity::ActivityUnion};
use dotenv::dotenv;
use std::env;

fn harness() -> TestHarness {
    TestHarness::new()
}

fn authenticated_harness() -> Option<TestHarness> {
    dotenv().ok();
    env::var("ANILIST_TOKEN")
        .ok()
        .map(|token| TestHarness::with_token(&token))
}

#[tokio::test]
async fn test_fetch_activities() {
    let h = harness();
    let client = h.client();

    let result = h
        .run(|| async {
            let options = FetchActivityOptions::builder().per_page(Some(5)).build();
            client.activity().fetch(&options).await
        })
        .await;

    assert!(
        result.is_ok(),
        "Should successfully fetch activities: {:?}",
        result.err()
    );

    let response = result.unwrap();
    let activities = &response.data;

    // Activities feed should return results
    assert!(activities.len() <= 5, "Should respect per_page limit");
    println!("Fetched {} activities", activities.len());
}

#[tokio::test]
async fn test_fetch_activities_by_user() {
    delay_between_tests().await;
    let h = harness();
    let client = h.client();

    let result = h
        .run(|| async {
            let options = FetchActivityOptions::builder()
                .user_id(Some(5429396))
                .per_page(Some(5))
                .build();
            client.activity().fetch(&options).await
        })
        .await;

    if let Ok(response) = result {
        let activities = &response.data;
        println!("Fetched {} activities for user", activities.len());

        // Verify all activities have valid IDs
        for activity in activities {
            match activity {
                ActivityUnion::TextActivity(a) => {
                    assert!(
                        a.id > 0 && a.user.as_ref().map(|u| u.id).is_some(),
                        "Text activity should have valid ID"
                    );
                }
                ActivityUnion::ListActivity(a) => {
                    assert!(
                        a.id > 0 && a.user.as_ref().map(|u| u.id).is_some(),
                        "List activity should have valid ID"
                    );
                }
                ActivityUnion::MessageActivity(a) => {
                    assert!(a.id > 0, "Message activity should have valid ID");
                }
                other => {
                    panic!(
                        "the API returned an activity type this crate does not know yet: {other:?}"
                    );
                }
            }
        }
    }
}

#[tokio::test]
async fn test_text_activity_full_lifecycle() {
    let Some(h) = authenticated_harness() else {
        eprintln!("Skipping test_text_activity_full_lifecycle: ANILIST_TOKEN not set");
        return;
    };
    let client = h.client().clone();

    println!("\n=== Testing Activity Full Lifecycle ===");

    // Step 1: Create a text activity
    println!("Step 1: Creating text activity...");
    let create_result = h
        .run(|| async {
            let save_options = SaveTextActivityOptions::builder()
                .text("Test activity from anilist_moe library - full lifecycle test".into())
                .build();
            client.activity().save_text_activity(&save_options).await
        })
        .await;

    let activity_id = match create_result {
        Ok(ActivityUnion::TextActivity(a)) => {
            println!("Created activity with ID: {}", a.id);
            a.id
        }
        Ok(_) => {
            println!("Unexpected activity type returned");
            return;
        }
        Err(e) => {
            allow_environmental_failure("creating a text activity", &e);
            return;
        }
    };

    delay_between_tests().await;

    // Step 2: Modify the activity
    println!("Step 2: Modifying activity...");
    let modify_result = h
        .run(|| async {
            let modify_options = SaveTextActivityOptions::builder()
                .id(Some(activity_id))
                .text("Modified test activity from anilist_moe library".into())
                .build();
            client.activity().save_text_activity(&modify_options).await
        })
        .await;

    match modify_result {
        Ok(_) => println!("Modified activity successfully"),
        Err(e) => allow_environmental_failure("updating a text activity", &e),
    }

    delay_between_tests().await;

    // Step 3: Fetch the activity
    println!("Step 3: Fetching activity...");
    let fetch_result = h
        .run(|| async {
            let fetch_options = FetchActivityOneOptions::builder().id(activity_id).build();
            client.activity().fetch_one(&fetch_options).await
        })
        .await;

    match fetch_result {
        Ok(ActivityUnion::TextActivity(a)) => {
            assert_eq!(a.id, activity_id, "Should fetch correct activity");
            println!("Fetched activity: {:?}", a.text);
        }
        Ok(_) => println!("Got unexpected activity type"),
        Err(e) => allow_environmental_failure("fetching a text activity", &e),
    }

    delay_between_tests().await;

    // Step 4: Delete the activity
    println!("Step 4: Deleting activity...");
    let delete_result = h
        .run(|| async {
            let delete_options = DeleteActivityOptions::builder().id(activity_id).build();
            client.activity().delete(&delete_options).await
        })
        .await;

    match delete_result {
        Ok(deleted) => {
            assert!(deleted, "Activity should be marked as deleted");
            println!("Deleted activity successfully");
        }
        Err(e) => allow_environmental_failure("deleting a text activity", &e),
    }

    println!("=== Activity Lifecycle Test Complete ===\n");
}

#[tokio::test]
async fn test_message_activity_full_lifecycle() {
    let Some(h) = authenticated_harness() else {
        eprintln!("Skipping test_message_activity_full_lifecycle: ANILIST_TOKEN not set");
        return;
    };
    let client = h.client().clone();

    println!("\n=== Testing Message Activity Full Lifecycle ===");

    let user_id = 5429396; // Your user ID - sending message to self

    // Step 1: Create a message activity
    println!("Step 1: Creating message activity...");
    let create_result = h
        .run(|| async {
            let save_options = SaveMessageActivityOptions::builder()
                .message("Heya - test from anilist_moe".into())
                .recipient_id(user_id)
                .private(Some(true))
                .build();
            client.activity().save_message_activity(&save_options).await
        })
        .await;

    let activity_id = match create_result {
        Ok(ActivityUnion::MessageActivity(a)) => {
            println!("Created message activity with ID: {}", a.id);
            a.id
        }
        Ok(_) => {
            println!("Unexpected activity type");
            return;
        }
        Err(e) => {
            allow_environmental_failure("creating a message activity", &e);
            return;
        }
    };

    delay_between_tests().await;

    // Step 2: Modify the message activity
    println!("Step 2: Modifying message activity...");
    let modify_result = h
        .run(|| async {
            let modify_options = SaveMessageActivityOptions::builder()
                .id(Some(activity_id))
                .message("Modified message from anilist_moe".into())
                .recipient_id(user_id)
                .private(Some(true))
                .build();

            client
                .activity()
                .save_message_activity(&modify_options)
                .await
        })
        .await;

    match modify_result {
        Ok(_) => println!("Modified message activity successfully"),
        Err(e) => allow_environmental_failure("updating a message activity", &e),
    }

    delay_between_tests().await;

    // Step 3: Delete the message activity
    println!("Step 3: Deleting message activity...");
    let delete_result = h
        .run(|| async {
            let delete_options = DeleteActivityOptions::builder().id(activity_id).build();
            client.activity().delete(&delete_options).await
        })
        .await;

    match delete_result {
        Ok(deleted) => {
            assert!(deleted, "Message activity should be marked as deleted");
            println!("Deleted message activity successfully");
        }
        Err(e) => allow_environmental_failure("deleting a message activity", &e),
    }

    println!("=== Message Activity Lifecycle Test Complete ===\n");
}

#[tokio::test]
async fn test_activity_reply() {
    let Some(h) = authenticated_harness() else {
        eprintln!("Skipping test_activity_reply: ANILIST_TOKEN not set");
        return;
    };
    let client = h.client().clone();

    println!("\n=== Testing Activity Reply ===");

    // First, create a text activity to reply to
    let create_result = h
        .run(|| async {
            let save_options = SaveTextActivityOptions::builder()
                .text("Test activity for reply test".into())
                .build();
            client.activity().save_text_activity(&save_options).await
        })
        .await;

    let activity_id = match create_result {
        Ok(ActivityUnion::TextActivity(a)) => a.id,
        _ => {
            println!("Failed to create activity for reply test");
            return;
        }
    };

    delay_between_tests().await;

    // Step 1: Create a reply
    println!("Step 1: Creating reply...");
    let reply_result = h
        .run(|| async {
            let save_reply_options = SaveActivityReplyOptions::builder()
                .text("Test reply from anilist_moe library".into())
                .activity_id(activity_id)
                .build();
            client.activity().save_reply(&save_reply_options).await
        })
        .await;

    let reply_id = match reply_result {
        Ok(reply) => {
            println!("Created reply with ID: {}", reply.id);
            reply.id
        }
        Err(e) => {
            allow_environmental_failure("creating an activity reply", &e);
            // Clean up the activity
            let _ = client
                .activity()
                .delete(&DeleteActivityOptions::builder().id(activity_id).build())
                .await;
            return;
        }
    };

    delay_between_tests().await;

    // Step 2: Fetch the reply on its own, by id
    println!("Step 2: Fetching the reply by id...");
    let single = h
        .run(|| async {
            let options = FetchActivityReplyOptions::builder()
                .id(Some(reply_id))
                .build();
            client.activity().fetch_reply(&options).await
        })
        .await;

    assert!(
        single.is_ok(),
        "Should fetch the reply by id: {:?}",
        single.err()
    );
    assert_eq!(
        single.unwrap().id,
        reply_id,
        "the reply fetched by id should be the one just created"
    );

    delay_between_tests().await;

    // Step 3: Fetch replies to verify
    println!("Step 2: Fetching replies...");
    let fetch_result = h
        .run(|| async {
            let fetch_options = FetchActivityRepliesOptions::builder()
                .activity_id(activity_id)
                .page(Some(1))
                .per_page(Some(10))
                .build();
            client.activity().fetch_replies(&fetch_options).await
        })
        .await;

    match fetch_result {
        Ok(response) => {
            let found = response.data.iter().any(|r| r.id == reply_id);
            assert!(found, "Reply should be found in activity replies");
            println!("Found reply in activity");
        }
        Err(e) => allow_environmental_failure("fetching activity replies", &e),
    }

    delay_between_tests().await;

    // Step 3: Delete the reply
    println!("Step 4: Deleting reply...");
    let delete_reply_result = h
        .run(|| async {
            let delete_options = DeleteActivityReplyOptions::builder().id(reply_id).build();
            client.activity().delete_reply(&delete_options).await
        })
        .await;

    match delete_reply_result {
        Ok(deleted) => {
            assert!(deleted, "Reply should be marked as deleted");
            println!("Deleted reply successfully");
        }
        Err(e) => allow_environmental_failure("deleting an activity reply", &e),
    }

    delay_between_tests().await;

    // Clean up: delete the activity
    let _ = h
        .run(|| async {
            client
                .activity()
                .delete(&DeleteActivityOptions::builder().id(activity_id).build())
                .await
        })
        .await;

    println!("=== Activity Reply Test Complete ===\n");
}
