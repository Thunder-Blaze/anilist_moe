use crate::enums::activity::{ActivitySort, ActivityType};
use crate::errors::AniListError;
use crate::objects::activity::{ActivityReply, MessageActivity, TextActivity};
use crate::objects::common::Deleted;
use crate::objects::responses::Page;
use crate::options;
#[cfg(feature = "pagination")]
use crate::pagination::Paginated;
use crate::unions::activity::ActivityUnion;
use crate::{client::AniListClient, queries::activity};
use serde::Serialize;
use serde_with::skip_serializing_none;
use std::borrow::Cow;

/// Options for fetching activity feed entries.
#[skip_serializing_none]
#[derive(Default, Debug, Serialize)]
pub struct FetchActivityOptions<'a> {
    // # Pagination
    #[serde(rename = "page")]
    pub page: Option<i32>,
    #[serde(rename = "perPage")]
    pub per_page: Option<i32>,

    // # Filters
    #[serde(rename = "id")]
    pub id: Option<u32>,
    #[serde(rename = "userId")]
    pub user_id: Option<u32>,
    #[serde(rename = "messengerId")]
    pub messenger_id: Option<u32>,
    #[serde(rename = "mediaId")]
    pub media_id: Option<u32>,
    #[serde(rename = "type")]
    pub type_: Option<ActivityType>,
    #[serde(rename = "isFollowing")]
    pub is_following: Option<bool>,
    #[serde(rename = "hasReplies")]
    pub has_replies: Option<bool>,
    #[serde(rename = "hasRepliesOrTypeText")]
    pub has_replies_or_type_text: Option<bool>,
    #[serde(rename = "createdAt")]
    pub created_at: Option<i32>,
    #[serde(rename = "id_not")]
    pub id_not: Option<u32>,
    #[serde(rename = "id_in")]
    pub id_in: Option<Cow<'a, [u32]>>,
    #[serde(rename = "id_not_in")]
    pub id_not_in: Option<Cow<'a, [u32]>>,
    #[serde(rename = "userId_not")]
    pub user_id_not: Option<u32>,
    #[serde(rename = "userId_in")]
    pub user_id_in: Option<Cow<'a, [i32]>>,
    #[serde(rename = "userId_not_in")]
    pub user_id_not_in: Option<Cow<'a, [i32]>>,
    #[serde(rename = "messengerId_not")]
    pub messenger_id_not: Option<u32>,
    #[serde(rename = "messengerId_in")]
    pub messenger_id_in: Option<Cow<'a, [i32]>>,
    #[serde(rename = "messengerId_not_in")]
    pub messenger_id_not_in: Option<Cow<'a, [i32]>>,
    #[serde(rename = "mediaId_not")]
    pub media_id_not: Option<u32>,
    #[serde(rename = "mediaId_in")]
    pub media_id_in: Option<Cow<'a, [i32]>>,
    #[serde(rename = "mediaId_not_in")]
    pub media_id_not_in: Option<Cow<'a, [i32]>>,
    #[serde(rename = "type_not")]
    pub type_not: Option<ActivityType>,
    #[serde(rename = "type_in")]
    pub type_in: Option<Cow<'a, [ActivityType]>>,
    #[serde(rename = "type_not_in")]
    pub type_not_in: Option<Cow<'a, [ActivityType]>>,
    #[serde(rename = "createdAt_greater")]
    pub created_at_greater: Option<i32>,
    #[serde(rename = "createdAt_lesser")]
    pub created_at_lesser: Option<i32>,

    // # Sort
    /// Sort order. The unique key (id) is appended, so paging stays stable;
    /// leaving this unset keeps the order the query document asks for.
    #[serde(rename = "sort")]
    #[serde(
        serialize_with = "crate::options::serialize_sort",
        skip_serializing_if = "Option::is_none"
    )]
    pub sort: Option<Cow<'a, [ActivitySort]>>,

    // # HTML rendering options
    #[serde(rename = "text_as_html")]
    pub text_as_html: Option<bool>,
    #[serde(rename = "message_as_html")]
    pub message_as_html: Option<bool>,
}

/// Options for fetching a single activity by ID.
#[skip_serializing_none]
#[derive(Default, Debug, Serialize)]
pub struct FetchActivityOneOptions {
    pub id: u32,
    // # HTML rendering options
    #[serde(rename = "text_as_html")]
    pub text_as_html: Option<bool>,
    #[serde(rename = "message_as_html")]
    pub message_as_html: Option<bool>,
}

/// Options for fetching replies to an activity.
#[skip_serializing_none]
#[derive(Default, Debug, Serialize)]
pub struct FetchActivityRepliesOptions {
    // # Pagination
    #[serde(rename = "page")]
    pub page: Option<i32>,
    #[serde(rename = "perPage")]
    pub per_page: Option<i32>,

    // # Filters
    #[serde(rename = "id")]
    pub id: Option<u32>,
    #[serde(rename = "activityId")]
    pub activity_id: u32,

    // # HTML rendering options
    #[serde(rename = "text_as_html")]
    pub text_as_html: Option<bool>,
}

/// Options for deleting an activity.
#[derive(Default, Debug, Serialize)]
pub struct DeleteActivityOptions {
    pub id: u32,
}

/// Options for deleting an activity reply.
#[derive(Default, Debug, Serialize)]
pub struct DeleteActivityReplyOptions {
    pub id: u32,
}

/// Options for pinning or unpinning an activity.
#[derive(Default, Debug, Serialize)]
pub struct PinActivityOptions {
    pub id: u32,
    pub pinned: bool,
}

/// Options for subscribing or unsubscribing to an activity.
#[derive(Default, Debug, Serialize)]
pub struct SubscribeActivityOptions {
    pub id: u32,
    pub subscribe: bool,
}

/// Options for saving a message activity.
#[skip_serializing_none]
#[derive(Default, Debug, Serialize)]
pub struct SaveMessageActivityOptions<'a> {
    pub id: Option<u32>,
    pub message: Cow<'a, str>,
    #[serde(rename = "recipientId")]
    pub recipient_id: u32,
    pub private: Option<bool>,
    pub locked: Option<bool>,
    pub as_mod: Option<bool>,
}

/// Options for saving a text activity.
#[skip_serializing_none]
#[derive(Default, Debug, Serialize)]
pub struct SaveTextActivityOptions<'a> {
    pub id: Option<u32>,
    pub text: Cow<'a, str>,
    pub locked: Option<bool>,
}

/// Options for saving a reply to an activity.
#[skip_serializing_none]
#[derive(Default, Debug, Serialize)]
pub struct SaveActivityReplyOptions<'a> {
    pub id: Option<u32>,
    pub text: Cow<'a, str>,
    #[serde(rename = "activityId")]
    pub activity_id: u32,
}

/// Endpoint for activity feed operations.
pub struct ActivityEndpoint<'a> {
    pub client: &'a AniListClient,
}

impl<'a> ActivityEndpoint<'a> {
    pub fn new(client: &'a AniListClient) -> Self {
        Self { client }
    }

    pub async fn fetch(
        &self,
        options: &FetchActivityOptions<'_>,
    ) -> Result<Page<Vec<ActivityUnion>>, AniListError> {
        let query = activity::FETCH;
        self.client.fetch(query, Some(&options)).await
    }

    pub async fn fetch_one(
        &self,
        options: &FetchActivityOneOptions,
    ) -> Result<ActivityUnion, AniListError> {
        let query = activity::FETCH_ONE;
        self.client.fetch(query, Some(&options)).await
    }

    pub async fn fetch_replies(
        &self,
        options: &FetchActivityRepliesOptions,
    ) -> Result<Page<Vec<ActivityReply>>, AniListError> {
        let query = activity::FETCH_REPLIES;
        self.client.fetch(query, Some(&options)).await
    }

    pub async fn delete(&self, options: &DeleteActivityOptions) -> Result<bool, AniListError> {
        let query = activity::DELETE;
        let response: Result<Deleted, AniListError> =
            self.client.fetch(query, Some(&options)).await;
        match response {
            Ok(res) => Ok(res.deleted.unwrap_or_default()),
            Err(err) => Err(err),
        }
    }

    pub async fn delete_reply(
        &self,
        options: &DeleteActivityReplyOptions,
    ) -> Result<bool, AniListError> {
        let query = activity::DELETE_REPLY;
        let response: Result<Deleted, AniListError> =
            self.client.fetch(query, Some(&options)).await;
        match response {
            Ok(res) => Ok(res.deleted.unwrap_or_default()),
            Err(err) => Err(err),
        }
    }

    pub async fn pin(&self, options: &PinActivityOptions) -> Result<ActivityUnion, AniListError> {
        let query = activity::PIN;
        self.client.fetch(query, Some(&options)).await
    }

    pub async fn subscribe(
        &self,
        options: &SubscribeActivityOptions,
    ) -> Result<ActivityUnion, AniListError> {
        let query = activity::SUBSCRIBE;
        self.client.fetch(query, Some(&options)).await
    }

    pub async fn save_message_activity(
        &self,
        options: &SaveMessageActivityOptions<'_>,
    ) -> Result<ActivityUnion, AniListError> {
        let query = activity::SAVE_MESSAGE_ACTIVITY;
        let response: Result<MessageActivity, AniListError> =
            self.client.fetch(query, Some(&options)).await;
        match response {
            Ok(res) => Ok(ActivityUnion::MessageActivity(res)),
            Err(err) => Err(err),
        }
    }

    pub async fn save_text_activity(
        &self,
        options: &SaveTextActivityOptions<'_>,
    ) -> Result<ActivityUnion, AniListError> {
        let query = activity::SAVE_TEXT_ACTIVITY;
        let response: Result<TextActivity, AniListError> =
            self.client.fetch(query, Some(&options)).await;
        match response {
            Ok(res) => Ok(ActivityUnion::TextActivity(res)),
            Err(err) => Err(err),
        }
    }

    pub async fn save_reply(
        &self,
        options: &SaveActivityReplyOptions<'_>,
    ) -> Result<ActivityReply, AniListError> {
        let query = activity::SAVE_REPLY;
        self.client.fetch(query, Some(&options)).await
    }

    // Convenience functions

    /// Get recent activities
    pub async fn get_recent(
        &self,
        page: Option<i32>,
        per_page: Option<i32>,
    ) -> Result<Page<Vec<ActivityUnion>>, AniListError> {
        self.fetch(&options!(
            FetchActivityOptions,
            page,
            per_page,
            sort: Some(&[ActivitySort::IdDesc]),
        ))
        .await
    }

    /// Get activities from users you're following (requires authentication)
    pub async fn get_following(
        &self,
        page: Option<i32>,
        per_page: Option<i32>,
    ) -> Result<Page<Vec<ActivityUnion>>, AniListError> {
        self.fetch(&options!(
            FetchActivityOptions,
            is_following: Some(true),
            page,
            per_page,
            sort: Some(&[ActivitySort::IdDesc]),
        ))
        .await
    }

    /// Get activity by ID
    pub async fn get_by_id(&self, id: u32) -> Result<ActivityUnion, AniListError> {
        self.fetch_one(&options!(
            FetchActivityOneOptions,
            id => id,
        ))
        .await
    }

    /// Create a text activity
    pub async fn create_text_activity(&self, text: &str) -> Result<ActivityUnion, AniListError> {
        self.save_text_activity(&options!(
            SaveTextActivityOptions,
            text => text,
        ))
        .await
    }

    /// Create a message activity
    pub async fn create_message(
        &self,
        recipient_id: u32,
        message: &str,
        private: Option<bool>,
    ) -> Result<ActivityUnion, AniListError> {
        self.save_message_activity(&options!(
            SaveMessageActivityOptions,
            message => message,
            recipient_id => recipient_id,
            private,
        ))
        .await
    }

    /// Reply to an activity
    pub async fn reply_to(
        &self,
        activity_id: u32,
        text: &str,
    ) -> Result<ActivityReply, AniListError> {
        self.save_reply(&options!(
            SaveActivityReplyOptions,
            text => text,
            activity_id => activity_id,
        ))
        .await
    }

    /// Delete an activity
    pub async fn delete_activity(&self, id: u32) -> Result<bool, AniListError> {
        self.delete(&options!(
            DeleteActivityOptions,
            id => id,
        ))
        .await
    }

    /// Delete an activity reply
    pub async fn delete_activity_reply(&self, id: u32) -> Result<bool, AniListError> {
        self.delete_reply(&options!(
            DeleteActivityReplyOptions,
            id => id,
        ))
        .await
    }

    /// Toggle activity subscription
    pub async fn toggle_subscription(
        &self,
        id: u32,
        subscribe: bool,
    ) -> Result<ActivityUnion, AniListError> {
        self.subscribe(&options!(
            SubscribeActivityOptions,
            id => id,
            subscribe => subscribe,
        ))
        .await
    }

    /// Toggle activity pin status
    pub async fn toggle_pin(&self, id: u32, pinned: bool) -> Result<ActivityUnion, AniListError> {
        self.pin(&options!(
            PinActivityOptions,
            id => id,
            pinned => pinned,
        ))
        .await
    }
}

#[cfg(feature = "pagination")]
impl<'a> Paginated for FetchActivityOptions<'a> {
    type Item = ActivityUnion;

    async fn fetch(
        &self,
        client: &AniListClient,
    ) -> Result<Page<Vec<ActivityUnion>>, AniListError> {
        client.activity().fetch(self).await
    }

    fn set_page(&mut self, page: i32) {
        self.page = Some(page);
    }
}

#[cfg(feature = "pagination")]
impl Paginated for FetchActivityRepliesOptions {
    type Item = ActivityReply;

    async fn fetch(
        &self,
        client: &AniListClient,
    ) -> Result<Page<Vec<ActivityReply>>, AniListError> {
        client.activity().fetch_replies(self).await
    }

    fn set_page(&mut self, page: i32) {
        self.page = Some(page);
    }
}
