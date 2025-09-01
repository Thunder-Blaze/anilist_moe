use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MediaListSort {
    #[serde(rename = "MEDIA_ID")]
    MediaId,
    #[serde(rename = "MEDIA_ID_DESC")]
    MediaIdDesc,
    #[serde(rename = "SCORE")]
    Score,
    #[serde(rename = "SCORE_DESC")]
    ScoreDesc,
    #[serde(rename = "STATUS")]
    Status,
    #[serde(rename = "STATUS_DESC")]
    StatusDesc,
    #[serde(rename = "PROGRESS")]
    Progress,
    #[serde(rename = "PROGRESS_DESC")]
    ProgressDesc,
    #[serde(rename = "PROGRESS_VOLUMES")]
    ProgressVolumes,
    #[serde(rename = "PROGRESS_VOLUMES_DESC")]
    ProgressVolumesDesc,
    #[serde(rename = "REPEAT")]
    Repeat,
    #[serde(rename = "REPEAT_DESC")]
    RepeatDesc,
    #[serde(rename = "PRIORITY")]
    Priority,
    #[serde(rename = "PRIORITY_DESC")]
    PriorityDesc,
    #[serde(rename = "STARTED_ON")]
    StartedOn,
    #[serde(rename = "STARTED_ON_DESC")]
    StartedOnDesc,
    #[serde(rename = "FINISHED_ON")]
    FinishedOn,
    #[serde(rename = "FINISHED_ON_DESC")]
    FinishedOnDesc,
    #[serde(rename = "ADDED_TIME")]
    AddedTime,
    #[serde(rename = "ADDED_TIME_DESC")]
    AddedTimeDesc,
    #[serde(rename = "UPDATED_TIME")]
    UpdatedTime,
    #[serde(rename = "UPDATED_TIME_DESC")]
    UpdatedTimeDesc,
    #[serde(rename = "MEDIA_TITLE_ROMAJI")]
    MediaTitleRomaji,
    #[serde(rename = "MEDIA_TITLE_ROMAJI_DESC")]
    MediaTitleRomajiDesc,
    #[serde(rename = "MEDIA_TITLE_ENGLISH")]
    MediaTitleEnglish,
    #[serde(rename = "MEDIA_TITLE_ENGLISH_DESC")]
    MediaTitleEnglishDesc,
    #[serde(rename = "MEDIA_TITLE_NATIVE")]
    MediaTitleNative,
    #[serde(rename = "MEDIA_TITLE_NATIVE_DESC")]
    MediaTitleNativeDesc,
    #[serde(rename = "MEDIA_POPULARITY")]
    MediaPopularity,
    #[serde(rename = "MEDIA_POPULARITY_DESC")]
    MediaPopularityDesc,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MediaListStatus {
    #[serde(rename = "CURRENT")]
    Current,
    #[serde(rename = "PLANNING")]
    Planning,
    #[serde(rename = "COMPLETED")]
    Completed,
    #[serde(rename = "DROPPED")]
    Dropped,
    #[serde(rename = "PAUSED")]
    Paused,
    #[serde(rename = "REPEATING")]
    Repeating,
}