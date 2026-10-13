use anilist_moe::client::AniListClient;
use anilist_moe::endpoints::media::FetchMediaOptions;
use anilist_moe::enums::media::MediaType;
use anilist_moe::enums::staff::StaffLanguage;

#[tokio::test]
async fn test_conditional_fetching() {
    let client = AniListClient::new();
    let options = FetchMediaOptions::builder()
        .media_type(MediaType::Anime)
        .page(1)
        .per_page(1)
        .include_tags(true)
        .include_characters(true)
        .include_staff(true)
        .include_studios(true)
        .include_reviews(true)
        .include_recommendations(true)
        .include_external_links(true)
        .include_streaming_episodes(true)
        .include_rankings(true)
        .include_stats(true)
        .include_next_airing_episode(true)
        .include_media_list_entry_details(true)
        .include_start_date(true)
        .include_end_date(true)
        .include_duration(true)
        .include_volumes(true)
        .include_country_of_origin(true)
        .include_is_licensed(true)
        .include_source(true)
        .include_hashtag(true)
        .include_trailer(true)
        .include_updated_at(true)
        .include_cover_image_extra_large(true)
        .include_mean_score(true)
        .include_is_locked(true)
        .include_trending(true)
        .include_auto_create_forum_thread(true)
        .include_is_recommendation_blocked(true)
        .include_is_review_blocked(true)
        .include_mod_notes(true)
        .include_relations(true)
        .characters_per_page(5)
        .staff_per_page(5)
        .reviews_per_page(5)
        .recommendations_per_page(5)
        .voice_actor_language(StaffLanguage::Japanese)
        .build();

    let result = client.anime().fetch(&options).await;
    assert!(result.is_ok());

    let page = result.unwrap();
    assert!(!page.data.is_empty());

    let media = &page.data[0];

    // Check if tags are present
    assert!(media.tags.is_some());

    // Check if characters are present
    assert!(media.characters.is_some());
    if let Some(characters) = &media.characters {
        if let Some(edges) = &characters.edges {
            if !edges.is_empty() {
                assert!(edges.len() <= 5);
            }
        }
    }

    // Assert other fields are present (using assertions, not just printing)
    // Note: Some fields might be legally None even if requested (e.g. if the data doesn't exist for that anime),
    // but verifying the structure is present is key. For fields that are almost always present for popular anime:

    assert!(media.start_date.is_some());
    assert!(media.cover_image.is_some());
    if let Some(_cover) = &media.cover_image {
        // extraLarge might be None if not available, but the field logic is active
        // Ideally we check if it was *attempted* to be fetched.
        // For a popular anime like Cowboy Bebop (ID 1), it should be there.
        // But we rely on the fact that if we didn't request it, it would definitely be None.
    }

    assert!(media.staff.is_some());
    assert!(media.studios_full.is_some()); // fullStudios mapped to studios_full
    assert!(media.reviews.is_some());
    assert!(media.recommendations.is_some());
    assert!(media.external_links.is_some());
    assert!(media.streaming_episodes.is_some());
    assert!(media.rankings.is_some());
    assert!(media.stats.is_some());
    assert!(media.relations.is_some());

    // Output the struct for manual check
    // println!("{:#?}", media);
}

#[tokio::test]
async fn test_conditional_fetching_without_includes() {
    let client = AniListClient::new();
    let options = FetchMediaOptions::builder()
        .media_type(MediaType::Anime)
        .page(1)
        .per_page(1)
        .include_tags(false)
        .include_characters(false)
        .build();

    let result = client.anime().fetch(&options).await;
    assert!(result.is_ok());

    let page = result.unwrap();
    assert!(!page.data.is_empty());

    let media = &page.data[0];

    // Check that tags are NOT present (or empty/None depending on how the API behaves, usually None for omitted fields)
    assert!(media.tags.is_none());

    // Check that characters are NOT present
    assert!(media.characters.is_none());
}
