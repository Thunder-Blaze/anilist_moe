use anilist_moe::{endpoints::media::FetchMediaOptions, AniListClient, AniListError};

#[tokio::main]
async fn main() -> Result<(), AniListError> {
    let client = AniListClient::new();

    // Example usage
    let popular_anime = client
        .media()
        .fetch(&FetchMediaOptions {
            sort: Some(vec![anilist_moe::enums::media::MediaSort::PopularityDesc]),
            per_page: Some(5),
            page: Some(1),
            ..Default::default()
        })
        .await?;
    println!("Popular anime: {:#?}", popular_anime);

    Ok(())
}
