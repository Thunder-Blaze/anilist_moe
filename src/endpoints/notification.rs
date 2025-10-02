use crate::{client::AniListClient, queries::notification};
use crate::errors::AniListError;
use crate::enums::notification::NotificationType;
use crate::objects::responses::NotificationResponse;
use serde::Serialize;
use serde_json::json;

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

pub struct NotificationEndpoint {
    pub client: AniListClient,
}

impl NotificationEndpoint {
    pub fn new(client: AniListClient) -> Self {
        Self { client }
    }

    pub async fn fetch(&self, options: NotificationSearchOptions) -> Result<NotificationResponse, AniListError> {
        let query = notification::FETCH;
        let variables = json!(options);
        let variables_map = crate::utils::json_to_hashmap(variables);
        println!("Variables Map: {:?}", variables_map);
        let x = self.client.query_typed(query, Some(&variables_map)).await;
        println!("Query Result: {:?}", x);
        x
    }
}
