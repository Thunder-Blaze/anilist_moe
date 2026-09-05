use crate::enums::studio::StudioSort;
use crate::errors::AniListError;
use crate::objects::responses::Page;
use crate::objects::studio::Studio;
use crate::{client::AniListClient, queries::studio};
use serde::Serialize;
use serde_with::skip_serializing_none;

/// Options for fetching studios.
#[derive(Default, Debug, Serialize)]
pub struct FetchStudioOptions<'a> {
    pub page: Option<i32>,
    #[serde(rename = "perPage")]
    pub per_page: Option<i32>,
    pub id: Option<i32>,
    pub search: Option<&'a str>,
    #[serde(rename = "id_not")]
    pub id_not: Option<i32>,
    #[serde(rename = "id_in")]
    pub id_in: Option<&'a [i32]>,
    #[serde(rename = "id_not_in")]
    pub id_not_in: Option<&'a [i32]>,
    pub sort: Option<&'a [StudioSort]>,
    // Extra
    #[serde(rename = "includeMedia")]
    pub include_media: Option<bool>,
    // Sub-pagination variables
    #[serde(rename = "mediaPage")]
    pub media_page: Option<i32>,
    #[serde(rename = "mediaPerPage")]
    pub media_per_page: Option<i32>,
}

/// Options for fetching a single studio by ID.
#[skip_serializing_none]
#[derive(Default, Debug, Serialize)]
pub struct FetchStudioOneOptions<'a> {
    pub id: Option<i32>,
    pub search: Option<&'a str>,
    #[serde(rename = "id_not")]
    pub id_not: Option<i32>,
    #[serde(rename = "id_in")]
    pub id_in: Option<&'a [i32]>,
    #[serde(rename = "id_not_in")]
    pub id_not_in: Option<&'a [i32]>,
    pub sort: Option<&'a [StudioSort]>,
    // Sub-pagination variables
    #[serde(rename = "mediaPage")]
    pub media_page: Option<i32>,
    #[serde(rename = "mediaPerPage")]
    pub media_per_page: Option<i32>,
}

/// Endpoint for studio operations.
pub struct StudioEndpoint<'a> {
    client: &'a AniListClient,
}

impl<'a> StudioEndpoint<'a> {
    pub fn new(client: &'a AniListClient) -> Self {
        Self { client }
    }

    pub async fn fetch(
        &self,
        options: FetchStudioOptions<'_>,
    ) -> Result<Page<Vec<Studio>>, AniListError> {
        let query = studio::FETCH;
        self.client.fetch(query, Some(&options)).await
    }

    pub async fn fetch_one(
        &self,
        options: FetchStudioOneOptions<'_>,
    ) -> Result<Studio, AniListError> {
        let query = studio::FETCH_ONE;
        self.client.fetch(query, Some(&options)).await
    }

    // Convenience functions

    /// Get popular studios sorted by favorites
    pub async fn get_popular(
        &self,
        page: Option<i32>,
        per_page: Option<i32>,
    ) -> Result<Page<Vec<Studio>>, AniListError> {
        self.fetch(FetchStudioOptions {
            page,
            per_page,
            sort: Some(&[StudioSort::FavouritesDesc]),
            ..Default::default()
        })
        .await
    }

    /// Search studios by name
    pub async fn search(
        &self,
        query: &str,
        page: Option<i32>,
        per_page: Option<i32>,
    ) -> Result<Page<Vec<Studio>>, AniListError> {
        self.fetch(FetchStudioOptions {
            search: Some(query),
            page,
            per_page,
            sort: Some(&[StudioSort::SearchMatch]),
            ..Default::default()
        })
        .await
    }

    /// Get studio by ID
    pub async fn get_by_id(&self, id: i32) -> Result<Studio, AniListError> {
        self.fetch_one(FetchStudioOneOptions {
            id: Some(id),
            ..Default::default()
        })
        .await
    }
}
