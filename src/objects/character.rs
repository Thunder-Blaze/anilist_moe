use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{
    enums::character::CharacterRole,
    objects::{
        common::{FuzzyDate, PageInfo},
        media::{Media, MediaConnection},
        staff::{Staff, StaffRoleType},
    },
};

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Character {
    pub id: i32,
    pub name: Option<CharacterName>,
    pub image: Option<CharacterImage>,
    pub description: Option<String>,
    pub gender: Option<String>,
    pub date_of_birth: Option<FuzzyDate>,
    pub age: Option<String>,
    pub blood_type: Option<String>,
    pub is_favourite: Option<bool>,
    pub is_favourite_blocked: Option<bool>,
    pub site_url: Option<String>,
    pub media: Option<MediaConnection>,
    pub favourites: Option<i32>,
    pub mod_notes: Option<String>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CharacterConnection {
    pub edges: Option<Vec<CharacterEdge>>,
    pub nodes: Option<Vec<Character>>,
    pub page_info: Option<PageInfo>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CharacterEdge {
    pub node: Option<Character>,
    pub id: Option<i32>,
    pub role: Option<CharacterRole>,
    pub name: Option<String>,
    pub voice_actors: Option<Vec<Staff>>,
    pub voice_actor_roles: Option<Vec<StaffRoleType>>,
    pub media: Option<Vec<Media>>,
    pub favourite_order: Option<i32>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CharacterImage {
    pub large: Option<String>,
    pub medium: Option<String>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CharacterName {
    pub first: Option<String>,
    pub middle: Option<String>,
    pub last: Option<String>,
    pub full: Option<String>,
    pub native: Option<String>,
    pub alternative: Option<Vec<String>>,
    pub alternative_spoiler: Option<Vec<String>>,
    pub user_preferred: Option<String>,
}
