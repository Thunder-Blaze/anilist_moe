#[tokio::test]
async fn debug_recommendation_response() {
    use anilist_moe::{AniListClient, endpoints::recommendation::RecommendationSearchOptions};
    use serde_json;
    use std::collections::HashMap;

    let client = AniListClient::new();

    let options = RecommendationSearchOptions {
        page: Some(1),
        per_page: Some(3),
        ..Default::default()
    };

    let query = include_str!("../src/queries/recommendation/search_recommendations.graphql");
    let variables = serde_json::json!(options);
    let variables_map = match variables {
        serde_json::Value::Object(map) => map.into_iter().collect(),
        _ => HashMap::new(),
    };

    match client.query(query, Some(&variables_map)).await {
        Ok(response) => {
            println!("=== RAW API RESPONSE ===");
            println!("{}", serde_json::to_string_pretty(&response).unwrap());
            println!("========================");
        }
        Err(e) => {
            println!("API Error: {:?}", e);
        }
    }
}
