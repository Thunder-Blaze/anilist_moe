use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AiringSort {
    #[serde(rename = "ID")]
    Id,
    #[serde(rename = "ID_DESC")]
    IdDesc,
    #[serde(rename = "MEDIA_ID")]
    MediaId,
    #[serde(rename = "MEDIA_ID_DESC")]
    MediaIdDesc,
    #[serde(rename = "TIME")]
    Time,
    #[serde(rename = "TIME_DESC")]
    TimeDesc,
    #[serde(rename = "EPISODE")]
    Episode,
    #[serde(rename = "EPISODE_DESC")]
    EpisodeDesc,
}
