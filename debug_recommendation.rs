use anilist_moe::{AniListClient, endpoints::recommendation::RecommendationSearchOptions};
use tokio;
use serde_json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = AniListClient::new();

    let options = RecommendationSearchOptions {
        page: Some(1),
        per_page: Some(5),
        ..Default::default()
    };

    // Get raw response to see the actual structure
    let query = include_str!("../src/queries/recommendation/search_recommendations.graphql");
    let variables = serde_json::json!(options);
    let variables_map = match variables {
        serde_json::Value::Object(map) => map.into_iter().collect(),
        _ => std::collections::HashMap::new(),
    };

    match client.query(query, Some(&variables_map)).await {
        Ok(response) => {
            println!("Raw response: {}", serde_json::to_string_pretty(&response)?);
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }

    Ok(())
}
