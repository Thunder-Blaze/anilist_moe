use crate::enums::thread::{ThreadCommentSort, ThreadSort};
use crate::errors::AniListError;
use crate::objects::common::Deleted;
use crate::objects::responses::Page;
use crate::objects::thread::{Thread, ThreadComment};
use crate::options;
#[cfg(feature = "pagination")]
use crate::pagination::Paginated;
use crate::{client::AniListClient, queries::forum};
use serde::Serialize;
use serde_with::skip_serializing_none;
use std::borrow::Cow;

/// Options for fetching forum threads.
#[skip_serializing_none]
#[derive(Default, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FetchThreadOptions<'a> {
    pub id: Option<u32>,
    pub search: Option<Cow<'a, str>>,
    pub id_in: Option<Cow<'a, [u32]>>,
    pub user_id: Option<u32>,
    pub reply_user_id: Option<u32>,
    pub subscribed: Option<bool>,
    pub category_id: Option<u32>,
    pub media_category_id: Option<u32>,
    /// Sort order. The unique key (id) is appended, so paging stays stable;
    /// leaving this unset sorts by the same key alone.
    #[serde(
        serialize_with = "crate::options::serialize_sort",
        skip_serializing_if = "crate::options::skip_never"
    )]
    pub sort: Option<Cow<'a, [ThreadSort]>>,
    pub page: Option<i32>,
    pub per_page: Option<i32>,
    // Extra
    pub include_body: Option<bool>,
    pub include_user: Option<bool>,
    pub include_reply_user: Option<bool>,
    pub include_categories: Option<bool>,
    pub include_media_categories: Option<bool>,
    // HTML rendering options
    pub body_as_html: Option<bool>,
}

/// Options for fetching a single forum thread by ID.
#[skip_serializing_none]
#[derive(Default, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FetchThreadOneOptions<'a> {
    pub id: Option<u32>,
    pub user_id: Option<u32>,
    pub reply_user_id: Option<u32>,
    pub subscribed: Option<bool>,
    pub category_id: Option<u32>,
    pub media_category_id: Option<u32>,
    pub search: Option<Cow<'a, str>>,
    pub id_in: Option<Cow<'a, [u32]>>,
    pub sort: Option<Cow<'a, [ThreadSort]>>,
    // HTML rendering options
    pub body_as_html: Option<bool>,
}

/// Options for fetching thread comments.
#[skip_serializing_none]
#[derive(Default, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FetchThreadCommentOptions<'a> {
    pub id: Option<u32>,
    pub thread_id: Option<u32>,
    pub user_id: Option<u32>,
    /// Sort order. The unique key (id) is appended, so paging stays stable;
    /// leaving this unset sorts by the same key alone.
    #[serde(
        serialize_with = "crate::options::serialize_sort",
        skip_serializing_if = "crate::options::skip_never"
    )]
    pub sort: Option<Cow<'a, [ThreadCommentSort]>>,
    pub page: Option<i32>,
    pub per_page: Option<i32>,
    // HTML rendering options
    pub comment_as_html: Option<bool>,
}

/// Options for fetching a single thread comment by ID.
#[skip_serializing_none]
#[derive(Default, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FetchThreadCommentOneOptions<'a> {
    pub id: Option<u32>,
    pub thread_id: Option<u32>,
    pub user_id: Option<u32>,
    pub sort: Option<Cow<'a, [ThreadCommentSort]>>,
    // HTML rendering options
    pub comment_as_html: Option<bool>,
}

/// Options for creating or updating a forum thread.
#[skip_serializing_none]
#[derive(Default, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SaveThreadOptions<'a> {
    pub id: Option<u32>,
    pub title: Option<Cow<'a, str>>,
    pub body: Option<Cow<'a, str>>,
    pub categories: Option<Cow<'a, [u32]>>,
    pub media_categories: Option<Cow<'a, [u32]>>,
    pub sticky: Option<bool>,
    pub locked: Option<bool>,
    // HTML rendering options
    pub body_as_html: Option<bool>,
}

/// Options for deleting a forum thread.
#[derive(Default, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteThreadOptions {
    pub id: u32,
}

/// Options for creating or updating a thread comment.
#[skip_serializing_none]
#[derive(Default, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SaveThreadCommentOptions<'a> {
    pub id: Option<u32>,
    pub thread_id: Option<u32>,
    pub parent_comment_id: Option<u32>,
    pub comment: Option<Cow<'a, str>>,
    pub locked: Option<bool>,
    // HTML rendering options
    pub comment_as_html: Option<bool>,
}

/// Options for deleting a thread comment.
#[derive(Default, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteThreadCommentOptions {
    pub id: u32,
}

/// Options for subscribing or unsubscribing to a thread.
#[skip_serializing_none]
#[derive(Default, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ToggleThreadSubscriptionOptions {
    pub thread_id: u32,
    pub subscribe: Option<bool>,
}

/// Endpoint for forum thread and comment operations.
pub struct ForumEndpoint<'a> {
    client: &'a AniListClient,
}

impl<'a> ForumEndpoint<'a> {
    pub fn new(client: &'a AniListClient) -> Self {
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
        options: &FetchThreadCommentOneOptions<'_>,
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
        self.fetch(&options!(
            FetchThreadOptions,
            sort: Some(&[ThreadSort::IdDesc]),
            page,
            per_page,
        ))
        .await
    }

    /// Get trending/popular threads
    pub async fn get_popular(
        &self,
        page: Option<i32>,
        per_page: Option<i32>,
    ) -> Result<Page<Vec<Thread>>, AniListError> {
        self.fetch(&options!(
            FetchThreadOptions,
            sort: Some(&[ThreadSort::RepliedAtDesc]),
            page,
            per_page,
        ))
        .await
    }

    /// Search threads by query
    pub async fn search(
        &self,
        query: &str,
        page: Option<i32>,
        per_page: Option<i32>,
    ) -> Result<Page<Vec<Thread>>, AniListError> {
        self.fetch(&options!(
            FetchThreadOptions,
            search: Some(query),
            sort: Some(&[ThreadSort::SearchMatch]),
            page,
            per_page,
        ))
        .await
    }

    /// Get threads by category
    pub async fn get_by_category(
        &self,
        category_id: u32,
        page: Option<i32>,
        per_page: Option<i32>,
    ) -> Result<Page<Vec<Thread>>, AniListError> {
        self.fetch(&options!(
            FetchThreadOptions,
            category_id: Some(category_id),
            sort: Some(&[ThreadSort::RepliedAtDesc]),
            page,
            per_page,
        ))
        .await
    }

    /// Get threads by user
    pub async fn get_by_user(
        &self,
        user_id: u32,
        page: Option<i32>,
        per_page: Option<i32>,
    ) -> Result<Page<Vec<Thread>>, AniListError> {
        self.fetch(&options!(
            FetchThreadOptions,
            user_id: Some(user_id),
            sort: Some(&[ThreadSort::IdDesc]),
            page,
            per_page,
        ))
        .await
    }

    /// Get subscribed threads (requires authentication)
    pub async fn get_subscribed(
        &self,
        page: Option<i32>,
        per_page: Option<i32>,
    ) -> Result<Page<Vec<Thread>>, AniListError> {
        self.fetch(&options!(
            FetchThreadOptions,
            subscribed: Some(true),
            sort: Some(&[ThreadSort::RepliedAtDesc]),
            page,
            per_page,
        ))
        .await
    }

    /// Get thread by ID
    pub async fn get_by_id(&self, id: u32) -> Result<Thread, AniListError> {
        self.fetch_one(&options!(
            FetchThreadOneOptions,
            id: Some(id),
        ))
        .await
    }

    /// Create a new thread
    pub async fn create_thread(
        &self,
        title: &str,
        body: &str,
        categories: Vec<u32>,
    ) -> Result<Thread, AniListError> {
        self.save(&options!(
            SaveThreadOptions,
            title: Some(title),
            body: Some(body),
            categories: Some(categories.as_slice()),
        ))
        .await
    }

    /// Update an existing thread
    pub async fn update_thread(
        &self,
        id: u32,
        title: Option<&str>,
        body: Option<&str>,
    ) -> Result<Thread, AniListError> {
        self.save(&options!(
            SaveThreadOptions,
            id: Some(id),
            title,
            body,
        ))
        .await
    }

    /// Delete a thread
    pub async fn delete_thread(&self, id: u32) -> Result<bool, AniListError> {
        self.delete(&options!(
            DeleteThreadOptions,
            id => id,
        ))
        .await
    }

    /// Get comments for a thread
    pub async fn get_thread_comments(
        &self,
        thread_id: u32,
        page: Option<i32>,
        per_page: Option<i32>,
    ) -> Result<Page<Vec<ThreadComment>>, AniListError> {
        self.fetch_comments(&options!(
            FetchThreadCommentOptions,
            thread_id: Some(thread_id),
            sort: Some(&[ThreadCommentSort::Id]),
            page,
            per_page,
        ))
        .await
    }

    /// Get comment by ID
    pub async fn get_comment_by_id(&self, id: u32) -> Result<ThreadComment, AniListError> {
        self.fetch_comment_one(&options!(
            FetchThreadCommentOneOptions,
            id: Some(id),
        ))
        .await
    }

    /// Create a thread comment (reply)
    pub async fn reply_to_thread(
        &self,
        thread_id: u32,
        comment: &str,
    ) -> Result<ThreadComment, AniListError> {
        self.save_comment(&options!(
            SaveThreadCommentOptions,
            thread_id: Some(thread_id),
            comment: Some(comment),
        ))
        .await
    }

    /// Reply to a comment
    pub async fn reply_to_comment(
        &self,
        thread_id: u32,
        parent_comment_id: u32,
        comment: &str,
    ) -> Result<ThreadComment, AniListError> {
        self.save_comment(&options!(
            SaveThreadCommentOptions,
            thread_id: Some(thread_id),
            parent_comment_id: Some(parent_comment_id),
            comment: Some(comment),
        ))
        .await
    }

    /// Update a comment
    pub async fn update_comment(
        &self,
        id: u32,
        comment: &str,
    ) -> Result<ThreadComment, AniListError> {
        self.save_comment(&options!(
            SaveThreadCommentOptions,
            id: Some(id),
            comment: Some(comment),
        ))
        .await
    }

    /// Delete a comment
    pub async fn delete_thread_comment(&self, id: u32) -> Result<bool, AniListError> {
        self.delete_comment(&options!(
            DeleteThreadCommentOptions,
            id => id,
        ))
        .await
    }

    /// Subscribe to a thread
    pub async fn subscribe_to_thread(&self, thread_id: u32) -> Result<Thread, AniListError> {
        self.subscription(&options!(
            ToggleThreadSubscriptionOptions,
            thread_id => thread_id,
            subscribe: Some(true),
        ))
        .await
    }

    /// Unsubscribe from a thread
    pub async fn unsubscribe_from_thread(&self, thread_id: u32) -> Result<Thread, AniListError> {
        self.subscription(&options!(
            ToggleThreadSubscriptionOptions,
            thread_id => thread_id,
            subscribe: Some(false),
        ))
        .await
    }

    /// Toggle thread subscription
    pub async fn toggle_thread_subscription(&self, thread_id: u32) -> Result<Thread, AniListError> {
        self.subscription(&options!(
            ToggleThreadSubscriptionOptions,
            thread_id => thread_id,
        ))
        .await
    }
}

#[cfg(feature = "pagination")]
impl<'a> Paginated for FetchThreadOptions<'a> {
    type Item = Thread;

    async fn fetch(&self, client: &AniListClient) -> Result<Page<Vec<Thread>>, AniListError> {
        client.forum().fetch(self).await
    }

    fn set_page(&mut self, page: i32) {
        self.page = Some(page);
    }
}

#[cfg(feature = "pagination")]
impl<'a> Paginated for FetchThreadCommentOptions<'a> {
    type Item = ThreadComment;

    async fn fetch(
        &self,
        client: &AniListClient,
    ) -> Result<Page<Vec<ThreadComment>>, AniListError> {
        client.forum().fetch_comments(self).await
    }

    fn set_page(&mut self, page: i32) {
        self.page = Some(page);
    }
}
