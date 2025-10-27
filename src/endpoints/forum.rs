use crate::enums::thread::{ThreadCommentSort, ThreadSort};
use crate::errors::AniListError;
use crate::objects::common::Deleted;
use crate::objects::responses::{GraphQLResponse, Page};
use crate::objects::thread::{Thread, ThreadComment};
use crate::{client::AniListClient, queries::forum};
use serde::{Deserialize, Serialize};
use serde_json::json;
use serde_with::skip_serializing_none;

/// Options for fetching forum threads.
#[skip_serializing_none]
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct FetchThreadOptions {
    pub id: Option<i32>,
    pub search: Option<String>,
    #[serde(rename = "userId")]
    pub user_id: Option<i32>,
    #[serde(rename = "replyUserId")]
    pub reply_user_id: Option<i32>,
    #[serde(rename = "subscribed")]
    pub subscribed: Option<bool>,
    #[serde(rename = "categoryId")]
    pub category_id: Option<i32>,
    #[serde(rename = "mediaCategoryId")]
    pub media_category_id: Option<i32>,
    pub sort: Option<Vec<ThreadSort>>,
    pub page: Option<i32>,
    #[serde(rename = "perPage")]
    pub per_page: Option<i32>,
}

/// Options for fetching a single forum thread by ID.
#[skip_serializing_none]
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct FetchThreadOneOptions {
    pub id: Option<i32>,
    #[serde(rename = "commentsPage")]
    pub comments_page: Option<i32>,
    #[serde(rename = "commentsPerPage")]
    pub comments_per_page: Option<i32>,
    #[serde(rename = "commentsSort")]
    pub comments_sort: Option<Vec<ThreadCommentSort>>,
}

/// Options for fetching thread comments.
#[skip_serializing_none]
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct FetchThreadCommentOptions {
    pub id: Option<i32>,
    #[serde(rename = "threadId")]
    pub thread_id: Option<i32>,
    #[serde(rename = "userId")]
    pub user_id: Option<i32>,
    pub sort: Option<Vec<ThreadCommentSort>>,
    pub page: Option<i32>,
    #[serde(rename = "perPage")]
    pub per_page: Option<i32>,
}

/// Options for fetching a single thread comment by ID.
#[skip_serializing_none]
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct FetchThreadCommentOneOptions {
    pub id: Option<i32>,
}

/// Options for creating or updating a forum thread.
#[skip_serializing_none]
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct SaveThreadOptions {
    pub id: Option<i32>,
    pub title: Option<String>,
    pub body: Option<String>,
    pub categories: Option<Vec<i32>>,
    #[serde(rename = "mediaCategories")]
    pub media_categories: Option<Vec<i32>>,
    pub sticky: Option<bool>,
    pub locked: Option<bool>,
}

/// Options for deleting a forum thread.
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct DeleteThreadOptions {
    pub id: i32,
}

/// Options for creating or updating a thread comment.
#[skip_serializing_none]
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct SaveThreadCommentOptions {
    pub id: Option<i32>,
    #[serde(rename = "threadId")]
    pub thread_id: Option<i32>,
    #[serde(rename = "parentCommentId")]
    pub parent_comment_id: Option<i32>,
    pub comment: Option<String>,
    pub locked: Option<bool>,
}

/// Options for deleting a thread comment.
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct DeleteThreadCommentOptions {
    pub id: i32,
}

/// Options for subscribing or unsubscribing to a thread.
#[skip_serializing_none]
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct ToggleThreadSubscriptionOptions {
    #[serde(rename = "threadId")]
    pub thread_id: i32,
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
        options: &FetchThreadOptions,
    ) -> Result<Page<Vec<Thread>>, AniListError> {
        let query = forum::FETCH;
        let variables = json!(options);
        let variables_map = crate::utils::json_to_hashmap(variables);
        let response: Result<GraphQLResponse<Page<Vec<Thread>>>, AniListError> =
            self.client.query_typed(query, Some(&variables_map)).await;
        match response {
            Ok(res) => Ok(res.data),
            Err(err) => Err(err),
        }
    }

    /// Fetch a single thread with full details
    pub async fn fetch_one(&self, options: &FetchThreadOneOptions) -> Result<Thread, AniListError> {
        let query: &str = forum::FETCH_ONE;
        let variables = json!(options);
        let variables_map = crate::utils::json_to_hashmap(variables);
        let response: Result<GraphQLResponse<Thread>, AniListError> =
            self.client.query_typed(query, Some(&variables_map)).await;
        match response {
            Ok(res) => Ok(res.data),
            Err(err) => Err(err),
        }
    }

    /// Fetch multiple thread comments with pagination
    pub async fn fetch_comments(
        &self,
        options: &FetchThreadCommentOptions,
    ) -> Result<Page<Vec<ThreadComment>>, AniListError> {
        let query = forum::FETCH_COMMENT;
        let variables = json!(options);
        let variables_map = crate::utils::json_to_hashmap(variables);
        let response: Result<GraphQLResponse<Page<Vec<ThreadComment>>>, AniListError> =
            self.client.query_typed(query, Some(&variables_map)).await;
        match response {
            Ok(res) => Ok(res.data),
            Err(err) => Err(err),
        }
    }

    /// Fetch a single thread comment
    pub async fn fetch_comment_one(
        &self,
        options: &FetchThreadCommentOneOptions,
    ) -> Result<ThreadComment, AniListError> {
        let query = forum::FETCH_COMMENT_ONE;
        let variables = json!(options);
        let variables_map = crate::utils::json_to_hashmap(variables);
        let response: Result<GraphQLResponse<ThreadComment>, AniListError> =
            self.client.query_typed(query, Some(&variables_map)).await;
        match response {
            Ok(res) => Ok(res.data),
            Err(err) => Err(err),
        }
    }

    /// Create or update a thread
    pub async fn save(&self, options: &SaveThreadOptions) -> Result<Thread, AniListError> {
        let query = forum::SAVE;
        let variables = json!(options);
        let variables_map = crate::utils::json_to_hashmap(variables);
        let response: Result<GraphQLResponse<Thread>, AniListError> =
            self.client.query_typed(query, Some(&variables_map)).await;
        match response {
            Ok(res) => Ok(res.data),
            Err(err) => Err(err),
        }
    }

    /// Delete a thread
    pub async fn delete(&self, options: &DeleteThreadOptions) -> Result<bool, AniListError> {
        let query = forum::DELETE;
        let variables = json!(options);
        let variables_map = crate::utils::json_to_hashmap(variables);
        let response: Result<GraphQLResponse<Deleted>, AniListError> =
            self.client.query_typed(query, Some(&variables_map)).await;
        match response {
            Ok(res) => Ok(res.data.deleted.unwrap_or_default()),
            Err(err) => Err(err),
        }
    }

    /// Create or update a thread comment
    pub async fn save_comment(
        &self,
        options: &SaveThreadCommentOptions,
    ) -> Result<ThreadComment, AniListError> {
        let query = forum::SAVE_COMMENT;
        let variables = json!(options);
        let variables_map = crate::utils::json_to_hashmap(variables);
        let response: Result<GraphQLResponse<ThreadComment>, AniListError> =
            self.client.query_typed(query, Some(&variables_map)).await;
        match response {
            Ok(res) => Ok(res.data),
            Err(err) => Err(err),
        }
    }

    /// Delete a thread comment
    pub async fn delete_comment(
        &self,
        options: &DeleteThreadCommentOptions,
    ) -> Result<bool, AniListError> {
        let query = forum::DELETE_COMMENT;
        let variables = json!(options);
        let variables_map = crate::utils::json_to_hashmap(variables);
        let response: Result<GraphQLResponse<Deleted>, AniListError> =
            self.client.query_typed(query, Some(&variables_map)).await;
        match response {
            Ok(res) => Ok(res.data.deleted.unwrap_or_default()),
            Err(err) => Err(err),
        }
    }

    /// Toggle thread subscription
    pub async fn subscription(
        &self,
        options: &ToggleThreadSubscriptionOptions,
    ) -> Result<Thread, AniListError> {
        let query = forum::SUBSCRIPTION;
        let variables = json!(options);
        let variables_map = crate::utils::json_to_hashmap(variables);
        let response: Result<GraphQLResponse<Thread>, AniListError> =
            self.client.query_typed(query, Some(&variables_map)).await;
        match response {
            Ok(res) => Ok(res.data),
            Err(err) => Err(err),
        }
    }

    // Convenience functions

    /// Get recent threads
    pub async fn get_recent(
        &self,
        page: Option<i32>,
        per_page: Option<i32>,
    ) -> Result<Page<Vec<Thread>>, AniListError> {
        self.fetch(&FetchThreadOptions {
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
    ) -> Result<Page<Vec<Thread>>, AniListError> {
        self.fetch(&FetchThreadOptions {
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
    ) -> Result<Page<Vec<Thread>>, AniListError> {
        self.fetch(&FetchThreadOptions {
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
    ) -> Result<Page<Vec<Thread>>, AniListError> {
        self.fetch(&FetchThreadOptions {
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
    ) -> Result<Page<Vec<Thread>>, AniListError> {
        self.fetch(&FetchThreadOptions {
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
    ) -> Result<Page<Vec<Thread>>, AniListError> {
        self.fetch(&FetchThreadOptions {
            subscribed: Some(true),
            sort: Some(vec![ThreadSort::RepliedAtDesc]),
            page,
            per_page,
            ..Default::default()
        })
        .await
    }

    /// Get thread by ID
    pub async fn get_by_id(&self, id: i32) -> Result<Thread, AniListError> {
        self.fetch_one(&FetchThreadOneOptions {
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
    ) -> Result<Thread, AniListError> {
        self.save(&SaveThreadOptions {
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
    ) -> Result<Thread, AniListError> {
        self.save(&SaveThreadOptions {
            id: Some(id),
            title: title.map(|s| s.to_string()),
            body: body.map(|s| s.to_string()),
            ..Default::default()
        })
        .await
    }

    /// Delete a thread
    pub async fn delete_thread(&self, id: i32) -> Result<bool, AniListError> {
        self.delete(&DeleteThreadOptions { id }).await
    }

    /// Get comments for a thread
    pub async fn get_thread_comments(
        &self,
        thread_id: i32,
        page: Option<i32>,
        per_page: Option<i32>,
    ) -> Result<Page<Vec<ThreadComment>>, AniListError> {
        self.fetch_comments(&FetchThreadCommentOptions {
            thread_id: Some(thread_id),
            sort: Some(vec![ThreadCommentSort::Id]),
            page,
            per_page,
            ..Default::default()
        })
        .await
    }

    /// Get comment by ID
    pub async fn get_comment_by_id(&self, id: i32) -> Result<ThreadComment, AniListError> {
        self.fetch_comment_one(&FetchThreadCommentOneOptions { id: Some(id) })
            .await
    }

    /// Create a thread comment (reply)
    pub async fn reply_to_thread(
        &self,
        thread_id: i32,
        comment: &str,
    ) -> Result<ThreadComment, AniListError> {
        self.save_comment(&SaveThreadCommentOptions {
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
    ) -> Result<ThreadComment, AniListError> {
        self.save_comment(&SaveThreadCommentOptions {
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
    ) -> Result<ThreadComment, AniListError> {
        self.save_comment(&SaveThreadCommentOptions {
            id: Some(id),
            comment: Some(comment.to_string()),
            ..Default::default()
        })
        .await
    }

    /// Delete a comment
    pub async fn delete_thread_comment(&self, id: i32) -> Result<bool, AniListError> {
        self.delete_comment(&DeleteThreadCommentOptions { id })
            .await
    }

    /// Subscribe to a thread
    pub async fn subscribe_to_thread(&self, thread_id: i32) -> Result<Thread, AniListError> {
        self.subscription(&ToggleThreadSubscriptionOptions {
            thread_id,
            subscribe: Some(true),
        })
        .await
    }

    /// Unsubscribe from a thread
    pub async fn unsubscribe_from_thread(&self, thread_id: i32) -> Result<Thread, AniListError> {
        self.subscription(&ToggleThreadSubscriptionOptions {
            thread_id,
            subscribe: Some(false),
        })
        .await
    }

    /// Toggle thread subscription
    pub async fn toggle_thread_subscription(&self, thread_id: i32) -> Result<Thread, AniListError> {
        self.subscription(&ToggleThreadSubscriptionOptions {
            thread_id,
            subscribe: None,
        })
        .await
    }
}
