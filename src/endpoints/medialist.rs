use crate::enums::media::MediaType;
use crate::enums::media_list::{MediaListSort, MediaListStatus};
use crate::errors::AniListError;
use crate::objects::common::FuzzyDate;
use crate::objects::responses::{
    DeleteMediaListEntryResponse, SaveMediaListEntryResponse, UserMediaListResponse,
};
use crate::{client::AniListClient, queries::medialist};
use serde::Serialize;
use serde_json::json;
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Default, Serialize)]
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

#[skip_serializing_none]
#[derive(Default, Serialize)]
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

#[skip_serializing_none]
#[derive(Default, Serialize)]
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

#[derive(Default, Serialize)]
pub struct DeleteMediaListOptions {
    pub id: i32,
}

pub struct MediaListEndpoint {
    client: AniListClient,
}

impl MediaListEndpoint {
    pub fn new(client: AniListClient) -> Self {
        Self { client }
    }

    pub async fn fetch(
        &self,
        options: FetchMediaListOptions,
    ) -> Result<UserMediaListResponse, AniListError> {
        let query = medialist::FETCH;
        let variables = serde_json::to_value(options)?;
        let variables_map = crate::utils::json_to_hashmap(variables);
        self.client.query_typed(query, Some(&variables_map)).await
    }

    pub async fn save(
        &self,
        options: SaveMediaListOptions,
    ) -> Result<SaveMediaListEntryResponse, AniListError> {
        let query = medialist::SAVE;
        let variables = json!(options);
        let variables_map = crate::utils::json_to_hashmap(variables);
        self.client.query_typed(query, Some(&variables_map)).await
    }

    pub async fn save_multiple(
        &self,
        options: SaveMediaListMultipleOptions,
    ) -> Result<Vec<SaveMediaListEntryResponse>, AniListError> {
        let query = medialist::SAVE_MULTIPLE;
        let variables = json!(options);
        let variables_map = crate::utils::json_to_hashmap(variables);
        self.client.query_typed(query, Some(&variables_map)).await
    }

    pub async fn delete(
        &self,
        options: DeleteMediaListOptions,
    ) -> Result<DeleteMediaListEntryResponse, AniListError> {
        let query = medialist::DELETE;
        let variables = json!(options);
        let variables_map = crate::utils::json_to_hashmap(variables);
        self.client.query_typed(query, Some(&variables_map)).await
    }
}
