use anilist_moe::client::AniListClient;
use anilist_moe::endpoints::character::FetchCharacterOptions;
use anilist_moe::endpoints::forum::FetchThreadOptions;
use anilist_moe::endpoints::staff::FetchStaffOptions;
use anilist_moe::endpoints::studio::FetchStudioOptions;
use anilist_moe::endpoints::user::FetchUserOptions;
use anilist_moe::enums::media::MediaSort;

mod support;

use support::allow_environmental_failure;

#[tokio::test]
async fn test_character_conditional_fetching() {
    let client = AniListClient::new();
    let options = FetchCharacterOptions::builder()
        .id(1)
        .include_media(true)
        .media_per_page(5)
        .media_sort(&[MediaSort::PopularityDesc])
        .include_mod_notes(false)
        .build();

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
        Err(e) => allow_environmental_failure("test_character_conditional_fetching", &e),
    }
}

#[tokio::test]
async fn test_staff_conditional_fetching() {
    let client = AniListClient::new();
    let options = FetchStaffOptions::builder()
        .id(95263) // Akira Toriyama
        .include_staff_media(true)
        .staff_media_per_page(5)
        .include_characters(true)
        .characters_per_page(5)
        .build();

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
        Err(e) => allow_environmental_failure("test_staff_conditional_fetching", &e),
    }
}

#[tokio::test]
async fn test_studio_conditional_fetching() {
    let client = AniListClient::new();
    let options = FetchStudioOptions::builder()
        .id(1)
        .include_media(true)
        .media_per_page(5)
        .build();

    let result = client.studio().fetch(&options).await;
    match result {
        Ok(page) => {
            if !page.data.is_empty() {
                let studio = &page.data[0];
                assert!(studio.media.is_some(), "Studio media should be present");
            }
        }
        Err(e) => allow_environmental_failure("test_studio_conditional_fetching", &e),
    }
}

#[tokio::test]
async fn test_user_conditional_fetching() {
    let client = AniListClient::new();
    let options = FetchUserOptions::builder()
        .name("Josh")
        .include_statistics(true)
        .include_favourites(true)
        .build();

    let result = client.user().fetch(&options).await;
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
        Err(e) => allow_environmental_failure("test_user_conditional_fetching", &e),
    }
}

#[tokio::test]
async fn test_forum_conditional_fetching() {
    let client = AniListClient::new();
    // Search for a random popular thread or just by generic search
    let options = FetchThreadOptions::builder()
        .search("Anime")
        .per_page(1)
        .include_body(true)
        .include_user(true)
        .build();

    let result = client.forum().fetch(&options).await;
    match result {
        Ok(page) => {
            if !page.data.is_empty() {
                let thread = &page.data[0];
                assert!(thread.body.is_some(), "Thread body should be present");
                assert!(thread.user.is_some(), "Thread user should be present");
            }
        }
        Err(e) => allow_environmental_failure("test_forum_conditional_fetching", &e),
    }
}

#[tokio::test]
async fn test_user_conditional_fetching_negative() {
    let client = AniListClient::new();
    let options = FetchUserOptions::builder()
        .name("Josh")
        .include_favourites(true)
        .include_statistics(true)
        .build();

    let result = client.user().fetch(&options).await;
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
        Err(e) => allow_environmental_failure("test_user_conditional_fetching_negative", &e),
    }
}
