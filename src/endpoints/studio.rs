use crate::client::AniListClient;
use crate::errors::AniListError;
use crate::enums::studio::StudioSort;
use crate::objects::responses::StudioListResponse;
use serde::Serialize;
use serde_json::{json, Value};
use std::collections::HashMap;

#[derive(Default, Serialize)]
pub struct StudioSearchOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i32>,
    #[serde(rename = "perPage", skip_serializing_if = "Option::is_none")]
    pub per_page: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "id_not", skip_serializing_if = "Option::is_none")]
    pub id_not: Option<i32>,
    #[serde(rename = "id_in", skip_serializing_if = "Option::is_none")]
    pub id_in: Option<Vec<i32>>,
    #[serde(rename = "id_not_in", skip_serializing_if = "Option::is_none")]
    pub id_not_in: Option<Vec<i32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<Vec<StudioSort>>,
}

pub struct StudioEndpoint(pub(crate) AniListClient);

impl StudioEndpoint {
    pub fn new(client: AniListClient) -> Self {
        Self(client)
    }

    /// Search studios
    pub async fn search(&self, query: &str, page: i32, per_page: i32) -> Result<StudioListResponse, AniListError> {
        let options = StudioSearchOptions {
            search: Some(query.to_string()),
            page: Some(page),
            per_page: Some(per_page),
            ..Default::default()
        };
        self.search_studios(options).await
    }

    /// Get popular studios
    pub async fn get_popular(&self, page: i32, per_page: i32) -> Result<StudioListResponse, AniListError> {
        let options = StudioSearchOptions {
            page: Some(page),
            per_page: Some(per_page),
            sort: Some(vec![StudioSort::Favourites]),
            ..Default::default()
        };
        self.search_studios(options).await
    }

    /// Get trending studios
    pub async fn get_trending(&self, page: i32, per_page: i32) -> Result<StudioListResponse, AniListError> {
        let options = StudioSearchOptions {
            page: Some(page),
            per_page: Some(per_page),
            sort: Some(vec![StudioSort::FavouritesDesc]),
            ..Default::default()
        };
        self.search_studios(options).await
    }

    /// Get studio by ID
    pub async fn get_by_id(&self, id: i32) -> Result<StudioListResponse, AniListError> {
        let options = StudioSearchOptions {
            id: Some(id),
            ..Default::default()
        };
        self.search_studios(options).await
    }

    /// Get most favorited studios
    pub async fn get_most_favorited(&self, page: i32, per_page: i32) -> Result<StudioListResponse, AniListError> {
        let options = StudioSearchOptions {
            page: Some(page),
            per_page: Some(per_page),
            sort: Some(vec![StudioSort::FavouritesDesc]),
            ..Default::default()
        };
        self.search_studios(options).await
    }

    /// Get studios by name (alphabetical)
    pub async fn get_by_name(&self, page: i32, per_page: i32) -> Result<StudioListResponse, AniListError> {
        let options = StudioSearchOptions {
            page: Some(page),
            per_page: Some(per_page),
            sort: Some(vec![StudioSort::Name]),
            ..Default::default()
        };
        self.search_studios(options).await
    }

    /// Search studios with custom options
    pub async fn search_with_options(&self, options: StudioSearchOptions) -> Result<StudioListResponse, AniListError> {
        self.search_studios(options).await
    }

    async fn search_studios(&self, options: StudioSearchOptions) -> Result<StudioListResponse, AniListError> {
        let query = include_str!("../queries/studio/search_studios.graphql");
        let variables = json!(options);
        let variables_map = self.value_to_hashmap(variables);
        self.0.query_typed(query, Some(&variables_map)).await
    }

    fn value_to_hashmap(&self, value: Value) -> HashMap<String, Value> {
        match value {
            Value::Object(map) => map.into_iter().collect(),
            _ => HashMap::new(),
        }
    }
}
