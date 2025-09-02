use serde::{Deserialize, Serialize};
use serde_json::Value as Json;

use crate::{
    enums::{media_list::MediaListStatus, score::ScoreFormat},
    objects::{common::FuzzyDate, media::Media, user::User},
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MediaList {
    pub id: i32,
    #[serde(rename = "userId")]
    pub user_id: i32,
    #[serde(rename = "mediaId")]
    pub media_id: i32,
    pub status: Option<MediaListStatus>,
    pub score: Option<f32>,
    pub progress: Option<i32>,
    #[serde(rename = "progressVolumes")]
    pub progress_volumes: Option<i32>,
    pub repeat: Option<i32>,
    pub priority: Option<i32>,
    pub private: Option<bool>,
    pub notes: Option<String>,
    #[serde(rename = "hiddenFromStatusLists")]
    pub hidden_from_status_lists: Option<bool>,
    #[serde(rename = "customLists")]
    pub custom_lists: Option<Json>,
    #[serde(rename = "asArray")]
    pub as_array: Option<bool>,
    #[serde(rename = "advancedScores")]
    pub advanced_scores: Option<Json>,
    #[serde(rename = "startedAt")]
    pub started_at: Option<FuzzyDate>,
    #[serde(rename = "completedAt")]
    pub completed_at: Option<FuzzyDate>,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<i32>,
    #[serde(rename = "createdAt")]
    pub created_at: Option<i32>,
    pub media: Option<Media>,
    pub user: Option<User>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MediaListCollection {
    pub lists: Option<Vec<MediaListGroup>>,
    pub user: Option<User>,
    #[serde(rename = "hasNextChunk")]
    pub has_next_chunk: Option<bool>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MediaListGroup {
    pub entries: Option<Vec<MediaList>>,
    pub name: Option<String>,
    #[serde(rename = "isCustomList")]
    pub is_custom_list: Option<bool>,
    #[serde(rename = "isSplitCompletedList")]
    pub is_split_completed_list: Option<bool>,
    pub status: Option<MediaListStatus>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MediaListOptions {
    #[serde(rename = "scoreFormat")]
    pub score_format: Option<ScoreFormat>,
    #[serde(rename = "rowOrder")]
    pub row_order: Option<String>,
    #[serde(rename = "animeList")]
    pub anime_list: Option<MediaListTypeOptions>,
    #[serde(rename = "mangaList")]
    pub manga_list: Option<MediaListTypeOptions>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MediaListTypeOptions {
    #[serde(rename = "sectionOrder")]
    pub section_order: Option<Vec<String>>,
    #[serde(rename = "splitCompletedSectionByFormat")]
    pub split_completed_section_by_format: Option<bool>,
    #[serde(rename = "customLists")]
    pub custom_lists: Option<Vec<String>>,
    #[serde(rename = "advancedScoring")]
    pub advanced_scoring: Option<Vec<String>>,
    #[serde(rename = "advancedScoringEnabled")]
    pub advanced_scoring_enabled: Option<bool>,
}
