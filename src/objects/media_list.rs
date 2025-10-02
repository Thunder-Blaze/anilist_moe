use serde::{Deserialize, Serialize};
use serde_json::Value as Json;

use crate::{
    enums::{media_list::MediaListStatus, score::ScoreFormat},
    objects::{common::FuzzyDate, media::Media, user::User},
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MediaList {
    pub id: i32,
    #[serde(rename = "userId", skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i32>,
    #[serde(rename = "mediaId")]
    pub media_id: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<MediaListStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub progress: Option<i32>,
    #[serde(rename = "progressVolumes", skip_serializing_if = "Option::is_none")]
    pub progress_volumes: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repeat: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    #[serde(rename = "hiddenFromStatusLists", skip_serializing_if = "Option::is_none")]
    pub hidden_from_status_lists: Option<bool>,
    #[serde(rename = "customLists", skip_serializing_if = "Option::is_none")]
    pub custom_lists: Option<Json>,
    #[serde(rename = "asArray", skip_serializing_if = "Option::is_none")]
    pub as_array: Option<bool>,
    #[serde(rename = "advancedScores", skip_serializing_if = "Option::is_none")]
    pub advanced_scores: Option<Json>,
    #[serde(rename = "startedAt", skip_serializing_if = "Option::is_none")]
    pub started_at: Option<FuzzyDate>,
    #[serde(rename = "completedAt", skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<FuzzyDate>,
    #[serde(rename = "updatedAt", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<i32>,
    #[serde(rename = "createdAt", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media: Option<Media>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<User>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MediaListCollection {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lists: Option<Vec<MediaListGroup>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<User>,
    #[serde(rename = "hasNextChunk", skip_serializing_if = "Option::is_none")]
    pub has_next_chunk: Option<bool>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MediaListGroup {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entries: Option<Vec<MediaList>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "isCustomList", skip_serializing_if = "Option::is_none")]
    pub is_custom_list: Option<bool>,
    #[serde(rename = "isSplitCompletedList", skip_serializing_if = "Option::is_none")]
    pub is_split_completed_list: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<MediaListStatus>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MediaListOptions {
    #[serde(rename = "scoreFormat", skip_serializing_if = "Option::is_none")]
    pub score_format: Option<ScoreFormat>,
    #[serde(rename = "rowOrder", skip_serializing_if = "Option::is_none")]
    pub row_order: Option<String>,
    #[serde(rename = "animeList", skip_serializing_if = "Option::is_none")]
    pub anime_list: Option<MediaListTypeOptions>,
    #[serde(rename = "mangaList", skip_serializing_if = "Option::is_none")]
    pub manga_list: Option<MediaListTypeOptions>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MediaListTypeOptions {
    #[serde(rename = "sectionOrder", skip_serializing_if = "Option::is_none")]
    pub section_order: Option<Vec<String>>,
    #[serde(rename = "splitCompletedSectionByFormat", skip_serializing_if = "Option::is_none")]
    pub split_completed_section_by_format: Option<bool>,
    #[serde(rename = "customLists", skip_serializing_if = "Option::is_none")]
    pub custom_lists: Option<Vec<String>>,
    #[serde(rename = "advancedScoring", skip_serializing_if = "Option::is_none")]
    pub advanced_scoring: Option<Vec<String>>,
    #[serde(rename = "advancedScoringEnabled", skip_serializing_if = "Option::is_none")]
    pub advanced_scoring_enabled: Option<bool>,
}
