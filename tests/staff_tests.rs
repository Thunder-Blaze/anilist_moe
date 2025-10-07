use anilist_moe::client::AniListClient;
use chrono::prelude::*;
use tokio::time::{Duration, sleep};

/// Helper function to add rate limiting between test requests
async fn rate_limit() {
    sleep(Duration::from_secs(1)).await;
}

#[tokio::test]
async fn test_get_popular_staff() {
    let client = AniListClient::new();
    let result = client.staff().get_popular(1, 5).await;

    assert!(result.is_ok());
    let staff_list = result.unwrap();
    assert!(!staff_list.is_empty());
    assert!(staff_list.len() <= 5);

    // Check that all staff have required fields
    for staff in &staff_list {
        assert!(staff.id > 0);
        assert!(staff.name.is_some());
    }

    rate_limit().await;
}

#[tokio::test]
async fn test_get_staff_by_id() {
    rate_limit().await;

    let client = AniListClient::new();
    // Using Ikue Ootani's ID (95128)
    let result = client.staff().get_by_id(95128).await;

    assert!(result.is_ok());
    let staff = result.unwrap();
    assert_eq!(staff.id, 95128);
    assert!(staff.name.is_some());

    rate_limit().await;
}

#[tokio::test]
async fn test_search_staff() {
    rate_limit().await;

    let client = AniListClient::new();
    let result = client.staff().search("Miyazaki", 1, 5).await;

    assert!(result.is_ok());
    let staff_list = result.unwrap();
    assert!(!staff_list.is_empty());

    // Check that results contain "Miyazaki" in some form
    let has_miyazaki = staff_list.iter().any(|staff| {
        if let Some(name) = &staff.name {
            name.full
                .as_ref()
                .map_or(false, |n| n.to_lowercase().contains("miyazaki"))
                || name
                    .last
                    .as_ref()
                    .map_or(false, |n| n.to_lowercase().contains("miyazaki"))
        } else {
            false
        }
    });
    assert!(has_miyazaki);

    rate_limit().await;
}

#[tokio::test]
async fn test_get_staff_today_birthday() {
    rate_limit().await;

    let client = AniListClient::new();
    let today = Local::now().date_naive();
    let day = today.day() as i32;
    let month = today.month() as i32;
    let result = client.staff().get_today_birthday(1, 10).await;

    assert!(result.is_ok());
    let staff_list = result.unwrap();
    // Note: This might be empty if no staff have this birthday

    for staff in &staff_list {
        assert!(staff.id > 0);
        if let Some(birth_date) = &staff.date_of_birth {
            assert_eq!(birth_date.month, Some(month));
            assert_eq!(birth_date.day, Some(day));
        }
    }

    rate_limit().await;
}

#[tokio::test]
async fn test_get_most_favorited_staff() {
    rate_limit().await;

    let client = AniListClient::new();
    let result = client.staff().get_most_favorited(1, 5).await;

    assert!(result.is_ok());
    let staff_list = result.unwrap();
    assert!(!staff_list.is_empty());

    // Check that staff are ordered by favorites (descending)
    let mut prev_favorites = i32::MAX;
    for staff in &staff_list {
        assert!(staff.id > 0);
        if let Some(favourites) = staff.favourites {
            assert!(favourites <= prev_favorites);
            prev_favorites = favourites;
        }
    }

    rate_limit().await;
}
