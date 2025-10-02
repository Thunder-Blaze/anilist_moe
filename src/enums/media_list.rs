use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MediaListSort {
    MediaId,
    MediaIdDesc,
    Score,
    ScoreDesc,
    Status,
    StatusDesc,
    Progress,
    ProgressDesc,
    ProgressVolumes,
    ProgressVolumesDesc,
    Repeat,
    RepeatDesc,
    Priority,
    PriorityDesc,
    StartedOn,
    StartedOnDesc,
    FinishedOn,
    FinishedOnDesc,
    AddedTime,
    AddedTimeDesc,
    UpdatedTime,
    UpdatedTimeDesc,
    MediaTitleRomaji,
    MediaTitleRomajiDesc,
    MediaTitleEnglish,
    MediaTitleEnglishDesc,
    MediaTitleNative,
    MediaTitleNativeDesc,
    MediaPopularity,
    MediaPopularityDesc,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MediaListStatus {
    Current,
    Planning,
    Completed,
    Dropped,
    Paused,
    Repeating,
}
