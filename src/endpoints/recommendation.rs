use crate::endpoints::Vth;
use crate::{client::AniListClient, queries::recommendation};
use crate::errors::AniListError;
use crate::enums::recommendation::RecommendationSort;
use crate::objects::responses::RecommendationListResponse;
use serde::Serialize;
use serde_json::json;

#[derive(Default, Serialize)]
pub struct FetchRecommendationOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i32>,
    #[serde(rename = "perPage", skip_serializing_if = "Option::is_none")]
    pub per_page: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "mediaId", skip_serializing_if = "Option::is_none")]
    pub media_id: Option<i32>,
    #[serde(rename = "mediaRecommendationId", skip_serializing_if = "Option::is_none")]
    pub media_recommendation_id: Option<i32>,
    #[serde(rename = "userId", skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rating: Option<i32>,
    #[serde(rename = "rating_greater", skip_serializing_if = "Option::is_none")]
    pub rating_greater: Option<i32>,
    #[serde(rename = "rating_lesser", skip_serializing_if = "Option::is_none")]
    pub rating_lesser: Option<i32>,
    #[serde(rename = "onList", skip_serializing_if = "Option::is_none")]
    pub on_list: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<Vec<RecommendationSort>>,
}

#[derive(Default, Serialize)]
pub struct SaveRecommendationOptions {
    #[serde(rename = "mediaId")]
    pub media_id: i32,
    #[serde(rename = "mediaRecommendationId")]
    pub media_recommendation_id: i32,
    pub rating: i32,
}

pub struct RecommendationEndpoint {
    pub client: AniListClient,
}

impl RecommendationEndpoint {
    pub fn new(client: AniListClient) -> Self {
        Self { client }
    }

    pub async fn fetch(
        &self,
        options: FetchRecommendationOptions,
    ) -> Result<RecommendationListResponse, AniListError> {
        let query = recommendation::FETCH;
        let variables = json!(options);
        let variables_map = self.value_to_hashmap(variables);
        self.client.query_typed(query, Some(&variables_map)).await
    }

    pub async fn save(
        &self,
        options: SaveRecommendationOptions,
    ) -> Result<RecommendationListResponse, AniListError> {
        let query = recommendation::SAVE;
        let variables = json!(options);
        let variables_map = self.value_to_hashmap(variables);
        self.client.query_typed(query, Some(&variables_map)).await
    }
}

impl Vth for RecommendationEndpoint {}
