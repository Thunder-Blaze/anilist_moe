use crate::errors::AniListError;
use crate::{client::AniListClient, queries::airing};
use crate::{enums::airing::AiringSort, objects::responses::AiringListResponse};
use serde::Serialize;
use serde_json::json;

#[derive(Default, Serialize)]
pub struct FetchAiringOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "mediaId", skip_serializing_if = "Option::is_none")]
    pub media_id: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub episode: Option<i32>,
    #[serde(rename = "airingAt", skip_serializing_if = "Option::is_none")]
    pub airing_at: Option<i32>,
    #[serde(rename = "notYetAired", skip_serializing_if = "Option::is_none")]
    pub not_yet_aired: Option<bool>,
    #[serde(rename = "id_not", skip_serializing_if = "Option::is_none")]
    pub id_not: Option<i32>,
    #[serde(rename = "id_in", skip_serializing_if = "Option::is_none")]
    pub id_in: Option<Vec<i32>>,
    #[serde(rename = "id_not_in", skip_serializing_if = "Option::is_none")]
    pub id_not_in: Option<Vec<i32>>,
    #[serde(rename = "mediaId_not", skip_serializing_if = "Option::is_none")]
    pub media_id_not: Option<i32>,
    #[serde(rename = "mediaId_in", skip_serializing_if = "Option::is_none")]
    pub media_id_in: Option<Vec<i32>>,
    #[serde(rename = "mediaId_not_in", skip_serializing_if = "Option::is_none")]
    pub media_id_not_in: Option<Vec<i32>>,
    #[serde(rename = "episode_not", skip_serializing_if = "Option::is_none")]
    pub episode_not: Option<i32>,
    #[serde(rename = "episode_in", skip_serializing_if = "Option::is_none")]
    pub episode_in: Option<Vec<i32>>,
    #[serde(rename = "episode_not_in", skip_serializing_if = "Option::is_none")]
    pub episode_not_in: Option<Vec<i32>>,
    #[serde(rename = "episode_greater", skip_serializing_if = "Option::is_none")]
    pub episode_greater: Option<i32>,
    #[serde(rename = "episode_lesser", skip_serializing_if = "Option::is_none")]
    pub episode_lesser: Option<i32>,
    #[serde(rename = "airingAt_greater", skip_serializing_if = "Option::is_none")]
    pub airing_at_greater: Option<i32>,
    #[serde(rename = "airingAt_lesser", skip_serializing_if = "Option::is_none")]
    pub airing_at_lesser: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<Vec<AiringSort>>,
    #[serde(rename = "perPage", skip_serializing_if = "Option::is_none")]
    pub per_page: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
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
        let variables_map = crate::utils::json_to_hashmap(variables);
        self.client.query_typed(query, Some(&variables_map)).await
    }
}
