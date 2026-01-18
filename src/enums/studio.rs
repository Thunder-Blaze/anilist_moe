use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
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
