use crate::client::AniListClient;
use crate::error::AniListError;
use crate::models::social::AiringSchedule;
use serde_json::json;
use std::collections::HashMap;

pub struct AiringEndpoint {
    client: AniListClient,
}

impl AiringEndpoint {
    pub(crate) fn new(client: AniListClient) -> Self {
        Self { client }
    }

    /// Get upcoming airing episodes
    pub async fn get_upcoming_episodes(
        &self,
        page: i32,
        per_page: i32,
    ) -> Result<Vec<AiringSchedule>, AniListError> {
        let query = r#"
            query ($page: Int, $perPage: Int, $airingAtGreater: Int, $sort: [AiringSort]) {
                Page(page: $page, perPage: $perPage) {
                    airingSchedules(airingAt_greater: $airingAtGreater, sort: $sort) {
                        id
                        airingAt
                        timeUntilAiring
                        episode
                        mediaId
                        media {
                            id
                            title {
                                romaji
                                english
                                native
                                userPreferred
                            }
                            coverImage {
                                extraLarge
                                large
                                medium
                                color
                            }
                            bannerImage
                            episodes
                            format
                            siteUrl
                        }
                    }
                }
            }
        "#;

        let current_timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;

        let mut variables = HashMap::new();
        variables.insert("page".to_string(), json!(page));
        variables.insert("perPage".to_string(), json!(per_page));
        variables.insert("airingAtGreater".to_string(), json!(current_timestamp));
        variables.insert("sort".to_string(), json!(["TIME"]));

        let response = self.client.query(query, Some(variables)).await?;
        let data = response["data"]["Page"]["airingSchedules"].clone();
        let schedules: Vec<AiringSchedule> = serde_json::from_value(data)?;
        Ok(schedules)
    }

    /// Get airing episodes for today
    pub async fn get_today_episodes(
        &self,
        page: i32,
        per_page: i32,
    ) -> Result<Vec<AiringSchedule>, AniListError> {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;

        let start_of_day = now - (now % 86400); // Beginning of today
        let end_of_day = start_of_day + 86400; // End of today

        let query = r#"
            query ($page: Int, $perPage: Int, $airingAtGreater: Int, $airingAtLesser: Int, $sort: [AiringSort]) {
                Page(page: $page, perPage: $perPage) {
                    airingSchedules(airingAt_greater: $airingAtGreater, airingAt_lesser: $airingAtLesser, sort: $sort) {
                        id
                        airingAt
                        timeUntilAiring
                        episode
                        mediaId
                        media {
                            id
                            title {
                                romaji
                                english
                                native
                                userPreferred
                            }
                            coverImage {
                                extraLarge
                                large
                                medium
                                color
                            }
                            bannerImage
                            episodes
                            format
                            siteUrl
                        }
                    }
                }
            }
        "#;

        let mut variables = HashMap::new();
        variables.insert("page".to_string(), json!(page));
        variables.insert("perPage".to_string(), json!(per_page));
        variables.insert("airingAtGreater".to_string(), json!(start_of_day));
        variables.insert("airingAtLesser".to_string(), json!(end_of_day));
        variables.insert("sort".to_string(), json!(["TIME"]));

        let response = self.client.query(query, Some(variables)).await?;
        let data = response["data"]["Page"]["airingSchedules"].clone();
        let schedules: Vec<AiringSchedule> = serde_json::from_value(data)?;
        Ok(schedules)
    }

    /// Get recently aired episodes
    pub async fn get_recently_aired(
        &self,
        page: i32,
        per_page: i32,
    ) -> Result<Vec<AiringSchedule>, AniListError> {
        let current_timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;

        let query = r#"
            query ($page: Int, $perPage: Int, $airingAtLesser: Int, $sort: [AiringSort]) {
                Page(page: $page, perPage: $perPage) {
                    airingSchedules(airingAt_lesser: $airingAtLesser, sort: $sort) {
                        id
                        airingAt
                        timeUntilAiring
                        episode
                        mediaId
                        media {
                            id
                            title {
                                romaji
                                english
                                native
                                userPreferred
                            }
                            coverImage {
                                extraLarge
                                large
                                medium
                                color
                            }
                            bannerImage
                            episodes
                            format
                            siteUrl
                        }
                    }
                }
            }
        "#;

        let mut variables = HashMap::new();
        variables.insert("page".to_string(), json!(page));
        variables.insert("perPage".to_string(), json!(per_page));
        variables.insert("airingAtLesser".to_string(), json!(current_timestamp));
        variables.insert("sort".to_string(), json!(["TIME_DESC"]));

        let response = self.client.query(query, Some(variables)).await?;
        let data = response["data"]["Page"]["airingSchedules"].clone();
        let schedules: Vec<AiringSchedule> = serde_json::from_value(data)?;
        Ok(schedules)
    }

    /// Get airing schedule for a specific media
    pub async fn get_schedule_for_media(
        &self,
        media_id: i32,
        page: i32,
        per_page: i32,
    ) -> Result<Vec<AiringSchedule>, AniListError> {
        let query = r#"
            query ($mediaId: Int, $page: Int, $perPage: Int, $sort: [AiringSort]) {
                Page(page: $page, perPage: $perPage) {
                    airingSchedules(mediaId: $mediaId, sort: $sort) {
                        id
                        airingAt
                        timeUntilAiring
                        episode
                        mediaId
                        media {
                            id
                            title {
                                romaji
                                english
                                native
                                userPreferred
                            }
                            coverImage {
                                extraLarge
                                large
                                medium
                                color
                            }
                            bannerImage
                            episodes
                            format
                            siteUrl
                        }
                    }
                }
            }
        "#;

        let mut variables = HashMap::new();
        variables.insert("mediaId".to_string(), json!(media_id));
        variables.insert("page".to_string(), json!(page));
        variables.insert("perPage".to_string(), json!(per_page));
        variables.insert("sort".to_string(), json!(["TIME"]));

        let response = self.client.query(query, Some(variables)).await?;
        let data = response["data"]["Page"]["airingSchedules"].clone();
        let schedules: Vec<AiringSchedule> = serde_json::from_value(data)?;
        Ok(schedules)
    }

    /// Get airing schedule by ID
    pub async fn get_schedule_by_id(&self, id: i32) -> Result<AiringSchedule, AniListError> {
        let query = r#"
            query ($id: Int) {
                AiringSchedule(id: $id) {
                    id
                    airingAt
                    timeUntilAiring
                    episode
                    mediaId
                    media {
                        id
                        title {
                            romaji
                            english
                            native
                            userPreferred
                        }
                        coverImage {
                            extraLarge
                            large
                            medium
                            color
                        }
                        bannerImage
                        episodes
                        format
                        siteUrl
                    }
                }
            }
        "#;

        let mut variables = HashMap::new();
        variables.insert("id".to_string(), json!(id));

        let response = self.client.query(query, Some(variables)).await?;
        let data = response["data"]["AiringSchedule"].clone();
        let schedule: AiringSchedule = serde_json::from_value(data)?;
        Ok(schedule)
    }

    /// Get airing episodes for a specific day range
    pub async fn get_episodes_in_range(
        &self,
        start_timestamp: i64,
        end_timestamp: i64,
        page: i32,
        per_page: i32,
    ) -> Result<Vec<AiringSchedule>, AniListError> {
        let query = r#"
            query ($page: Int, $perPage: Int, $airingAtGreater: Int, $airingAtLesser: Int, $sort: [AiringSort]) {
                Page(page: $page, perPage: $perPage) {
                    airingSchedules(airingAt_greater: $airingAtGreater, airingAt_lesser: $airingAtLesser, sort: $sort) {
                        id
                        airingAt
                        timeUntilAiring
                        episode
                        mediaId
                        media {
                            id
                            title {
                                romaji
                                english
                                native
                                userPreferred
                            }
                            coverImage {
                                extraLarge
                                large
                                medium
                                color
                            }
                            bannerImage
                            episodes
                            format
                            siteUrl
                        }
                    }
                }
            }
        "#;

        let mut variables = HashMap::new();
        variables.insert("page".to_string(), json!(page));
        variables.insert("perPage".to_string(), json!(per_page));
        variables.insert("airingAtGreater".to_string(), json!(start_timestamp));
        variables.insert("airingAtLesser".to_string(), json!(end_timestamp));
        variables.insert("sort".to_string(), json!(["TIME"]));

        let response = self.client.query(query, Some(variables)).await?;
        let data = response["data"]["Page"]["airingSchedules"].clone();
        let schedules: Vec<AiringSchedule> = serde_json::from_value(data)?;
        Ok(schedules)
    }

    /// Get next episode for specific anime (helper method)
    pub async fn get_next_episode(
        &self,
        media_id: i32,
    ) -> Result<Option<AiringSchedule>, AniListError> {
        let current_timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;

        let query = r#"
            query ($mediaId: Int, $airingAtGreater: Int) {
                Page(page: 1, perPage: 1) {
                    airingSchedules(mediaId: $mediaId, airingAt_greater: $airingAtGreater, sort: TIME) {
                        id
                        airingAt
                        timeUntilAiring
                        episode
                        mediaId
                        media {
                            id
                            title {
                                userPreferred
                            }
                        }
                    }
                }
            }
        "#;

        let mut variables = HashMap::new();
        variables.insert("mediaId".to_string(), json!(media_id));
        variables.insert("airingAtGreater".to_string(), json!(current_timestamp));

        let response = self.client.query(query, Some(variables)).await?;
        let schedules_array = response["data"]["Page"]["airingSchedules"].as_array();

        if let Some(schedules) = schedules_array
            && !schedules.is_empty()
        {
            let schedule: AiringSchedule = serde_json::from_value(schedules[0].clone())?;
            return Ok(Some(schedule));
        }

        Ok(None)
    }
}
