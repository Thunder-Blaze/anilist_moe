use crate::options::SortSpec;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[non_exhaustive]
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
