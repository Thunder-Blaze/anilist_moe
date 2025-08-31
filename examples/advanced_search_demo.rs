use anilist_moe::client::AniListClient;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = AniListClient::new();

    println!("🔍 AniList Advanced Search Demo\n");

    // Simple search (existing functionality)
    println!("📺 Simple Anime Search: 'Attack on Titan'");
    let simple_results = client.anime().search("Attack on Titan", 1, 3).await?;
    for anime in &simple_results {
        if let Some(title) = &anime.title {
            println!("  - {}", title.romaji.as_ref().unwrap_or(&"N/A".to_string()));
        }
    }

    // Advanced anime search with filters
    println!("\n📺 Advanced Anime Search: Action anime from 2023");
    let advanced_anime = client.anime()
        .search_anime(
            None,                     // No search term
            1,                        // Page 1
            5,                        // 5 results per page
            Some("Action"),           // Action genre
            Some(2023),              // Year 2023
            Some("FALL"),            // Fall season
            Some("TV")               // TV format
        ).await?;
    
    for anime in &advanced_anime {
        if let Some(title) = &anime.title {
            let format_str = anime.format.as_ref().map_or("N/A".to_string(), |f| format!("{:?}", f));
            println!("  - {} ({})", 
                title.romaji.as_ref().unwrap_or(&"N/A".to_string()),
                format_str
            );
        }
    }

    // Simple manga search
    println!("\n📚 Simple Manga Search: 'One Piece'");
    let simple_manga = client.manga().search("One Piece", 1, 3).await?;
    for manga in &simple_manga {
        if let Some(title) = &manga.title {
            println!("  - {}", title.romaji.as_ref().unwrap_or(&"N/A".to_string()));
        }
    }

    // Advanced manga search with filters
    println!("\n📚 Advanced Manga Search: Fantasy manga");
    let advanced_manga = client.manga()
        .search_manga(
            Some("fantasy"),          // Search term
            1,                        // Page 1
            3,                        // 3 results per page
            Some("Fantasy"),          // Fantasy genre
            None,                     // No year filter
            Some("MANGA")            // Manga format
        ).await?;
    
    for manga in &advanced_manga {
        if let Some(title) = &manga.title {
            let status_str = manga.status.as_ref().map_or("N/A".to_string(), |s| format!("{:?}", s));
            println!("  - {} ({})", 
                title.romaji.as_ref().unwrap_or(&"N/A".to_string()),
                status_str
            );
        }
    }

    println!("\n✨ Demo completed! Both simple and advanced search methods are available.");
    
    Ok(())
}
