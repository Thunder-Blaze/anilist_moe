use crate::enums::recommendation::{RecommendationRating, RecommendationSort};
use crate::errors::AniListError;
use crate::objects::recommendation::Recommendation;
use crate::objects::responses::Page;
use crate::{client::AniListClient, queries::recommendation};
use serde::{Deserialize, Serialize};
use serde_json::json;
use serde_with::skip_serializing_none;

/// Options for fetching media recommendations.
#[skip_serializing_none]
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct FetchRecommendationOptions {
    pub page: Option<i32>,
    #[serde(rename = "perPage")]
    pub per_page: Option<i32>,
    pub id: Option<i32>,
    #[serde(rename = "mediaId")]
    pub media_id: Option<i32>,
    #[serde(rename = "mediaRecommendationId")]
    pub media_recommendation_id: Option<i32>,
    #[serde(rename = "userId")]
    pub user_id: Option<i32>,
    pub rating: Option<i32>,
    #[serde(rename = "rating_greater")]
    pub rating_greater: Option<i32>,
    #[serde(rename = "rating_lesser")]
    pub rating_lesser: Option<i32>,
    #[serde(rename = "onList")]
    pub on_list: Option<bool>,
    pub sort: Option<Vec<RecommendationSort>>,
}

/// Options for saving a recommendation rating.
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct SaveRecommendationOptions {
    #[serde(rename = "mediaId")]
    pub media_id: i32,
    #[serde(rename = "mediaRecommendationId")]
    pub media_recommendation_id: i32,
    pub rating: RecommendationRating,
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
        options: &FetchRecommendationOptions,
    ) -> Result<Page<Vec<Recommendation>>, AniListError> {
        let query = recommendation::FETCH;
        let variables = json!(options);
        let variables_map = crate::utils::json_to_hashmap(variables);
        self.client.fetch(query, Some(&variables_map)).await
    }

    pub async fn save(
        &self,
        options: &SaveRecommendationOptions,
    ) -> Result<Recommendation, AniListError> {
        let query = recommendation::SAVE;
        let variables = json!(options);
        let variables_map = crate::utils::json_to_hashmap(variables);
        self.client.fetch(query, Some(&variables_map)).await
    }

    // Convenience functions

    /// Get recommendations for a specific media
    pub async fn get_by_media_id(
        &self,
        media_id: i32,
        page: Option<i32>,
        per_page: Option<i32>,
    ) -> Result<Page<Vec<Recommendation>>, AniListError> {
        self.fetch(&FetchRecommendationOptions {
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
    ) -> Result<Page<Vec<Recommendation>>, AniListError> {
        self.fetch(&FetchRecommendationOptions {
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
    ) -> Result<Page<Vec<Recommendation>>, AniListError> {
        self.fetch(&FetchRecommendationOptions {
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
        rating: RecommendationRating,
    ) -> Result<Recommendation, AniListError> {
        self.save(&SaveRecommendationOptions {
            media_id,
            media_recommendation_id,
            rating,
        })
        .await
    }
}
