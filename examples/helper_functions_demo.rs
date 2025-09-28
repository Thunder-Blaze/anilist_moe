use anilist_moe::*;
use anilist_moe::enums::media::*;
use anilist_moe::enums::activity::*;
use anilist_moe::enums::user::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize client (unauthenticated)
    let client = AniListClient::new();

    // Example 1: Search for popular anime with helper functions
    println!("🔍 Searching for popular anime...");

    let popular_anime = client.search_media()
        .media_type(Some(MediaType::Anime))
        .sort(Some(vec![MediaSort::PopularityDesc]))
        .format(Some(vec![MediaFormat::Tv, MediaFormat::Movie]))
        .average_score_greater(Some(75))
        .page(Some(1))
        .per_page(Some(10))
        .extended(Some(false)) // Basic data only
        .build();

    println!("Generated variables: {}", serde_json::to_string_pretty(&popular_anime)?);

    // Example 2: Advanced media search with extended data
    println!("\n🎯 Advanced search with extended data...");

    let advanced_search = MediaSearchBuilder::new()
        .search(Some("Attack on Titan".to_string()))
        .media_type(Some(MediaType::Anime))
        .genre(Some(vec!["Action".to_string(), "Drama".to_string()]))
        .season_year(Some(2013))
        .average_score_greater(Some(85))
        .extended(Some(true)) // Full data with characters, staff, reviews, etc.
        .build();

    println!("Extended search variables: {}", serde_json::to_string_pretty(&advanced_search)?);

    // Example 3: Activity search
    println!("\n💬 Activity search...");

    let activity_search = ActivitySearchBuilder::new()
        .activity_type(Some(ActivityType::Text))
        .has_replies(Some(true))
        .is_following(Some(true))
        .page(Some(1))
        .per_page(Some(25))
        .build();

    println!("Activity variables: {}", serde_json::to_string_pretty(&activity_search)?);

    // Example 4: User search
    println!("\n👤 User search...");

    let user_search = UserSearchBuilder::new()
        .search(Some("anime".to_string()))
        .is_moderator(Some(false))
        .sort(Some(vec![UserSort::WatchedTimeDesc]))
        .page(Some(1))
        .per_page(Some(20))
        .build();

    println!("User search variables: {}", serde_json::to_string_pretty(&user_search)?);

    // Example 5: Mutation builder for saving media list entry
    println!("\n💾 Media list entry mutation...");

    let list_entry = MutationBuilder::new()
        .media_id(Some(12345))
        .status(Some("COMPLETED".to_string()))
        .score_raw(Some(85))
        .progress(Some(12))
        .notes(Some("Amazing anime!".to_string()))
        .private(Some(false))
        .custom_lists(Some(vec!["Favorites".to_string(), "Must Watch".to_string()]))
        .build();

    println!("List entry variables: {}", serde_json::to_string_pretty(&list_entry)?);

    // Example 6: Query utils demonstration
    println!("\n🛠️ Query utilities...");

    // Create request body
    let query = "query SearchMedia($search: String, $type: MediaType) { Page { media(search: $search, type: $type) { id title { romaji } } } }";
    let variables = json!({
        "search": "Naruto",
        "type": "ANIME",
        "emptyField": null,
        "emptyString": "",
        "emptyArray": []
    });

    let cleaned_vars = QueryUtils::clean_variables(variables);
    let request_body = QueryUtils::create_request_body(query, cleaned_vars);

    println!("Cleaned request body: {}", serde_json::to_string_pretty(&request_body)?);

    // Example 7: Query presets
    println!("\n⚡ Query presets...");

    let trending = QueryPresets::trending_anime();
    let top_rated = QueryPresets::top_rated_anime();
    let popular_manga = QueryPresets::popular_manga();

    println!("Trending anime preset: {}", serde_json::to_string_pretty(&trending)?);
    println!("Top rated preset: {}", serde_json::to_string_pretty(&top_rated)?);
    println!("Popular manga preset: {}", serde_json::to_string_pretty(&popular_manga)?);

    // Example 8: Demonstrate null filtering
    println!("\n🚫 Null filtering demonstration...");

    let search_with_nulls = MediaSearchBuilder::new()
        .search(None) // This will be filtered out
        .media_type(Some(MediaType::Anime)) // This will be kept
        .format(Some(vec![])) // Empty vec will be filtered out
        .genre(Some(vec!["Action".to_string()])) // This will be kept
        .season_year(None) // This will be filtered out
        .extended(Some(true)) // This will be kept
        .build();

    println!("Filtered variables (nulls removed): {}", serde_json::to_string_pretty(&search_with_nulls)?);

    println!("\n✅ All examples completed successfully!");
    println!("\n📝 Key features demonstrated:");
    println!("  • Fluent API builders for all query types");
    println!("  • Automatic null/None value filtering");
    println!("  • Extended vs basic data fetching");
    println!("  • Query utility functions");
    println!("  • Preset configurations");
    println!("  • Clean request body generation");

    Ok(())
}

// Additional helper functions for common use cases
pub fn search_anime_by_title(title: &str, extended: bool) -> serde_json::Value {
    MediaSearchBuilder::new()
        .search(Some(title.to_string()))
        .media_type(Some(MediaType::Anime))
        .sort(Some(vec![MediaSort::SearchMatch]))
        .extended(Some(extended))
        .page(Some(1))
        .per_page(Some(25))
        .build()
}

pub fn search_manga_by_genre(genres: Vec<String>, min_score: Option<i32>) -> serde_json::Value {
    MediaSearchBuilder::new()
        .media_type(Some(MediaType::Manga))
        .genre(Some(genres))
        .average_score_greater(min_score)
        .sort(Some(vec![MediaSort::ScoreDesc, MediaSort::PopularityDesc]))
        .page(Some(1))
        .per_page(Some(50))
        .build()
}

pub fn get_seasonal_anime(year: i32, season: MediaSeason, page: Option<i32>) -> serde_json::Value {
    MediaSearchBuilder::new()
        .media_type(Some(MediaType::Anime))
        .season_year(Some(year))
        .season(Some(season))
        .sort(Some(vec![MediaSort::PopularityDesc]))
        .page(page.or(Some(1)))
        .per_page(Some(25))
        .build()
}

pub fn create_list_entry(media_id: i32, status: &str, score: Option<i32>, progress: Option<i32>) -> serde_json::Value {
    MutationBuilder::new()
        .media_id(Some(media_id))
        .status(Some(status.to_string()))
        .score_raw(score)
        .progress(progress)
        .build()
}
