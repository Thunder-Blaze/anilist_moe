use crate::enums::activity::{ActivitySort, ActivityType};
use crate::errors::AniListError;
use crate::objects::activity::{ActivityReply, ListActivity, MessageActivity, TextActivity};
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

/// Options for fetching one activity reply.
#[skip_serializing_none]
#[derive(Default, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FetchActivityReplyOptions {
    pub id: Option<u32>,
    pub activity_id: Option<u32>,
    pub text_as_html: Option<bool>,
}

/// Options for updating a list activity.
///
/// This is a moderator mutation: the API refuses it for an ordinary token. The
/// activity's `id` is required, and `locked` decides whether it ends up locked.
#[skip_serializing_none]
#[derive(Default, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SaveListActivityOptions {
    pub id: u32,
    pub locked: Option<bool>,
}

/// Options for fetching activity feed entries.
#[skip_serializing_none]
#[derive(Default, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FetchActivityOptions<'a> {
    // # Pagination
    pub page: Option<i32>,
    pub per_page: Option<i32>,

    // # Filters
    pub id: Option<u32>,
    pub user_id: Option<u32>,
    pub messenger_id: Option<u32>,
    pub media_id: Option<u32>,
    pub type_: Option<ActivityType>,
    pub is_following: Option<bool>,
    pub has_replies: Option<bool>,
    pub has_replies_or_type_text: Option<bool>,
    pub created_at: Option<i32>,
    pub id_not: Option<u32>,
    pub id_in: Option<Cow<'a, [u32]>>,
    pub id_not_in: Option<Cow<'a, [u32]>>,
    pub user_id_not: Option<u32>,
    pub user_id_in: Option<Cow<'a, [u32]>>,
    pub user_id_not_in: Option<Cow<'a, [u32]>>,
    pub messenger_id_not: Option<u32>,
    pub messenger_id_in: Option<Cow<'a, [u32]>>,
    pub messenger_id_not_in: Option<Cow<'a, [u32]>>,
    pub media_id_not: Option<u32>,
    pub media_id_in: Option<Cow<'a, [u32]>>,
    pub media_id_not_in: Option<Cow<'a, [u32]>>,
    pub type_not: Option<ActivityType>,
    pub type_in: Option<Cow<'a, [ActivityType]>>,
    pub type_not_in: Option<Cow<'a, [ActivityType]>>,
    pub created_at_greater: Option<i32>,
    pub created_at_lesser: Option<i32>,

    // # Sort
    /// Sort order. The unique key (id) is appended, so paging stays stable;
    /// leaving this unset keeps the order the query document asks for.
    #[serde(
        serialize_with = "crate::options::serialize_sort",
        skip_serializing_if = "Option::is_none"
    )]
    pub sort: Option<Cow<'a, [ActivitySort]>>,

    // # HTML rendering options
    pub text_as_html: Option<bool>,
    pub message_as_html: Option<bool>,
}

/// Options for fetching a single activity by ID.
#[skip_serializing_none]
#[derive(Default, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FetchActivityOneOptions {
    pub id: u32,
    // # HTML rendering options
    pub text_as_html: Option<bool>,
    pub message_as_html: Option<bool>,
}

/// Options for fetching replies to an activity.
#[skip_serializing_none]
#[derive(Default, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FetchActivityRepliesOptions {
    // # Pagination
    pub page: Option<i32>,
    pub per_page: Option<i32>,

    // # Filters
    pub id: Option<u32>,
    pub activity_id: u32,

    // # HTML rendering options
    pub text_as_html: Option<bool>,
}

/// Options for deleting an activity.
#[derive(Default, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteActivityOptions {
    pub id: u32,
}

/// Options for deleting an activity reply.
#[derive(Default, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteActivityReplyOptions {
    pub id: u32,
}

/// Options for pinning or unpinning an activity.
#[derive(Default, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PinActivityOptions {
    pub id: u32,
    pub pinned: bool,
}

/// Options for subscribing or unsubscribing to an activity.
#[derive(Default, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SubscribeActivityOptions {
    /// The activity to subscribe to or unsubscribe from.
    pub activity_id: u32,
    pub subscribe: bool,
}

/// Options for saving a message activity.
#[skip_serializing_none]
#[derive(Default, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SaveMessageActivityOptions<'a> {
    pub id: Option<u32>,
    pub message: Cow<'a, str>,
    pub recipient_id: u32,
    pub private: Option<bool>,
    /// Mod only.
    pub locked: Option<bool>,
    /// Mod only: send it from the moderator account.
    pub as_mod: Option<bool>,
}

/// Options for saving a text activity.
#[skip_serializing_none]
#[derive(Default, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SaveTextActivityOptions<'a> {
    pub id: Option<u32>,
    pub text: Cow<'a, str>,
    /// Mod only.
    pub locked: Option<bool>,
}

/// Options for saving a reply to an activity.
#[skip_serializing_none]
#[derive(Default, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SaveActivityReplyOptions<'a> {
    pub id: Option<u32>,
    pub text: Cow<'a, str>,
    pub activity_id: u32,
    /// Mod only: send it from the moderator account.
    pub as_mod: Option<bool>,
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

    /// Fetch one activity reply by id, or by the activity it belongs to.
    pub async fn fetch_reply(
        &self,
        options: &FetchActivityReplyOptions,
    ) -> Result<ActivityReply, AniListError> {
        let query = activity::FETCH_REPLY;
        self.client.fetch(query, Some(&options)).await
    }

    /// Update a list activity, or lock it. Moderators only: the API refuses it
    /// for an ordinary token.
    pub async fn save_list_activity(
        &self,
        options: &SaveListActivityOptions,
    ) -> Result<ListActivity, AniListError> {
        let query = activity::SAVE_LIST_ACTIVITY;
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
            activity_id => id,
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
