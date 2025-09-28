use crate::client::AniListClient;
use crate::errors::AniListError;
use serde::Serialize;
use serde_json::{json, Value};
use std::collections::HashMap;

#[derive(Default, Serialize)]
pub struct ReviewSearchOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i32>,
    #[serde(rename = "perPage", skip_serializing_if = "Option::is_none")]
    pub per_page: Option<i32>,
    #[serde(rename = "mediaId", skip_serializing_if = "Option::is_none")]
    pub media_id: Option<i32>,
    #[serde(rename = "userId", skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i32>,
    #[serde(rename = "score_greater", skip_serializing_if = "Option::is_none")]
    pub score_greater: Option<i32>,
    #[serde(rename = "score_lesser", skip_serializing_if = "Option::is_none")]
    pub score_lesser: Option<i32>,
}

pub struct ReviewEndpoint(pub(crate) AniListClient);

impl ReviewEndpoint {
    pub fn new(client: AniListClient) -> Self {
        Self(client)
    }

    /// Get recent reviews
    pub async fn get_recent_reviews(&self, page: i32, per_page: i32) -> Result<Value, AniListError> {
        let options = ReviewSearchOptions {
            page: Some(page),
            per_page: Some(per_page),
            ..Default::default()
        };
        self.search_reviews(options).await
    }

    /// Get reviews for specific media
    pub async fn get_reviews_for_media(&self, media_id: i32, page: i32, per_page: i32) -> Result<Value, AniListError> {
        let options = ReviewSearchOptions {
            media_id: Some(media_id),
            page: Some(page),
            per_page: Some(per_page),
            ..Default::default()
        };
        self.search_reviews(options).await
    }

    /// Get reviews by user
    pub async fn get_reviews_by_user(&self, user_id: i32, page: i32, per_page: i32) -> Result<Value, AniListError> {
        let options = ReviewSearchOptions {
            user_id: Some(user_id),
            page: Some(page),
            per_page: Some(per_page),
            ..Default::default()
        };
        self.search_reviews(options).await
    }

    /// Get highly scored reviews
    pub async fn get_high_scored_reviews(&self, min_score: i32, page: i32, per_page: i32) -> Result<Value, AniListError> {
        let options = ReviewSearchOptions {
            score_greater: Some(min_score),
            page: Some(page),
            per_page: Some(per_page),
            ..Default::default()
        };
        self.search_reviews(options).await
    }

    /// Get top rated reviews
    pub async fn get_top_rated_reviews(&self, page: i32, per_page: i32) -> Result<Value, AniListError> {
        let options = ReviewSearchOptions {
            score_greater: Some(80), // Top rated threshold
            page: Some(page),
            per_page: Some(per_page),
            ..Default::default()
        };
        self.search_reviews(options).await
    }

    /// Get review by ID
    pub async fn get_review_by_id(&self, id: i32) -> Result<Value, AniListError> {
        let query = include_str!("../queries/review/search_reviews.graphql");
        let variables = json!({ "id": id });
        let variables_map = self.value_to_hashmap(variables);
        self.0.query(query, Some(&variables_map)).await
    }

    /// Save review (requires authentication)
    pub async fn save_review(&self, media_id: i32, body: &str, summary: &str, score: i32, private: Option<bool>) -> Result<Value, AniListError> {
        let query = include_str!("../queries/review/save_review.graphql");
        let mut variables = json!({ 
            "mediaId": media_id,
            "body": body,
            "summary": summary,
            "score": score
        });
        
        if let Some(is_private) = private {
            variables["private"] = json!(is_private);
        }
        
        let variables_map = self.value_to_hashmap(variables);
        self.0.query(query, Some(&variables_map)).await
    }

    /// Delete review (requires authentication)
    pub async fn delete_review(&self, id: i32) -> Result<Value, AniListError> {
        let query = include_str!("../queries/review/delete_review.graphql");
        let variables = json!({ "id": id });
        let variables_map = self.value_to_hashmap(variables);
        self.0.query(query, Some(&variables_map)).await
    }

    /// Search reviews with custom options
    pub async fn search_with_options(&self, options: ReviewSearchOptions) -> Result<Value, AniListError> {
        self.search_reviews(options).await
    }

    async fn search_reviews(&self, options: ReviewSearchOptions) -> Result<Value, AniListError> {
        let query = include_str!("../queries/review/search_reviews.graphql");
        let variables = json!(options);
        let variables_map = self.value_to_hashmap(variables);
        self.0.query(query, Some(&variables_map)).await
    }

    fn value_to_hashmap(&self, value: Value) -> HashMap<String, Value> {
        match value {
            Value::Object(map) => map.into_iter().collect(),
            _ => HashMap::new(),
        }
    }
}