use anilist_moe::{
    client::AniListClient,
    enums::media::{MediaFormat, MediaSort, MediaType, MediaSeason},
    helpers::query_builders::{QueryBuilder, QueryType},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize client (unauthenticated)
    let _client = AniListClient::new();

    // Example 1: Search for popular anime with helper functions
    println!("🔍 Searching for popular anime...");

    let popular_anime = QueryBuilder::new(QueryType::MediaSearch)
        .media_type(Some(MediaType::Anime))
        .sort_media(Some(vec![MediaSort::PopularityDesc]))
        .format(Some(vec![MediaFormat::Tv, MediaFormat::Movie]))
        .average_score_greater(Some(75))
        .page(Some(1))
        .per_page(Some(10))
        .extended(Some(false)) // Basic data only
        .build();

    println!(
        "Generated variables: {}",
        serde_json::to_string_pretty(&popular_anime)?
    );

    // Example 2: Advanced media search with extended data
    println!("\n🎯 Advanced search with extended data...");

    let advanced_search = QueryBuilder::new(QueryType::MediaSearch)
        .search(Some("Attack on Titan".to_string()))
        .media_type(Some(MediaType::Anime))
        .genre(Some(vec!["Action".to_string(), "Drama".to_string()]))
        .season_year(Some(2013))
        .average_score_greater(Some(85))
        .extended(Some(true)) // Full data with characters, staff, reviews, etc.
        .build();

    println!(
        "Extended search variables: {}",
        serde_json::to_string_pretty(&advanced_search)?
    );

    // NOTE: The following examples are commented out as they rely on features
    // (ActivitySearchBuilder, UserSearchBuilder, MutationBuilder, QueryUtils, QueryPresets)
    // that are not currently implemented in this refactored version of the library.

    // Example 3: Activity search
    // println!("\n� Activity search...");
    // ...

    // Example 4: User search
    // println!("\n👤 User search...");
    // ...

    // Example 5: Mutation builder for saving media list entry
    // println!("\n💾 Media list entry mutation...");
    // ...

    // Example 6: Query utils demonstration
    // println!("\n🛠️ Query utilities...");
    // ...

    // Example 7: Query presets
    // println!("\n⚡ Query presets...");
    // ...

    // Example 8: Demonstrate null filtering
    println!("\n🚫 Null filtering demonstration...");

    let search_with_nulls = QueryBuilder::new(QueryType::MediaSearch)
        .search(None) // This will be filtered out
        .media_type(Some(MediaType::Anime)) // This will be kept
        .format(Some(vec![])) // Empty vec will be filtered out
        .genre(Some(vec!["Action".to_string()])) // This will be kept
        .season_year(None) // This will be filtered out
        .extended(Some(true)) // This will be kept
        .build();

    println!(
        "Filtered variables (nulls removed): {}",
        serde_json::to_string_pretty(&search_with_nulls)?
    );

    println!("\n✅ All examples completed successfully!");
    println!("\n📝 Key features demonstrated:");
    println!("  • Fluent API builders for all query types");
    println!("  • Automatic null/None value filtering");
    println!("  • Extended vs basic data fetching");

    Ok(())
}

// Additional helper functions for common use cases
pub fn search_anime_by_title(title: &str, extended: bool) -> serde_json::Value {
    QueryBuilder::new(QueryType::MediaSearch)
        .search(Some(title.to_string()))
        .media_type(Some(MediaType::Anime))
        .sort_media(Some(vec![MediaSort::SearchMatch]))
        .extended(Some(extended))
        .page(Some(1))
        .per_page(Some(25))
        .build()
}

pub fn search_manga_by_genre(genres: Vec<String>, min_score: Option<i32>) -> serde_json::Value {
    QueryBuilder::new(QueryType::MediaSearch)
        .media_type(Some(MediaType::Manga))
        .genre(Some(genres))
        .average_score_greater(min_score)
        .sort_media(Some(vec![MediaSort::ScoreDesc, MediaSort::PopularityDesc]))
        .page(Some(1))
        .per_page(Some(50))
        .build()
}

pub fn get_seasonal_anime(year: i32, season: MediaSeason, page: Option<i32>) -> serde_json::Value {
    QueryBuilder::new(QueryType::MediaSearch)
        .media_type(Some(MediaType::Anime))
        .season_year(Some(year))
        .season(Some(season))
        .sort_media(Some(vec![MediaSort::PopularityDesc]))
        .page(page.or(Some(1)))
        .per_page(Some(25))
        .build()
}
