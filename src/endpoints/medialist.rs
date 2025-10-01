use crate::endpoints::Vth;
use crate::enums::media::MediaType;
use crate::enums::media_list::{MediaListSort, MediaListStatus};
use crate::objects::common::FuzzyDate;
use crate::{client::AniListClient, queries::medialist};
use crate::errors::AniListError;
use crate::objects::responses::MediaListResponse;
use serde::Serialize;
use serde_json::json;

#[derive(Default, Serialize)]
pub struct FetchMediaListOptions {
    #[serde(skip_serializing_if = "Option::is_none", rename = "userId")]
    pub user_id: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "userName")]
    pub user_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "type")]
    pub media_type: Option<MediaType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<MediaListStatus>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "mediaId")]
    pub media_id: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "isFollowing")]
    pub is_following: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "startedAt")]
    pub started_at: Option<FuzzyDate>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "completedAt")]
    pub completed_at: Option<FuzzyDate>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "compareWithAuthList")]
    pub compare_with_auth_list: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "userId_in")]
    pub user_id_in: Option<Vec<i32>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "status_in")]
    pub status_in: Option<Vec<MediaListStatus>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "status_not_in")]
    pub status_not_in: Option<Vec<MediaListStatus>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "status_not")]
    pub status_not: Option<MediaListStatus>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "mediaId_in")]
    pub media_id_in: Option<Vec<i32>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "mediaId_not_in")]
    pub media_id_not_in: Option<Vec<i32>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "notes_like")]
    pub notes_like: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "startedAt_greater")]
    pub started_at_greater: Option<FuzzyDate>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "startedAt_lesser")]
    pub started_at_lesser: Option<FuzzyDate>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "startedAt_like")]
    pub started_at_like: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "completedAt_greater")]
    pub completed_at_greater: Option<FuzzyDate>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "completedAt_lesser")]
    pub completed_at_lesser: Option<FuzzyDate>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "completedAt_like")]
    pub completed_at_like: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<Vec<MediaListSort>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "perPage")]
    pub per_page: Option<i32>,
}

#[derive(Default, Serialize)]
pub struct SaveMediaListOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "mediaId")]
    pub media_id: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<MediaListStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "scoreRaw")]
    pub score_raw: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub progress: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "progressVolumes")]
    pub progress_volumes: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repeat: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "hiddenFromStatusLists")]
    pub hidden_from_status_lists: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "customLists")]
    pub custom_lists: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "advancedScores")]
    pub advanced_scores: Option<Vec<f64>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "startedAt")]
    pub started_at: Option<FuzzyDate>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "completedAt")]
    pub completed_at: Option<FuzzyDate>,
}

#[derive(Default, Serialize)]
pub struct SaveMediaListMultipleOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<MediaListStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "scoreRaw")]
    pub score_raw: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub progress: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "progressVolumes")]
    pub progress_volumes: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repeat: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "hiddenFromStatusLists")]
    pub hidden_from_status_lists: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "advancedScores")]
    pub advanced_scores: Option<Vec<f64>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "startedAt")]
    pub started_at: Option<FuzzyDate>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "completedAt")]
    pub completed_at: Option<FuzzyDate>,
    #[serde(rename = "ids")]
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
    ) -> Result<MediaListResponse, AniListError> {
        let query = medialist::FETCH;
        let variables = json!(options);
        let variables_map = self.value_to_hashmap(variables);
        self.client.query_typed(query, Some(&variables_map)).await
    }

    pub async fn save(
        &self,
        options: SaveMediaListOptions,
    ) -> Result<MediaListResponse, AniListError> {
        let query = medialist::SAVE;
        let variables = json!(options);
        let variables_map = self.value_to_hashmap(variables);
        self.client.query_typed(query, Some(&variables_map)).await
    }

    pub async fn save_multiple(
        &self,
        options: SaveMediaListMultipleOptions,
    ) -> Result<Vec<MediaListResponse>, AniListError> {
        let query = medialist::SAVE_MULTIPLE;
        let variables = json!(options);
        let variables_map = self.value_to_hashmap(variables);
        self.client.query_typed(query, Some(&variables_map)).await
    }

    pub async fn delete(
        &self,
        options: DeleteMediaListOptions,
    ) -> Result<MediaListResponse, AniListError> {
        let query = medialist::DELETE;
        let variables = json!(options);
        let variables_map = self.value_to_hashmap(variables);
        self.client.query_typed(query, Some(&variables_map)).await
    }
}

impl Vth for MediaListEndpoint {}
