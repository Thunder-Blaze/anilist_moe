use crate::enums::thread::{ThreadCommentSort, ThreadSort};
use crate::errors::AniListError;
use crate::objects::common::Deleted;
use crate::objects::responses::Page;
use crate::objects::thread::{Thread, ThreadComment};
use crate::{client::AniListClient, queries::forum};
use serde::Serialize;
use serde_with::skip_serializing_none;

/// Options for fetching forum threads.
#[skip_serializing_none]
#[derive(Default, Debug, Serialize)]
pub struct FetchThreadOptions<'a> {
    pub id: Option<i32>,
    pub search: Option<&'a str>,
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
    pub sort: Option<&'a [ThreadSort]>,
    pub page: Option<i32>,
    #[serde(rename = "perPage")]
    pub per_page: Option<i32>,
    // Extra
    #[serde(rename = "includeBody")]
    pub include_body: Option<bool>,
    #[serde(rename = "includeUser")]
    pub include_user: Option<bool>,
    #[serde(rename = "includeReplyUser")]
    pub include_reply_user: Option<bool>,
    #[serde(rename = "includeCategories")]
    pub include_categories: Option<bool>,
    #[serde(rename = "includeMediaCategories")]
    pub include_media_categories: Option<bool>,
    // HTML rendering options
    #[serde(rename = "body_as_html")]
    pub body_as_html: Option<bool>,
}

/// Options for fetching a single forum thread by ID.
#[skip_serializing_none]
#[derive(Default, Debug, Serialize)]
pub struct FetchThreadOneOptions<'a> {
    pub id: Option<i32>,
    #[serde(rename = "commentsPage")]
    pub comments_page: Option<i32>,
    #[serde(rename = "commentsPerPage")]
    pub comments_per_page: Option<i32>,
    #[serde(rename = "commentsSort")]
    pub comments_sort: Option<&'a [ThreadCommentSort]>,
    // HTML rendering options
    #[serde(rename = "body_as_html")]
    pub body_as_html: Option<bool>,
}

/// Options for fetching thread comments.
#[skip_serializing_none]
#[derive(Default, Debug, Serialize)]
pub struct FetchThreadCommentOptions<'a> {
    pub id: Option<i32>,
    #[serde(rename = "threadId")]
    pub thread_id: Option<i32>,
    #[serde(rename = "userId")]
    pub user_id: Option<i32>,
    pub sort: Option<&'a [ThreadCommentSort]>,
    pub page: Option<i32>,
    #[serde(rename = "perPage")]
    pub per_page: Option<i32>,
    // HTML rendering options
    #[serde(rename = "comment_as_html")]
    pub comment_as_html: Option<bool>,
}

/// Options for fetching a single thread comment by ID.
#[skip_serializing_none]
#[derive(Default, Debug, Serialize)]
pub struct FetchThreadCommentOneOptions {
    pub id: Option<i32>,
    // HTML rendering options
    #[serde(rename = "comment_as_html")]
    pub comment_as_html: Option<bool>,
}

/// Options for creating or updating a forum thread.
#[skip_serializing_none]
#[derive(Default, Debug, Serialize)]
pub struct SaveThreadOptions<'a> {
    pub id: Option<i32>,
    pub title: Option<&'a str>,
    pub body: Option<&'a str>,
    pub categories: Option<&'a [i32]>,
    #[serde(rename = "mediaCategories")]
    pub media_categories: Option<&'a [i32]>,
    pub sticky: Option<bool>,
    pub locked: Option<bool>,
    // HTML rendering options
    #[serde(rename = "body_as_html")]
    pub body_as_html: Option<bool>,
}

/// Options for deleting a forum thread.
#[derive(Default, Debug, Serialize)]
pub struct DeleteThreadOptions {
    pub id: i32,
}

/// Options for creating or updating a thread comment.
#[skip_serializing_none]
#[derive(Default, Debug, Serialize)]
pub struct SaveThreadCommentOptions<'a> {
    pub id: Option<i32>,
    #[serde(rename = "threadId")]
    pub thread_id: Option<i32>,
    #[serde(rename = "parentCommentId")]
    pub parent_comment_id: Option<i32>,
    pub comment: Option<&'a str>,
    pub locked: Option<bool>,
    // HTML rendering options
    #[serde(rename = "comment_as_html")]
    pub comment_as_html: Option<bool>,
}

/// Options for deleting a thread comment.
#[derive(Default, Debug, Serialize)]
pub struct DeleteThreadCommentOptions {
    pub id: i32,
}

/// Options for subscribing or unsubscribing to a thread.
#[skip_serializing_none]
#[derive(Default, Debug, Serialize)]
pub struct ToggleThreadSubscriptionOptions {
    #[serde(rename = "threadId")]
    pub thread_id: i32,
    pub subscribe: Option<bool>,
}

/// Endpoint for forum thread and comment operations.
pub struct ForumEndpoint<'a> {
    client: &'a AniListClient,
}

impl<'a> ForumEndpoint<'a> {
    #[must_use]
    pub const fn new(client: &'a AniListClient) -> Self {
        Self { client }
    }

    /// Fetch multiple threads with pagination
    pub async fn fetch(
        &self,
        options: &FetchThreadOptions<'_>,
    ) -> Result<Page<Vec<Thread>>, AniListError> {
        let query = forum::FETCH;
        self.client.fetch(query, Some(&options)).await
    }

    /// Fetch a single thread with full details
    pub async fn fetch_one(
        &self,
        options: &FetchThreadOneOptions<'_>,
    ) -> Result<Thread, AniListError> {
        let query: &str = forum::FETCH_ONE;
        self.client.fetch(query, Some(&options)).await
    }

    /// Fetch multiple thread comments with pagination
    pub async fn fetch_comments(
        &self,
        options: &FetchThreadCommentOptions<'_>,
    ) -> Result<Page<Vec<ThreadComment>>, AniListError> {
        let query = forum::FETCH_COMMENT;
        self.client.fetch(query, Some(&options)).await
    }

    /// Fetch a single thread comment
    pub async fn fetch_comment_one(
        &self,
        options: &FetchThreadCommentOneOptions,
    ) -> Result<ThreadComment, AniListError> {
        let query = forum::FETCH_COMMENT_ONE;
        self.client.fetch(query, Some(&options)).await
    }

    /// Create or update a thread
    pub async fn save(&self, options: &SaveThreadOptions<'_>) -> Result<Thread, AniListError> {
        let query = forum::SAVE;
        self.client.fetch(query, Some(&options)).await
    }

    /// Delete a thread
    pub async fn delete(&self, options: &DeleteThreadOptions) -> Result<bool, AniListError> {
        let query = forum::DELETE;
        let response: Result<Deleted, AniListError> =
            self.client.fetch(query, Some(&options)).await;
        match response {
            Ok(res) => Ok(res.deleted.unwrap_or_default()),
            Err(err) => Err(err),
        }
    }

    /// Create or update a thread comment
    pub async fn save_comment(
        &self,
        options: &SaveThreadCommentOptions<'_>,
    ) -> Result<ThreadComment, AniListError> {
        let query = forum::SAVE_COMMENT;
        self.client.fetch(query, Some(&options)).await
    }

    /// Delete a thread comment
    pub async fn delete_comment(
        &self,
        options: &DeleteThreadCommentOptions,
    ) -> Result<bool, AniListError> {
        let query = forum::DELETE_COMMENT;
        let response: Result<Deleted, AniListError> =
            self.client.fetch(query, Some(&options)).await;
        match response {
            Ok(res) => Ok(res.deleted.unwrap_or_default()),
            Err(err) => Err(err),
        }
    }

    /// Toggle thread subscription
    pub async fn subscription(
        &self,
        options: &ToggleThreadSubscriptionOptions,
    ) -> Result<Thread, AniListError> {
        let query = forum::SUBSCRIPTION;
        self.client.fetch(query, Some(&options)).await
    }

    // Convenience functions

    /// Get recent threads
    pub async fn get_recent(
        &self,
        page: Option<i32>,
        per_page: Option<i32>,
    ) -> Result<Page<Vec<Thread>>, AniListError> {
        self.fetch(&FetchThreadOptions {
            sort: Some(&[ThreadSort::CreatedAtDesc]),
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
            sort: Some(&[ThreadSort::RepliedAtDesc]),
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
            search: Some(query),
            sort: Some(&[ThreadSort::SearchMatch]),
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
            sort: Some(&[ThreadSort::RepliedAtDesc]),
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
            sort: Some(&[ThreadSort::CreatedAtDesc]),
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
            sort: Some(&[ThreadSort::RepliedAtDesc]),
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
            title: Some(title),
            body: Some(body),
            categories: Some(categories.as_slice()),
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
            title,
            body,
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
            sort: Some(&[ThreadCommentSort::Id]),
            page,
            per_page,
            ..Default::default()
        })
        .await
    }

    /// Get comment by ID
    pub async fn get_comment_by_id(&self, id: i32) -> Result<ThreadComment, AniListError> {
        self.fetch_comment_one(&FetchThreadCommentOneOptions {
            id: Some(id),
            ..Default::default()
        })
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
            comment: Some(comment),
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
            comment: Some(comment),
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
            comment: Some(comment),
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
