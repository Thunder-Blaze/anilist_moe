//! Tests for Forum endpoint

use crate::test_harness::{delay_between_tests, get_authenticated_harness, TestHarness};
use anilist_moe::endpoints::forum::*;

fn harness() -> TestHarness {
    TestHarness::new()
}

#[tokio::test]
async fn test_fetch_forum_threads() {
    let h = harness();
    let client = h.client();

    let result = h
        .run(|| async {
            let options = FetchThreadOptions {
                per_page: Some(5),
                ..Default::default()
            };
            client.forum().fetch(&options).await
        })
        .await;

    assert!(
        result.is_ok(),
        "Should successfully fetch forum threads: {:?}",
        result.err()
    );

    let response = result.unwrap();
    let threads = &response.data;
    assert!(!threads.is_empty(), "Should return at least one thread");
}

#[tokio::test]
async fn test_fetch_one_forum_thread() {
    delay_between_tests().await;
    let h = harness();
    let client = h.client();

    // First fetch some threads to get a valid ID
    let list_result = h
        .run(|| async {
            let options = FetchThreadOptions {
                per_page: Some(1),
                ..Default::default()
            };
            client.forum().fetch(&options).await
        })
        .await;

    assert!(list_result.is_ok(), "Should successfully fetch threads");

    let response = list_result.unwrap();
    let threads = &response.data;
    if threads.is_empty() {
        println!("No threads found to test fetch_one");
        return;
    }

    delay_between_tests().await;

    let thread_id = threads[0].id;
    let result = h
        .run(|| async {
            let options = FetchThreadOneOptions {
                id: Some(thread_id),
                comments_page: None,
                comments_per_page: None,
                comments_sort: None,
                ..Default::default()
            };
            client.forum().fetch_one(&options).await
        })
        .await;

    assert!(
        result.is_ok(),
        "Should successfully fetch one thread: {:?}",
        result.err()
    );
}

#[tokio::test]
async fn test_fetch_forum_comments() {
    delay_between_tests().await;
    let h = harness();
    let client = h.client();

    // First fetch a thread to get a valid thread ID
    let list_result = h
        .run(|| async {
            let options = FetchThreadOptions {
                per_page: Some(1),
                ..Default::default()
            };
            client.forum().fetch(&options).await
        })
        .await;

    if let Ok(response) = list_result {
        let threads = &response.data;
        if !threads.is_empty() {
            delay_between_tests().await;

            let thread_id = threads[0].id;
            let result = h
                .run(|| async {
                    let options = FetchThreadCommentOptions {
                        thread_id: Some(thread_id),
                        per_page: Some(5),
                        ..Default::default()
                    };
                    client.forum().fetch_comments(&options).await
                })
                .await;

            assert!(
                result.is_ok(),
                "Should successfully fetch forum comments: {:?}",
                result.err()
            );
        }
    }
}

#[tokio::test]
async fn test_forum_data_types() {
    delay_between_tests().await;
    let h = harness();
    let client = h.client();

    let result = h
        .run(|| async {
            let options = FetchThreadOptions {
                per_page: Some(1),
                ..Default::default()
            };
            client.forum().fetch(&options).await
        })
        .await;

    assert!(result.is_ok(), "Should successfully fetch threads");

    let response = result.unwrap();
    let threads = &response.data;
    if !threads.is_empty() {
        let thread = &threads[0];
        assert!(thread.id > 0, "Thread ID should be positive");
        assert!(
            thread
                .title
                .as_ref()
                .map(|s| !s.is_empty())
                .unwrap_or(false),
            "Thread should have a title"
        );
    }
}

#[tokio::test]
async fn test_fetch_comment_one() {
    delay_between_tests().await;
    let h = harness();
    let client = h.client();

    // First fetch some comments to get a valid ID
    let list_result = h
        .run(|| async {
            let options = FetchThreadCommentOptions {
                per_page: Some(1),
                ..Default::default()
            };
            client.forum().fetch_comments(&options).await
        })
        .await;

    if let Ok(response) = list_result {
        let comments = &response.data;
        if !comments.is_empty() {
            delay_between_tests().await;

            let comment_id = comments[0].id;
            let result = h
                .run(|| async {
                    let options = FetchThreadCommentOneOptions {
                        id: Some(comment_id),
                        ..Default::default()
                    };
                    client.forum().fetch_comment_one(&options).await
                })
                .await;

            assert!(
                result.is_ok(),
                "Should successfully fetch one comment: {:?}",
                result.err()
            );
        }
    }
}

// Authentication required tests - these gracefully handle missing auth
#[tokio::test]
async fn test_save_forum_thread() {
    let Some(h) = get_authenticated_harness() else {
        eprintln!("Skipping test_save_forum_thread: ANILIST_TOKEN not set");
        return;
    };
    let client = h.client().clone();

    let result = h
        .run(|| async {
            let options = SaveThreadOptions {
                title: Some("Test Thread from anilist_moe".to_string()),
                body: Some("This is a test thread body.".to_string()),
                categories: Some(vec![1]),
                id: None,
                media_categories: None,
                sticky: None,
                locked: None,
                ..Default::default()
            };
            client.forum().save(&options).await
        })
        .await;

    match result {
        Ok(response) => {
            println!("Created thread with ID: {}", response.id);
            // Clean up - delete the thread
            delay_between_tests().await;
            let _ = h
                .run(|| async {
                    let delete_options = DeleteThreadOptions { id: response.id };
                    client.forum().delete(&delete_options).await
                })
                .await;
        }
        Err(e) => {
            println!("Expected authentication error or permission issue: {:?}", e);
        }
    }
}

#[tokio::test]
async fn test_toggle_thread_subscription() {
    let Some(h) = get_authenticated_harness() else {
        eprintln!("Skipping test_toggle_thread_subscription: ANILIST_TOKEN not set");
        return;
    };
    let client = h.client().clone();

    // First fetch a thread to subscribe to
    let list_result = h
        .run(|| async {
            let options = FetchThreadOptions {
                per_page: Some(1),
                ..Default::default()
            };
            client.forum().fetch(&options).await
        })
        .await;

    if let Ok(response) = list_result {
        let threads = &response.data;
        if !threads.is_empty() {
            let thread_id = threads[0].id;

            delay_between_tests().await;

            let result = h
                .run(|| async {
                    let options = ToggleThreadSubscriptionOptions {
                        thread_id,
                        subscribe: Some(true),
                    };
                    client.forum().subscription(&options).await
                })
                .await;

            match result {
                Ok(_) => {
                    println!("Successfully subscribed to thread");
                    // Unsubscribe
                    delay_between_tests().await;
                    let _ = h
                        .run(|| async {
                            let unsub_options = ToggleThreadSubscriptionOptions {
                                thread_id,
                                subscribe: Some(false),
                            };
                            client.forum().subscription(&unsub_options).await
                        })
                        .await;
                }
                Err(e) => {
                    println!("Expected authentication error or permission issue: {:?}", e);
                }
            }
        }
    }
}
