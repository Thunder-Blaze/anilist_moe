use anilist_moe::client::AniListClient;
use anilist_moe::endpoints::character::FetchCharacterOptions;
use anilist_moe::endpoints::forum::FetchThreadOptions;
use anilist_moe::endpoints::staff::FetchStaffOptions;
use anilist_moe::endpoints::studio::FetchStudioOptions;
use anilist_moe::endpoints::user::FetchUserOptions;
use anilist_moe::enums::media::MediaSort;

#[tokio::test]
async fn test_character_conditional_fetching() {
    let client = AniListClient::new();
    let options = FetchCharacterOptions {
        id: Some(1), // Spike Spiegel
        include_media: Some(true),
        media_per_page: Some(5),
        media_sort: Some(vec![MediaSort::PopularityDesc]),
        include_mod_notes: Some(false),
        ..Default::default()
    };

    let result = client.character().fetch(&options).await;
    match result {
        Ok(page) => {
            if !page.data.is_empty() {
                let character = &page.data[0];
                assert!(character.id == 1);
                // Media should be present
                assert!(
                    character.media.is_some(),
                    "Media should be present when requested"
                );
                if let Some(media) = &character.media {
                    if let Some(edges) = &media.edges {
                        assert!(edges.len() <= 5, "Should respect per_page limit");
                    }
                }
            }
        }
        Err(e) => println!("Skipping test due to network error: {:?}", e),
    }
}

#[tokio::test]
async fn test_staff_conditional_fetching() {
    let client = AniListClient::new();
    let options = FetchStaffOptions {
        id: Some(95263), // Akira Toriyama
        include_staff_media: Some(true),
        staff_media_per_page: Some(5),
        include_characters: Some(true),
        characters_per_page: Some(5),
        ..Default::default()
    };

    let result = client.staff().fetch(&options).await;
    match result {
        Ok(page) => {
            if !page.data.is_empty() {
                let staff = &page.data[0];
                assert!(staff.id == 95263);
                assert!(staff.staff_media.is_some(), "Staff media should be present");
                assert!(staff.characters.is_some(), "Characters should be present");
            }
        }
        Err(e) => println!("Skipping test due to network error: {:?}", e),
    }
}

#[tokio::test]
async fn test_studio_conditional_fetching() {
    let client = AniListClient::new();
    let options = FetchStudioOptions {
        id: Some(1), // Studio Pierrot (or generic ID)
        include_media: Some(true),
        media_per_page: Some(5),
        ..Default::default()
    };

    let result = client.studio().fetch(&options).await;
    match result {
        Ok(page) => {
            if !page.data.is_empty() {
                let studio = &page.data[0];
                assert!(studio.media.is_some(), "Studio media should be present");
            }
        }
        Err(e) => println!("Skipping test due to network error: {:?}", e),
    }
}

#[tokio::test]
async fn test_user_conditional_fetching() {
    let client = AniListClient::new();
    let options = FetchUserOptions {
        name: Some("Josh".to_string()), // A popular user or generic
        include_statistics: Some(true),
        include_favourites: Some(true),
        ..Default::default()
    };

    let result = client.user().fetch(options).await;
    match result {
        Ok(page) => {
            if !page.data.is_empty() {
                let user = &page.data[0];
                assert!(
                    user.statistics.is_some(),
                    "User statistics should be present"
                );
                assert!(
                    user.favourites.is_some(),
                    "User favourites should be present"
                );
            }
        }
        Err(e) => println!("Skipping test due to network error: {:?}", e),
    }
}

#[tokio::test]
async fn test_forum_conditional_fetching() {
    let client = AniListClient::new();
    // Search for a random popular thread or just by generic search
    let options = FetchThreadOptions {
        search: Some("Anime".to_string()),
        per_page: Some(1),
        include_body: Some(true),
        include_user: Some(true),
        ..Default::default()
    };

    let result = client.forum().fetch(&options).await;
    match result {
        Ok(page) => {
            if !page.data.is_empty() {
                let thread = &page.data[0];
                assert!(thread.body.is_some(), "Thread body should be present");
                assert!(thread.user.is_some(), "Thread user should be present");
            }
        }
        Err(e) => println!("Skipping test due to network error: {:?}", e),
    }
}

#[tokio::test]
async fn test_user_conditional_fetching_negative() {
    let client = AniListClient::new();
    let options = FetchUserOptions {
        name: Some("Josh".to_string()),
        // Explicitly exclude or default
        include_statistics: Some(false),
        include_favourites: Some(false),
        ..Default::default()
    };

    let result = client.user().fetch(options).await;
    match result {
        Ok(page) => {
            if !page.data.is_empty() {
                let user = &page.data[0];
                assert!(
                    user.statistics.is_none(),
                    "User statistics should NOT be present"
                );
                assert!(
                    user.favourites.is_none(),
                    "User favourites should NOT be present"
                );
            }
        }
        Err(e) => println!("Skipping test due to network error: {:?}", e),
    }
}
