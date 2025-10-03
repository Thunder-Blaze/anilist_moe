use crate::enums::activity::{ActivitySort, ActivityType};
use crate::errors::AniListError;
use crate::objects::responses::{
    ActivityListResponse, ActivityReplyListResponse, ActivitySingleResponse,
    DeleteActivityReplyResponse, DeleteActivityResponse, SaveActivityReplyResponse,
    SaveMessageActivityResponse, SaveTextActivityResponse,
};
use crate::{client::AniListClient, queries::activity};
use serde::Serialize;
use serde_json::json;

/// Options for fetching activity feed entries.
#[derive(Default, Serialize)]
pub struct FetchActivityOptions {
    // # Pagination
    #[serde(rename = "page", skip_serializing_if = "Option::is_none")]
    pub page: Option<i32>,
    #[serde(rename = "perPage", skip_serializing_if = "Option::is_none")]
    pub per_page: Option<i32>,

    // # Filters
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "userId", skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i32>,
    #[serde(rename = "messengerId", skip_serializing_if = "Option::is_none")]
    pub messenger_id: Option<i32>,
    #[serde(rename = "mediaId", skip_serializing_if = "Option::is_none")]
    pub media_id: Option<i32>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<ActivityType>,
    #[serde(rename = "isFollowing", skip_serializing_if = "Option::is_none")]
    pub is_following: Option<bool>,
    #[serde(rename = "hasReplies", skip_serializing_if = "Option::is_none")]
    pub has_replies: Option<bool>,
    #[serde(
        rename = "hasRepliesOrTypeText",
        skip_serializing_if = "Option::is_none"
    )]
    pub has_replies_or_type_text: Option<bool>,
    #[serde(rename = "createdAt", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<i32>,
    #[serde(rename = "id_not", skip_serializing_if = "Option::is_none")]
    pub id_not: Option<i32>,
    #[serde(rename = "id_in", skip_serializing_if = "Option::is_none")]
    pub id_in: Option<Vec<i32>>,
    #[serde(rename = "id_not_in", skip_serializing_if = "Option::is_none")]
    pub id_not_in: Option<Vec<i32>>,
    #[serde(rename = "userId_not", skip_serializing_if = "Option::is_none")]
    pub user_id_not: Option<i32>,
    #[serde(rename = "userId_in", skip_serializing_if = "Option::is_none")]
    pub user_id_in: Option<Vec<i32>>,
    #[serde(rename = "userId_not_in", skip_serializing_if = "Option::is_none")]
    pub user_id_not_in: Option<Vec<i32>>,
    #[serde(rename = "messengerId_not", skip_serializing_if = "Option::is_none")]
    pub messenger_id_not: Option<i32>,
    #[serde(rename = "messengerId_in", skip_serializing_if = "Option::is_none")]
    pub messenger_id_in: Option<Vec<i32>>,
    #[serde(rename = "messengerId_not_in", skip_serializing_if = "Option::is_none")]
    pub messenger_id_not_in: Option<Vec<i32>>,
    #[serde(rename = "mediaId_not", skip_serializing_if = "Option::is_none")]
    pub media_id_not: Option<i32>,
    #[serde(rename = "mediaId_in", skip_serializing_if = "Option::is_none")]
    pub media_id_in: Option<Vec<i32>>,
    #[serde(rename = "mediaId_not_in", skip_serializing_if = "Option::is_none")]
    pub media_id_not_in: Option<Vec<i32>>,
    #[serde(rename = "type_not", skip_serializing_if = "Option::is_none")]
    pub type_not: Option<ActivityType>,
    #[serde(rename = "type_in", skip_serializing_if = "Option::is_none")]
    pub type_in: Option<Vec<ActivityType>>,
    #[serde(rename = "type_not_in", skip_serializing_if = "Option::is_none")]
    pub type_not_in: Option<Vec<ActivityType>>,
    #[serde(rename = "createdAt_greater", skip_serializing_if = "Option::is_none")]
    pub created_at_greater: Option<i32>,
    #[serde(rename = "createdAt_lesser", skip_serializing_if = "Option::is_none")]
    pub created_at_lesser: Option<i32>,

    // # Sort
    #[serde(rename = "sort", skip_serializing_if = "Option::is_none")]
    pub sort: Option<Vec<ActivitySort>>,
}

/// Options for fetching a single activity by ID.
#[derive(Default, Serialize)]
pub struct FetchActivityOneOptions {
    pub id: i32,
}

/// Options for fetching replies to an activity.
#[derive(Default, Serialize)]
pub struct FetchActivityRepliesOptions {
    // # Pagination
    #[serde(rename = "page", skip_serializing_if = "Option::is_none")]
    pub page: Option<i32>,
    #[serde(rename = "perPage", skip_serializing_if = "Option::is_none")]
    pub per_page: Option<i32>,

    // # Filters
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "activityId")]
    pub activity_id: i32,
}

/// Options for deleting an activity.
#[derive(Default, Serialize)]
pub struct DeleteActivityOptions {
    pub id: i32,
}

/// Options for deleting an activity reply.
#[derive(Default, Serialize)]
pub struct DeleteActivityReplyOptions {
    pub id: i32,
}

/// Options for pinning or unpinning an activity.
#[derive(Default, Serialize)]
pub struct PinActivityOptions {
    pub id: i32,
    pub pinned: bool,
}

/// Options for subscribing or unsubscribing to an activity.
#[derive(Default, Serialize)]
pub struct SubscribeActivityOptions {
    pub id: i32,
    pub subscribe: bool,
}

/// Options for saving a message activity.
#[derive(Default, Serialize)]
pub struct SaveMessageActivityOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    pub message: String,
    #[serde(rename = "recipientId")]
    pub recipient_id: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locked: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub as_mod: Option<bool>,
}

/// Options for saving a text activity.
#[derive(Default, Serialize)]
pub struct SaveTextActivityOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locked: Option<bool>,
}

/// Options for saving a reply to an activity.
#[derive(Default, Serialize)]
pub struct SaveActivityReplyOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
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
        options: FetchActivityOptions,
    ) -> Result<ActivityListResponse, AniListError> {
        let query = activity::FETCH;
        let variables = json!(options);
        let variables_map = crate::utils::json_to_hashmap(variables);
        self.client.query_typed(query, Some(&variables_map)).await
    }

    pub async fn fetch_one(
        &self,
        options: FetchActivityOneOptions,
    ) -> Result<ActivitySingleResponse, AniListError> {
        let query = activity::FETCH_ONE;
        let variables = json!(options);
        let variables_map = crate::utils::json_to_hashmap(variables);
        self.client.query_typed(query, Some(&variables_map)).await
    }

    pub async fn fetch_replies(
        &self,
        options: FetchActivityRepliesOptions,
    ) -> Result<ActivityReplyListResponse, AniListError> {
        let query = activity::FETCH_REPLIES;
        let variables = json!(options);
        let variables_map = crate::utils::json_to_hashmap(variables);
        self.client.query_typed(query, Some(&variables_map)).await
    }

    pub async fn delete(
        &self,
        options: DeleteActivityOptions,
    ) -> Result<DeleteActivityResponse, AniListError> {
        let query = activity::DELETE;
        let variables = json!(options);
        let variables_map = crate::utils::json_to_hashmap(variables);
        self.client.query_typed(query, Some(&variables_map)).await
    }

    pub async fn delete_reply(
        &self,
        options: DeleteActivityReplyOptions,
    ) -> Result<DeleteActivityReplyResponse, AniListError> {
        let query = activity::DELETE_REPLY;
        let variables = json!(options);
        let variables_map = crate::utils::json_to_hashmap(variables);
        self.client.query_typed(query, Some(&variables_map)).await
    }

    pub async fn pin(
        &self,
        options: PinActivityOptions,
    ) -> Result<ActivitySingleResponse, AniListError> {
        let query = activity::PIN;
        let variables = json!(options);
        let variables_map = crate::utils::json_to_hashmap(variables);
        self.client.query_typed(query, Some(&variables_map)).await
    }

    pub async fn subscribe(
        &self,
        options: SubscribeActivityOptions,
    ) -> Result<ActivitySingleResponse, AniListError> {
        let query = activity::SUBSCRIBE;
        let variables = json!(options);
        let variables_map = crate::utils::json_to_hashmap(variables);
        self.client.query_typed(query, Some(&variables_map)).await
    }

    pub async fn save_message_activity(
        &self,
        options: SaveMessageActivityOptions,
    ) -> Result<SaveMessageActivityResponse, AniListError> {
        let query = activity::SAVE_MESSAGE_ACTIVITY;
        let variables = json!(options);
        let variables_map = crate::utils::json_to_hashmap(variables);
        self.client.query_typed(query, Some(&variables_map)).await
    }

    pub async fn save_text_activity(
        &self,
        options: SaveTextActivityOptions,
    ) -> Result<SaveTextActivityResponse, AniListError> {
        let query = activity::SAVE_TEXT_ACTIVITY;
        let variables = json!(options);
        let variables_map = crate::utils::json_to_hashmap(variables);
        self.client.query_typed(query, Some(&variables_map)).await
    }

    pub async fn save_reply(
        &self,
        options: SaveActivityReplyOptions,
    ) -> Result<SaveActivityReplyResponse, AniListError> {
        let query = activity::SAVE_REPLY;
        let variables = json!(options);
        let variables_map = crate::utils::json_to_hashmap(variables);
        self.client.query_typed(query, Some(&variables_map)).await
    }

    // Convenience functions

    /// Get recent activities
    pub async fn get_recent(
        &self,
        page: Option<i32>,
        per_page: Option<i32>,
    ) -> Result<ActivityListResponse, AniListError> {
        self.fetch(FetchActivityOptions {
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
    ) -> Result<ActivityListResponse, AniListError> {
        self.fetch(FetchActivityOptions {
            is_following: Some(true),
            page,
            per_page,
            sort: Some(vec![ActivitySort::IdDesc]),
            ..Default::default()
        })
        .await
    }

    /// Get activity by ID
    pub async fn get_by_id(&self, id: i32) -> Result<ActivitySingleResponse, AniListError> {
        self.fetch_one(FetchActivityOneOptions { id }).await
    }

    /// Create a text activity
    pub async fn create_text_activity(
        &self,
        text: &str,
    ) -> Result<SaveTextActivityResponse, AniListError> {
        self.save_text_activity(SaveTextActivityOptions {
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
    ) -> Result<SaveMessageActivityResponse, AniListError> {
        self.save_message_activity(SaveMessageActivityOptions {
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
    ) -> Result<SaveActivityReplyResponse, AniListError> {
        self.save_reply(SaveActivityReplyOptions {
            id: None,
            text: text.to_string(),
            activity_id,
        })
        .await
    }

    /// Delete an activity
    pub async fn delete_activity(&self, id: i32) -> Result<DeleteActivityResponse, AniListError> {
        self.delete(DeleteActivityOptions { id }).await
    }

    /// Delete an activity reply
    pub async fn delete_activity_reply(
        &self,
        id: i32,
    ) -> Result<DeleteActivityReplyResponse, AniListError> {
        self.delete_reply(DeleteActivityReplyOptions { id }).await
    }

    /// Toggle activity subscription
    pub async fn toggle_subscription(
        &self,
        id: i32,
        subscribe: bool,
    ) -> Result<ActivitySingleResponse, AniListError> {
        self.subscribe(SubscribeActivityOptions { id, subscribe })
            .await
    }

    /// Toggle activity pin status
    pub async fn toggle_pin(
        &self,
        id: i32,
        pinned: bool,
    ) -> Result<ActivitySingleResponse, AniListError> {
        self.pin(PinActivityOptions { id, pinned }).await
    }
}
