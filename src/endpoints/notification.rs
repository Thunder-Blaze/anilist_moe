use crate::client::AniListClient;
use crate::error::AniListError;
use crate::models::social::Notification;
use serde_json::json;
use std::collections::HashMap;

pub struct NotificationEndpoint {
    client: AniListClient,
}

impl NotificationEndpoint {
    pub(crate) fn new(client: AniListClient) -> Self {
        Self { client }
    }

    /// Get user notifications (requires authentication)
    pub async fn get_notifications(
        &self,
        page: i32,
        per_page: i32,
    ) -> Result<Vec<Notification>, AniListError> {
        let query = r#"
            query ($page: Int, $perPage: Int) {
                Page(page: $page, perPage: $perPage) {
                    notifications(sort: ID_DESC) {
                        ... on AiringNotification {
                            id
                            userId
                            type
                            animeId
                            episode
                            contexts
                            createdAt
                            media {
                                id
                                type
                                title {
                                    romaji
                                    english
                                    native
                                    userPreferred
                                }
                                coverImage {
                                    large
                                    medium
                                    color
                                }
                            }
                        }
                        ... on FollowingNotification {
                            id
                            userId
                            type
                            contexts
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
                        ... on ActivityMessageNotification {
                            id
                            userId
                            type
                            contexts
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
                        ... on ActivityMentionNotification {
                            id
                            userId
                            type
                            contexts
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
                        ... on ActivityReplyNotification {
                            id
                            userId
                            type
                            contexts
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
                        ... on ThreadCommentMentionNotification {
                            id
                            userId
                            type
                            contexts
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
                        ... on ThreadCommentReplyNotification {
                            id
                            userId
                            type
                            contexts
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
                        ... on ThreadCommentSubscribedNotification {
                            id
                            userId
                            type
                            contexts
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
                        ... on ActivityLikeNotification {
                            id
                            userId
                            type
                            contexts
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
                        ... on ActivityReplyLikeNotification {
                            id
                            userId
                            type
                            contexts
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
                        ... on ThreadLikeNotification {
                            id
                            userId
                            type
                            contexts
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
                        ... on ThreadCommentLikeNotification {
                            id
                            userId
                            type
                            contexts
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
                        ... on ActivityReplySubscribedNotification {
                            id
                            userId
                            type
                            contexts
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
                        ... on RelatedMediaAdditionNotification {
                            id
                            userId
                            type
                            contexts
                            createdAt
                            media {
                                id
                                type
                                title {
                                    userPreferred
                                }
                                coverImage {
                                    large
                                }
                            }
                        }
                        ... on MediaDataChangeNotification {
                            id
                            userId
                            type
                            contexts
                            createdAt
                            media {
                                id
                                type
                                title {
                                    userPreferred
                                }
                                coverImage {
                                    large
                                }
                            }
                        }
                        ... on MediaMergeNotification {
                            id
                            userId
                            type
                            contexts
                            createdAt
                        }
                        ... on MediaDeletionNotification {
                            id
                            userId
                            type
                            contexts
                            createdAt
                        }
                    }
                }
            }
        "#;

        let mut variables = HashMap::new();
        variables.insert("page".to_string(), json!(page));
        variables.insert("perPage".to_string(), json!(per_page));

        let response = self.client.query(query, Some(variables)).await?;
        let data = response["data"]["Page"]["notifications"].clone();
        let notifications: Vec<Notification> = serde_json::from_value(data)?;
        Ok(notifications)
    }

    /// Get unread notification count (requires authentication)
    pub async fn get_unread_count(&self) -> Result<i32, AniListError> {
        let query = r#"
            query {
                Viewer {
                    unreadNotificationCount
                }
            }
        "#;

        let response = self.client.query(query, None).await?;
        let count = response["data"]["Viewer"]["unreadNotificationCount"]
            .as_i64()
            .unwrap_or(0) as i32;
        Ok(count)
    }

    /// Get notifications by type (requires authentication)
    pub async fn get_notifications_by_type(
        &self,
        notification_type: &str,
        page: i32,
        per_page: i32,
    ) -> Result<Vec<Notification>, AniListError> {
        let query = r#"
            query ($type: [NotificationType], $page: Int, $perPage: Int) {
                Page(page: $page, perPage: $perPage) {
                    notifications(type_in: $type, sort: ID_DESC) {
                        ... on AiringNotification {
                            id
                            userId
                            type
                            animeId
                            episode
                            contexts
                            createdAt
                            media {
                                id
                                type
                                title {
                                    userPreferred
                                }
                                coverImage {
                                    large
                                }
                            }
                        }
                        ... on FollowingNotification {
                            id
                            userId
                            type
                            contexts
                            createdAt
                            user {
                                id
                                name
                                avatar {
                                    large
                                }
                            }
                        }
                        ... on ActivityMessageNotification {
                            id
                            userId
                            type
                            contexts
                            createdAt
                            user {
                                id
                                name
                                avatar {
                                    large
                                }
                            }
                        }
                    }
                }
            }
        "#;

        let mut variables = HashMap::new();
        variables.insert("type".to_string(), json!([notification_type]));
        variables.insert("page".to_string(), json!(page));
        variables.insert("perPage".to_string(), json!(per_page));

        let response = self.client.query(query, Some(variables)).await?;
        let data = response["data"]["Page"]["notifications"].clone();
        let notifications: Vec<Notification> = serde_json::from_value(data)?;
        Ok(notifications)
    }

    /// Mark notifications as read (requires authentication)
    pub async fn mark_notifications_as_read(
        &self,
        notification_ids: Vec<i32>,
    ) -> Result<bool, AniListError> {
        let query = r#"
            mutation ($notificationIds: [Int]) {
                SaveNotificationSettings(notificationIds: $notificationIds) {
                    id
                }
            }
        "#;

        let mut variables = HashMap::new();
        variables.insert("notificationIds".to_string(), json!(notification_ids));

        let response = self.client.query(query, Some(variables)).await?;
        // If we get a response without errors, consider it successful
        Ok(response["data"]["SaveNotificationSettings"].is_object())
    }
}
