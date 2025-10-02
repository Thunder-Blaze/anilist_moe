use crate::enums::thread::{ThreadCommentSort, ThreadSort};
use crate::errors::AniListError;
use crate::{client::AniListClient, queries::forum};
use serde::Serialize;
use serde_json::{Value, json};

#[derive(Default, Serialize)]
pub struct FetchThreadOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search: Option<String>,
    #[serde(rename = "userId", skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i32>,
    #[serde(rename = "replyUserId", skip_serializing_if = "Option::is_none")]
    pub reply_user_id: Option<i32>,
    #[serde(rename = "subscribed", skip_serializing_if = "Option::is_none")]
    pub subscribed: Option<bool>,
    #[serde(rename = "categoryId", skip_serializing_if = "Option::is_none")]
    pub category_id: Option<i32>,
    #[serde(rename = "mediaCategoryId", skip_serializing_if = "Option::is_none")]
    pub media_category_id: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<Vec<ThreadSort>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i32>,
    #[serde(rename = "perPage", skip_serializing_if = "Option::is_none")]
    pub per_page: Option<i32>,
}

#[derive(Default, Serialize)]
pub struct FetchThreadOneOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "commentsPage", skip_serializing_if = "Option::is_none")]
    pub comments_page: Option<i32>,
    #[serde(rename = "commentsPerPage", skip_serializing_if = "Option::is_none")]
    pub comments_per_page: Option<i32>,
    #[serde(rename = "commentsSort", skip_serializing_if = "Option::is_none")]
    pub comments_sort: Option<Vec<ThreadCommentSort>>,
}

#[derive(Default, Serialize)]
pub struct FetchThreadCommentOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "threadId", skip_serializing_if = "Option::is_none")]
    pub thread_id: Option<i32>,
    #[serde(rename = "userId", skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<Vec<ThreadCommentSort>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i32>,
    #[serde(rename = "perPage", skip_serializing_if = "Option::is_none")]
    pub per_page: Option<i32>,
}

#[derive(Default, Serialize)]
pub struct FetchThreadCommentOneOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
}

#[derive(Default, Serialize)]
pub struct SaveThreadOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub categories: Option<Vec<i32>>,
    #[serde(rename = "mediaCategories", skip_serializing_if = "Option::is_none")]
    pub media_categories: Option<Vec<i32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sticky: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locked: Option<bool>,
}

#[derive(Default, Serialize)]
pub struct DeleteThreadOptions {
    pub id: i32,
}

#[derive(Default, Serialize)]
pub struct SaveThreadCommentOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "threadId", skip_serializing_if = "Option::is_none")]
    pub thread_id: Option<i32>,
    #[serde(rename = "parentCommentId", skip_serializing_if = "Option::is_none")]
    pub parent_comment_id: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locked: Option<bool>,
}

#[derive(Default, Serialize)]
pub struct DeleteThreadCommentOptions {
    pub id: i32,
}

#[derive(Default, Serialize)]
pub struct ToggleThreadSubscriptionOptions {
    #[serde(rename = "threadId")]
    pub thread_id: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscribe: Option<bool>,
}

pub struct ForumEndpoint {
    client: AniListClient,
}

impl ForumEndpoint {
    pub fn new(client: AniListClient) -> Self {
        Self { client }
    }

    /// Fetch multiple threads with pagination
    pub async fn fetch(&self, options: FetchThreadOptions) -> Result<Value, AniListError> {
        let query = forum::FETCH;
        let variables = json!(options);
        let variables_map = crate::utils::json_to_hashmap(variables);
        self.client.query(query, Some(&variables_map)).await
    }

    /// Fetch a single thread with full details
    pub async fn fetch_one(&self, options: FetchThreadOneOptions) -> Result<Value, AniListError> {
        let query = forum::FETCH_ONE;
        let variables = json!(options);
        let variables_map = crate::utils::json_to_hashmap(variables);
        self.client.query(query, Some(&variables_map)).await
    }

    /// Fetch multiple thread comments with pagination
    pub async fn fetch_comments(
        &self,
        options: FetchThreadCommentOptions,
    ) -> Result<Value, AniListError> {
        let query = forum::FETCH_COMMENT;
        let variables = json!(options);
        let variables_map = crate::utils::json_to_hashmap(variables);
        self.client.query(query, Some(&variables_map)).await
    }

    /// Fetch a single thread comment
    pub async fn fetch_comment_one(
        &self,
        options: FetchThreadCommentOneOptions,
    ) -> Result<Value, AniListError> {
        let query = forum::FETCH_COMMENT_ONE;
        let variables = json!(options);
        let variables_map = crate::utils::json_to_hashmap(variables);
        self.client.query(query, Some(&variables_map)).await
    }

    /// Create or update a thread
    pub async fn save(&self, options: SaveThreadOptions) -> Result<Value, AniListError> {
        let query = forum::SAVE;
        let variables = json!(options);
        let variables_map = crate::utils::json_to_hashmap(variables);
        self.client.query(query, Some(&variables_map)).await
    }

    /// Delete a thread
    pub async fn delete(&self, options: DeleteThreadOptions) -> Result<Value, AniListError> {
        let query = forum::DELETE;
        let variables = json!(options);
        let variables_map = crate::utils::json_to_hashmap(variables);
        self.client.query(query, Some(&variables_map)).await
    }

    /// Create or update a thread comment
    pub async fn save_comment(
        &self,
        options: SaveThreadCommentOptions,
    ) -> Result<Value, AniListError> {
        let query = forum::SAVE_COMMENT;
        let variables = json!(options);
        let variables_map = crate::utils::json_to_hashmap(variables);
        self.client.query(query, Some(&variables_map)).await
    }

    /// Delete a thread comment
    pub async fn delete_comment(
        &self,
        options: DeleteThreadCommentOptions,
    ) -> Result<Value, AniListError> {
        let query = forum::DELETE_COMMENT;
        let variables = json!(options);
        let variables_map = crate::utils::json_to_hashmap(variables);
        self.client.query(query, Some(&variables_map)).await
    }

    /// Toggle thread subscription
    pub async fn subscription(
        &self,
        options: ToggleThreadSubscriptionOptions,
    ) -> Result<Value, AniListError> {
        let query = forum::SUBSCRIPTION;
        let variables = json!(options);
        let variables_map = crate::utils::json_to_hashmap(variables);
        self.client.query(query, Some(&variables_map)).await
    }
}
