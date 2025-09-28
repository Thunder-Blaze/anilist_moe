use anilist_moe::{
    client::AniListClient,
    endpoints::staff::StaffSearchOptions,
    enums::staff::StaffSort,
};
use chrono::prelude::*;
use tokio::time::{Duration, sleep};

async fn rate_limit() {
    sleep(Duration::from_secs(1)).await;
}

#[tokio::test]
async fn test_get_popular_staff() {
    let client = AniListClient::new();
    let options = StaffSearchOptions {
        page: Some(1),
        per_page: Some(5),
        sort: Some(vec![StaffSort::FavouritesDesc]),
        ..Default::default()
    };
    let result = client.staff().search_staff(options).await;

    assert!(result.is_ok());
    let staff_list = result.unwrap();
    let media = staff_list
        .get("data")
        .and_then(|d| d.get("Page"))
        .and_then(|p| p.get("staff"))
        .and_then(|m| m.as_array())
        .unwrap();
    assert!(!media.is_empty());
    assert!(media.len() <= 5);

    // Check that all staff have required fields
    for staff in media {
        assert!(staff.get("id").and_then(|id| id.as_i64()).is_some());
        assert!(staff.get("name").is_some());
    }

    rate_limit().await;
}

#[tokio::test]
async fn test_get_staff_by_id() {
    rate_limit().await;

    let client = AniListClient::new();
    // Using Ikue Ootani's ID (95128)
    let result = client.staff().get_staff_by_id(95128).await;

    if let Err(e) = &result {
        println!("Error in test_get_staff_by_id: {:?}", e);
    }
    assert!(result.is_ok());
    let staff = result.unwrap();
    assert_eq!(
        staff
            .get("data")
            .and_then(|d| d.get("Page"))
            .and_then(|p| p.get("staff"))
            .and_then(|s| s.as_array())
            .and_then(|s| s.get(0))
            .and_then(|s| s.get("id"))
            .and_then(|id| id.as_i64()),
        Some(95128)
    );

    rate_limit().await;
}

#[tokio::test]
async fn test_search_staff() {
    rate_limit().await;

    let client = AniListClient::new();
    let options = StaffSearchOptions {
        search: Some("Miyazaki".to_string()),
        page: Some(1),
        per_page: Some(5),
        ..Default::default()
    };
    let result = client.staff().search_staff(options).await;

    assert!(result.is_ok());
    let staff_list = result.unwrap();
    let media = staff_list
        .get("data")
        .and_then(|d| d.get("Page"))
        .and_then(|p| p.get("staff"))
        .and_then(|m| m.as_array())
        .unwrap();
    assert!(!media.is_empty());

    // Check that results contain "Miyazaki" in some form
    let has_miyazaki = media.iter().any(|staff| {
        if let Some(name) = staff.get("name") {
            name.get("full")
                .and_then(|n| n.as_str())
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
    let options = StaffSearchOptions {
        is_birthday: Some(true),
        page: Some(1),
        per_page: Some(10),
        sort: Some(vec![StaffSort::Id]),
        ..Default::default()
    };
    let result = client.staff().search_staff(options).await;

    assert!(result.is_ok());
    let staff_list = result.unwrap();
    if let Some(staff_list) = staff_list
        .get("data")
        .and_then(|d| d.get("Page"))
        .and_then(|p| p.get("staff"))
        .and_then(|s| s.as_array())
    {
        for staff in staff_list {
            if let Some(birth_date) = staff.get("dateOfBirth") {
                if birth_date.get("day") == Some(&serde_json::Value::from(day))
                    && birth_date.get("month") == Some(&serde_json::Value::from(month))
                {
                    // Found a staff with today's birthday
                    return;
                }
            }
        }
    }

    rate_limit().await;
}

#[tokio::test]
async fn test_get_most_favorited_staff() {
    rate_limit().await;

    let client = AniListClient::new();
    let options = StaffSearchOptions {
        page: Some(1),
        per_page: Some(5),
        sort: Some(vec![StaffSort::FavouritesDesc]),
        ..Default::default()
    };
    let result = client.staff().search_staff(options).await;

    assert!(result.is_ok());
    let staff_list = result.unwrap();
    let media = staff_list
        .get("data")
        .and_then(|d| d.get("Page"))
        .and_then(|p| p.get("staff"))
        .and_then(|m| m.as_array())
        .unwrap();
    assert!(!media.is_empty());

    // Check that staff are ordered by favorites (descending)
    let mut prev_favorites = i32::MAX;
    for staff in media {
        assert!(staff.get("id").and_then(|id| id.as_i64()).is_some());
        if let Some(favourites) = staff.get("favourites").and_then(|f| f.as_i64()) {
            assert!(favourites <= prev_favorites as i64);
            prev_favorites = favourites as i32;
        }
    }

    rate_limit().await;
}
