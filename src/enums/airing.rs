use crate::options::SortSpec;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[non_exhaustive]
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
