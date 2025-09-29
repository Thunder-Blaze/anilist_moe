use anilist_moe::AniListClient;
use serde_json::Value;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = AniListClient::new();
    
    // Make a raw query to see the actual response structure
    let query = include_str!("src/queries/activity/search_activities.graphql");
    let variables = std::collections::HashMap::from([
        ("page".to_string(), serde_json::Value::from(1)),
        ("perPage".to_string(), serde_json::Value::from(2)),
    ]);
    
    match client.query_raw(query, Some(&variables)).await {
        Ok(response) => {
            println!("Raw response: {}", serde_json::to_string_pretty(&response)?);
        }
        Err(e) => {
            println!("Error: {:?}", e);
        }
    }
    
    Ok(())
}