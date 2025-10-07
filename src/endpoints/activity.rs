use crate::client::AniListClient;
use crate::error::AniListError;
use crate::models::social::{Activity, ActivityReply, TextActivity};
use crate::queries;
use serde_json::json;
use std::collections::HashMap;

pub struct ActivityEndpoint {
    client: AniListClient,
}

impl ActivityEndpoint {
    pub(crate) fn new(client: AniListClient) -> Self {
        Self { client }
    }

    /// Get recent activities from the global feed
    pub async fn get_recent_activities(
        &self,
        page: i32,
        per_page: i32,
    ) -> Result<Vec<Activity>, AniListError> {
        let query = r#"
            query ($page: Int, $perPage: Int) {
                Page(page: $page, perPage: $perPage) {
                    activities(sort: ID_DESC) {
                        ... on TextActivity {
                            id
                            userId
                            type
                            replyCount
                            likeCount
                            isLiked
                            isSubscribed
                            createdAt
                            siteUrl
                            user {
                                id
                                name
                                avatar {
                                    large
                                    medium
                                }
                            }
                        }
                        ... on ListActivity {
                            id
                            userId
                            type
                            replyCount
                            likeCount
                            isLiked
                            createdAt
                            siteUrl
                            user {
                                id
                                name
                                avatar {
                                    large
                                    medium
                                }
                            }
                        }
                        ... on MessageActivity {
                            id
                            recipientId
                            messengerId
                            type
                            replyCount
                            likeCount
                            isLiked
                            createdAt
                            siteUrl
                            recipient {
                                id
                                name
                                avatar {
                                    large
                                    medium
                                }
                            }
                            messenger {
                                id
                                name
                                avatar {
                                    large
                                    medium
                                }
                            }
                        }
                    }
                }
            }
        "#;

        let mut variables = HashMap::new();
        variables.insert("page".to_string(), json!(page));
        variables.insert("perPage".to_string(), json!(per_page));

        let response = self.client.query(query, Some(variables)).await?;
        let data = response["data"]["Page"]["activities"].clone();
        let activities: Vec<Activity> = serde_json::from_value(data)?;
        Ok(activities)
    }

    /// Get activities from following users (requires authentication)
    pub async fn get_following_activities(
        &self,
        page: i32,
        per_page: i32,
    ) -> Result<Vec<Activity>, AniListError> {
        let query = r#"
            query ($page: Int, $perPage: Int) {
                Page(page: $page, perPage: $perPage) {
                    activities(sort: ID_DESC, isFollowing: true) {
                        ... on TextActivity {
                            id
                            userId
                            type
                            replyCount
                            likeCount
                            isLiked
                            isSubscribed
                            createdAt
                            siteUrl
                            user {
                                id
                                name
                                avatar {
                                    large
                                    medium
                                }
                            }
                        }
                        ... on ListActivity {
                            id
                            userId
                            type
                            replyCount
                            likeCount
                            isLiked
                            createdAt
                            siteUrl
                            user {
                                id
                                name
                                avatar {
                                    large
                                    medium
                                }
                            }
                        }
                    }
                }
            }
        "#;

        let mut variables = HashMap::new();
        variables.insert("page".to_string(), json!(page));
        variables.insert("perPage".to_string(), json!(per_page));

        let response = self.client.query(query, Some(variables)).await?;
        let data = response["data"]["Page"]["activities"].clone();
        let activities: Vec<Activity> = serde_json::from_value(data)?;
        Ok(activities)
    }

    /// Get user activities by user ID
    pub async fn get_user_activities(
        &self,
        user_id: i32,
        page: i32,
        per_page: i32,
    ) -> Result<Vec<Activity>, AniListError> {
        let query = r#"
            query ($userId: Int, $page: Int, $perPage: Int) {
                Page(page: $page, perPage: $perPage) {
                    activities(userId: $userId, sort: ID_DESC) {
                        ... on TextActivity {
                            id
                            userId
                            type
                            replyCount
                            likeCount
                            isLiked
                            isSubscribed
                            createdAt
                            siteUrl
                            user {
                                id
                                name
                                avatar {
                                    large
                                    medium
                                }
                            }
                        }
                        ... on ListActivity {
                            id
                            userId
                            type
                            replyCount
                            likeCount
                            isLiked
                            createdAt
                            siteUrl
                            user {
                                id
                                name
                                avatar {
                                    large
                                    medium
                                }
                            }
                        }
                    }
                }
            }
        "#;

        let mut variables = HashMap::new();
        variables.insert("userId".to_string(), json!(user_id));
        variables.insert("page".to_string(), json!(page));
        variables.insert("perPage".to_string(), json!(per_page));

        let response = self.client.query(query, Some(variables)).await?;
        let data = response["data"]["Page"]["activities"].clone();
        let activities: Vec<Activity> = serde_json::from_value(data)?;
        Ok(activities)
    }

    /// Get text activities
    pub async fn get_text_activities(
        &self,
        page: i32,
        per_page: i32,
    ) -> Result<Vec<TextActivity>, AniListError> {
        let query = r#"
            query ($page: Int, $perPage: Int) {
                Page(page: $page, perPage: $perPage) {
                    activities(type: TEXT, sort: ID_DESC) {
                        ... on TextActivity {
                            id
                            userId
                            text
                            replyCount
                            likeCount
                            isLiked
                            isPinned
                            siteUrl
                            createdAt
                            user {
                                id
                                name
                                avatar {
                                    large
                                    medium
                                }
                            }
                        }
                    }
                }
            }
        "#;

        let mut variables = HashMap::new();
        variables.insert("page".to_string(), json!(page));
        variables.insert("perPage".to_string(), json!(per_page));

        let response = self.client.query(query, Some(variables)).await?;
        let data = response["data"]["Page"]["activities"].clone();
        let activities: Vec<TextActivity> = serde_json::from_value(data)?;
        Ok(activities)
    }

    /// Get activity by ID
    pub async fn get_activity_by_id(&self, id: i32) -> Result<Activity, AniListError> {
        let query = r#"
            query ($id: Int) {
                Activity(id: $id) {
                    ... on TextActivity {
                        id
                        userId
                        type
                        replyCount
                        likeCount
                        isLiked
                        isSubscribed
                        createdAt
                        siteUrl
                        user {
                            id
                            name
                            avatar {
                                large
                                medium
                            }
                        }
                    }
                    ... on ListActivity {
                        id
                        userId
                        type
                        replyCount
                        likeCount
                        isLiked
                        createdAt
                        siteUrl
                        user {
                            id
                            name
                            avatar {
                                large
                                medium
                            }
                        }
                    }
                    ... on MessageActivity {
                        id
                        recipientId
                        messengerId
                        type
                        replyCount
                        likeCount
                        isLiked
                        createdAt
                        siteUrl
                        recipient {
                            id
                            name
                            avatar {
                                large
                                medium
                            }
                        }
                        messenger {
                            id
                            name
                            avatar {
                                large
                                medium
                            }
                        }
                    }
                }
            }
        "#;

        let mut variables = HashMap::new();
        variables.insert("id".to_string(), json!(id));

        let response = self.client.query(query, Some(variables)).await?;
        let data = response["data"]["Activity"].clone();
        let activity: Activity = serde_json::from_value(data)?;
        Ok(activity)
    }

    /// Get activity replies
    pub async fn get_activity_replies(
        &self,
        activity_id: i32,
        page: i32,
        per_page: i32,
    ) -> Result<Vec<ActivityReply>, AniListError> {
        let query = r#"
            query ($activityId: Int, $page: Int, $perPage: Int) {
                Page(page: $page, perPage: $perPage) {
                    activityReplies(activityId: $activityId) {
                        id
                        userId
                        activityId
                        text
                        likeCount
                        isLiked
                        createdAt
                        user {
                            id
                            name
                            avatar {
                                large
                                medium
                            }
                        }
                    }
                }
            }
        "#;

        let mut variables = HashMap::new();
        variables.insert("activityId".to_string(), json!(activity_id));
        variables.insert("page".to_string(), json!(page));
        variables.insert("perPage".to_string(), json!(per_page));

        let response = self.client.query(query, Some(variables)).await?;
        let data = response["data"]["Page"]["activityReplies"].clone();
        let replies: Vec<ActivityReply> = serde_json::from_value(data)?;
        Ok(replies)
    }

    /// Create a text activity (requires authentication)
    pub async fn create_text_activity(&self, text: &str) -> Result<TextActivity, AniListError> {
        let query = r#"
            mutation ($text: String) {
                SaveTextActivity(text: $text) {
                    id
                    userId
                    text
                    replyCount
                    likeCount
                    isLiked
                    isPinned
                    siteUrl
                    createdAt
                    user {
                        id
                        name
                        avatar {
                            large
                            medium
                        }
                    }
                }
            }
        "#;

        let mut variables = HashMap::new();
        variables.insert("text".to_string(), json!(text));

        let response = self.client.query(query, Some(variables)).await?;
        let data = response["data"]["SaveTextActivity"].clone();
        let activity: TextActivity = serde_json::from_value(data)?;
        Ok(activity)
    }

    /// Post a reply to an activity (requires authentication)
    pub async fn post_activity_reply(
        &self,
        activity_id: i32,
        text: &str,
    ) -> Result<ActivityReply, AniListError> {
        let query = queries::activity::REPLY_TO_ACTIVITY;

        let mut variables = HashMap::new();
        variables.insert("activityId".to_string(), json!(activity_id));
        variables.insert("text".to_string(), json!(text));

        let response = self.client.query(query, Some(variables)).await?;
        let data = response["data"]["SaveActivityReply"].clone();
        let reply: ActivityReply = serde_json::from_value(data)?;
        Ok(reply)
    }

    /// Toggle like on an activity (requires authentication)
    pub async fn toggle_activity_like(&self, id: i32) -> Result<Activity, AniListError> {
        let query = queries::activity::TOGGLE_LIKE;

        let mut variables = HashMap::new();
        variables.insert("id".to_string(), json!(id));
        variables.insert("type".to_string(), json!("ACTIVITY"));

        let response = self.client.query(query, Some(variables)).await?;
        let data = response["data"]["ToggleLikeV2"].clone();
        let activity: Activity = serde_json::from_value(data)?;
        Ok(activity)
    }

    /// Toggle like on an activity reply (requires authentication)
    pub async fn toggle_activity_reply_like(&self, id: i32) -> Result<ActivityReply, AniListError> {
        let query = r#"
            mutation ($id: Int, $type: LikeableType) {
                ToggleLikeV2(id: $id, type: $type) {
                    ... on ActivityReply {
                        id
                        text
                        likeCount
                        isLiked
                    }
                }
            }
        "#;

        let mut variables = HashMap::new();
        variables.insert("id".to_string(), json!(id));
        variables.insert("type".to_string(), json!("ACTIVITY_REPLY"));

        let response = self.client.query(query, Some(variables)).await?;
        let data = response["data"]["ToggleLikeV2"].clone();
        let reply: ActivityReply = serde_json::from_value(data)?;
        Ok(reply)
    }

    /// Delete an activity (requires authentication and ownership)
    pub async fn delete_activity(&self, id: i32) -> Result<bool, AniListError> {
        let query = r#"
            mutation ($id: Int) {
                DeleteActivity(id: $id) {
                    deleted
                }
            }
        "#;

        let mut variables = HashMap::new();
        variables.insert("id".to_string(), json!(id));

        let response = self.client.query(query, Some(variables)).await?;
        let deleted = response["data"]["DeleteActivity"]["deleted"]
            .as_bool()
            .unwrap_or(false);
        Ok(deleted)
    }
}
