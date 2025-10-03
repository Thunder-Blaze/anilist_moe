use crate::enums::recommendation::RecommendationSort;
use crate::errors::AniListError;
use crate::objects::responses::RecommendationListResponse;
use crate::{client::AniListClient, queries::recommendation};
use serde::Serialize;
use serde_json::json;

/// Options for fetching media recommendations.
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
    #[serde(
        rename = "mediaRecommendationId",
        skip_serializing_if = "Option::is_none"
    )]
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

/// Options for saving a recommendation rating.
#[derive(Default, Serialize)]
pub struct SaveRecommendationOptions {
    #[serde(rename = "mediaId")]
    pub media_id: i32,
    #[serde(rename = "mediaRecommendationId")]
    pub media_recommendation_id: i32,
    pub rating: i32,
}

/// Endpoint for media recommendation operations.
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
        let variables_map = crate::utils::json_to_hashmap(variables);
        self.client.query_typed(query, Some(&variables_map)).await
    }

    pub async fn save(
        &self,
        options: SaveRecommendationOptions,
    ) -> Result<RecommendationListResponse, AniListError> {
        let query = recommendation::SAVE;
        let variables = json!(options);
        let variables_map = crate::utils::json_to_hashmap(variables);
        self.client.query_typed(query, Some(&variables_map)).await
    }

    // Convenience functions

    /// Get recommendations for a specific media
    pub async fn get_by_media_id(
        &self,
        media_id: i32,
        page: Option<i32>,
        per_page: Option<i32>,
    ) -> Result<RecommendationListResponse, AniListError> {
        self.fetch(FetchRecommendationOptions {
            media_id: Some(media_id),
            page,
            per_page,
            sort: Some(vec![RecommendationSort::RatingDesc]),
            ..Default::default()
        })
        .await
    }

    /// Get recommendations by a specific user
    pub async fn get_by_user_id(
        &self,
        user_id: i32,
        page: Option<i32>,
        per_page: Option<i32>,
    ) -> Result<RecommendationListResponse, AniListError> {
        self.fetch(FetchRecommendationOptions {
            user_id: Some(user_id),
            page,
            per_page,
            sort: Some(vec![RecommendationSort::IdDesc]),
            ..Default::default()
        })
        .await
    }

    /// Get recent recommendations
    pub async fn get_recent(
        &self,
        page: Option<i32>,
        per_page: Option<i32>,
    ) -> Result<RecommendationListResponse, AniListError> {
        self.fetch(FetchRecommendationOptions {
            page,
            per_page,
            sort: Some(vec![RecommendationSort::IdDesc]),
            ..Default::default()
        })
        .await
    }

    /// Create or update a recommendation rating
    pub async fn rate_recommendation(
        &self,
        media_id: i32,
        media_recommendation_id: i32,
        rating: i32,
    ) -> Result<RecommendationListResponse, AniListError> {
        self.save(SaveRecommendationOptions {
            media_id,
            media_recommendation_id,
            rating,
        })
        .await
    }
}
