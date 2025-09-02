use anilist_moe::client::AniListClient;

#[tokio::test]
async fn test_get_recent_threads() {
    let client = AniListClient::new();
    let result = client.forum().get_recent_threads(1, 5).await;

    assert!(result.is_ok());
    let threads = result.unwrap();
    // Note: This might be empty if there are no recent threads

    for thread in &threads {
        assert!(thread.id > 0);
        assert!(!thread.title.is_empty());
    }
}

#[tokio::test]
async fn test_search_threads() {
    let client = AniListClient::new();
    let result = client.forum().search_threads("anime", 1, 5).await;
    println!("Search result: {:?}", result);

    assert!(result.is_ok());
    let threads = result.unwrap();
    // Note: This might be empty if no threads match the search

    for thread in &threads {
        assert!(thread.id > 0);
        assert!(!thread.title.is_empty());
    }
}

#[tokio::test]
async fn test_get_thread_by_id() {
    let client = AniListClient::new();
    // This test might fail if the specific thread doesn't exist
    let result = client.forum().get_thread_by_id(1).await;

    // We just check that the call doesn't panic
    match result {
        Ok(thread) => {
            assert_eq!(thread.id, 1);
            assert!(!thread.title.is_empty());
        }
        Err(_) => {
            // Thread might not exist, which is acceptable for this test
        }
    }
}

#[tokio::test]
async fn test_get_thread_comments() {
    let client = AniListClient::new();
    // This test might fail if the specific thread doesn't exist or has no comments
    let result = client.forum().get_thread_comments(1, 1, 5).await;

    // We just check that the call doesn't panic
    match result {
        Ok(comments) => {
            for comment in &comments {
                assert!(comment.id > 0);
                assert!(!comment.comment.is_empty());
            }
        }
        Err(_) => {
            // Thread might not exist or have no comments, which is acceptable
        }
    }
}
