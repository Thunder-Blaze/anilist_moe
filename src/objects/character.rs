use serde::{Deserialize, Serialize};

use crate::{
    enums::character::CharacterRole,
    objects::{
        common::{FuzzyDate, PageInfo},
        media::{Media, MediaConnection},
        staff::{Staff, StaffRoleType},
    },
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Character {
    pub id: i32,
    pub name: Option<CharacterName>,
    pub image: Option<CharacterImage>,
    pub description: Option<String>,
    pub gender: Option<String>,
    #[serde(rename = "dateOfBirth")]
    pub date_of_birth: Option<FuzzyDate>,
    pub age: Option<String>,
    #[serde(rename = "bloodType")]
    pub blood_type: Option<String>,
    #[serde(rename = "isFavourite")]
    pub is_favourite: Option<bool>,
    #[serde(rename = "isFavouriteBlocked")]
    pub is_favourite_blocked: Option<bool>,
    #[serde(rename = "siteUrl")]
    pub site_url: Option<String>,
    pub media: Option<MediaConnection>,
    pub favourites: Option<i32>,
    #[serde(rename = "modNotes")]
    pub mod_notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterConnection {
    pub edges: Option<Vec<CharacterEdge>>,
    pub nodes: Option<Vec<Character>>,
    #[serde(rename = "pageInfo")]
    pub page_info: Option<PageInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterEdge {
    pub node: Option<Character>,
    pub id: Option<i32>,
    pub role: Option<CharacterRole>,
    pub name: Option<String>,
    #[serde(rename = "voiceActors")]
    pub voice_actors: Option<Vec<Staff>>,
    #[serde(rename = "voiceActorRoles")]
    pub voice_actor_roles: Option<Vec<StaffRoleType>>,
    pub media: Option<Vec<Media>>,
    #[serde(rename = "favouriteOrder")]
    pub favourite_order: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterImage {
    pub large: Option<String>,
    pub medium: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterName {
    pub first: Option<String>,
    pub middle: Option<String>,
    pub last: Option<String>,
    pub full: Option<String>,
    pub native: Option<String>,
    pub alternative: Option<Vec<String>>,
    #[serde(rename = "alternativeSpoiler")]
    pub alternative_spoiler: Option<Vec<String>>,
    #[serde(rename = "userPreferred")]
    pub user_preferred: Option<String>,
}
