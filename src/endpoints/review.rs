use crate::enums::media::MediaType;
use crate::enums::review::{ReviewRating, ReviewSort};
use crate::errors::AniListError;
use crate::objects::common::Deleted;
use crate::objects::responses::Page;
use crate::objects::review::Review;
use crate::options;
#[cfg(feature = "pagination")]
use crate::pagination::Paginated;
use crate::{client::AniListClient, queries::review};
use serde::Serialize;
use serde_with::skip_serializing_none;
use std::borrow::Cow;

/// Options for fetching reviews.
#[skip_serializing_none]
#[derive(Default, Debug, Serialize)]
pub struct FetchReviewOptions<'a> {
    pub page: Option<i32>,
    #[serde(rename = "perPage")]
    pub per_page: Option<i32>,
    pub id: Option<i32>,
    #[serde(rename = "mediaId")]
    pub media_id: Option<i32>,
    #[serde(rename = "userId")]
    pub user_id: Option<i32>,
    #[serde(rename = "mediaType")]
    pub media_type: Option<MediaType>,
    /// Sort order. The unique key (id) is appended, so paging stays stable;
    /// leaving this unset sorts by the same key alone.
    #[serde(
        serialize_with = "crate::options::serialize_sort",
        skip_serializing_if = "crate::options::skip_never"
    )]
    pub sort: Option<Cow<'a, [ReviewSort]>>,
    // HTML rendering options
    #[serde(rename = "body_as_html")]
    pub body_as_html: Option<bool>,
    #[serde(rename = "description_as_html")]
    pub description_as_html: Option<bool>,
}

/// Options for creating or updating a review.
#[skip_serializing_none]
#[derive(Default, Debug, Serialize)]
pub struct SaveReviewOptions<'a> {
    pub id: Option<i32>,
    #[serde(rename = "mediaId")]
    pub media_id: i32,
    pub score: Option<i32>,
    pub summary: Option<Cow<'a, str>>,
    pub body: Option<Cow<'a, str>>,
    pub private: Option<bool>,
    // HTML rendering options
    #[serde(rename = "body_as_html")]
    pub body_as_html: Option<bool>,
}

/// Options for deleting a review.
#[derive(Default, Debug, Serialize)]
pub struct DeleteReviewOptions {
    pub id: i32,
}

/// Options for rating a review.
#[derive(Default, Debug, Serialize)]
pub struct RateReviewOptions {
    #[serde(rename = "reviewId")]
    pub review_id: i32,
    pub rating: ReviewRating,
}

/// Endpoint for review operations.
pub struct ReviewEndpoint<'a> {
    pub client: &'a AniListClient,
}

impl<'a> ReviewEndpoint<'a> {
    pub fn new(client: &'a AniListClient) -> Self {
        Self { client }
    }

    pub async fn fetch(
        &self,
        options: &FetchReviewOptions<'_>,
    ) -> Result<Page<Vec<Review>>, AniListError> {
        let query = review::FETCH;
        self.client.fetch(query, Some(&options)).await
    }

    pub async fn save(&self, options: &SaveReviewOptions<'_>) -> Result<Review, AniListError> {
        let query = review::SAVE;
        self.client.fetch(query, Some(&options)).await
    }

    pub async fn delete(&self, options: &DeleteReviewOptions) -> Result<bool, AniListError> {
        let query = review::DELETE;
        let response: Result<Deleted, AniListError> =
            self.client.fetch(query, Some(&options)).await;
        match response {
            Ok(res) => Ok(res.deleted.unwrap_or_default()),
            Err(err) => Err(err),
        }
    }

    pub async fn rate(&self, options: &RateReviewOptions) -> Result<Review, AniListError> {
        let query = review::RATE;
        self.client.fetch(query, Some(&options)).await
    }

    // Convenience functions

    /// Get reviews for a specific media
    pub async fn get_by_media_id(
        &self,
        media_id: i32,
        page: Option<i32>,
        per_page: Option<i32>,
    ) -> Result<Page<Vec<Review>>, AniListError> {
        self.fetch(&options!(
            FetchReviewOptions,
            media_id: Some(media_id),
            page,
            per_page,
            sort: Some(&[ReviewSort::RatingDesc]),
        ))
        .await
    }

    /// Get reviews by a specific user
    pub async fn get_by_user_id(
        &self,
        user_id: i32,
        page: Option<i32>,
        per_page: Option<i32>,
    ) -> Result<Page<Vec<Review>>, AniListError> {
        self.fetch(&options!(
            FetchReviewOptions,
            user_id: Some(user_id),
            page,
            per_page,
            sort: Some(&[ReviewSort::CreatedAtDesc]),
        ))
        .await
    }

    /// Get recent reviews
    pub async fn get_recent(
        &self,
        page: Option<i32>,
        per_page: Option<i32>,
    ) -> Result<Page<Vec<Review>>, AniListError> {
        self.fetch(&options!(
            FetchReviewOptions,
            page,
            per_page,
            sort: Some(&[ReviewSort::CreatedAtDesc]),
        ))
        .await
    }

    /// Get review by ID
    pub async fn get_by_id(&self, id: i32) -> Result<Review, AniListError> {
        let mut response = self
            .fetch(&options!(
                FetchReviewOptions,
                id: Some(id),
            ))
            .await?;

        if response.data.is_empty() {
            return Err(AniListError::NotFound);
        }

        Ok(response.data.swap_remove(0))
    }

    /// Create a new review
    pub async fn create(
        &self,
        media_id: i32,
        score: i32,
        summary: &str,
        body: &str,
        private: Option<bool>,
    ) -> Result<Review, AniListError> {
        self.save(&options!(
            SaveReviewOptions,
            media_id => media_id,
            score: Some(score),
            summary: Some(summary),
            body: Some(body),
            private,
        ))
        .await
    }

    /// Delete a review
    pub async fn delete_review(&self, id: i32) -> Result<bool, AniListError> {
        self.delete(&options!(
            DeleteReviewOptions,
            id => id,
        ))
        .await
    }

    /// Rate a review
    pub async fn rate_review(
        &self,
        review_id: i32,
        rating: ReviewRating,
    ) -> Result<Review, AniListError> {
        self.rate(&options!(
            RateReviewOptions,
            review_id => review_id,
            rating => rating,
        ))
        .await
    }
}

#[cfg(feature = "pagination")]
impl<'a> Paginated for FetchReviewOptions<'a> {
    type Item = Review;

    async fn fetch(&self, client: &AniListClient) -> Result<Page<Vec<Review>>, AniListError> {
        client.review().fetch(self).await
    }

    fn set_page(&mut self, page: i32) {
        self.page = Some(page);
    }
}
