//! Tests for Forum endpoint

use anilist_moe::{AniListClient, endpoints::forum::*};
use dotenv::dotenv;
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
    assert!(result.is_ok(), "Should successfully fetch forum threads: {:?}", result.err());

    let response = result.unwrap();
    if let Some(page) = response.get("data").and_then(|d| d.get("Page")) {
        if let Some(threads) = page.get("threads").and_then(|t| t.as_array()) {
            assert!(!threads.is_empty(), "Should return at least one thread");
        }
    }
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
    if let Some(page) = response.get("data").and_then(|d| d.get("Page")) {
        if let Some(threads) = page.get("threads").and_then(|t| t.as_array()) {
            if threads.is_empty() {
                println!("No threads found to test fetch_one");
                return;
            }

            if let Some(thread_id) = threads[0].get("id").and_then(|id| id.as_i64()) {
                let options = FetchThreadOneOptions {
                    id: Some(thread_id as i32),
                    comments_page: None,
                    comments_per_page: None,
                    comments_sort: None,
                };

                let result = client.forum().fetch_one(options).await;
                if let Err(ref e) = result {
                    eprintln!("Error fetching one thread: {:?}", e);
                }
                assert!(result.is_ok(), "Should successfully fetch one thread: {:?}", result.err());
            }
        }
    }
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
        if let Some(page) = response.get("data").and_then(|d| d.get("Page")) {
            if let Some(threads) = page.get("threads").and_then(|t| t.as_array()) {
                if !threads.is_empty() {
                    if let Some(thread_id) = threads[0].get("id").and_then(|id| id.as_i64()) {
                        let options = FetchThreadCommentOptions {
                            thread_id: Some(thread_id as i32),
                            per_page: Some(5),
                            ..Default::default()
                        };

                        let result = client.forum().fetch_comments(options).await;
                        if let Err(ref e) = result {
                            eprintln!("Error fetching forum comments: {:?}", e);
                        }
                        assert!(result.is_ok(), "Should successfully fetch forum comments: {:?}", result.err());
                    }
                }
            }
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
    if let Some(page) = response.get("data").and_then(|d| d.get("Page")) {
        if let Some(threads) = page.get("threads").and_then(|t| t.as_array()) {
            if !threads.is_empty() {
                let thread = &threads[0];
                assert!(thread.get("id").and_then(|id| id.as_i64()).unwrap_or(0) > 0, "Thread ID should be positive");
                assert!(thread.get("title").and_then(|t| t.as_str()).map(|s| !s.is_empty()).unwrap_or(false), "Thread should have a title");
            }
        }
    }
}
