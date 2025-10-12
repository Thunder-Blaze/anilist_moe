use crate::enums::media::MediaType;
use crate::enums::media_list::{MediaListSort, MediaListStatus};
use crate::errors::AniListError;
use crate::objects::common::{Deleted, FuzzyDate};
use crate::objects::media_list::MediaList;
use crate::objects::responses::{
    GraphQLResponse, Page
};
use crate::{client::AniListClient, queries::medialist};
use serde::Serialize;
use serde_json::json;
use serde_with::skip_serializing_none;

/// Options for fetching media list entries.
#[skip_serializing_none]
#[derive(Default, Debug, Serialize)]
pub struct FetchMediaListOptions {
    #[serde(rename = "userId")]
    pub user_id: Option<i32>,
    #[serde(rename = "userName")]
    pub user_name: Option<String>,
    #[serde(rename = "type")]
    pub media_type: Option<MediaType>,
    pub status: Option<MediaListStatus>,
    #[serde(rename = "mediaId")]
    pub media_id: Option<i32>,
    #[serde(rename = "isFollowing")]
    pub is_following: Option<bool>,
    pub notes: Option<String>,
    #[serde(rename = "startedAt")]
    pub started_at: Option<FuzzyDate>,
    #[serde(rename = "completedAt")]
    pub completed_at: Option<FuzzyDate>,
    #[serde(rename = "compareWithAuthList")]
    pub compare_with_auth_list: Option<bool>,
    #[serde(rename = "userId_in")]
    pub user_id_in: Option<Vec<i32>>,
    #[serde(rename = "status_in")]
    pub status_in: Option<Vec<MediaListStatus>>,
    #[serde(rename = "status_not_in")]
    pub status_not_in: Option<Vec<MediaListStatus>>,
    #[serde(rename = "status_not")]
    pub status_not: Option<MediaListStatus>,
    #[serde(rename = "mediaId_in")]
    pub media_id_in: Option<Vec<i32>>,
    #[serde(rename = "mediaId_not_in")]
    pub media_id_not_in: Option<Vec<i32>>,
    #[serde(rename = "notes_like")]
    pub notes_like: Option<String>,
    #[serde(rename = "startedAt_greater")]
    pub started_at_greater: Option<FuzzyDate>,
    #[serde(rename = "startedAt_lesser")]
    pub started_at_lesser: Option<FuzzyDate>,
    #[serde(rename = "startedAt_like")]
    pub started_at_like: Option<String>,
    #[serde(rename = "completedAt_greater")]
    pub completed_at_greater: Option<FuzzyDate>,
    #[serde(rename = "completedAt_lesser")]
    pub completed_at_lesser: Option<FuzzyDate>,
    #[serde(rename = "completedAt_like")]
    pub completed_at_like: Option<String>,
    pub sort: Option<Vec<MediaListSort>>,
    pub page: Option<i32>,
    #[serde(rename = "perPage")]
    pub per_page: Option<i32>,
}

/// Options for creating or updating a media list entry.
#[skip_serializing_none]
#[derive(Default, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SaveMediaListOptions {
    pub id: Option<i32>,
    pub media_id: Option<i32>,
    pub status: Option<MediaListStatus>,
    pub score: Option<f64>,
    pub score_raw: Option<i32>,
    pub progress: Option<i32>,
    pub progress_volumes: Option<i32>,
    pub repeat: Option<i32>,
    pub priority: Option<i32>,
    pub private: Option<bool>,
    pub notes: Option<String>,
    pub hidden_from_status_lists: Option<bool>,
    pub custom_lists: Option<Vec<String>>,
    pub advanced_scores: Option<Vec<f64>>,
    pub started_at: Option<FuzzyDate>,
    pub completed_at: Option<FuzzyDate>,
}

/// Options for bulk updating multiple media list entries.
#[skip_serializing_none]
#[derive(Default, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SaveMediaListMultipleOptions {
    pub status: Option<MediaListStatus>,
    pub score: Option<f64>,
    pub score_raw: Option<i32>,
    pub progress: Option<i32>,
    pub progress_volumes: Option<i32>,
    pub repeat: Option<i32>,
    pub priority: Option<i32>,
    pub private: Option<bool>,
    pub notes: Option<String>,
    pub hidden_from_status_lists: Option<bool>,
    pub advanced_scores: Option<Vec<f64>>,
    pub started_at: Option<FuzzyDate>,
    pub completed_at: Option<FuzzyDate>,
    pub ids: Vec<i32>,
}

/// Options for deleting a media list entry.
#[derive(Default, Debug, Serialize)]
pub struct DeleteMediaListOptions {
    pub id: i32,
}

/// Endpoint for media list operations.
pub struct MediaListEndpoint {
    client: AniListClient,
}

impl MediaListEndpoint {
    pub fn new(client: AniListClient) -> Self {
        Self { client }
    }

    pub async fn fetch(
        &self,
        options: &FetchMediaListOptions,
    ) -> Result<Page<Vec<MediaList>>, AniListError> {
        let query = medialist::FETCH;
        let variables = json!(options);
        let variables_map = crate::utils::json_to_hashmap(variables);
        let response: Result<GraphQLResponse<Page<Vec<MediaList>>>, AniListError> = self.client.query_typed(query, Some(&variables_map)).await;
        match response {
            Ok(res) => Ok(res.data),
            Err(err) => Err(err),
        }
    }

    pub async fn save(
        &self,
        options: &SaveMediaListOptions,
    ) -> Result<MediaList, AniListError> {
        let query = medialist::SAVE;
        let variables = json!(options);
        let variables_map = crate::utils::json_to_hashmap(variables);
        let response: Result<GraphQLResponse<MediaList>, AniListError> = self.client.query_typed(query, Some(&variables_map)).await;
        match response {
            Ok(res) => Ok(res.data),
            Err(err) => Err(err),
        }
    }

    pub async fn save_multiple(
        &self,
        options: &SaveMediaListMultipleOptions,
    ) -> Result<Vec<MediaList>, AniListError> {
        let query = medialist::SAVE_MULTIPLE;
        let variables = json!(options);
        let variables_map = crate::utils::json_to_hashmap(variables);
        let response: Result<GraphQLResponse<Vec<MediaList>>, AniListError> = self.client.query_typed(query, Some(&variables_map)).await;
        match response {
            Ok(res) => Ok(res.data),
            Err(err) => Err(err),
        }
    }

    pub async fn delete(
        &self,
        options: &DeleteMediaListOptions,
    ) -> Result<bool, AniListError> {
        let query = medialist::DELETE;
        let variables = json!(options);
        let variables_map = crate::utils::json_to_hashmap(variables);
        let response: Result<GraphQLResponse<Deleted>, AniListError> = self.client.query_typed(query, Some(&variables_map)).await;
        match response {
            Ok(res) => Ok(res.data.deleted.unwrap_or_default()),
            Err(err) => Err(err),
        }
    }

    // Convenience functions

    /// Get user's anime list by username
    pub async fn get_user_anime_list(
        &self,
        username: &str,
        status: Option<MediaListStatus>,
        page: Option<i32>,
        per_page: Option<i32>,
    ) -> Result<Page<Vec<MediaList>>, AniListError> {
        self.fetch(&FetchMediaListOptions {
            user_name: Some(username.to_string()),
            media_type: Some(MediaType::Anime),
            status,
            sort: Some(vec![MediaListSort::UpdatedTimeDesc]),
            page,
            per_page,
            ..Default::default()
        })
        .await
    }

    /// Get user's manga list by username
    pub async fn get_user_manga_list(
        &self,
        username: &str,
        status: Option<MediaListStatus>,
        page: Option<i32>,
        per_page: Option<i32>,
    ) -> Result<Page<Vec<MediaList>>, AniListError> {
        self.fetch(&FetchMediaListOptions {
            user_name: Some(username.to_string()),
            media_type: Some(MediaType::Manga),
            status,
            sort: Some(vec![MediaListSort::UpdatedTimeDesc]),
            page,
            per_page,
            ..Default::default()
        })
        .await
    }

    /// Get current user's anime list (requires authentication)
    pub async fn get_my_anime_list(
        &self,
        status: Option<MediaListStatus>,
        page: Option<i32>,
        per_page: Option<i32>,
    ) -> Result<Page<Vec<MediaList>>, AniListError> {
        self.fetch(&FetchMediaListOptions {
            media_type: Some(MediaType::Anime),
            status,
            sort: Some(vec![MediaListSort::UpdatedTimeDesc]),
            page,
            per_page,
            ..Default::default()
        })
        .await
    }

    /// Get current user's manga list (requires authentication)
    pub async fn get_my_manga_list(
        &self,
        status: Option<MediaListStatus>,
        page: Option<i32>,
        per_page: Option<i32>,
    ) -> Result<Page<Vec<MediaList>>, AniListError> {
        self.fetch(&FetchMediaListOptions {
            media_type: Some(MediaType::Manga),
            status,
            sort: Some(vec![MediaListSort::UpdatedTimeDesc]),
            page,
            per_page,
            ..Default::default()
        })
        .await
    }

    /// Get user's currently watching anime
    pub async fn get_watching(
        &self,
        username: Option<&str>,
        page: Option<i32>,
        per_page: Option<i32>,
    ) -> Result<Page<Vec<MediaList>>, AniListError> {
        self.fetch(&FetchMediaListOptions {
            user_name: username.map(|s| s.to_string()),
            media_type: Some(MediaType::Anime),
            status: Some(MediaListStatus::Current),
            sort: Some(vec![MediaListSort::UpdatedTimeDesc]),
            page,
            per_page,
            ..Default::default()
        })
        .await
    }

    /// Get user's currently reading manga
    pub async fn get_reading(
        &self,
        username: Option<&str>,
        page: Option<i32>,
        per_page: Option<i32>,
    ) -> Result<Page<Vec<MediaList>>, AniListError> {
        self.fetch(&FetchMediaListOptions {
            user_name: username.map(|s| s.to_string()),
            media_type: Some(MediaType::Manga),
            status: Some(MediaListStatus::Current),
            sort: Some(vec![MediaListSort::UpdatedTimeDesc]),
            page,
            per_page,
            ..Default::default()
        })
        .await
    }

    /// Get user's completed anime
    pub async fn get_completed_anime(
        &self,
        username: Option<&str>,
        page: Option<i32>,
        per_page: Option<i32>,
    ) -> Result<Page<Vec<MediaList>>, AniListError> {
        self.fetch(&FetchMediaListOptions {
            user_name: username.map(|s| s.to_string()),
            media_type: Some(MediaType::Anime),
            status: Some(MediaListStatus::Completed),
            sort: Some(vec![MediaListSort::ScoreDesc]),
            page,
            per_page,
            ..Default::default()
        })
        .await
    }

    /// Get user's completed manga
    pub async fn get_completed_manga(
        &self,
        username: Option<&str>,
        page: Option<i32>,
        per_page: Option<i32>,
    ) -> Result<Page<Vec<MediaList>>, AniListError> {
        self.fetch(&FetchMediaListOptions {
            user_name: username.map(|s| s.to_string()),
            media_type: Some(MediaType::Manga),
            status: Some(MediaListStatus::Completed),
            sort: Some(vec![MediaListSort::ScoreDesc]),
            page,
            per_page,
            ..Default::default()
        })
        .await
    }

    /// Get user's plan to watch list
    pub async fn get_plan_to_watch(
        &self,
        username: Option<&str>,
        page: Option<i32>,
        per_page: Option<i32>,
    ) -> Result<Page<Vec<MediaList>>, AniListError> {
        self.fetch(&FetchMediaListOptions {
            user_name: username.map(|s| s.to_string()),
            media_type: Some(MediaType::Anime),
            status: Some(MediaListStatus::Planning),
            sort: Some(vec![MediaListSort::UpdatedTimeDesc]),
            page,
            per_page,
            ..Default::default()
        })
        .await
    }

    /// Get user's plan to read list
    pub async fn get_plan_to_read(
        &self,
        username: Option<&str>,
        page: Option<i32>,
        per_page: Option<i32>,
    ) -> Result<Page<Vec<MediaList>>, AniListError> {
        self.fetch(&FetchMediaListOptions {
            user_name: username.map(|s| s.to_string()),
            media_type: Some(MediaType::Manga),
            status: Some(MediaListStatus::Planning),
            sort: Some(vec![MediaListSort::UpdatedTimeDesc]),
            page,
            per_page,
            ..Default::default()
        })
        .await
    }

    /// Add anime to list
    pub async fn add_anime(
        &self,
        media_id: i32,
        status: MediaListStatus,
    ) -> Result<MediaList, AniListError> {
        self.save(&SaveMediaListOptions {
            media_id: Some(media_id),
            status: Some(status),
            ..Default::default()
        })
        .await
    }

    /// Add manga to list
    pub async fn add_manga(
        &self,
        media_id: i32,
        status: MediaListStatus,
    ) -> Result<MediaList, AniListError> {
        self.save(&SaveMediaListOptions {
            media_id: Some(media_id),
            status: Some(status),
            ..Default::default()
        })
        .await
    }

    /// Update progress for an entry
    pub async fn update_progress(
        &self,
        entry_id: i32,
        progress: i32,
    ) -> Result<MediaList, AniListError> {
        self.save(&SaveMediaListOptions {
            id: Some(entry_id),
            progress: Some(progress),
            ..Default::default()
        })
        .await
    }

    /// Update score for an entry
    pub async fn update_score(
        &self,
        entry_id: i32,
        score: f64,
    ) -> Result<MediaList, AniListError> {
        self.save(&SaveMediaListOptions {
            id: Some(entry_id),
            score: Some(score),
            ..Default::default()
        })
        .await
    }

    /// Update status for an entry
    pub async fn update_status(
        &self,
        entry_id: i32,
        status: MediaListStatus,
    ) -> Result<MediaList, AniListError> {
        self.save(&SaveMediaListOptions {
            id: Some(entry_id),
            status: Some(status),
            ..Default::default()
        })
        .await
    }

    /// Delete a media list entry
    pub async fn delete_entry(
        &self,
        entry_id: i32,
    ) -> Result<bool, AniListError> {
        self.delete(&DeleteMediaListOptions { id: entry_id }).await
    }
}
