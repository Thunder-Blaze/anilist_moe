use anilist_moe::AniListClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = AniListClient::new();

    // Example usage
    let popular_anime = client.anime().get_popular(1, 5).await?;
    println!("Popular anime: {:#?}", popular_anime);

    Ok(())
}
