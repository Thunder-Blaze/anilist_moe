use serde::{Deserialize, Serialize};
use serde_json::Value as Json;
use serde_with::skip_serializing_none;

use crate::{
    enums::{media_list::MediaListStatus, score::ScoreFormat},
    objects::{common::FuzzyDate, media::Media, user::User},
};

#[skip_serializing_none]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MediaList {
    pub id: i32,
    pub user_id: Option<i32>,
    pub media_id: Option<i32>,
    pub status: Option<MediaListStatus>,
    pub score: Option<f32>,
    pub progress: Option<i32>,
    pub progress_volumes: Option<i32>,
    pub repeat: Option<i32>,
    pub priority: Option<i32>,
    pub private: Option<bool>,
    pub notes: Option<String>,
    pub hidden_from_status_lists: Option<bool>,
    pub custom_lists: Option<Json>,
    pub as_array: Option<bool>,
    pub advanced_scores: Option<Json>,
    pub started_at: Option<FuzzyDate>,
    pub completed_at: Option<FuzzyDate>,
    pub updated_at: Option<i32>,
    pub created_at: Option<i32>,
    pub media: Option<Media>,
    pub user: Option<User>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MediaListCollection {
    pub lists: Option<Vec<MediaListGroup>>,
    pub user: Option<User>,
    pub has_next_chunk: Option<bool>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MediaListGroup {
    pub entries: Option<Vec<MediaList>>,
    pub name: Option<String>,
    pub is_custom_list: Option<bool>,
    pub is_split_completed_list: Option<bool>,
    pub status: Option<MediaListStatus>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MediaListOptions {
    pub score_format: Option<ScoreFormat>,
    pub row_order: Option<String>,
    pub anime_list: Option<MediaListTypeOptions>,
    pub manga_list: Option<MediaListTypeOptions>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MediaListTypeOptions {
    pub section_order: Option<Vec<String>>,
    pub split_completed_section_by_format: Option<bool>,
    pub custom_lists: Option<Vec<String>>,
    pub advanced_scoring: Option<Vec<String>>,
    pub advanced_scoring_enabled: Option<bool>,
}
