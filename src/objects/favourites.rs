use serde::{Deserialize, Serialize};

use crate::objects::{
    character::CharacterConnection, media::MediaConnection, staff::StaffConnection,
    studio::StudioConnection,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Favourites {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anime: Option<Vec<MediaConnection>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manga: Option<Vec<MediaConnection>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub characters: Option<Vec<CharacterConnection>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub staff: Option<Vec<StaffConnection>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub studios: Option<Vec<StudioConnection>>,
}
