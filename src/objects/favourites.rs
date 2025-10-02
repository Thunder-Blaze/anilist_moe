use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::objects::{
    character::CharacterConnection, media::MediaConnection, staff::StaffConnection,
    studio::StudioConnection,
};

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Favourites {
    pub anime: Option<MediaConnection>,
    pub manga: Option<MediaConnection>,
    pub characters: Option<CharacterConnection>,
    pub staff: Option<StaffConnection>,
    pub studios: Option<StudioConnection>,
}
