use crate::client::AniListClient;
use crate::error::AniListError;
use crate::models::social::{Thread, ThreadComment};
use crate::queries;
use serde_json::json;
use std::collections::HashMap;

pub struct ForumEndpoint {
    client: AniListClient,
}

impl ForumEndpoint {
    pub(crate) fn new(client: AniListClient) -> Self {
        Self { client }
    }

    /// Get recent threads
    pub async fn get_recent_threads(
        &self,
        page: i32,
        per_page: i32,
    ) -> Result<Vec<Thread>, AniListError> {
        let query = r#"
            query ($page: Int, $perPage: Int) {
                Page(page: $page, perPage: $perPage) {
                    threads(sort: UPDATED_AT_DESC) {
                        id
                        title
                        body
                        userId
                        replyUserId
                        replyCommentId
                        categories {
                            id
                            name
                        }
                        isLocked
                        isSticky
                        isSubscribed
                        likeCount
                        isLiked
                        repliedAt
                        createdAt
                        updatedAt
                        replyCount
                        viewCount
                        siteUrl
                        user {
                            id
                            name
                            avatar {
                                large
                                medium
                            }
                            donatorTier
                            donatorBadge
                            moderatorRoles
                        }
                        replyUser {
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
        variables.insert("page".to_string(), json!(page));
        variables.insert("perPage".to_string(), json!(per_page));

        let response = self.client.query(query, Some(variables)).await?;
        let data = response["data"]["Page"]["threads"].clone();
        let threads: Vec<Thread> = serde_json::from_value(data)?;
        Ok(threads)
    }

    /// Get thread by ID
    pub async fn get_thread_by_id(&self, id: i32) -> Result<Thread, AniListError> {
        let query = r#"
            query ($id: Int) {
                Thread(id: $id) {
                    id
                    title
                    body
                    userId
                    replyUserId
                    replyCommentId
                    categories {
                        id
                        name
                    }
                    isLocked
                    isSticky
                    isSubscribed
                    likeCount
                    isLiked
                    repliedAt
                    createdAt
                    updatedAt
                    replyCount
                    viewCount
                    siteUrl
                    user {
                        id
                        name
                        avatar {
                            large
                            medium
                        }
                        donatorTier
                        donatorBadge
                        moderatorRoles
                    }
                    replyUser {
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
        variables.insert("id".to_string(), json!(id));

        let response = self.client.query(query, Some(variables)).await?;
        let data = response["data"]["Thread"].clone();
        let thread: Thread = serde_json::from_value(data)?;
        Ok(thread)
    }

    /// Search threads
    pub async fn search_threads(
        &self,
        search: &str,
        page: i32,
        per_page: i32,
    ) -> Result<Vec<Thread>, AniListError> {
        let query = r#"
            query ($search: String, $page: Int, $perPage: Int) {
                Page(page: $page, perPage: $perPage) {
                    threads(search: $search, sort: SEARCH_MATCH) {
                        id
                        title
                        body
                        userId
                        categories {
                            id
                            name
                        }
                        likeCount
                        replyCount
                        viewCount
                        createdAt
                        updatedAt
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
        "#;

        let mut variables = HashMap::new();
        variables.insert("search".to_string(), json!(search));
        variables.insert("page".to_string(), json!(page));
        variables.insert("perPage".to_string(), json!(per_page));

        let response = self.client.query(query, Some(variables)).await?;
        let data = response["data"]["Page"]["threads"].clone();
        let threads: Vec<Thread> = serde_json::from_value(data)?;
        Ok(threads)
    }

    /// Get thread comments
    pub async fn get_thread_comments(
        &self,
        thread_id: i32,
        page: i32,
        per_page: i32,
    ) -> Result<Vec<ThreadComment>, AniListError> {
        let query = r#"
            query ($threadId: Int, $page: Int, $perPage: Int) {
                Page(page: $page, perPage: $perPage) {
                    threadComments(threadId: $threadId) {
                        id
                        userId
                        threadId
                        comment
                        likeCount
                        isLiked
                        createdAt
                        updatedAt
                        siteUrl
                        user {
                            id
                            name
                            avatar {
                                large
                                medium
                            }
                            donatorTier
                            moderatorRoles
                        }
                    }
                }
            }
        "#;

        let mut variables = HashMap::new();
        variables.insert("threadId".to_string(), json!(thread_id));
        variables.insert("page".to_string(), json!(page));
        variables.insert("perPage".to_string(), json!(per_page));

        let response = self.client.query(query, Some(variables)).await?;
        let data = response["data"]["Page"]["threadComments"].clone();
        let comments: Vec<ThreadComment> = serde_json::from_value(data)?;
        Ok(comments)
    }

    /// Create a new thread (requires authentication)
    pub async fn create_thread(
        &self,
        title: &str,
        body: &str,
        categories: Option<Vec<i32>>,
    ) -> Result<Thread, AniListError> {
        let query = r#"
            mutation ($title: String, $body: String, $categories: [Int]) {
                SaveThread(title: $title, body: $body, categories: $categories) {
                    id
                    title
                    body
                    userId
                    categories {
                        id
                        name
                    }
                    isLocked
                    isSticky
                    likeCount
                    replyCount
                    viewCount
                    createdAt
                    updatedAt
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
        "#;

        let mut variables = HashMap::new();
        variables.insert("title".to_string(), json!(title));
        variables.insert("body".to_string(), json!(body));
        if let Some(cats) = categories {
            variables.insert("categories".to_string(), json!(cats));
        }

        let response = self.client.query(query, Some(variables)).await?;
        let data = response["data"]["SaveThread"].clone();
        let thread: Thread = serde_json::from_value(data)?;
        Ok(thread)
    }

    /// Post a comment on a thread (requires authentication)
    pub async fn post_comment(
        &self,
        thread_id: i32,
        comment: &str,
    ) -> Result<ThreadComment, AniListError> {
        let query = queries::forum::COMMENT_ON_THREAD;

        let mut variables = HashMap::new();
        variables.insert("threadId".to_string(), json!(thread_id));
        variables.insert("comment".to_string(), json!(comment));

        let response = self.client.query(query, Some(variables)).await?;
        let data = response["data"]["SaveThreadComment"].clone();
        let thread_comment: ThreadComment = serde_json::from_value(data)?;
        Ok(thread_comment)
    }

    /// Toggle like on a thread (requires authentication)
    pub async fn toggle_thread_like(&self, id: i32) -> Result<Thread, AniListError> {
        let query = r#"
            mutation ($id: Int, $type: LikeableType) {
                ToggleLikeV2(id: $id, type: $type) {
                    ... on Thread {
                        id
                        title
                        likeCount
                        isLiked
                        siteUrl
                    }
                }
            }
        "#;

        let mut variables = HashMap::new();
        variables.insert("id".to_string(), json!(id));
        variables.insert("type".to_string(), json!("THREAD"));

        let response = self.client.query(query, Some(variables)).await?;
        let data = response["data"]["ToggleLikeV2"].clone();
        let thread: Thread = serde_json::from_value(data)?;
        Ok(thread)
    }

    /// Toggle like on a thread comment (requires authentication)
    pub async fn toggle_comment_like(&self, id: i32) -> Result<ThreadComment, AniListError> {
        let query = queries::forum::LIKE_THREAD_COMMENT;

        let mut variables = HashMap::new();
        variables.insert("id".to_string(), json!(id));
        variables.insert("type".to_string(), json!("THREAD_COMMENT"));

        let response = self.client.query(query, Some(variables)).await?;
        let data = response["data"]["ToggleLikeV2"].clone();
        let comment: ThreadComment = serde_json::from_value(data)?;
        Ok(comment)
    }
}
