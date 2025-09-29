use crate::client::AniListClient;
use crate::errors::AniListError;
use crate::enums::notification::NotificationType;
use crate::objects::responses::{NotificationResponse, UnreadCountResponse};
use serde::Serialize;
use serde_json::{json, Value};
use std::collections::HashMap;

#[derive(Default, Serialize)]
pub struct NotificationSearchOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i32>,
    #[serde(rename = "perPage", skip_serializing_if = "Option::is_none")]
    pub per_page: Option<i32>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub notification_type: Option<NotificationType>,
    #[serde(rename = "type_in", skip_serializing_if = "Option::is_none")]
    pub type_in: Option<Vec<NotificationType>>,
    #[serde(rename = "resetNotificationCount", skip_serializing_if = "Option::is_none")]
    pub reset_notification_count: Option<bool>,
}

pub struct NotificationEndpoint(pub(crate) AniListClient);

impl NotificationEndpoint {
    pub fn new(client: AniListClient) -> Self {
        Self(client)
    }

    /// Get notifications (requires authentication)
    pub async fn get_notifications(&self, page: i32, per_page: i32) -> Result<NotificationResponse, AniListError> {
        let options = NotificationSearchOptions {
            page: Some(page),
            per_page: Some(per_page),
            ..Default::default()
        };
        self.search_notifications(options).await
    }

    /// Get notifications by type (requires authentication)
    pub async fn get_notifications_by_type(&self, notification_type: NotificationType, page: i32, per_page: i32) -> Result<NotificationResponse, AniListError> {
        let options = NotificationSearchOptions {
            page: Some(page),
            per_page: Some(per_page),
            notification_type: Some(notification_type),
            ..Default::default()
        };
        self.search_notifications(options).await
    }

    /// Get and reset unread notification count (requires authentication)
    pub async fn get_unread_count(&self) -> Result<UnreadCountResponse, AniListError> {
        let options = NotificationSearchOptions {
            page: Some(1),
            per_page: Some(1),
            reset_notification_count: Some(true),
            ..Default::default()
        };
        let query = include_str!("../queries/notification/unread_count.graphql");
        let variables = json!(options);
        let variables_map = self.value_to_hashmap(variables);
        self.0.query_typed(query, Some(&variables_map)).await
    }

    /// Get activity notifications (requires authentication)
    pub async fn get_activity_notifications(&self, page: i32, per_page: i32) -> Result<NotificationResponse, AniListError> {
        let options = NotificationSearchOptions {
            page: Some(page),
            per_page: Some(per_page),
            notification_type: Some(NotificationType::ActivityMessage),
            ..Default::default()
        };
        self.search_notifications(options).await
    }

    /// Get airing notifications (requires authentication)
    pub async fn get_airing_notifications(&self, page: i32, per_page: i32) -> Result<NotificationResponse, AniListError> {
        let options = NotificationSearchOptions {
            page: Some(page),
            per_page: Some(per_page),
            notification_type: Some(NotificationType::Airing),
            ..Default::default()
        };
        self.search_notifications(options).await
    }

    /// Get follow notifications (requires authentication)
    pub async fn get_follow_notifications(&self, page: i32, per_page: i32) -> Result<NotificationResponse, AniListError> {
        let options = NotificationSearchOptions {
            page: Some(page),
            per_page: Some(per_page),
            notification_type: Some(NotificationType::Following),
            ..Default::default()
        };
        self.search_notifications(options).await
    }

    /// Get thread notifications (requires authentication)
    pub async fn get_thread_notifications(&self, page: i32, per_page: i32) -> Result<NotificationResponse, AniListError> {
        let options = NotificationSearchOptions {
            page: Some(page),
            per_page: Some(per_page),
            notification_type: Some(NotificationType::ThreadSubscribed),
            ..Default::default()
        };
        self.search_notifications(options).await
    }

    /// Search notifications with custom options (requires authentication)
    pub async fn search_with_options(&self, options: NotificationSearchOptions) -> Result<NotificationResponse, AniListError> {
        self.search_notifications(options).await
    }

    async fn search_notifications(&self, options: NotificationSearchOptions) -> Result<NotificationResponse, AniListError> {
        let query = include_str!("../queries/notification/notifications.graphql");
        let variables = json!(options);
        let variables_map = self.value_to_hashmap(variables);
        self.0.query_typed(query, Some(&variables_map)).await
    }

    fn value_to_hashmap(&self, value: Value) -> HashMap<String, Value> {
        match value {
            Value::Object(map) => map.into_iter().collect(),
            _ => HashMap::new(),
        }
    }
}
