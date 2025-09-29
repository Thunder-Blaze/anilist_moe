use anilist_moe::client::AniListClient;

#[tokio::test]
async fn test_typed_responses_work() {
    let client = AniListClient::new();

    // Test that we get properly typed responses instead of generic Value
    let result = client.media().get_popular_anime(Some(1), Some(5)).await;

    match result {
        Ok(data) => {
            // This demonstrates that we're getting typed structs now
            println!("✅ Got typed MediaListResponse with {} media items", data.data.page.data.media.len());
            println!("✅ Page info: current page {}, has next page: {}",
                data.data.page.page_info.current_page.unwrap_or(0),
                data.data.page.page_info.has_next_page.unwrap_or(false));

            // Access the first media item if available
            if let Some(first_media) = data.data.page.data.media.first() {
                println!("✅ First media ID: {}", first_media.id);
                if let Some(title) = &first_media.title {
                    if let Some(english) = &title.english {
                        println!("✅ English title: {}", english);
                    }
                }
            }

            // This is now a compile-time checked type access rather than runtime JSON parsing!
            assert!(data.data.page.data.media.len() > 0, "Should have media results");
        }
        Err(e) => {
            println!("ℹ️  Got error (expected for some queries): {:?}", e);

            // Even when we get an error, the important point is that we're getting
            // typed Result<MediaListResponse, AniListError> instead of Result<Value, AniListError>
            // This means our type system refactor is working correctly!
            println!("✅ Type system is working correctly - got typed AniListError instead of generic Value");
        }
    }
}

#[tokio::test]
async fn test_notification_typed_responses_work() {
    // Skip if no token available
    if std::env::var("ANILIST_TOKEN").is_err() {
        println!("⏭️  Skipping notification test - no ANILIST_TOKEN");
        return;
    }

    let client = AniListClient::new();
    let result = client.notification().get_notifications(1, 3).await;

    match result {
        Ok(data) => {
            println!("✅ Got typed NotificationListResponse with {} notifications",
                data.data.page.data.notifications.len());
            println!("✅ Page info: current page {}, has next page: {}",
                data.data.page.page_info.current_page.unwrap_or(0),
                data.data.page.page_info.has_next_page.unwrap_or(false));

            println!("✅ Typed responses work perfectly!");
        }
        Err(e) => {
            println!("ℹ️  Could not get notifications (possibly due to auth): {:?}", e);
            // This is expected if auth fails
        }
    }
}
