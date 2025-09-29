use crate::client::AniListClient;
use crate::errors::AniListError;
use crate::enums::recommendation::RecommendationSort;
use crate::objects::responses::RecommendationListResponse;
use serde::Serialize;
use serde_json::{json, Value};
use std::collections::HashMap;

#[derive(Default, Serialize)]
pub struct RecommendationSearchOptions {
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

pub struct RecommendationEndpoint(pub(crate) AniListClient);

impl RecommendationEndpoint {
    pub fn new(client: AniListClient) -> Self {
        Self(client)
    }

    /// Get recent recommendations
    pub async fn get_recent(&self, page: i32, per_page: i32) -> Result<RecommendationListResponse, AniListError> {
        let options = RecommendationSearchOptions {
            page: Some(page),
            per_page: Some(per_page),
            ..Default::default()
        };
        self.search_recommendations(options).await
    }

    /// Get recommendations for specific media
    pub async fn get_for_media(&self, media_id: i32, page: i32, per_page: i32) -> Result<RecommendationListResponse, AniListError> {
        let options = RecommendationSearchOptions {
            media_id: Some(media_id),
            page: Some(page),
            per_page: Some(per_page),
            ..Default::default()
        };
        self.search_recommendations(options).await
    }

    /// Get recommendations by user
    pub async fn get_by_user(&self, user_id: i32, page: i32, per_page: i32) -> Result<RecommendationListResponse, AniListError> {
        let options = RecommendationSearchOptions {
            user_id: Some(user_id),
            page: Some(page),
            per_page: Some(per_page),
            ..Default::default()
        };
        self.search_recommendations(options).await
    }

    /// Get highly rated recommendations
    pub async fn get_highly_rated(&self, min_rating: i32, page: i32, per_page: i32) -> Result<RecommendationListResponse, AniListError> {
        let options = RecommendationSearchOptions {
            rating_greater: Some(min_rating),
            page: Some(page),
            per_page: Some(per_page),
            ..Default::default()
        };
        self.search_recommendations(options).await
    }

    /// Get recommendation by ID
    pub async fn get_by_id(&self, id: i32) -> Result<RecommendationListResponse, AniListError> {
        let options = RecommendationSearchOptions {
            id: Some(id),
            page: Some(1),
            per_page: Some(1),
            ..Default::default()
        };
        self.search_recommendations(options).await
    }

    /// Save recommendation (requires authentication)
    pub async fn save_recommendation(&self, media_id: i32, media_recommendation_id: i32, rating: i32) -> Result<RecommendationListResponse, AniListError> {
        let query = include_str!("../queries/recommendation/save_recommendation.graphql");
        let variables = json!({
            "mediaId": media_id,
            "mediaRecommendationId": media_recommendation_id,
            "rating": rating
        });
        let variables_map = self.value_to_hashmap(variables);
        self.0.query_typed(query, Some(&variables_map)).await
    }

    /// Search recommendations with custom options
    pub async fn search_with_options(&self, options: RecommendationSearchOptions) -> Result<RecommendationListResponse, AniListError> {
        self.search_recommendations(options).await
    }

    async fn search_recommendations(&self, options: RecommendationSearchOptions) -> Result<RecommendationListResponse, AniListError> {
        let query = include_str!("../queries/recommendation/search_recommendations.graphql");
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
