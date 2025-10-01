use crate::{client::AniListClient, queries::review};
use crate::errors::AniListError;
use crate::enums::media::MediaType;
use crate::enums::review::ReviewSort;
use crate::objects::responses::{ReviewListResponse, ReviewSingleResponse};
use serde_json::json;
use serde::Serialize;

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
}
