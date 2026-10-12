use crate::options::SortSpec;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum StudioSort {
    Id,
    IdDesc,
    Name,
    NameDesc,
    SearchMatch,
    Favourites,
    FavouritesDesc,
}

impl SortSpec for StudioSort {
    fn id_tiebreaker() -> Self {
        Self::Id
    }
}
