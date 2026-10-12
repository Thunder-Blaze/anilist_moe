use crate::options::SortSpec;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[non_exhaustive]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[non_exhaustive]
pub enum MediaListStatus {
    Current,
    Planning,
    Completed,
    Dropped,
    Paused,
    Repeating,
    /// A value the API returned that this version of the crate does not know yet.
    ///
    /// Only ever produced by deserialization; never send it as input.
    #[serde(other)]
    Unknown,
}

impl SortSpec for MediaListSort {
    fn id_tiebreaker() -> Self {
        Self::MediaId
    }
}
