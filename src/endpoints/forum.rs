use crate::client::AniListClient;
use crate::errors::AniListError;
use serde::Serialize;
use serde_json::{json, Value};
use std::collections::HashMap;

#[derive(Default, Serialize)]
pub struct ThreadSearchOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i32>,
    #[serde(rename = "perPage", skip_serializing_if = "Option::is_none")]
    pub per_page: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search: Option<String>,
    #[serde(rename = "userId", skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i32>,
    #[serde(rename = "categoryId", skip_serializing_if = "Option::is_none")]
    pub category_id: Option<i32>,
    #[serde(rename = "mediaCategoryId", skip_serializing_if = "Option::is_none")]
    pub media_category_id: Option<i32>,
    #[serde(rename = "subscribed", skip_serializing_if = "Option::is_none")]
    pub subscribed: Option<bool>,
}

#[derive(Default, Serialize)]
pub struct ThreadCommentOptions {
    #[serde(rename = "threadId", skip_serializing_if = "Option::is_none")]
    pub thread_id: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i32>,
    #[serde(rename = "perPage", skip_serializing_if = "Option::is_none")]
    pub per_page: Option<i32>,
}

pub struct ForumEndpoint(pub(crate) AniListClient);

impl ForumEndpoint {
    pub fn new(client: AniListClient) -> Self {
        Self(client)
    }

    /// Get recent threads
    pub async fn get_recent_threads(&self, page: i32, per_page: i32) -> Result<Value, AniListError> {
        let options = ThreadSearchOptions {
            page: Some(page),
            per_page: Some(per_page),
            ..Default::default()
        };
        self.search_threads_with_options(options).await
    }

    /// Search threads by query
    pub async fn search_threads(&self, query: &str, page: i32, per_page: i32) -> Result<Value, AniListError> {
        let options = ThreadSearchOptions {
            search: Some(query.to_string()),
            page: Some(page),
            per_page: Some(per_page),
            ..Default::default()
        };
        self.search_threads_with_options(options).await
    }

    /// Get threads by user
    pub async fn get_threads_by_user(&self, user_id: i32, page: i32, per_page: i32) -> Result<Value, AniListError> {
        let options = ThreadSearchOptions {
            user_id: Some(user_id),
            page: Some(page),
            per_page: Some(per_page),
            ..Default::default()
        };
        self.search_threads_with_options(options).await
    }

    /// Get threads by category
    pub async fn get_threads_by_category(&self, category_id: i32, page: i32, per_page: i32) -> Result<Value, AniListError> {
        let options = ThreadSearchOptions {
            category_id: Some(category_id),
            page: Some(page),
            per_page: Some(per_page),
            ..Default::default()
        };
        self.search_threads_with_options(options).await
    }

    /// Get subscribed threads (requires authentication)
    pub async fn get_subscribed_threads(&self, page: i32, per_page: i32) -> Result<Value, AniListError> {
        let options = ThreadSearchOptions {
            subscribed: Some(true),
            page: Some(page),
            per_page: Some(per_page),
            ..Default::default()
        };
        self.search_threads_with_options(options).await
    }

    /// Get thread by ID
    pub async fn get_thread_by_id(&self, id: i32) -> Result<Value, AniListError> {
        let query = include_str!("../queries/forum/search_threads.graphql");
        let variables = json!({ "id": id });
        let variables_map = self.value_to_hashmap(variables);
        self.0.query(query, Some(&variables_map)).await
    }

    /// Get thread comments
    pub async fn get_thread_comments(&self, thread_id: i32, page: i32, per_page: i32) -> Result<Value, AniListError> {
        let options = ThreadCommentOptions {
            thread_id: Some(thread_id),
            page: Some(page),
            per_page: Some(per_page),
        };
        
        let query = include_str!("../queries/forum/search_threads.graphql");
        let variables = json!(options);
        let variables_map = self.value_to_hashmap(variables);
        self.0.query(query, Some(&variables_map)).await
    }

    /// Save thread comment (requires authentication)
    pub async fn save_thread_comment(&self, thread_id: i32, comment: &str, parent_comment_id: Option<i32>) -> Result<Value, AniListError> {
        let query = include_str!("../queries/forum/save_thread_comment.graphql");
        let mut variables = json!({ 
            "threadId": thread_id, 
            "comment": comment 
        });
        
        if let Some(parent_id) = parent_comment_id {
            variables["parentCommentId"] = json!(parent_id);
        }
        
        let variables_map = self.value_to_hashmap(variables);
        self.0.query(query, Some(&variables_map)).await
    }

    /// Comment on thread (requires authentication)
    pub async fn comment_on_thread(&self, thread_id: i32, comment: &str) -> Result<Value, AniListError> {
        let query = include_str!("../queries/forum/comment_on_thread.graphql");
        let variables = json!({ 
            "threadId": thread_id, 
            "comment": comment 
        });
        let variables_map = self.value_to_hashmap(variables);
        self.0.query(query, Some(&variables_map)).await
    }

    /// Toggle like on thread (requires authentication)
    pub async fn toggle_thread_like(&self, thread_id: i32) -> Result<Value, AniListError> {
        let query = include_str!("../queries/forum/toggle_thread_like.graphql");
        let variables = json!({ "threadId": thread_id });
        let variables_map = self.value_to_hashmap(variables);
        self.0.query(query, Some(&variables_map)).await
    }

    /// Like thread comment (requires authentication)
    pub async fn like_thread_comment(&self, comment_id: i32) -> Result<Value, AniListError> {
        let query = include_str!("../queries/forum/like_thread_comment.graphql");
        let variables = json!({ "commentId": comment_id });
        let variables_map = self.value_to_hashmap(variables);
        self.0.query(query, Some(&variables_map)).await
    }

    /// Search threads with custom options
    pub async fn search_with_options(&self, options: ThreadSearchOptions) -> Result<Value, AniListError> {
        self.search_threads_with_options(options).await
    }

    async fn search_threads_with_options(&self, options: ThreadSearchOptions) -> Result<Value, AniListError> {
        let query = include_str!("../queries/forum/search_threads.graphql");
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