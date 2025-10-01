use crate::endpoints::Vth;
use crate::{client::AniListClient, queries::airing};
use crate::errors::AniListError;
use crate::{
    objects::responses::AiringListResponse,
    enums::airing::AiringSort,
};
use serde::Serialize;
use serde_json::json;

#[derive(Default, Serialize)]
pub struct FetchAiringOptions {
    pub id: Option<i32>,
    #[serde(rename = "mediaId")]
    pub media_id: Option<i32>,
    pub episode: Option<i32>,
    #[serde(rename = "airingAt")]
    pub airing_at: Option<i32>,
    #[serde(rename = "notYetAired")]
    pub not_yet_aired: Option<bool>,
    #[serde(rename = "id_not")]
    pub id_not: Option<i32>,
    #[serde(rename = "id_in")]
    pub id_in: Option<Vec<i32>>,
    #[serde(rename = "id_not_in")]
    pub id_not_in: Option<Vec<i32>>,
    #[serde(rename = "mediaId_not")]
    pub media_id_not: Option<i32>,
    #[serde(rename = "mediaId_in")]
    pub media_id_in: Option<Vec<i32>>,
    #[serde(rename = "mediaId_not_in")]
    pub media_id_not_in: Option<Vec<i32>>,
    #[serde(rename = "episode_not")]
    pub episode_not: Option<i32>,
    #[serde(rename = "episode_in")]
    pub episode_in: Option<Vec<i32>>,
    #[serde(rename = "episode_not_in")]
    pub episode_not_in: Option<Vec<i32>>,
    #[serde(rename = "episode_greater")]
    pub episode_greater: Option<i32>,
    #[serde(rename = "episode_lesser")]
    pub episode_lesser: Option<i32>,
    #[serde(rename = "airingAt_greater")]
    pub airing_at_greater: Option<i32>,
    #[serde(rename = "airingAt_lesser")]
    pub airing_at_lesser: Option<i32>,
    pub sort: Option<Vec<AiringSort>>,
    #[serde(rename = "perPage")]
    pub per_page: Option<i32>,
    pub page: Option<i32>,
}

pub struct AiringEndpoint {
    pub client: AniListClient,
}

impl AiringEndpoint {
    pub fn new(client: AniListClient) -> Self {
        Self { client }
    }

    pub async fn fetch(
        &self,
        options: FetchAiringOptions,
    ) -> Result<AiringListResponse, AniListError> {
        let query = airing::FETCH;
        let variables = json!(options);
        let variables_map = self.value_to_hashmap(variables);
        self.client.query_typed(query, Some(&variables_map)).await
    }
}

impl Vth for AiringEndpoint {}
