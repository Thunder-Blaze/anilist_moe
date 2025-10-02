use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AiringSort {
    Id,
    IdDesc,
    MediaId,
    MediaIdDesc,
    Time,
    TimeDesc,
    Episode,
    EpisodeDesc,
}
