use serde::{Deserialize, Serialize};

use crate::objects::{
    character::CharacterConnection, media::MediaConnection, staff::StaffConnection,
    studio::StudioConnection,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Favourites {
    pub anime: Option<Vec<MediaConnection>>,
    pub manga: Option<Vec<MediaConnection>>,
    pub characters: Option<Vec<CharacterConnection>>,
    pub staff: Option<Vec<StaffConnection>>,
    pub studios: Option<Vec<StudioConnection>>,
}
