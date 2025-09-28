use anilist_moe::AniListClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = AniListClient::new();

    // Example usage
    let popular_anime = client.media().get_popular_anime(Some(1), Some(5)).await?;
    println!("Popular anime: {:#?}", popular_anime);

    Ok(())
}
