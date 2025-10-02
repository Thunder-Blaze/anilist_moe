//! Tests for User endpoint

use anilist_moe::{AniListClient, endpoints::user::*};
use log::info;

#[tokio::test]
async fn test_fetch_user_by_search() {
    let client = AniListClient::new();
    let options = FetchUserOptions {
        search: Some("ThunderBlaze".to_string()),
        per_page: Some(5),
        ..Default::default()
    };

    let result = client.user().fetch(options).await;
    if let Err(ref e) = result {
        eprintln!("Error fetching users: {:?}", e);
    }
    assert!(
        result.is_ok(),
        "Should successfully fetch users: {:?}",
        result.err()
    );

    let response = result.unwrap();
    info!("Response: {:?}", response);
    let users = &response.data.page.data.users;
    assert!(!users.is_empty(), "Should return at least one user");

    let first_user = &users[0];
    assert!(first_user.id > 0, "User should have a positive ID");
    assert!(
        !first_user.name.as_ref().unwrap().is_empty(),
        "User should have a name"
    );
}

#[tokio::test]
async fn test_fetch_user_by_id() {
    let client = AniListClient::new();
    let options = FetchUserOptions {
        id: Some(5429396),
        ..Default::default()
    };

    let result = client.user().fetch(options).await;
    assert!(result.is_ok(), "Should successfully fetch user by ID");

    let response = result.unwrap();
    info!("Response: {:?}", response);
    let users = &response.data.page.data.users;
    assert_eq!(users.len(), 1, "Should return exactly one user");
}

#[tokio::test]
async fn test_fetch_one_user() {
    let client = AniListClient::new();
    let options = FetchUserOneOptions {
        id: Some(5429396),
        ..Default::default()
    };

    let result = client.user().fetch_one(options).await;
    assert!(result.is_ok(), "Should successfully fetch one user");

    let response = result.unwrap();
    info!("Response: {:?}", response);
    let user = &response.data.user;
    assert!(user.id > 0, "User should have positive ID");
    assert!(
        !user.name.as_ref().unwrap().is_empty(),
        "User should have name"
    );
}

#[tokio::test]
async fn test_user_data_types() {
    let client = AniListClient::new();
    let options = FetchUserOptions {
        id: Some(3225),
        ..Default::default()
    };

    let result = client.user().fetch(options).await;
    assert!(result.is_ok(), "Should successfully fetch user");

    let response = result.unwrap();
    info!("Response: {:?}", response);
    let user = &response.data.page.data.users[0];

    assert!(
        !user.name.as_ref().unwrap().is_empty(),
        "Name should not be empty"
    );

    if let Some(ref avatar) = user.avatar {
        assert!(
            avatar.large.is_some() || avatar.medium.is_some(),
            "Avatar should have at least one size"
        );
    }
}
