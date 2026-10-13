//! Example binary demonstrating a `media` endpoint request.
//!
//! Requires the `media` feature (enabled by the default `full` feature set).

#[cfg(feature = "media")]
use anilist_moe::{endpoints::media::FetchMediaOptions, AniListClient, AniListError};
use std::borrow::Cow;

#[cfg(feature = "media")]
#[tokio::main]
async fn main() -> Result<(), AniListError> {
    let client = AniListClient::new();

    // Example usage
    let popular_anime = client
        .media()
        .fetch(
            &FetchMediaOptions::builder()
                .sort(Some(Cow::Borrowed(&[
                    anilist_moe::enums::media::MediaSort::PopularityDesc,
                ])))
                .per_page(Some(5))
                .page(Some(1))
                .build(),
        )
        .await?;
    println!("Popular anime: {:#?}", popular_anime);

    Ok(())
}

/// Fallback entry point used when the `media` feature is disabled.
#[cfg(not(feature = "media"))]
fn main() {
    eprintln!("This example requires the `media` feature (enabled by default).");
}
