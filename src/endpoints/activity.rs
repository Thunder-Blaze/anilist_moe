use crate::client::AniListClient;
use crate::errors::AniListError;
use crate::enums::activity::{ActivitySort, ActivityType};
use crate::objects::responses::{ActivityListResponse, ActivityResponse, ActivitySingleResponse};
use serde::Serialize;
use serde_json::{json, Value};
use std::collections::HashMap;

#[derive(Default, Serialize)]
pub struct ActivitySearchOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i32>,
    #[serde(rename = "perPage", skip_serializing_if = "Option::is_none")]
    pub per_page: Option<i32>,
    #[serde(rename = "userId", skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i32>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub activity_type: Option<ActivityType>,
    #[serde(rename = "isFollowing", skip_serializing_if = "Option::is_none")]
    pub is_following: Option<bool>,
    #[serde(rename = "hasReplies", skip_serializing_if = "Option::is_none")]
    pub has_replies: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<Vec<ActivitySort>>,
}

pub struct ActivityEndpoint(pub(crate) AniListClient);

impl ActivityEndpoint {
    pub fn new(client: AniListClient) -> Self {
        Self(client)
    }

    /// Search activities with options
    pub async fn search(&self, options: ActivitySearchOptions) -> Result<ActivityListResponse, AniListError> {
        let query = include_str!("../queries/activity/search_activities.graphql");
        let variables = json!(options);
        let variables_map = self.value_to_hashmap(variables);
        self.0.query_typed(query, Some(&variables_map)).await
    }

    /// Get activities for a specific user
    pub async fn get_user_activities(&self, user_id: i32, page: i32, per_page: i32) -> Result<ActivityListResponse, AniListError> {
        let options = ActivitySearchOptions {
            user_id: Some(user_id),
            page: Some(page),
            per_page: Some(per_page),
            sort: Some(vec![ActivitySort::IdDesc]),
            ..Default::default()
        };
        self.search(options).await
    }

    /// Get following activities (requires authentication)
    pub async fn get_following_activities(&self, page: i32, per_page: i32) -> Result<ActivityListResponse, AniListError> {
        let options = ActivitySearchOptions {
            page: Some(page),
            per_page: Some(per_page),
            is_following: Some(true),
            sort: Some(vec![ActivitySort::IdDesc]),
            ..Default::default()
        };
        self.search(options).await
    }

    /// Get global activities (all public activities)
    pub async fn get_global_activities(&self, page: i32, per_page: i32) -> Result<ActivityListResponse, AniListError> {
        let options = ActivitySearchOptions {
            page: Some(page),
            per_page: Some(per_page),
            sort: Some(vec![ActivitySort::IdDesc]),
            ..Default::default()
        };
        self.search(options).await
    }

    /// Get activities by type
    pub async fn get_by_type(&self, activity_type: ActivityType, page: i32, per_page: i32) -> Result<ActivityListResponse, AniListError> {
        let options = ActivitySearchOptions {
            activity_type: Some(activity_type),
            page: Some(page),
            per_page: Some(per_page),
            sort: Some(vec![ActivitySort::IdDesc]),
            ..Default::default()
        };
        self.search(options).await
    }

    /// Get activity by ID
    pub async fn get_by_id(&self, id: i32) -> Result<ActivitySingleResponse, AniListError> {
        let query = include_str!("../queries/activity/fetch_activity.graphql");
        let variables = json!({ "id": id });
        let variables_map = self.value_to_hashmap(variables);
        self.0.query_typed(query, Some(&variables_map)).await
    }

    /// Save text activity (requires authentication)
    pub async fn save_text_activity(&self, text: &str, activity_id: Option<i32>) -> Result<ActivityResponse, AniListError> {
        let query = include_str!("../queries/activity/save_text_activity.graphql");
        let mut variables = json!({ "text": text });

        if let Some(id) = activity_id {
            variables["id"] = json!(id);
        }

        let variables_map = self.value_to_hashmap(variables);
        self.0.query_typed(query, Some(&variables_map)).await
    }

    /// Reply to activity (requires authentication)
    pub async fn reply_to_activity(&self, activity_id: i32, text: &str) -> Result<ActivityResponse, AniListError> {
        let query = include_str!("../queries/activity/reply_to_activity.graphql");
        let variables = json!({
            "activityId": activity_id,
            "text": text
        });
        let variables_map = self.value_to_hashmap(variables);
        self.0.query_typed(query, Some(&variables_map)).await
    }

    /// Save activity reply (requires authentication)
    pub async fn save_activity_reply(&self, activity_id: i32, text: &str, reply_id: Option<i32>) -> Result<ActivityResponse, AniListError> {
        let query = include_str!("../queries/activity/save_activity_reply.graphql");
        let mut variables = json!({
            "activityId": activity_id,
            "text": text
        });

        if let Some(id) = reply_id {
            variables["id"] = json!(id);
        }

        let variables_map = self.value_to_hashmap(variables);
        self.0.query_typed(query, Some(&variables_map)).await
    }

    /// Toggle like on activity (requires authentication)
    pub async fn toggle_like(&self, activity_id: i32) -> Result<ActivityResponse, AniListError> {
        let query = include_str!("../queries/activity/toggle_like.graphql");
        let variables = json!({ "activityId": activity_id });
        let variables_map = self.value_to_hashmap(variables);
        self.0.query_typed(query, Some(&variables_map)).await
    }

    /// Delete activity (requires authentication)
    pub async fn delete_activity(&self, id: i32) -> Result<Value, AniListError> {
        let query = include_str!("../queries/activity/delete_activity.graphql");
        let variables = json!({ "id": id });
        let variables_map = self.value_to_hashmap(variables);
        self.0.query(query, Some(&variables_map)).await
    }

    /// Delete activity reply (requires authentication)
    pub async fn delete_activity_reply(&self, id: i32) -> Result<Value, AniListError> {
        let query = include_str!("../queries/activity/delete_activity_reply.graphql");
        let variables = json!({ "id": id });
        let variables_map = self.value_to_hashmap(variables);
        self.0.query(query, Some(&variables_map)).await
    }

    fn value_to_hashmap(&self, value: Value) -> HashMap<String, Value> {
        match value {
            Value::Object(map) => map.into_iter().collect(),
            _ => HashMap::new(),
        }
    }
}
