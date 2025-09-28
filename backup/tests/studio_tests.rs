use anilist_moe::client::AniListClient;

#[tokio::test]
async fn test_get_popular_studios() {
    let client = AniListClient::new();
    let result = client.studio().get_popular(1, 5).await;

    assert!(result.is_ok());
    let studios = result.unwrap();
    assert!(!studios.is_empty());
    assert!(studios.len() <= 5);

    for studio in &studios {
        assert!(studio.id > 0);
        assert!(!studio.name.is_empty());
    }
}

#[tokio::test]
async fn test_get_studio_by_id() {
    let client = AniListClient::new();
    // Using Studio Ghibli's ID (21)
    let result = client.studio().get_by_id(21).await;

    assert!(result.is_ok());
    let studio = result.unwrap();
    assert_eq!(studio.id, 21);
    assert!(!studio.name.is_empty());
}

#[tokio::test]
async fn test_search_studios() {
    let client = AniListClient::new();
    let result = client.studio().search("Ghibli", 1, 5).await;

    assert!(result.is_ok());
    let studios = result.unwrap();
    assert!(!studios.is_empty());

    // Check that results contain "Ghibli" in some form
    let has_ghibli = studios
        .iter()
        .any(|studio| studio.name.to_lowercase().contains("ghibli"));
    assert!(has_ghibli);
}

#[tokio::test]
async fn test_get_most_favorited_studios() {
    let client = AniListClient::new();
    let result = client.studio().get_most_favorited(1, 5).await;

    assert!(result.is_ok());
    let studios = result.unwrap();
    assert!(!studios.is_empty());

    // Check that studios are ordered by favorites (descending)
    let mut prev_favorites = i32::MAX;
    for studio in &studios {
        assert!(studio.id > 0);
        if let Some(favourites) = studio.favourites {
            assert!(favourites <= prev_favorites);
            prev_favorites = favourites;
        }
    }
}
