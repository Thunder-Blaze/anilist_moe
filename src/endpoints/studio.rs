use crate::endpoints::Vth;
use crate::{client::AniListClient, queries::studio};
use crate::errors::AniListError;
use crate::enums::studio::StudioSort;
use crate::objects::responses::StudioListResponse;
use serde::Serialize;
use serde_json::json;

#[derive(Default, Serialize)]
pub struct FetchStudioOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i32>,
    #[serde(rename = "perPage", skip_serializing_if = "Option::is_none")]
    pub per_page: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search: Option<String>,
    #[serde(rename = "id_not", skip_serializing_if = "Option::is_none")]
    pub id_not: Option<i32>,
    #[serde(rename = "id_in", skip_serializing_if = "Option::is_none")]
    pub id_in: Option<Vec<i32>>,
    #[serde(rename = "id_not_in", skip_serializing_if = "Option::is_none")]
    pub id_not_in: Option<Vec<i32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<Vec<StudioSort>>,
}

#[derive(Default, Serialize)]
pub struct FetchStudioOneOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search: Option<String>,
    #[serde(rename = "id_not", skip_serializing_if = "Option::is_none")]
    pub id_not: Option<i32>,
    #[serde(rename = "id_in", skip_serializing_if = "Option::is_none")]
    pub id_in: Option<Vec<i32>>,
    #[serde(rename = "id_not_in", skip_serializing_if = "Option::is_none")]
    pub id_not_in: Option<Vec<i32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<Vec<StudioSort>>,
    // Sub-pagination variables
    #[serde(rename = "mediaPage", skip_serializing_if = "Option::is_none")]
    pub media_page: Option<i32>,
    #[serde(rename = "mediaPerPage", skip_serializing_if = "Option::is_none")]
    pub media_per_page: Option<i32>,
}

pub struct StudioEndpoint {
    client: AniListClient,
}

impl StudioEndpoint {
    pub fn new(client: AniListClient) -> Self {
        Self { client }
    }

    pub async fn fetch(
        &self,
        options: FetchStudioOptions,
    ) -> Result<StudioListResponse, AniListError> {
        let query = studio::FETCH;
        let variables = json!(options);
        let variables_map = self.value_to_hashmap(variables);
        self.client.query_typed(query, Some(&variables_map)).await
    }

    pub async fn fetch_one(
        &self,
        options: FetchStudioOneOptions,
    ) -> Result<StudioListResponse, AniListError> {
        let query = studio::FETCH_ONE;
        let variables = json!(options);
        let variables_map = self.value_to_hashmap(variables);
        self.client.query_typed(query, Some(&variables_map)).await
    }
}

impl Vth for StudioEndpoint {}
