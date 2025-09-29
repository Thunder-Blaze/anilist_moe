use anilist_moe::{AniListClient, enums::activity::{ActivityType, ActivitySort}, endpoints::activity::ActivitySearchOptions};
use tokio::time::{Duration, sleep};
use std::env;
use dotenv::dotenv;

async fn rate_limit() {
    sleep(Duration::from_secs(1)).await;
}

#[tokio::test]
async fn test_get_global_activities() {
    let client = AniListClient::new();
    let result = client.activity().get_global_activities(1, 5).await;

    println!("Result: {:?}", result);
    assert!(result.is_ok());

    rate_limit().await;
}

#[tokio::test]
async fn test_get_user_activities() {
    rate_limit().await;

    let client = AniListClient::new();
    // Using a known user ID
    let result = client.activity().get_user_activities(1, 1, 3).await;

    println!("Result: {:?}", result);
    assert!(result.is_ok());

    rate_limit().await;
}

#[tokio::test]
async fn test_get_activities_by_type() {
    rate_limit().await;

    let client = AniListClient::new();
    let result = client.activity().get_by_type(ActivityType::Text, 1, 3).await;

    println!("Result: {:?}", result);
    assert!(result.is_ok());

    rate_limit().await;
}

#[tokio::test]
async fn test_get_activity_by_id() {
    rate_limit().await;

    let client = AniListClient::new();
    let result = client.activity().get_by_id(1).await;

    println!("Result: {:?}", result);
    // This test might fail if the specific activity doesn't exist, which is acceptable
    rate_limit().await;
}

#[tokio::test]
async fn test_search_activities() {
    rate_limit().await;

    let client = AniListClient::new();
    let options = ActivitySearchOptions {
        page: Some(1),
        per_page: Some(3),
        activity_type: Some(ActivityType::Text),
        sort: Some(vec![ActivitySort::IdDesc]),
        ..Default::default()
    };

    let result = client.activity().search(options).await;

    println!("Result: {:?}", result);
    assert!(result.is_ok());

    rate_limit().await;
}

#[tokio::test]
async fn fetch_list_activity() {
    rate_limit().await;
    dotenv().ok();

    let token = env::var("ANILIST_TOKEN").expect("ANILIST_TOKEN must be set");
    let client = AniListClient::with_token(&token);

    let result = client.activity().get_by_id(962366160).await;

    println!("Result: {:#?}", result);
    assert!(result.is_ok());

    rate_limit().await;
}
