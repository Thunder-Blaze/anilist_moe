use crate::errors::AniListError;
use crate::objects::airing::AiringSchedule;
use crate::objects::responses::{GraphQLResponse, Page};
use crate::{client::AniListClient, queries::airing};
use crate::{enums::airing::AiringSort};
use serde::Serialize;
use serde_json::json;
use serde_with::skip_serializing_none;

/// Options for fetching airing schedule information.
#[skip_serializing_none]
#[derive(Default, Debug, Serialize)]
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

/// Endpoint for airing schedule operations.
pub struct AiringEndpoint {
    pub client: AniListClient,
}

impl AiringEndpoint {
    pub fn new(client: AniListClient) -> Self {
        Self { client }
    }

    pub async fn fetch(
        &self,
        options: &FetchAiringOptions,
    ) -> Result<Page<Vec<AiringSchedule>>, AniListError> {
        let query = airing::FETCH;
        let variables = json!(options);
        let variables_map = crate::utils::json_to_hashmap(variables);
        let response: Result<GraphQLResponse<Page<Vec<AiringSchedule>>>, AniListError> = self.client.query_typed(query, Some(&variables_map)).await;
        match response {
            Ok(res) => Ok(res.data),
            Err(err) => Err(err),
        }
    }

    // Convenience functions

    /// Get upcoming airing episodes
    pub async fn get_upcoming(
        &self,
        page: Option<i32>,
        per_page: Option<i32>,
    ) -> Result<Page<Vec<AiringSchedule>>, AniListError> {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() as i32;

        self.fetch(&FetchAiringOptions {
            airing_at_greater: Some(now),
            sort: Some(vec![AiringSort::Time]),
            page,
            per_page,
            ..Default::default()
        })
        .await
    }

    /// Get recently aired episodes
    pub async fn get_recent(
        &self,
        page: Option<i32>,
        per_page: Option<i32>,
    ) -> Result<Page<Vec<AiringSchedule>>, AniListError> {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() as i32;

        self.fetch(&FetchAiringOptions {
            airing_at_lesser: Some(now),
            sort: Some(vec![AiringSort::TimeDesc]),
            page,
            per_page,
            ..Default::default()
        })
        .await
    }

    /// Get airing schedule for a specific media ID
    pub async fn get_by_media_id(
        &self,
        media_id: i32,
        page: Option<i32>,
        per_page: Option<i32>,
    ) -> Result<Page<Vec<AiringSchedule>>, AniListError> {
        self.fetch(&FetchAiringOptions {
            media_id: Some(media_id),
            sort: Some(vec![AiringSort::Time]),
            page,
            per_page,
            ..Default::default()
        })
        .await
    }
}
