use crate::enums::activity::{ActivitySort, ActivityType};
use crate::errors::AniListError;
use crate::objects::activity::{ActivityReply, ListActivity, MessageActivity, TextActivity};
use crate::objects::common::Deleted;
use crate::objects::responses::Page;
#[cfg(feature = "pagination")]
use crate::pagination::Paginated;
use crate::unions::activity::ActivityUnion;
use crate::{client::AniListClient, queries::activity};
use serde::Serialize;
use serde_with::skip_serializing_none;
use std::borrow::Cow;
use typed_builder::TypedBuilder;

/// Options for fetching one activity reply.
#[skip_serializing_none]
#[derive(Default, Debug, Serialize, TypedBuilder)]
#[builder(field_defaults(default))]
#[serde(rename_all = "camelCase")]
pub struct FetchActivityReplyOptions {
    id: Option<u32>,
    activity_id: Option<u32>,
    text_as_html: Option<bool>,
}

/// Options for updating a list activity.
///
/// This is a moderator mutation: the API refuses it for an ordinary token. The
/// activity's `id` is required, and `locked` decides whether it ends up locked.
#[skip_serializing_none]
#[derive(Default, Debug, Serialize, TypedBuilder)]
#[builder(field_defaults(default))]
#[serde(rename_all = "camelCase")]
pub struct SaveListActivityOptions {
    id: u32,
    locked: Option<bool>,
}

/// Options for fetching activity feed entries.
#[skip_serializing_none]
#[derive(Default, Debug, Serialize, TypedBuilder)]
#[builder(field_defaults(default))]
#[serde(rename_all = "camelCase")]
pub struct FetchActivityOptions<'a> {
    // # Pagination
    page: Option<i32>,
    per_page: Option<i32>,

    // # Filters
    id: Option<u32>,
    user_id: Option<u32>,
    messenger_id: Option<u32>,
    media_id: Option<u32>,
    type_: Option<ActivityType>,
    is_following: Option<bool>,
    has_replies: Option<bool>,
    has_replies_or_type_text: Option<bool>,
    created_at: Option<i32>,
    id_not: Option<u32>,
    id_in: Option<Cow<'a, [u32]>>,
    id_not_in: Option<Cow<'a, [u32]>>,
    user_id_not: Option<u32>,
    user_id_in: Option<Cow<'a, [u32]>>,
    user_id_not_in: Option<Cow<'a, [u32]>>,
    messenger_id_not: Option<u32>,
    messenger_id_in: Option<Cow<'a, [u32]>>,
    messenger_id_not_in: Option<Cow<'a, [u32]>>,
    media_id_not: Option<u32>,
    media_id_in: Option<Cow<'a, [u32]>>,
    media_id_not_in: Option<Cow<'a, [u32]>>,
    type_not: Option<ActivityType>,
    type_in: Option<Cow<'a, [ActivityType]>>,
    type_not_in: Option<Cow<'a, [ActivityType]>>,
    created_at_greater: Option<i32>,
    created_at_lesser: Option<i32>,

    // # Sort
    /// Sort order. The unique key (id) is appended, so paging stays stable;
    /// leaving this unset keeps the order the query document asks for.
    #[serde(
        serialize_with = "crate::options::serialize_sort",
        skip_serializing_if = "Option::is_none"
    )]
    sort: Option<Cow<'a, [ActivitySort]>>,

    // # HTML rendering options
    text_as_html: Option<bool>,
    message_as_html: Option<bool>,
}

/// Options for fetching a single activity by ID.
#[skip_serializing_none]
#[derive(Default, Debug, Serialize, TypedBuilder)]
#[builder(field_defaults(default))]
#[serde(rename_all = "camelCase")]
pub struct FetchActivityOneOptions {
    id: u32,
    // # HTML rendering options
    text_as_html: Option<bool>,
    message_as_html: Option<bool>,
}

/// Options for fetching replies to an activity.
#[skip_serializing_none]
#[derive(Default, Debug, Serialize, TypedBuilder)]
#[builder(field_defaults(default))]
#[serde(rename_all = "camelCase")]
pub struct FetchActivityRepliesOptions {
    // # Pagination
    page: Option<i32>,
    per_page: Option<i32>,

    // # Filters
    id: Option<u32>,
    activity_id: u32,

    // # HTML rendering options
    text_as_html: Option<bool>,
}

/// Options for deleting an activity.
#[derive(Default, Debug, Serialize, TypedBuilder)]
#[builder(field_defaults(default))]
#[serde(rename_all = "camelCase")]
pub struct DeleteActivityOptions {
    id: u32,
}

/// Options for deleting an activity reply.
#[derive(Default, Debug, Serialize, TypedBuilder)]
#[builder(field_defaults(default))]
#[serde(rename_all = "camelCase")]
pub struct DeleteActivityReplyOptions {
    id: u32,
}

/// Options for pinning or unpinning an activity.
#[derive(Default, Debug, Serialize, TypedBuilder)]
#[builder(field_defaults(default))]
#[serde(rename_all = "camelCase")]
pub struct PinActivityOptions {
    id: u32,
    pinned: bool,
}

/// Options for subscribing or unsubscribing to an activity.
#[derive(Default, Debug, Serialize, TypedBuilder)]
#[builder(field_defaults(default))]
#[serde(rename_all = "camelCase")]
pub struct SubscribeActivityOptions {
    /// The activity to subscribe to or unsubscribe from.
    activity_id: u32,
    subscribe: bool,
}

/// Options for saving a message activity.
#[skip_serializing_none]
#[derive(Default, Debug, Serialize, TypedBuilder)]
#[builder(field_defaults(default))]
#[serde(rename_all = "camelCase")]
pub struct SaveMessageActivityOptions<'a> {
    id: Option<u32>,
    message: Cow<'a, str>,
    recipient_id: u32,
    private: Option<bool>,
    locked: Option<bool>,
    as_mod: Option<bool>,
}

/// Options for saving a text activity.
#[skip_serializing_none]
#[derive(Default, Debug, Serialize, TypedBuilder)]
#[builder(field_defaults(default))]
#[serde(rename_all = "camelCase")]
pub struct SaveTextActivityOptions<'a> {
    id: Option<u32>,
    text: Cow<'a, str>,
    locked: Option<bool>,
}

/// Options for saving a reply to an activity.
#[skip_serializing_none]
#[derive(Default, Debug, Serialize, TypedBuilder)]
#[builder(field_defaults(default))]
#[serde(rename_all = "camelCase")]
pub struct SaveActivityReplyOptions<'a> {
    id: Option<u32>,
    text: Cow<'a, str>,
    activity_id: u32,
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
        self.fetch(
            &FetchActivityOptions::builder()
                .page(page)
                .per_page(per_page)
                .build(),
        )
        .await
    }

    /// Get activities from users you're following (requires authentication)
    pub async fn get_following(
        &self,
        page: Option<i32>,
        per_page: Option<i32>,
    ) -> Result<Page<Vec<ActivityUnion>>, AniListError> {
        self.fetch(
            &FetchActivityOptions::builder()
                .is_following(Some(true))
                .page(page)
                .per_page(per_page)
                .sort(Some(Cow::Borrowed(&[ActivitySort::IdDesc])))
                .build(),
        )
        .await
    }

    /// Get activity by ID
    pub async fn get_by_id(&self, id: u32) -> Result<ActivityUnion, AniListError> {
        self.fetch_one(&FetchActivityOneOptions::builder().id(id).build())
            .await
    }

    /// Create a text activity
    pub async fn create_text_activity(&self, text: &str) -> Result<ActivityUnion, AniListError> {
        self.save_text_activity(
            &SaveTextActivityOptions::builder()
                .text(Cow::Borrowed(text))
                .build(),
        )
        .await
    }

    /// Create a message activity
    pub async fn create_message(
        &self,
        recipient_id: u32,
        message: &str,
        private: Option<bool>,
    ) -> Result<ActivityUnion, AniListError> {
        self.save_message_activity(
            &SaveMessageActivityOptions::builder()
                .recipient_id(recipient_id)
                .message(Cow::Borrowed(message))
                .private(private)
                .build(),
        )
        .await
    }

    /// Reply to an activity
    pub async fn reply_to(
        &self,
        activity_id: u32,
        text: &str,
    ) -> Result<ActivityReply, AniListError> {
        self.save_reply(
            &SaveActivityReplyOptions::builder()
                .activity_id(activity_id)
                .text(Cow::Borrowed(text))
                .build(),
        )
        .await
    }

    /// Delete an activity
    pub async fn delete_activity(&self, id: u32) -> Result<bool, AniListError> {
        self.delete(&DeleteActivityOptions::builder().id(id).build())
            .await
    }

    /// Delete an activity reply
    pub async fn delete_activity_reply(&self, id: u32) -> Result<bool, AniListError> {
        self.delete_reply(&DeleteActivityReplyOptions::builder().id(id).build())
            .await
    }

    /// Toggle activity subscription
    pub async fn toggle_subscription(
        &self,
        id: u32,
        subscribe: bool,
    ) -> Result<ActivityUnion, AniListError> {
        self.subscribe(
            &SubscribeActivityOptions::builder()
                .activity_id(id)
                .subscribe(subscribe)
                .build(),
        )
        .await
    }

    /// Toggle activity pin status
    pub async fn toggle_pin(&self, id: u32, pinned: bool) -> Result<ActivityUnion, AniListError> {
        self.pin(&PinActivityOptions::builder().id(id).pinned(pinned).build())
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
