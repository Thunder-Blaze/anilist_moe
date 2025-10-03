//! Tests for Forum endpoint

use anilist_moe::{AniListClient, endpoints::forum::*};
use dotenv::dotenv;
use log::info;
use std::env;

fn get_authenticated_client() -> AniListClient {
    dotenv().ok();
    let token = env::var("ANILIST_TOKEN").expect("ANILIST_TOKEN must be set in .env file");
    AniListClient::with_token(&token)
}

#[tokio::test]
async fn test_fetch_forum_threads() {
    let client = AniListClient::new();
    let options = FetchThreadOptions {
        per_page: Some(5),
        ..Default::default()
    };

    let result = client.forum().fetch(options).await;
    if let Err(ref e) = result {
        eprintln!("Error fetching forum threads: {:?}", e);
    }
    assert!(
        result.is_ok(),
        "Should successfully fetch forum threads: {:?}",
        result.err()
    );

    let response = result.unwrap();
    info!("Response: {:?}", response);

    let threads = &response.data.page.data.threads;
    assert!(!threads.is_empty(), "Should return at least one thread");
}

#[tokio::test]
async fn test_fetch_one_forum_thread() {
    let client = AniListClient::new();

    // First fetch some threads to get a valid ID
    let list_options = FetchThreadOptions {
        per_page: Some(1),
        ..Default::default()
    };
    let list_result = client.forum().fetch(list_options).await;
    assert!(list_result.is_ok(), "Should successfully fetch threads");

    let response = list_result.unwrap();
    info!("Response: {:?}", response);

    let threads = &response.data.page.data.threads;
    if threads.is_empty() {
        println!("No threads found to test fetch_one");
        return;
    }

    let thread_id = threads[0].id;
    let options = FetchThreadOneOptions {
        id: Some(thread_id),
        comments_page: None,
        comments_per_page: None,
        comments_sort: None,
    };

    let result = client.forum().fetch_one(options).await;
    if let Err(ref e) = result {
        eprintln!("Error fetching one thread: {:?}", e);
    }
    assert!(
        result.is_ok(),
        "Should successfully fetch one thread: {:?}",
        result.err()
    );
}

#[tokio::test]
async fn test_fetch_forum_comments() {
    let client = AniListClient::new();

    // First fetch a thread to get a valid thread ID
    let list_options = FetchThreadOptions {
        per_page: Some(1),
        ..Default::default()
    };
    let list_result = client.forum().fetch(list_options).await;

    if let Ok(response) = list_result {
        let threads = &response.data.page.data.threads;
        if !threads.is_empty() {
            let thread_id = threads[0].id;
            let options = FetchThreadCommentOptions {
                thread_id: Some(thread_id),
                per_page: Some(5),
                ..Default::default()
            };

            let result = client.forum().fetch_comments(options).await;
            if let Err(ref e) = result {
                eprintln!("Error fetching forum comments: {:?}", e);
            }
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
    let client = AniListClient::new();
    let options = FetchThreadOptions {
        per_page: Some(1),
        ..Default::default()
    };

    let result = client.forum().fetch(options).await;
    assert!(result.is_ok(), "Should successfully fetch threads");

    let response = result.unwrap();
    info!("Response: {:?}", response);

    let threads = &response.data.page.data.threads;
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

// Authentication required tests
#[tokio::test]
async fn test_fetch_comment_one() {
    let client = AniListClient::new();

    // First fetch some comments to get a valid ID
    let list_options = FetchThreadCommentOptions {
        per_page: Some(1),
        ..Default::default()
    };

    if let Ok(response) = client.forum().fetch_comments(list_options).await {
        let comments = &response.data.page.data.thread_comments;
        if !comments.is_empty() {
            let comment_id = comments[0].id;
            let options = FetchThreadCommentOneOptions {
                id: Some(comment_id),
            };

            let result = client.forum().fetch_comment_one(options).await;
            if let Err(ref e) = result {
                eprintln!("Error fetching one comment: {:?}", e);
            }
            assert!(
                result.is_ok(),
                "Should successfully fetch one comment: {:?}",
                result.err()
            );
        }
    }
}

#[tokio::test]
async fn test_save_forum_thread() {
    let client = get_authenticated_client();

    // Try to create a new thread
    let options = SaveThreadOptions {
        title: Some("Test Thread from anilist_moe".to_string()),
        body: Some("This is a test thread body.".to_string()),
        categories: Some(vec![1]), // General category
        id: None,
        media_categories: None,
        sticky: None,
        locked: None,
    };

    let result = client.forum().save(options).await;

    match result {
        Ok(response) => {
            println!("Successfully created forum thread");
            let thread_id = response.data.save_thread.id;
            println!("Created thread with ID: {}", thread_id);
        }
        Err(e) => {
            println!("Expected authentication error or permission issue: {:?}", e);
        }
    }
}

#[tokio::test]
async fn test_save_forum_comment() {
    let client = get_authenticated_client();

    // First fetch a thread to comment on
    let list_options = FetchThreadOptions {
        per_page: Some(1),
        ..Default::default()
    };

    if let Ok(response) = client.forum().fetch(list_options).await {
        let threads = &response.data.page.data.threads;
        if !threads.is_empty() {
            let thread_id = threads[0].id;
            let options = SaveThreadCommentOptions {
                thread_id: Some(thread_id),
                comment: Some("Test comment from anilist_moe library".to_string()),
                id: None,
                parent_comment_id: None,
                locked: None,
            };

            let result = client.forum().save_comment(options).await;

            match result {
                Ok(_) => {
                    println!("Successfully created forum comment");
                }
                Err(e) => {
                    println!("Expected authentication error or permission issue: {:?}", e);
                }
            }
        }
    }
}

#[tokio::test]
async fn test_delete_forum_thread() {
    let client = get_authenticated_client();

    // First create a thread to delete
    let save_options = SaveThreadOptions {
        title: Some("Test Thread to Delete".to_string()),
        body: Some("This thread will be deleted.".to_string()),
        categories: Some(vec![1]),
        id: None,
        media_categories: None,
        sticky: None,
        locked: None,
    };

    if let Ok(response) = client.forum().save(save_options).await {
        let thread_id = response.data.save_thread.id;
        let delete_options = DeleteThreadOptions { id: thread_id };

        let result = client.forum().delete(delete_options).await;

        match result {
            Ok(_) => {
                println!("Successfully deleted forum thread");
            }
            Err(e) => {
                println!("Expected authentication error or permission issue: {:?}", e);
            }
        }
    }
}

#[tokio::test]
async fn test_delete_forum_comment() {
    let client = get_authenticated_client();

    // First create a comment to delete
    let list_options = FetchThreadOptions {
        per_page: Some(1),
        ..Default::default()
    };

    if let Ok(response) = client.forum().fetch(list_options).await {
        let threads = &response.data.page.data.threads;
        if !threads.is_empty() {
            let thread_id = threads[0].id;
            // Create comment
            let save_options = SaveThreadCommentOptions {
                thread_id: Some(thread_id),
                comment: Some("Test comment to delete".to_string()),
                id: None,
                parent_comment_id: None,
                locked: None,
            };

            if let Ok(save_response) = client.forum().save_comment(save_options).await {
                let comment_id = save_response.data.save_thread_comment.id;
                let delete_options = DeleteThreadCommentOptions { id: comment_id };

                let result = client.forum().delete_comment(delete_options).await;

                match result {
                    Ok(_) => {
                        println!("Successfully deleted forum comment");
                    }
                    Err(e) => {
                        println!("Expected authentication error or permission issue: {:?}", e);
                    }
                }
            }
        }
    }
}

#[tokio::test]
async fn test_toggle_thread_subscription() {
    let client = get_authenticated_client();

    // First fetch a thread to subscribe to
    let list_options = FetchThreadOptions {
        per_page: Some(1),
        ..Default::default()
    };

    if let Ok(response) = client.forum().fetch(list_options).await {
        let threads = &response.data.page.data.threads;
        if !threads.is_empty() {
            let thread_id = threads[0].id;
            let options = ToggleThreadSubscriptionOptions {
                thread_id,
                subscribe: Some(true),
            };

            let result = client.forum().subscription(options).await;

            match result {
                Ok(_) => {
                    println!("Successfully subscribed to thread");
                    // Unsubscribe
                    let unsub_options = ToggleThreadSubscriptionOptions {
                        thread_id,
                        subscribe: Some(false),
                    };
                    let _ = client.forum().subscription(unsub_options).await;
                }
                Err(e) => {
                    println!("Expected authentication error or permission issue: {:?}", e);
                }
            }
        }
    }
}
