use crate::client::AniListClient;
use crate::errors::AniListError;
use crate::objects::responses::AiringListResponse;
use serde::Serialize;
use serde_json::{json, Value};
use std::collections::HashMap;

#[derive(Default, Serialize)]
pub struct AiringSearchOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i32>,
    #[serde(rename = "perPage", skip_serializing_if = "Option::is_none")]
    pub per_page: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "mediaId", skip_serializing_if = "Option::is_none")]
    pub media_id: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub episode: Option<i32>,
    #[serde(rename = "airingAt", skip_serializing_if = "Option::is_none")]
    pub airing_at: Option<i32>,
    #[serde(rename = "airingAt_greater", skip_serializing_if = "Option::is_none")]
    pub airing_at_greater: Option<i32>,
    #[serde(rename = "airingAt_lesser", skip_serializing_if = "Option::is_none")]
    pub airing_at_lesser: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<Vec<crate::enums::airing::AiringSort>>,
}

pub struct AiringEndpoint(pub(crate) AniListClient);

impl AiringEndpoint {
    pub fn new(client: AniListClient) -> Self {
        Self(client)
    }

    /// Get upcoming episodes
    pub async fn get_upcoming_episodes(&self, page: i32, per_page: i32) -> Result<AiringListResponse, AniListError> {
        let options = AiringSearchOptions {
            page: Some(page),
            per_page: Some(per_page),
            sort: Some(vec![crate::enums::airing::AiringSort::Time]),
            ..Default::default()
        };
        self.search_airing_schedule(options).await
    }

    /// Get episodes airing today
    pub async fn get_today_episodes(&self, page: i32, per_page: i32) -> Result<AiringListResponse, AniListError> {
        let options = AiringSearchOptions {
            page: Some(page),
            per_page: Some(per_page),
            sort: Some(vec![crate::enums::airing::AiringSort::Time]),
            ..Default::default()
        };
        self.search_airing_schedule(options).await
    }

    /// Get recently aired episodes
    pub async fn get_recently_aired(&self, page: i32, per_page: i32) -> Result<AiringListResponse, AniListError> {
        let options = AiringSearchOptions {
            page: Some(page),
            per_page: Some(per_page),
            sort: Some(vec![crate::enums::airing::AiringSort::Time]),
            ..Default::default()
        };
        self.search_airing_schedule(options).await
    }

    /// Get airing schedule for specific media
    pub async fn get_schedule_for_media(&self, media_id: i32, page: i32, per_page: i32) -> Result<AiringListResponse, AniListError> {
        let options = AiringSearchOptions {
            page: Some(page),
            per_page: Some(per_page),
            media_id: Some(media_id),
            ..Default::default()
        };
        self.search_airing_schedule(options).await
    }

    /// Get airing schedule by ID
    pub async fn get_schedule_by_id(&self, id: i32) -> Result<AiringListResponse, AniListError> {
        let query = include_str!("../queries/airing/airing_schedule.graphql");
        let variables = json!({ "id": id });
        let variables_map = self.value_to_hashmap(variables);
        self.0.query_typed(query, Some(&variables_map)).await
    }

    /// Search airing schedules with custom criteria
    pub async fn search_with_options(&self, options: AiringSearchOptions) -> Result<AiringListResponse, AniListError> {
        self.search_airing_schedule(options).await
    }

    async fn search_airing_schedule(&self, options: AiringSearchOptions) -> Result<AiringListResponse, AniListError> {
        let query = include_str!("../queries/airing/airing_schedule.graphql");
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
