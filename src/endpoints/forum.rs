use crate::enums::thread::{ThreadCommentSort, ThreadSort};
use crate::errors::AniListError;
use crate::objects::common::Deleted;
use crate::objects::responses::Page;
use crate::objects::thread::{Thread, ThreadComment};
#[cfg(feature = "pagination")]
use crate::pagination::Paginated;
use crate::{client::AniListClient, queries::forum};
use serde::Serialize;
use serde_with::skip_serializing_none;
use std::borrow::Cow;
use typed_builder::TypedBuilder;

/// Options for fetching forum threads.
#[skip_serializing_none]
#[derive(Default, Debug, Serialize, TypedBuilder)]
#[builder(field_defaults(
    default,
    setter(strip_option(ignore_invalid, fallback_suffix = "_opt"))
))]
#[serde(rename_all = "camelCase")]
pub struct FetchThreadOptions<'a> {
    id: Option<u32>,
    search: Option<Cow<'a, str>>,
    id_in: Option<Cow<'a, [u32]>>,
    user_id: Option<u32>,
    reply_user_id: Option<u32>,
    subscribed: Option<bool>,
    category_id: Option<u32>,
    media_category_id: Option<u32>,
    /// Sort order. The unique key (id) is appended, so paging stays stable;
    /// leaving this unset sorts by the same key alone.
    #[serde(
        serialize_with = "crate::options::serialize_sort",
        skip_serializing_if = "crate::options::skip_never"
    )]
    sort: Option<Cow<'a, [ThreadSort]>>,
    page: Option<i32>,
    per_page: Option<i32>,
    // Extra
    include_body: Option<bool>,
    include_user: Option<bool>,
    include_reply_user: Option<bool>,
    include_categories: Option<bool>,
    include_media_categories: Option<bool>,
    // HTML rendering options
    body_as_html: Option<bool>,
}

/// Options for fetching a single forum thread by ID.
#[skip_serializing_none]
#[derive(Default, Debug, Serialize, TypedBuilder)]
#[builder(field_defaults(
    default,
    setter(strip_option(ignore_invalid, fallback_suffix = "_opt"))
))]
#[serde(rename_all = "camelCase")]
pub struct FetchThreadOneOptions<'a> {
    id: Option<u32>,
    user_id: Option<u32>,
    reply_user_id: Option<u32>,
    subscribed: Option<bool>,
    category_id: Option<u32>,
    media_category_id: Option<u32>,
    search: Option<Cow<'a, str>>,
    id_in: Option<Cow<'a, [u32]>>,
    sort: Option<Cow<'a, [ThreadSort]>>,
    // HTML rendering options
    body_as_html: Option<bool>,
}

/// Options for fetching thread comments.
#[skip_serializing_none]
#[derive(Default, Debug, Serialize, TypedBuilder)]
#[builder(field_defaults(
    default,
    setter(strip_option(ignore_invalid, fallback_suffix = "_opt"))
))]
#[serde(rename_all = "camelCase")]
pub struct FetchThreadCommentOptions<'a> {
    id: Option<u32>,
    thread_id: Option<u32>,
    user_id: Option<u32>,
    /// Sort order. The unique key (id) is appended, so paging stays stable;
    /// leaving this unset sorts by the same key alone.
    #[serde(
        serialize_with = "crate::options::serialize_sort",
        skip_serializing_if = "crate::options::skip_never"
    )]
    sort: Option<Cow<'a, [ThreadCommentSort]>>,
    page: Option<i32>,
    per_page: Option<i32>,
    // HTML rendering options
    comment_as_html: Option<bool>,
}

/// Options for fetching a single thread comment by ID.
#[skip_serializing_none]
#[derive(Default, Debug, Serialize, TypedBuilder)]
#[builder(field_defaults(
    default,
    setter(strip_option(ignore_invalid, fallback_suffix = "_opt"))
))]
#[serde(rename_all = "camelCase")]
pub struct FetchThreadCommentOneOptions<'a> {
    id: Option<u32>,
    thread_id: Option<u32>,
    user_id: Option<u32>,
    sort: Option<Cow<'a, [ThreadCommentSort]>>,
    // HTML rendering options
    comment_as_html: Option<bool>,
}

/// Options for creating or updating a forum thread.
#[skip_serializing_none]
#[derive(Default, Debug, Serialize, TypedBuilder)]
#[builder(field_defaults(
    default,
    setter(strip_option(ignore_invalid, fallback_suffix = "_opt"))
))]
#[serde(rename_all = "camelCase")]
pub struct SaveThreadOptions<'a> {
    id: Option<u32>,
    title: Option<Cow<'a, str>>,
    body: Option<Cow<'a, str>>,
    categories: Option<Cow<'a, [u32]>>,
    media_categories: Option<Cow<'a, [u32]>>,
    /// Mod only.
    sticky: Option<bool>,
    /// Mod only.
    locked: Option<bool>,
    // HTML rendering options
    body_as_html: Option<bool>,
}

/// Options for deleting a forum thread.
#[derive(Default, Debug, Serialize, TypedBuilder)]
#[serde(rename_all = "camelCase")]
pub struct DeleteThreadOptions {
    id: u32,
}

/// Options for creating or updating a thread comment.
#[skip_serializing_none]
#[derive(Default, Debug, Serialize, TypedBuilder)]
#[builder(field_defaults(
    default,
    setter(strip_option(ignore_invalid, fallback_suffix = "_opt"))
))]
#[serde(rename_all = "camelCase")]
pub struct SaveThreadCommentOptions<'a> {
    id: Option<u32>,
    thread_id: Option<u32>,
    parent_comment_id: Option<u32>,
    comment: Option<Cow<'a, str>>,
    /// Mod only.
    locked: Option<bool>,
    // HTML rendering options
    comment_as_html: Option<bool>,
}

/// Options for deleting a thread comment.
#[derive(Default, Debug, Serialize, TypedBuilder)]
#[serde(rename_all = "camelCase")]
pub struct DeleteThreadCommentOptions {
    id: u32,
}

/// Options for subscribing or unsubscribing to a thread.
///
/// The API's own validator requires `subscribe`, so it is not optional here:
/// leaving it out fails the request with "The subscribe field is required",
/// even though the reference lists the argument as nullable.
#[derive(Default, Debug, Serialize, TypedBuilder)]
#[serde(rename_all = "camelCase")]
pub struct ToggleThreadSubscriptionOptions {
    thread_id: u32,
    subscribe: bool,
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
        self.fetch(
            &FetchThreadOptions::builder()
                .sort(Cow::Borrowed(&[ThreadSort::IdDesc]))
                .page_opt(page)
                .per_page_opt(per_page)
                .build(),
        )
        .await
    }

    /// Get trending/popular threads
    pub async fn get_popular(
        &self,
        page: Option<i32>,
        per_page: Option<i32>,
    ) -> Result<Page<Vec<Thread>>, AniListError> {
        self.fetch(
            &FetchThreadOptions::builder()
                .sort(Cow::Borrowed(&[ThreadSort::RepliedAtDesc]))
                .page_opt(page)
                .per_page_opt(per_page)
                .build(),
        )
        .await
    }

    /// Search threads by query
    pub async fn search(
        &self,
        query: &str,
        page: Option<i32>,
        per_page: Option<i32>,
    ) -> Result<Page<Vec<Thread>>, AniListError> {
        self.fetch(
            &FetchThreadOptions::builder()
                .sort(Cow::Borrowed(&[ThreadSort::SearchMatch]))
                .search(Cow::Borrowed(query))
                .page_opt(page)
                .per_page_opt(per_page)
                .build(),
        )
        .await
    }

    /// Get threads by category
    pub async fn get_by_category(
        &self,
        category_id: u32,
        page: Option<i32>,
        per_page: Option<i32>,
    ) -> Result<Page<Vec<Thread>>, AniListError> {
        self.fetch(
            &FetchThreadOptions::builder()
                .category_id(category_id)
                .page_opt(page)
                .per_page_opt(per_page)
                .build(),
        )
        .await
    }

    /// Get threads by user
    pub async fn get_by_user(
        &self,
        user_id: u32,
        page: Option<i32>,
        per_page: Option<i32>,
    ) -> Result<Page<Vec<Thread>>, AniListError> {
        self.fetch(
            &FetchThreadOptions::builder()
                .sort(Cow::Borrowed(&[ThreadSort::IdDesc]))
                .user_id(user_id)
                .page_opt(page)
                .per_page_opt(per_page)
                .build(),
        )
        .await
    }

    /// Get subscribed threads (requires authentication)
    pub async fn get_subscribed(
        &self,
        page: Option<i32>,
        per_page: Option<i32>,
    ) -> Result<Page<Vec<Thread>>, AniListError> {
        self.fetch(
            &FetchThreadOptions::builder()
                .subscribed(true)
                .sort(Cow::Borrowed(&[ThreadSort::RepliedAtDesc]))
                .page_opt(page)
                .per_page_opt(per_page)
                .build(),
        )
        .await
    }

    /// Get thread by ID
    pub async fn get_by_id(&self, id: u32) -> Result<Thread, AniListError> {
        self.fetch_one(&FetchThreadOneOptions::builder().id(id).build())
            .await
    }

    /// Create a new thread
    pub async fn create_thread(
        &self,
        title: &str,
        body: &str,
        categories: Vec<u32>,
    ) -> Result<Thread, AniListError> {
        self.save(
            &SaveThreadOptions::builder()
                .title(Cow::Borrowed(title))
                .body(Cow::Borrowed(body))
                .categories(Cow::Owned(categories))
                .build(),
        )
        .await
    }

    /// Update an existing thread
    pub async fn update_thread(
        &self,
        id: u32,
        title: Option<&str>,
        body: Option<&str>,
    ) -> Result<Thread, AniListError> {
        self.save(
            &SaveThreadOptions::builder()
                .id(id)
                .title_opt(title.map(Cow::Borrowed))
                .body_opt(body.map(Cow::Borrowed))
                .build(),
        )
        .await
    }

    /// Delete a thread
    pub async fn delete_thread(&self, id: u32) -> Result<bool, AniListError> {
        self.delete(&DeleteThreadOptions::builder().id(id).build())
            .await
    }

    /// Get comments for a thread
    pub async fn get_thread_comments(
        &self,
        thread_id: u32,
        page: Option<i32>,
        per_page: Option<i32>,
    ) -> Result<Page<Vec<ThreadComment>>, AniListError> {
        self.fetch_comments(
            &FetchThreadCommentOptions::builder()
                .thread_id(thread_id)
                .sort(Cow::Borrowed(&[ThreadCommentSort::Id]))
                .page_opt(page)
                .per_page_opt(per_page)
                .build(),
        )
        .await
    }

    /// Get comment by ID
    pub async fn get_comment_by_id(&self, id: u32) -> Result<ThreadComment, AniListError> {
        self.fetch_comment_one(&FetchThreadCommentOneOptions::builder().id(id).build())
            .await
    }

    /// Create a thread comment (reply)
    pub async fn reply_to_thread(
        &self,
        thread_id: u32,
        comment: &str,
    ) -> Result<ThreadComment, AniListError> {
        self.save_comment(
            &SaveThreadCommentOptions::builder()
                .thread_id(thread_id)
                .comment(Cow::Borrowed(comment))
                .build(),
        )
        .await
    }

    /// Reply to a comment
    pub async fn reply_to_comment(
        &self,
        thread_id: u32,
        parent_comment_id: u32,
        comment: &str,
    ) -> Result<ThreadComment, AniListError> {
        self.save_comment(
            &SaveThreadCommentOptions::builder()
                .thread_id(thread_id)
                .parent_comment_id(parent_comment_id)
                .comment(Cow::Borrowed(comment))
                .build(),
        )
        .await
    }

    /// Update a comment
    pub async fn update_comment(
        &self,
        id: u32,
        comment: &str,
    ) -> Result<ThreadComment, AniListError> {
        self.save_comment(
            &SaveThreadCommentOptions::builder()
                .id(id)
                .comment(Cow::Borrowed(comment))
                .build(),
        )
        .await
    }

    /// Delete a comment
    pub async fn delete_thread_comment(&self, id: u32) -> Result<bool, AniListError> {
        self.delete_comment(&DeleteThreadCommentOptions::builder().id(id).build())
            .await
    }

    /// Subscribe to a thread
    pub async fn subscribe_to_thread(&self, thread_id: u32) -> Result<Thread, AniListError> {
        self.subscription(
            &ToggleThreadSubscriptionOptions::builder()
                .thread_id(thread_id)
                .subscribe(true)
                .build(),
        )
        .await
    }

    /// Unsubscribe from a thread
    pub async fn unsubscribe_from_thread(&self, thread_id: u32) -> Result<Thread, AniListError> {
        self.subscription(
            &ToggleThreadSubscriptionOptions::builder()
                .thread_id(thread_id)
                .subscribe(false)
                .build(),
        )
        .await
    }

    /// Toggle thread subscription
    ///
    /// `ToggleThreadSubscription` takes the state it should end up in rather than
    /// flipping it, so this reads the thread first and sends the opposite. Use
    /// [`Self::subscribe_to_thread`] or [`Self::unsubscribe_from_thread`] when the
    /// state is already known, and [`Self::subscription`] to set it directly — both
    /// cost one request instead of two.
    pub async fn toggle_thread_subscription(&self, thread_id: u32) -> Result<Thread, AniListError> {
        let current = self
            .get_by_id(thread_id)
            .await?
            .is_subscribed
            .unwrap_or(false);

        self.subscription(
            &ToggleThreadSubscriptionOptions::builder()
                .thread_id(thread_id)
                .subscribe(!current)
                .build(),
        )
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
