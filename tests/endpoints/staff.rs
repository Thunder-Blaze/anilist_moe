//! Tests for Staff endpoint

use anilist_moe::{AniListClient, endpoints::staff::*};

#[tokio::test]
async fn test_fetch_staff_by_search() {
    let client = AniListClient::new();
    let options = FetchStaffOptions {
        search: Some("Hayao".to_string()),
        ..Default::default()
    };

    let result = client.staff().fetch(options).await;
    assert!(result.is_ok(), "Should successfully fetch staff");

    let response = result.unwrap();
    let staff_list = &response.data.page.data.staff;
    assert!(!staff_list.is_empty(), "Should return at least one staff member");

    let first_staff = &staff_list[0];
    assert!(first_staff.id > 0, "Staff should have a positive ID");
    assert!(first_staff.name.is_some(), "Staff should have a name");
}

#[tokio::test]
async fn test_fetch_staff_by_id() {
    let client = AniListClient::new();
    let options = FetchStaffOptions {
        id: Some(95269),
        ..Default::default()
    };

    let result = client.staff().fetch(options).await;
    assert!(result.is_ok(), "Should successfully fetch staff by ID");

    let response = result.unwrap();
    let staff_list = &response.data.page.data.staff;
    assert_eq!(staff_list.len(), 1, "Should return exactly one staff member");
    assert_eq!(staff_list[0].id, 95269, "Should return correct staff ID");
}

#[tokio::test]
async fn test_fetch_one_staff() {
    let client = AniListClient::new();
    let options = FetchStaffOneOptions {
        id: Some(95269),
        ..Default::default()
    };

    let result = client.staff().fetch_one(options).await;
    assert!(result.is_ok(), "Should successfully fetch one staff");

    let response = result.unwrap();
    let staff = &response.data.staff;
    assert_eq!(staff.id, 95269, "Should return correct staff ID");
}

#[tokio::test]
async fn test_staff_data_types() {
    let client = AniListClient::new();
    let options = FetchStaffOptions {
        id: Some(95269),
        ..Default::default()
    };

    let result = client.staff().fetch(options).await;
    assert!(result.is_ok(), "Should successfully fetch staff");

    let response = result.unwrap();
    let staff = &response.data.page.data.staff[0];

    assert!(staff.id > 0, "ID should be positive");

    if let Some(favourites) = staff.favourites {
        assert!(favourites >= 0, "Favourites should be non-negative");
    }

    if let Some(ref name) = staff.name {
        let has_name = name.first.is_some() || name.last.is_some() || name.full.is_some();
        assert!(has_name, "Name should have at least one field");
    }
}
