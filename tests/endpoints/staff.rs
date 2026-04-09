//! Tests for Staff endpoint

use crate::test_harness::{delay_between_tests, TestHarness};
use anilist_moe::endpoints::staff::*;

fn harness() -> TestHarness {
    TestHarness::new()
}

#[tokio::test]
async fn test_fetch_staff_by_search() {
    let h = harness();
    let client = h.client();

    let result = h
        .run(|| async {
            let options = FetchStaffOptions {
                search: Some("Hayao".to_string()),
                ..Default::default()
            };
            client.staff().fetch(&options).await
        })
        .await;

    assert!(
        result.is_ok(),
        "Should successfully fetch staff: {:?}",
        result.err()
    );

    let response = result.unwrap();
    let staff_list = &response.data;
    assert!(
        !staff_list.is_empty(),
        "Should return at least one staff member"
    );

    let first_staff = &staff_list[0];
    assert!(first_staff.id > 0, "Staff should have a positive ID");
    assert!(first_staff.name.is_some(), "Staff should have a name");

    // Verify it's likely Hayao Miyazaki or similar
    if let Some(ref name) = first_staff.name {
        println!("Found staff: {:?}", name.full);
    }
}

#[tokio::test]
async fn test_fetch_staff_by_id() {
    delay_between_tests().await;
    let h = harness();
    let client = h.client();

    let result = h
        .run(|| async {
            let options = FetchStaffOptions {
                id: Some(95269), // Hayao Miyazaki
                ..Default::default()
            };
            client.staff().fetch(&options).await
        })
        .await;

    assert!(
        result.is_ok(),
        "Should successfully fetch staff by ID: {:?}",
        result.err()
    );

    let response = result.unwrap();
    let staff_list = &response.data;
    assert_eq!(
        staff_list.len(),
        1,
        "Should return exactly one staff member"
    );
    assert_eq!(staff_list[0].id, 95269, "Should return correct staff ID");
}

#[tokio::test]
async fn test_fetch_one_staff() {
    delay_between_tests().await;
    let h = harness();
    let client = h.client();

    let result = h
        .run(|| async {
            let options = FetchStaffOneOptions {
                id: Some(95269),
                ..Default::default()
            };
            client.staff().fetch_one(&options).await
        })
        .await;

    assert!(
        result.is_ok(),
        "Should successfully fetch one staff: {:?}",
        result.err()
    );

    let staff = result.unwrap();
    assert_eq!(staff.id, 95269, "Should return correct staff ID");
    assert!(staff.name.is_some(), "Staff should have a name");
}

#[tokio::test]
async fn test_staff_data_types() {
    delay_between_tests().await;
    let h = harness();
    let client = h.client();

    let result = h
        .run(|| async {
            let options = FetchStaffOptions {
                id: Some(95269),
                ..Default::default()
            };
            client.staff().fetch(&options).await
        })
        .await;

    assert!(result.is_ok(), "Should successfully fetch staff");

    let response = result.unwrap();
    let staff = &response.data[0];

    // Verify required fields
    assert!(staff.id > 0, "ID should be positive");

    // Verify optional numeric fields
    if let Some(favourites) = staff.favourites {
        assert!(favourites >= 0, "Favourites should be non-negative");
    }

    // Verify name structure
    if let Some(ref name) = staff.name {
        let has_name = name.first.is_some() || name.last.is_some() || name.full.is_some();
        assert!(has_name, "Name should have at least one field");
    }

    // Staff may have description
    if let Some(ref description) = staff.description {
        assert!(
            !description.is_empty(),
            "Description should not be empty if present"
        );
    }
}

#[tokio::test]
async fn test_staff_with_popularity() {
    delay_between_tests().await;
    let h = harness();
    let client = h.client();

    // Fetch popular staff (voice actors are often popular)
    let result = h
        .run(|| async {
            let options = FetchStaffOptions {
                search: Some("Kamiya".to_string()), // Common voice actor name
                per_page: Some(5),
                ..Default::default()
            };
            client.staff().fetch(&options).await
        })
        .await;

    assert!(result.is_ok(), "Should successfully fetch staff");

    let response = result.unwrap();
    assert!(!response.data.is_empty(), "Should return results");

    for staff in &response.data {
        assert!(staff.id > 0, "Staff ID should be positive");
        println!(
            "Staff: {:?} (ID: {})",
            staff.name.as_ref().and_then(|n| n.full.as_ref()),
            staff.id
        );
    }
}
