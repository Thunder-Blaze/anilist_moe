use crate::enums::recommendation::{RecommendationRating, RecommendationSort};
use crate::errors::AniListError;
use crate::objects::recommendation::Recommendation;
use crate::objects::responses::Page;
#[cfg(feature = "pagination")]
use crate::pagination::Paginated;
use crate::queries::recommendation;
use crate::AniListClient;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use std::borrow::Cow;
use typed_builder::TypedBuilder;

/// Options for fetching media recommendations.
#[skip_serializing_none]
#[derive(Default, Debug, Serialize, TypedBuilder)]
#[builder(field_defaults(default))]
#[serde(rename_all = "camelCase")]
pub struct FetchRecommendationOptions<'a> {
    page: Option<i32>,
    per_page: Option<i32>,
    id: Option<u32>,
    media_id: Option<u32>,
    media_recommendation_id: Option<u32>,
    user_id: Option<u32>,
    rating: Option<i32>,
    rating_greater: Option<i32>,
    rating_lesser: Option<i32>,
    on_list: Option<bool>,
    /// Sort order. The unique key (id) is appended, so paging stays stable;
    /// leaving this unset sorts by the same key alone.
    #[serde(
        serialize_with = "crate::options::serialize_sort",
        skip_serializing_if = "crate::options::skip_never"
    )]
    sort: Option<Cow<'a, [RecommendationSort]>>,
}

/// Options for saving a recommendation rating.
#[derive(Default, Debug, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(default))]
#[serde(rename_all = "camelCase")]
pub struct SaveRecommendationOptions {
    media_id: u32,
    media_recommendation_id: u32,
    rating: RecommendationRating,
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
        self.fetch(
            &FetchRecommendationOptions::builder()
                .media_id(Some(media_id))
                .page(page)
                .per_page(per_page)
                .sort(Some(Cow::Borrowed(&[RecommendationSort::RatingDesc])))
                .build(),
        )
        .await
    }

    /// Get recommendations by a specific user
    pub async fn get_by_user_id(
        &self,
        user_id: u32,
        page: Option<i32>,
        per_page: Option<i32>,
    ) -> Result<Page<Vec<Recommendation>>, AniListError> {
        self.fetch(
            &FetchRecommendationOptions::builder()
                .user_id(Some(user_id))
                .page(page)
                .per_page(per_page)
                .sort(Some(Cow::Borrowed(&[RecommendationSort::IdDesc])))
                .build(),
        )
        .await
    }

    /// Get recent recommendations
    pub async fn get_recent(
        &self,
        page: Option<i32>,
        per_page: Option<i32>,
    ) -> Result<Page<Vec<Recommendation>>, AniListError> {
        self.fetch(
            &FetchRecommendationOptions::builder()
                .page(page)
                .per_page(per_page)
                .sort(Some(Cow::Borrowed(&[RecommendationSort::IdDesc])))
                .build(),
        )
        .await
    }

    /// Create or update a recommendation rating
    pub async fn rate_recommendation(
        &self,
        media_id: u32,
        media_recommendation_id: u32,
        rating: RecommendationRating,
    ) -> Result<Recommendation, AniListError> {
        self.save(
            &SaveRecommendationOptions::builder()
                .media_id(media_id)
                .media_recommendation_id(media_recommendation_id)
                .rating(rating)
                .build(),
        )
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
