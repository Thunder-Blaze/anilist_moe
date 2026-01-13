use crate::enums::activity::{ActivitySort, ActivityType};
use crate::errors::AniListError;
use crate::objects::activity::{ActivityReply, MessageActivity, TextActivity};
use crate::objects::common::Deleted;
use crate::objects::responses::Page;
use crate::unions::activity::ActivityUnion;
use crate::{client::AniListClient, queries::activity};
use serde::{Deserialize, Serialize};
use serde_json::json;
use serde_with::skip_serializing_none;

/// Options for fetching activity feed entries.
#[skip_serializing_none]
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct FetchActivityOptions {
    // # Pagination
    #[serde(rename = "page")]
    pub page: Option<i32>,
    #[serde(rename = "perPage")]
    pub per_page: Option<i32>,

    // # Filters
    #[serde(rename = "id")]
    pub id: Option<i32>,
    #[serde(rename = "userId")]
    pub user_id: Option<i32>,
    #[serde(rename = "messengerId")]
    pub messenger_id: Option<i32>,
    #[serde(rename = "mediaId")]
    pub media_id: Option<i32>,
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
    pub id_not: Option<i32>,
    #[serde(rename = "id_in")]
    pub id_in: Option<Vec<i32>>,
    #[serde(rename = "id_not_in")]
    pub id_not_in: Option<Vec<i32>>,
    #[serde(rename = "userId_not")]
    pub user_id_not: Option<i32>,
    #[serde(rename = "userId_in")]
    pub user_id_in: Option<Vec<i32>>,
    #[serde(rename = "userId_not_in")]
    pub user_id_not_in: Option<Vec<i32>>,
    #[serde(rename = "messengerId_not")]
    pub messenger_id_not: Option<i32>,
    #[serde(rename = "messengerId_in")]
    pub messenger_id_in: Option<Vec<i32>>,
    #[serde(rename = "messengerId_not_in")]
    pub messenger_id_not_in: Option<Vec<i32>>,
    #[serde(rename = "mediaId_not")]
    pub media_id_not: Option<i32>,
    #[serde(rename = "mediaId_in")]
    pub media_id_in: Option<Vec<i32>>,
    #[serde(rename = "mediaId_not_in")]
    pub media_id_not_in: Option<Vec<i32>>,
    #[serde(rename = "type_not")]
    pub type_not: Option<ActivityType>,
    #[serde(rename = "type_in")]
    pub type_in: Option<Vec<ActivityType>>,
    #[serde(rename = "type_not_in")]
    pub type_not_in: Option<Vec<ActivityType>>,
    #[serde(rename = "createdAt_greater")]
    pub created_at_greater: Option<i32>,
    #[serde(rename = "createdAt_lesser")]
    pub created_at_lesser: Option<i32>,

    // # Sort
    #[serde(rename = "sort")]
    pub sort: Option<Vec<ActivitySort>>,
}

/// Options for fetching a single activity by ID.
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct FetchActivityOneOptions {
    pub id: i32,
}

/// Options for fetching replies to an activity.
#[skip_serializing_none]
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct FetchActivityRepliesOptions {
    // # Pagination
    #[serde(rename = "page")]
    pub page: Option<i32>,
    #[serde(rename = "perPage")]
    pub per_page: Option<i32>,

    // # Filters
    #[serde(rename = "id")]
    pub id: Option<i32>,
    #[serde(rename = "activityId")]
    pub activity_id: i32,
}

/// Options for deleting an activity.
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct DeleteActivityOptions {
    pub id: i32,
}

/// Options for deleting an activity reply.
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct DeleteActivityReplyOptions {
    pub id: i32,
}

/// Options for pinning or unpinning an activity.
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct PinActivityOptions {
    pub id: i32,
    pub pinned: bool,
}

/// Options for subscribing or unsubscribing to an activity.
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct SubscribeActivityOptions {
    pub id: i32,
    pub subscribe: bool,
}

/// Options for saving a message activity.
#[skip_serializing_none]
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct SaveMessageActivityOptions {
    pub id: Option<i32>,
    pub message: String,
    #[serde(rename = "recipientId")]
    pub recipient_id: i32,
    pub private: Option<bool>,
    pub locked: Option<bool>,
    pub as_mod: Option<bool>,
}

/// Options for saving a text activity.
#[skip_serializing_none]
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct SaveTextActivityOptions {
    pub id: Option<i32>,
    pub text: String,
    pub locked: Option<bool>,
}

/// Options for saving a reply to an activity.
#[skip_serializing_none]
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct SaveActivityReplyOptions {
    pub id: Option<i32>,
    pub text: String,
    #[serde(rename = "activityId")]
    pub activity_id: i32,
}

/// Endpoint for activity feed operations.
pub struct ActivityEndpoint {
    pub client: AniListClient,
}

impl ActivityEndpoint {
    pub fn new(client: AniListClient) -> Self {
        Self { client }
    }

    pub async fn fetch(
        &self,
        options: &FetchActivityOptions,
    ) -> Result<Page<Vec<ActivityUnion>>, AniListError> {
        let query = activity::FETCH;
        let variables = json!(options);
        let variables_map = crate::utils::json_to_hashmap(variables);
        self.client.fetch(query, Some(&variables_map)).await
    }

    pub async fn fetch_one(
        &self,
        options: &FetchActivityOneOptions,
    ) -> Result<ActivityUnion, AniListError> {
        let query = activity::FETCH_ONE;
        let variables = json!(options);
        let variables_map = crate::utils::json_to_hashmap(variables);
        self.client.fetch(query, Some(&variables_map)).await
    }

    pub async fn fetch_replies(
        &self,
        options: &FetchActivityRepliesOptions,
    ) -> Result<Page<Vec<ActivityReply>>, AniListError> {
        let query = activity::FETCH_REPLIES;
        let variables = json!(options);
        let variables_map = crate::utils::json_to_hashmap(variables);
        self.client.fetch(query, Some(&variables_map)).await
    }

    pub async fn delete(&self, options: &DeleteActivityOptions) -> Result<bool, AniListError> {
        let query = activity::DELETE;
        let variables = json!(options);
        let variables_map = crate::utils::json_to_hashmap(variables);
        let response: Result<Deleted, AniListError> =
            self.client.fetch(query, Some(&variables_map)).await;
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
        let variables = json!(options);
        let variables_map = crate::utils::json_to_hashmap(variables);
        let response: Result<Deleted, AniListError> =
            self.client.fetch(query, Some(&variables_map)).await;
        match response {
            Ok(res) => Ok(res.deleted.unwrap_or_default()),
            Err(err) => Err(err),
        }
    }

    pub async fn pin(&self, options: &PinActivityOptions) -> Result<ActivityUnion, AniListError> {
        let query = activity::PIN;
        let variables = json!(options);
        let variables_map = crate::utils::json_to_hashmap(variables);
        self.client.fetch(query, Some(&variables_map)).await
    }

    pub async fn subscribe(
        &self,
        options: &SubscribeActivityOptions,
    ) -> Result<ActivityUnion, AniListError> {
        let query = activity::SUBSCRIBE;
        let variables = json!(options);
        let variables_map = crate::utils::json_to_hashmap(variables);
        self.client.fetch(query, Some(&variables_map)).await
    }

    pub async fn save_message_activity(
        &self,
        options: &SaveMessageActivityOptions,
    ) -> Result<ActivityUnion, AniListError> {
        let query = activity::SAVE_MESSAGE_ACTIVITY;
        let variables = json!(options);
        let variables_map = crate::utils::json_to_hashmap(variables);
        let response: Result<MessageActivity, AniListError> =
            self.client.fetch(query, Some(&variables_map)).await;
        match response {
            Ok(res) => Ok(ActivityUnion::MessageActivity(res)),
            Err(err) => Err(err),
        }
    }

    pub async fn save_text_activity(
        &self,
        options: &SaveTextActivityOptions,
    ) -> Result<ActivityUnion, AniListError> {
        let query = activity::SAVE_TEXT_ACTIVITY;
        let variables = json!(options);
        let variables_map = crate::utils::json_to_hashmap(variables);
        let response: Result<TextActivity, AniListError> =
            self.client.fetch(query, Some(&variables_map)).await;
        match response {
            Ok(res) => Ok(ActivityUnion::TextActivity(res)),
            Err(err) => Err(err),
        }
    }

    pub async fn save_reply(
        &self,
        options: &SaveActivityReplyOptions,
    ) -> Result<ActivityReply, AniListError> {
        let query = activity::SAVE_REPLY;
        let variables = json!(options);
        let variables_map = crate::utils::json_to_hashmap(variables);
        self.client.fetch(query, Some(&variables_map)).await
    }

    // Convenience functions

    /// Get recent activities
    pub async fn get_recent(
        &self,
        page: Option<i32>,
        per_page: Option<i32>,
    ) -> Result<Page<Vec<ActivityUnion>>, AniListError> {
        self.fetch(&FetchActivityOptions {
            page,
            per_page,
            sort: Some(vec![ActivitySort::IdDesc]),
            ..Default::default()
        })
        .await
    }

    /// Get activities from users you're following (requires authentication)
    pub async fn get_following(
        &self,
        page: Option<i32>,
        per_page: Option<i32>,
    ) -> Result<Page<Vec<ActivityUnion>>, AniListError> {
        self.fetch(&FetchActivityOptions {
            is_following: Some(true),
            page,
            per_page,
            sort: Some(vec![ActivitySort::IdDesc]),
            ..Default::default()
        })
        .await
    }

    /// Get activity by ID
    pub async fn get_by_id(&self, id: i32) -> Result<ActivityUnion, AniListError> {
        self.fetch_one(&FetchActivityOneOptions { id }).await
    }

    /// Create a text activity
    pub async fn create_text_activity(&self, text: &str) -> Result<ActivityUnion, AniListError> {
        self.save_text_activity(&SaveTextActivityOptions {
            id: None,
            text: text.to_string(),
            locked: None,
        })
        .await
    }

    /// Create a message activity
    pub async fn create_message(
        &self,
        recipient_id: i32,
        message: &str,
        private: Option<bool>,
    ) -> Result<ActivityUnion, AniListError> {
        self.save_message_activity(&SaveMessageActivityOptions {
            id: None,
            message: message.to_string(),
            recipient_id,
            private,
            locked: None,
            as_mod: None,
        })
        .await
    }

    /// Reply to an activity
    pub async fn reply_to(
        &self,
        activity_id: i32,
        text: &str,
    ) -> Result<ActivityReply, AniListError> {
        self.save_reply(&SaveActivityReplyOptions {
            id: None,
            text: text.to_string(),
            activity_id,
        })
        .await
    }

    /// Delete an activity
    pub async fn delete_activity(&self, id: i32) -> Result<bool, AniListError> {
        self.delete(&DeleteActivityOptions { id }).await
    }

    /// Delete an activity reply
    pub async fn delete_activity_reply(&self, id: i32) -> Result<bool, AniListError> {
        self.delete_reply(&DeleteActivityReplyOptions { id }).await
    }

    /// Toggle activity subscription
    pub async fn toggle_subscription(
        &self,
        id: i32,
        subscribe: bool,
    ) -> Result<ActivityUnion, AniListError> {
        self.subscribe(&SubscribeActivityOptions { id, subscribe })
            .await
    }

    /// Toggle activity pin status
    pub async fn toggle_pin(&self, id: i32, pinned: bool) -> Result<ActivityUnion, AniListError> {
        self.pin(&PinActivityOptions { id, pinned }).await
    }
}
