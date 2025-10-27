use crate::enums::studio::StudioSort;
use crate::errors::AniListError;
use crate::objects::responses::{GraphQLResponse, Page};
use crate::objects::studio::Studio;
use crate::{client::AniListClient, queries::studio};
use serde::{Deserialize, Serialize};
use serde_json::json;
use serde_with::skip_serializing_none;

/// Options for fetching studios.
#[skip_serializing_none]
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct FetchStudioOptions {
    pub page: Option<i32>,
    #[serde(rename = "perPage")]
    pub per_page: Option<i32>,
    pub id: Option<i32>,
    pub search: Option<String>,
    #[serde(rename = "id_not")]
    pub id_not: Option<i32>,
    #[serde(rename = "id_in")]
    pub id_in: Option<Vec<i32>>,
    #[serde(rename = "id_not_in")]
    pub id_not_in: Option<Vec<i32>>,
    pub sort: Option<Vec<StudioSort>>,
}

/// Options for fetching a single studio by ID.
#[skip_serializing_none]
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct FetchStudioOneOptions {
    pub id: Option<i32>,
    pub search: Option<String>,
    #[serde(rename = "id_not")]
    pub id_not: Option<i32>,
    #[serde(rename = "id_in")]
    pub id_in: Option<Vec<i32>>,
    #[serde(rename = "id_not_in")]
    pub id_not_in: Option<Vec<i32>>,
    pub sort: Option<Vec<StudioSort>>,
    // Sub-pagination variables
    #[serde(rename = "mediaPage")]
    pub media_page: Option<i32>,
    #[serde(rename = "mediaPerPage")]
    pub media_per_page: Option<i32>,
}

/// Endpoint for studio operations.
pub struct StudioEndpoint {
    client: AniListClient,
}

impl StudioEndpoint {
    pub fn new(client: AniListClient) -> Self {
        Self { client }
    }

    pub async fn fetch(
        &self,
        options: &FetchStudioOptions,
    ) -> Result<Page<Vec<Studio>>, AniListError> {
        let query = studio::FETCH;
        let variables = json!(options);
        let variables_map = crate::utils::json_to_hashmap(variables);
        let response: Result<GraphQLResponse<Page<Vec<Studio>>>, AniListError> =
            self.client.query_typed(query, Some(&variables_map)).await;
        match response {
            Ok(res) => Ok(res.data),
            Err(err) => Err(err),
        }
    }

    pub async fn fetch_one(&self, options: &FetchStudioOneOptions) -> Result<Studio, AniListError> {
        let query = studio::FETCH_ONE;
        let variables = json!(options);
        let variables_map = crate::utils::json_to_hashmap(variables);
        let response: Result<GraphQLResponse<Studio>, AniListError> =
            self.client.query_typed(query, Some(&variables_map)).await;
        match response {
            Ok(res) => Ok(res.data),
            Err(err) => Err(err),
        }
    }

    // Convenience functions

    /// Get popular studios sorted by favorites
    pub async fn get_popular(
        &self,
        page: Option<i32>,
        per_page: Option<i32>,
    ) -> Result<Page<Vec<Studio>>, AniListError> {
        self.fetch(&FetchStudioOptions {
            page,
            per_page,
            sort: Some(vec![StudioSort::FavouritesDesc]),
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
        self.fetch(&FetchStudioOptions {
            search: Some(query.to_string()),
            page,
            per_page,
            sort: Some(vec![StudioSort::SearchMatch]),
            ..Default::default()
        })
        .await
    }

    /// Get studio by ID
    pub async fn get_by_id(&self, id: i32) -> Result<Studio, AniListError> {
        self.fetch_one(&FetchStudioOneOptions {
            id: Some(id),
            ..Default::default()
        })
        .await
    }
}
