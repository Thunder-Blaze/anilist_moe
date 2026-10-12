use crate::options::SortSpec;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
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

impl SortSpec for AiringSort {
    fn id_tiebreaker() -> Self {
        Self::Id
    }
}
