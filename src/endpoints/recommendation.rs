use crate::enums::recommendation::{RecommendationRating, RecommendationSort};
use crate::errors::AniListError;
use crate::objects::recommendation::Recommendation;
use crate::objects::responses::Page;
use crate::options;
#[cfg(feature = "pagination")]
use crate::pagination::Paginated;
use crate::{client::AniListClient, queries::recommendation};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use std::borrow::Cow;

/// Options for fetching media recommendations.
#[skip_serializing_none]
#[derive(Default, Debug, Serialize)]
pub struct FetchRecommendationOptions<'a> {
    pub page: Option<i32>,
    #[serde(rename = "perPage")]
    pub per_page: Option<i32>,
    pub id: Option<u32>,
    #[serde(rename = "mediaId")]
    pub media_id: Option<u32>,
    #[serde(rename = "mediaRecommendationId")]
    pub media_recommendation_id: Option<u32>,
    #[serde(rename = "userId")]
    pub user_id: Option<u32>,
    pub rating: Option<i32>,
    #[serde(rename = "rating_greater")]
    pub rating_greater: Option<i32>,
    #[serde(rename = "rating_lesser")]
    pub rating_lesser: Option<i32>,
    #[serde(rename = "onList")]
    pub on_list: Option<bool>,
    /// Sort order. The unique key (id) is appended, so paging stays stable;
    /// leaving this unset sorts by the same key alone.
    #[serde(
        serialize_with = "crate::options::serialize_sort",
        skip_serializing_if = "crate::options::skip_never"
    )]
    pub sort: Option<Cow<'a, [RecommendationSort]>>,
}

/// Options for saving a recommendation rating.
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct SaveRecommendationOptions {
    #[serde(rename = "mediaId")]
    pub media_id: u32,
    #[serde(rename = "mediaRecommendationId")]
    pub media_recommendation_id: u32,
    pub rating: RecommendationRating,
}

/// Endpoint for media recommendation operations.
pub struct RecommendationEndpoint<'a> {
    pub client: &'a AniListClient,
}

impl<'a> RecommendationEndpoint<'a> {
    pub fn new(client: &'a AniListClient) -> Self {
        Self { client }
    }

    pub async fn fetch(
        &self,
        options: &FetchRecommendationOptions<'_>,
    ) -> Result<Page<Vec<Recommendation>>, AniListError> {
        let query = recommendation::FETCH;
        self.client.fetch(query, Some(&options)).await
    }

    pub async fn save(
        &self,
        options: &SaveRecommendationOptions,
    ) -> Result<Recommendation, AniListError> {
        let query = recommendation::SAVE;
        self.client.fetch(query, Some(&options)).await
    }

    // Convenience functions

    /// Get recommendations for a specific media
    pub async fn get_by_media_id(
        &self,
        media_id: u32,
        page: Option<i32>,
        per_page: Option<i32>,
    ) -> Result<Page<Vec<Recommendation>>, AniListError> {
        self.fetch(&options!(
            FetchRecommendationOptions,
            media_id: Some(media_id),
            page,
            per_page,
            sort: Some(&[RecommendationSort::RatingDesc]),
        ))
        .await
    }

    /// Get recommendations by a specific user
    pub async fn get_by_user_id(
        &self,
        user_id: u32,
        page: Option<i32>,
        per_page: Option<i32>,
    ) -> Result<Page<Vec<Recommendation>>, AniListError> {
        self.fetch(&options!(
            FetchRecommendationOptions,
            user_id: Some(user_id),
            page,
            per_page,
            sort: Some(&[RecommendationSort::IdDesc]),
        ))
        .await
    }

    /// Get recent recommendations
    pub async fn get_recent(
        &self,
        page: Option<i32>,
        per_page: Option<i32>,
    ) -> Result<Page<Vec<Recommendation>>, AniListError> {
        self.fetch(&options!(
            FetchRecommendationOptions,
            page,
            per_page,
            sort: Some(&[RecommendationSort::IdDesc]),
        ))
        .await
    }

    /// Create or update a recommendation rating
    pub async fn rate_recommendation(
        &self,
        media_id: u32,
        media_recommendation_id: u32,
        rating: RecommendationRating,
    ) -> Result<Recommendation, AniListError> {
        self.save(&options!(
            SaveRecommendationOptions,
            media_id => media_id,
            media_recommendation_id => media_recommendation_id,
            rating => rating,
        ))
        .await
    }
}

#[cfg(feature = "pagination")]
impl<'a> Paginated for FetchRecommendationOptions<'a> {
    type Item = Recommendation;

    async fn fetch(
        &self,
        client: &AniListClient,
    ) -> Result<Page<Vec<Recommendation>>, AniListError> {
        client.recommendation().fetch(self).await
    }

    fn set_page(&mut self, page: i32) {
        self.page = Some(page);
    }
}
