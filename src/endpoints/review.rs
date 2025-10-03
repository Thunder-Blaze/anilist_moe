use crate::enums::media::MediaType;
use crate::enums::review::ReviewSort;
use crate::errors::AniListError;
use crate::objects::responses::{ReviewListResponse, ReviewSingleResponse};
use crate::{client::AniListClient, queries::review};
use serde::Serialize;
use serde_json::json;

#[derive(Default, Serialize)]
pub struct FetchReviewOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i32>,
    #[serde(rename = "perPage", skip_serializing_if = "Option::is_none")]
    pub per_page: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "mediaId", skip_serializing_if = "Option::is_none")]
    pub media_id: Option<i32>,
    #[serde(rename = "userId", skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i32>,
    #[serde(rename = "mediaType", skip_serializing_if = "Option::is_none")]
    pub media_type: Option<MediaType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<Vec<ReviewSort>>,
}

#[derive(Default, Serialize)]
pub struct SaveReviewOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "mediaId")]
    pub media_id: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private: Option<bool>,
}

#[derive(Default, Serialize)]
pub struct DeleteReviewOptions {
    pub id: i32,
}

#[derive(Default, Serialize)]
pub struct RateReviewOptions {
    #[serde(rename = "reviewId")]
    pub review_id: i32,
    pub rating: i32,
}

pub struct ReviewEndpoint {
    pub client: AniListClient,
}

impl ReviewEndpoint {
    pub fn new(client: AniListClient) -> Self {
        Self { client }
    }

    pub async fn fetch(
        &self,
        options: FetchReviewOptions,
    ) -> Result<ReviewListResponse, AniListError> {
        let query = review::FETCH;
        let variables = json!(options);
        let variables_map = crate::utils::json_to_hashmap(variables);
        self.client.query_typed(query, Some(&variables_map)).await
    }

    pub async fn save(
        &self,
        options: SaveReviewOptions,
    ) -> Result<ReviewSingleResponse, AniListError> {
        let query = review::SAVE;
        let variables = json!(options);
        let variables_map = crate::utils::json_to_hashmap(variables);
        self.client.query_typed(query, Some(&variables_map)).await
    }

    pub async fn delete(
        &self,
        options: DeleteReviewOptions,
    ) -> Result<ReviewSingleResponse, AniListError> {
        let query = review::DELETE;
        let variables = json!(options);
        let variables_map = crate::utils::json_to_hashmap(variables);
        self.client.query_typed(query, Some(&variables_map)).await
    }

    pub async fn rate(
        &self,
        options: RateReviewOptions,
    ) -> Result<ReviewSingleResponse, AniListError> {
        let query = review::RATE;
        let variables = json!(options);
        let variables_map = crate::utils::json_to_hashmap(variables);
        self.client.query_typed(query, Some(&variables_map)).await
    }

    // Convenience functions

    /// Get reviews for a specific media
    pub async fn get_by_media_id(
        &self,
        media_id: i32,
        page: Option<i32>,
        per_page: Option<i32>,
    ) -> Result<ReviewListResponse, AniListError> {
        self.fetch(FetchReviewOptions {
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
    ) -> Result<ReviewListResponse, AniListError> {
        self.fetch(FetchReviewOptions {
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
    ) -> Result<ReviewListResponse, AniListError> {
        self.fetch(FetchReviewOptions {
            page,
            per_page,
            sort: Some(vec![ReviewSort::CreatedAtDesc]),
            ..Default::default()
        })
        .await
    }

    /// Get review by ID
    pub async fn get_by_id(&self, id: i32) -> Result<ReviewListResponse, AniListError> {
        self.fetch(FetchReviewOptions {
            id: Some(id),
            ..Default::default()
        })
        .await
    }

    /// Create a new review
    pub async fn create(
        &self,
        media_id: i32,
        score: i32,
        summary: &str,
        body: &str,
        private: Option<bool>,
    ) -> Result<ReviewSingleResponse, AniListError> {
        self.save(SaveReviewOptions {
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
    pub async fn delete_review(&self, id: i32) -> Result<ReviewSingleResponse, AniListError> {
        self.delete(DeleteReviewOptions { id }).await
    }

    /// Rate a review
    pub async fn rate_review(
        &self,
        review_id: i32,
        rating: i32,
    ) -> Result<ReviewSingleResponse, AniListError> {
        self.rate(RateReviewOptions { review_id, rating })
            .await
    }
}
