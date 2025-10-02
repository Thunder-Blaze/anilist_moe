use anilist_moe::{AniListClient, endpoints::media::FetchMediaOptions};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = AniListClient::new();

    // Example usage
    let popular_anime = client
        .media()
        .fetch(FetchMediaOptions {
            sort: Some(vec![anilist_moe::enums::media::MediaSort::PopularityDesc]),
            per_page: Some(5),
            page: Some(1),
            ..Default::default()
        })
        .await?;
    println!("Popular anime: {:#?}", popular_anime);

    Ok(())
}
