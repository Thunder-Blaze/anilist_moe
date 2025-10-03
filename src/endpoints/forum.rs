use crate::enums::thread::{ThreadCommentSort, ThreadSort};
use crate::errors::AniListError;
use crate::objects::responses::{
    DeleteThreadCommentResponse, DeleteThreadResponse, SaveThreadCommentResponse,
    SaveThreadResponse, ThreadCommentListResponse, ThreadCommentSingleResponse, ThreadListResponse,
    ThreadSingleResponse, ToggleThreadSubscriptionResponse,
};
use crate::{client::AniListClient, queries::forum};
use serde::Serialize;
use serde_json::json;

/// Options for fetching forum threads.
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

/// Options for fetching a single forum thread by ID.
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

/// Options for fetching thread comments.
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

/// Options for fetching a single thread comment by ID.
#[derive(Default, Serialize)]
pub struct FetchThreadCommentOneOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
}

/// Options for creating or updating a forum thread.
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

/// Options for deleting a forum thread.
#[derive(Default, Serialize)]
pub struct DeleteThreadOptions {
    pub id: i32,
}

/// Options for creating or updating a thread comment.
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

/// Options for deleting a thread comment.
#[derive(Default, Serialize)]
pub struct DeleteThreadCommentOptions {
    pub id: i32,
}

/// Options for subscribing or unsubscribing to a thread.
#[derive(Default, Serialize)]
pub struct ToggleThreadSubscriptionOptions {
    #[serde(rename = "threadId")]
    pub thread_id: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscribe: Option<bool>,
}

/// Endpoint for forum thread and comment operations.
pub struct ForumEndpoint {
    client: AniListClient,
}

impl ForumEndpoint {
    pub fn new(client: AniListClient) -> Self {
        Self { client }
    }

    /// Fetch multiple threads with pagination
    pub async fn fetch(
        &self,
        options: FetchThreadOptions,
    ) -> Result<ThreadListResponse, AniListError> {
        let query = forum::FETCH;
        let variables = json!(options);
        let variables_map = crate::utils::json_to_hashmap(variables);
        self.client.query_typed(query, Some(&variables_map)).await
    }

    /// Fetch a single thread with full details
    pub async fn fetch_one(
        &self,
        options: FetchThreadOneOptions,
    ) -> Result<ThreadSingleResponse, AniListError> {
        let query = forum::FETCH_ONE;
        let variables = json!(options);
        let variables_map = crate::utils::json_to_hashmap(variables);
        self.client.query_typed(query, Some(&variables_map)).await
    }

    /// Fetch multiple thread comments with pagination
    pub async fn fetch_comments(
        &self,
        options: FetchThreadCommentOptions,
    ) -> Result<ThreadCommentListResponse, AniListError> {
        let query = forum::FETCH_COMMENT;
        let variables = json!(options);
        let variables_map = crate::utils::json_to_hashmap(variables);
        self.client.query_typed(query, Some(&variables_map)).await
    }

    /// Fetch a single thread comment
    pub async fn fetch_comment_one(
        &self,
        options: FetchThreadCommentOneOptions,
    ) -> Result<ThreadCommentSingleResponse, AniListError> {
        let query = forum::FETCH_COMMENT_ONE;
        let variables = json!(options);
        let variables_map = crate::utils::json_to_hashmap(variables);
        self.client.query_typed(query, Some(&variables_map)).await
    }

    /// Create or update a thread
    pub async fn save(
        &self,
        options: SaveThreadOptions,
    ) -> Result<SaveThreadResponse, AniListError> {
        let query = forum::SAVE;
        let variables = json!(options);
        let variables_map = crate::utils::json_to_hashmap(variables);
        self.client.query_typed(query, Some(&variables_map)).await
    }

    /// Delete a thread
    pub async fn delete(
        &self,
        options: DeleteThreadOptions,
    ) -> Result<DeleteThreadResponse, AniListError> {
        let query = forum::DELETE;
        let variables = json!(options);
        let variables_map = crate::utils::json_to_hashmap(variables);
        self.client.query_typed(query, Some(&variables_map)).await
    }

    /// Create or update a thread comment
    pub async fn save_comment(
        &self,
        options: SaveThreadCommentOptions,
    ) -> Result<SaveThreadCommentResponse, AniListError> {
        let query = forum::SAVE_COMMENT;
        let variables = json!(options);
        let variables_map = crate::utils::json_to_hashmap(variables);
        self.client.query_typed(query, Some(&variables_map)).await
    }

    /// Delete a thread comment
    pub async fn delete_comment(
        &self,
        options: DeleteThreadCommentOptions,
    ) -> Result<DeleteThreadCommentResponse, AniListError> {
        let query = forum::DELETE_COMMENT;
        let variables = json!(options);
        let variables_map = crate::utils::json_to_hashmap(variables);
        self.client.query_typed(query, Some(&variables_map)).await
    }

    /// Toggle thread subscription
    pub async fn subscription(
        &self,
        options: ToggleThreadSubscriptionOptions,
    ) -> Result<ToggleThreadSubscriptionResponse, AniListError> {
        let query = forum::SUBSCRIPTION;
        let variables = json!(options);
        let variables_map = crate::utils::json_to_hashmap(variables);
        self.client.query_typed(query, Some(&variables_map)).await
    }

    // Convenience functions

    /// Get recent threads
    pub async fn get_recent(
        &self,
        page: Option<i32>,
        per_page: Option<i32>,
    ) -> Result<ThreadListResponse, AniListError> {
        self.fetch(FetchThreadOptions {
            sort: Some(vec![ThreadSort::CreatedAtDesc]),
            page,
            per_page,
            ..Default::default()
        })
        .await
    }

    /// Get trending/popular threads
    pub async fn get_popular(
        &self,
        page: Option<i32>,
        per_page: Option<i32>,
    ) -> Result<ThreadListResponse, AniListError> {
        self.fetch(FetchThreadOptions {
            sort: Some(vec![ThreadSort::RepliedAtDesc]),
            page,
            per_page,
            ..Default::default()
        })
        .await
    }

    /// Search threads by query
    pub async fn search(
        &self,
        query: &str,
        page: Option<i32>,
        per_page: Option<i32>,
    ) -> Result<ThreadListResponse, AniListError> {
        self.fetch(FetchThreadOptions {
            search: Some(query.to_string()),
            sort: Some(vec![ThreadSort::SearchMatch]),
            page,
            per_page,
            ..Default::default()
        })
        .await
    }

    /// Get threads by category
    pub async fn get_by_category(
        &self,
        category_id: i32,
        page: Option<i32>,
        per_page: Option<i32>,
    ) -> Result<ThreadListResponse, AniListError> {
        self.fetch(FetchThreadOptions {
            category_id: Some(category_id),
            sort: Some(vec![ThreadSort::RepliedAtDesc]),
            page,
            per_page,
            ..Default::default()
        })
        .await
    }

    /// Get threads by user
    pub async fn get_by_user(
        &self,
        user_id: i32,
        page: Option<i32>,
        per_page: Option<i32>,
    ) -> Result<ThreadListResponse, AniListError> {
        self.fetch(FetchThreadOptions {
            user_id: Some(user_id),
            sort: Some(vec![ThreadSort::CreatedAtDesc]),
            page,
            per_page,
            ..Default::default()
        })
        .await
    }

    /// Get subscribed threads (requires authentication)
    pub async fn get_subscribed(
        &self,
        page: Option<i32>,
        per_page: Option<i32>,
    ) -> Result<ThreadListResponse, AniListError> {
        self.fetch(FetchThreadOptions {
            subscribed: Some(true),
            sort: Some(vec![ThreadSort::RepliedAtDesc]),
            page,
            per_page,
            ..Default::default()
        })
        .await
    }

    /// Get thread by ID
    pub async fn get_by_id(&self, id: i32) -> Result<ThreadSingleResponse, AniListError> {
        self.fetch_one(FetchThreadOneOptions {
            id: Some(id),
            ..Default::default()
        })
        .await
    }

    /// Create a new thread
    pub async fn create_thread(
        &self,
        title: &str,
        body: &str,
        categories: Vec<i32>,
    ) -> Result<SaveThreadResponse, AniListError> {
        self.save(SaveThreadOptions {
            id: None,
            title: Some(title.to_string()),
            body: Some(body.to_string()),
            categories: Some(categories),
            ..Default::default()
        })
        .await
    }

    /// Update an existing thread
    pub async fn update_thread(
        &self,
        id: i32,
        title: Option<&str>,
        body: Option<&str>,
    ) -> Result<SaveThreadResponse, AniListError> {
        self.save(SaveThreadOptions {
            id: Some(id),
            title: title.map(|s| s.to_string()),
            body: body.map(|s| s.to_string()),
            ..Default::default()
        })
        .await
    }

    /// Delete a thread
    pub async fn delete_thread(&self, id: i32) -> Result<DeleteThreadResponse, AniListError> {
        self.delete(DeleteThreadOptions { id }).await
    }

    /// Get comments for a thread
    pub async fn get_thread_comments(
        &self,
        thread_id: i32,
        page: Option<i32>,
        per_page: Option<i32>,
    ) -> Result<ThreadCommentListResponse, AniListError> {
        self.fetch_comments(FetchThreadCommentOptions {
            thread_id: Some(thread_id),
            sort: Some(vec![ThreadCommentSort::Id]),
            page,
            per_page,
            ..Default::default()
        })
        .await
    }

    /// Get comment by ID
    pub async fn get_comment_by_id(
        &self,
        id: i32,
    ) -> Result<ThreadCommentSingleResponse, AniListError> {
        self.fetch_comment_one(FetchThreadCommentOneOptions { id: Some(id) })
            .await
    }

    /// Create a thread comment (reply)
    pub async fn reply_to_thread(
        &self,
        thread_id: i32,
        comment: &str,
    ) -> Result<SaveThreadCommentResponse, AniListError> {
        self.save_comment(SaveThreadCommentOptions {
            id: None,
            thread_id: Some(thread_id),
            comment: Some(comment.to_string()),
            ..Default::default()
        })
        .await
    }

    /// Reply to a comment
    pub async fn reply_to_comment(
        &self,
        thread_id: i32,
        parent_comment_id: i32,
        comment: &str,
    ) -> Result<SaveThreadCommentResponse, AniListError> {
        self.save_comment(SaveThreadCommentOptions {
            id: None,
            thread_id: Some(thread_id),
            parent_comment_id: Some(parent_comment_id),
            comment: Some(comment.to_string()),
            ..Default::default()
        })
        .await
    }

    /// Update a comment
    pub async fn update_comment(
        &self,
        id: i32,
        comment: &str,
    ) -> Result<SaveThreadCommentResponse, AniListError> {
        self.save_comment(SaveThreadCommentOptions {
            id: Some(id),
            comment: Some(comment.to_string()),
            ..Default::default()
        })
        .await
    }

    /// Delete a comment
    pub async fn delete_thread_comment(
        &self,
        id: i32,
    ) -> Result<DeleteThreadCommentResponse, AniListError> {
        self.delete_comment(DeleteThreadCommentOptions { id }).await
    }

    /// Subscribe to a thread
    pub async fn subscribe_to_thread(
        &self,
        thread_id: i32,
    ) -> Result<ToggleThreadSubscriptionResponse, AniListError> {
        self.subscription(ToggleThreadSubscriptionOptions {
            thread_id,
            subscribe: Some(true),
        })
        .await
    }

    /// Unsubscribe from a thread
    pub async fn unsubscribe_from_thread(
        &self,
        thread_id: i32,
    ) -> Result<ToggleThreadSubscriptionResponse, AniListError> {
        self.subscription(ToggleThreadSubscriptionOptions {
            thread_id,
            subscribe: Some(false),
        })
        .await
    }

    /// Toggle thread subscription
    pub async fn toggle_thread_subscription(
        &self,
        thread_id: i32,
    ) -> Result<ToggleThreadSubscriptionResponse, AniListError> {
        self.subscription(ToggleThreadSubscriptionOptions {
            thread_id,
            subscribe: None,
        })
        .await
    }
}
