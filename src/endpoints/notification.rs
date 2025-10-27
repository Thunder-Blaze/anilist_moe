use crate::enums::notification::NotificationType;
use crate::errors::AniListError;
use crate::objects::responses::{GraphQLResponse, Page};
use crate::unions::notification::NotificationUnion;
use crate::{client::AniListClient, queries::notification};
use serde::{Deserialize, Serialize};
use serde_json::json;
use serde_with::skip_serializing_none;

/// Options for searching and filtering notifications.
#[skip_serializing_none]
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct NotificationSearchOptions {
    pub page: Option<i32>,
    #[serde(rename = "perPage")]
    pub per_page: Option<i32>,
    #[serde(rename = "type")]
    pub notification_type: Option<NotificationType>,
    #[serde(rename = "type_in")]
    pub type_in: Option<Vec<NotificationType>>,
    #[serde(rename = "resetNotificationCount")]
    pub reset_notification_count: Option<bool>,
}

/// Endpoint for notification operations.
pub struct NotificationEndpoint {
    pub client: AniListClient,
}

impl NotificationEndpoint {
    pub fn new(client: AniListClient) -> Self {
        Self { client }
    }

    pub async fn fetch(
        &self,
        options: &NotificationSearchOptions,
    ) -> Result<Page<Vec<NotificationUnion>>, AniListError> {
        let query = notification::FETCH;
        let variables = json!(options);
        let variables_map = crate::utils::json_to_hashmap(variables);
        let response: Result<GraphQLResponse<Page<Vec<NotificationUnion>>>, AniListError> =
            self.client.query_typed(query, Some(&variables_map)).await;
        match response {
            Ok(res) => Ok(res.data),
            Err(err) => Err(err),
        }
    }

    // Convenience functions

    /// Get all notifications (requires authentication)
    pub async fn get_all(
        &self,
        page: Option<i32>,
        per_page: Option<i32>,
    ) -> Result<Page<Vec<NotificationUnion>>, AniListError> {
        self.fetch(&NotificationSearchOptions {
            page,
            per_page,
            ..Default::default()
        })
        .await
    }

    /// Get unread notifications and mark them as read
    pub async fn get_and_mark_read(
        &self,
        page: Option<i32>,
        per_page: Option<i32>,
    ) -> Result<Page<Vec<NotificationUnion>>, AniListError> {
        self.fetch(&NotificationSearchOptions {
            page,
            per_page,
            reset_notification_count: Some(true),
            ..Default::default()
        })
        .await
    }

    /// Get notifications of a specific type
    pub async fn get_by_type(
        &self,
        notification_type: NotificationType,
        page: Option<i32>,
        per_page: Option<i32>,
    ) -> Result<Page<Vec<NotificationUnion>>, AniListError> {
        self.fetch(&NotificationSearchOptions {
            notification_type: Some(notification_type),
            page,
            per_page,
            ..Default::default()
        })
        .await
    }
}
