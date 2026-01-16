use crate::enums::media::MediaType;
use crate::enums::review::{ReviewRating, ReviewSort};
use crate::errors::AniListError;
use crate::objects::common::Deleted;
use crate::objects::responses::Page;
use crate::objects::review::Review;
use crate::{client::AniListClient, queries::review};
use serde::{Deserialize, Serialize};
use serde_json::json;
use serde_with::skip_serializing_none;

/// Options for fetching reviews.
#[skip_serializing_none]
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct FetchReviewOptions {
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
    pub sort: Option<Vec<ReviewSort>>,
}

/// Options for creating or updating a review.
#[skip_serializing_none]
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct SaveReviewOptions {
    pub id: Option<i32>,
    #[serde(rename = "mediaId")]
    pub media_id: i32,
    pub score: Option<i32>,
    pub summary: Option<String>,
    pub body: Option<String>,
    pub private: Option<bool>,
}

/// Options for deleting a review.
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct DeleteReviewOptions {
    pub id: i32,
}

/// Options for rating a review.
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct RateReviewOptions {
    #[serde(rename = "reviewId")]
    pub review_id: i32,
    pub rating: ReviewRating,
}

/// Endpoint for review operations.
pub struct ReviewEndpoint {
    pub client: AniListClient,
}

impl ReviewEndpoint {
    pub fn new(client: AniListClient) -> Self {
        Self { client }
    }

    pub async fn fetch(
        &self,
        options: &FetchReviewOptions,
    ) -> Result<Page<Vec<Review>>, AniListError> {
        let query = review::FETCH;
        let variables = json!(options);
        let variables_map = crate::utils::json_to_hashmap(variables);
        self.client.fetch(query, Some(&variables_map)).await
    }

    pub async fn save(&self, options: &SaveReviewOptions) -> Result<Review, AniListError> {
        let query = review::SAVE;
        let variables = json!(options);
        let variables_map = crate::utils::json_to_hashmap(variables);
        self.client.fetch(query, Some(&variables_map)).await
    }

    pub async fn delete(&self, options: &DeleteReviewOptions) -> Result<bool, AniListError> {
        let query = review::DELETE;
        let variables = json!(options);
        let variables_map = crate::utils::json_to_hashmap(variables);
        let response: Result<Deleted, AniListError> =
            self.client.fetch(query, Some(&variables_map)).await;
        match response {
            Ok(res) => Ok(res.deleted.unwrap_or_default()),
            Err(err) => Err(err),
        }
    }

    pub async fn rate(&self, options: &RateReviewOptions) -> Result<Review, AniListError> {
        let query = review::RATE;
        let variables = json!(options);
        let variables_map = crate::utils::json_to_hashmap(variables);
        self.client.fetch(query, Some(&variables_map)).await
    }

    // Convenience functions

    /// Get reviews for a specific media
    pub async fn get_by_media_id(
        &self,
        media_id: i32,
        page: Option<i32>,
        per_page: Option<i32>,
    ) -> Result<Page<Vec<Review>>, AniListError> {
        self.fetch(&FetchReviewOptions {
            media_id: Some(media_id),
            page,
            per_page,
            sort: Some(vec![ReviewSort::RatingDesc]),
            ..Default::default()
        })
        .await
    }

    /// Get reviews by a specific user
    pub async fn get_by_user_id(
        &self,
        user_id: i32,
        page: Option<i32>,
        per_page: Option<i32>,
    ) -> Result<Page<Vec<Review>>, AniListError> {
        self.fetch(&FetchReviewOptions {
            user_id: Some(user_id),
            page,
            per_page,
            sort: Some(vec![ReviewSort::CreatedAtDesc]),
            ..Default::default()
        })
        .await
    }

    /// Get recent reviews
    pub async fn get_recent(
        &self,
        page: Option<i32>,
        per_page: Option<i32>,
    ) -> Result<Page<Vec<Review>>, AniListError> {
        self.fetch(&FetchReviewOptions {
            page,
            per_page,
            sort: Some(vec![ReviewSort::CreatedAtDesc]),
            ..Default::default()
        })
        .await
    }

    /// Get review by ID
    pub async fn get_by_id(&self, id: i32) -> Result<Review, AniListError> {
        let response = self
            .fetch(&FetchReviewOptions {
                id: Some(id),
                ..Default::default()
            })
            .await;
        match response {
            Ok(res) => Ok(res.data[0].clone()),
            Err(err) => Err(err),
        }
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
        self.save(&SaveReviewOptions {
            id: None,
            media_id,
            score: Some(score),
            summary: Some(summary.to_string()),
            body: Some(body.to_string()),
            private,
        })
        .await
    }

    /// Delete a review
    pub async fn delete_review(&self, id: i32) -> Result<bool, AniListError> {
        self.delete(&DeleteReviewOptions { id }).await
    }

    /// Rate a review
    pub async fn rate_review(
        &self,
        review_id: i32,
        rating: ReviewRating,
    ) -> Result<Review, AniListError> {
        self.rate(&RateReviewOptions { review_id, rating }).await
    }
}
