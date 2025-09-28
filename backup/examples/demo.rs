use anilist_moe::client::AniListClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = AniListClient::new();

    println!("🎌 AniList API Demo");
    println!("==================");

    // Popular Anime
    println!("\n📺 Popular Anime:");
    let popular_anime = client.anime().get_popular(1, 5).await?;
    for (i, anime) in popular_anime.iter().enumerate() {
        if let Some(title) = &anime.title {
            let unknown_title = "Unknown Title".to_string();
            let display_title = title
                .english
                .as_ref()
                .or(title.romaji.as_ref())
                .unwrap_or(&unknown_title);
            println!("{}. {} (ID: {})", i + 1, display_title, anime.id);
            if let Some(score) = anime.average_score {
                println!("   Score: {}/100", score);
            }
            if let Some(popularity) = anime.popularity {
                println!("   Popularity: {}", popularity);
            }
        }
    }

    // Popular Manga
    println!("\n📚 Popular Manga:");
    let popular_manga = client.manga().get_popular(1, 5).await?;
    for (i, manga) in popular_manga.iter().enumerate() {
        if let Some(title) = &manga.title {
            let unknown_title = "Unknown Title".to_string();
            let display_title = title
                .english
                .as_ref()
                .or(title.romaji.as_ref())
                .unwrap_or(&unknown_title);
            println!("{}. {} (ID: {})", i + 1, display_title, manga.id);
            if let Some(score) = manga.average_score {
                println!("   Score: {}/100", score);
            }
            if let Some(chapters) = manga.chapters {
                println!("   Chapters: {}", chapters);
            }
            if let Some(volumes) = manga.volumes {
                println!("   Volumes: {}", volumes);
            }
        }
    }

    // Popular Characters
    println!("\n👥 Popular Characters:");
    let popular_characters = client.character().get_popular(1, 5).await?;
    for (i, character) in popular_characters.iter().enumerate() {
        if let Some(name) = &character.name {
            let unknown_name = "Unknown Name".to_string();
            let display_name = name
                .full
                .as_ref()
                .or(name.user_preferred.as_ref())
                .unwrap_or(&unknown_name);
            println!("{}. {} (ID: {})", i + 1, display_name, character.id);
            if let Some(favourites) = character.favourites {
                println!("   Favourites: {}", favourites);
            }
        }
    }

    // Popular Staff
    println!("\n🎬 Popular Staff:");
    let popular_staff = client.staff().get_popular(1, 5).await?;
    for (i, staff) in popular_staff.iter().enumerate() {
        if let Some(name) = &staff.name {
            let unknown_name = "Unknown Name".to_string();
            let display_name = name
                .full
                .as_ref()
                .or(name.user_preferred.as_ref())
                .unwrap_or(&unknown_name);
            println!("{}. {} (ID: {})", i + 1, display_name, staff.id);
            if let Some(favourites) = staff.favourites {
                println!("   Favourites: {}", favourites);
            }
            if let Some(occupations) = &staff.primary_occupations {
                if !occupations.is_empty() {
                    println!("   Primary Occupation: {}", occupations[0]);
                }
            }
        }
    }

    // Search Example
    println!("\n🔍 Search Example - 'One Piece':");
    let search_results = client.anime().search("One Piece", 1, 3).await?;
    for (i, anime) in search_results.iter().enumerate() {
        if let Some(title) = &anime.title {
            let unknown_title = "Unknown Title".to_string();
            let display_title = title
                .english
                .as_ref()
                .or(title.romaji.as_ref())
                .unwrap_or(&unknown_title);
            println!("{}. {} (ID: {})", i + 1, display_title, anime.id);
            if let Some(episodes) = anime.episodes {
                println!("   Episodes: {}", episodes);
            }
            if let Some(status) = &anime.status {
                println!("   Status: {:?}", status);
            }
        }
    }

    // Seasonal Anime Example
    println!("\n🍂 Fall 2023 Anime:");
    let seasonal_anime = client.anime().get_by_season("FALL", 2023, 1, 3).await?;
    for (i, anime) in seasonal_anime.iter().enumerate() {
        if let Some(title) = &anime.title {
            let unknown_title = "Unknown Title".to_string();
            let display_title = title
                .english
                .as_ref()
                .or(title.romaji.as_ref())
                .unwrap_or(&unknown_title);
            println!("{}. {} (ID: {})", i + 1, display_title, anime.id);
            if let Some(season_year) = anime.season_year {
                if let Some(season) = &anime.season {
                    println!("   Season: {:?} {}", season, season_year);
                }
            }
        }
    }

    println!("\n✨ Demo completed successfully!");

    Ok(())
}
