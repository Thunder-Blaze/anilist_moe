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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<CharacterName>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<CharacterImage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gender: Option<String>,
    #[serde(rename = "dateOfBirth", skip_serializing_if = "Option::is_none")]
    pub date_of_birth: Option<FuzzyDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub age: Option<String>,
    #[serde(rename = "bloodType", skip_serializing_if = "Option::is_none")]
    pub blood_type: Option<String>,
    #[serde(rename = "isFavourite", skip_serializing_if = "Option::is_none")]
    pub is_favourite: Option<bool>,
    #[serde(rename = "isFavouriteBlocked", skip_serializing_if = "Option::is_none")]
    pub is_favourite_blocked: Option<bool>,
    #[serde(rename = "siteUrl", skip_serializing_if = "Option::is_none")]
    pub site_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media: Option<MediaConnection>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub favourites: Option<i32>,
    #[serde(rename = "modNotes", skip_serializing_if = "Option::is_none")]
    pub mod_notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterConnection {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edges: Option<Vec<CharacterEdge>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nodes: Option<Vec<Character>>,
    #[serde(rename = "pageInfo", skip_serializing_if = "Option::is_none")]
    pub page_info: Option<PageInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterEdge {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node: Option<Character>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<CharacterRole>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "voiceActors", skip_serializing_if = "Option::is_none")]
    pub voice_actors: Option<Vec<Staff>>,
    #[serde(rename = "voiceActorRoles", skip_serializing_if = "Option::is_none")]
    pub voice_actor_roles: Option<Vec<StaffRoleType>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media: Option<Vec<Media>>,
    #[serde(rename = "favouriteOrder", skip_serializing_if = "Option::is_none")]
    pub favourite_order: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterImage {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub large: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub medium: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterName {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub middle: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub full: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub native: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alternative: Option<Vec<String>>,
    #[serde(rename = "alternativeSpoiler", skip_serializing_if = "Option::is_none")]
    pub alternative_spoiler: Option<Vec<String>>,
    #[serde(rename = "userPreferred", skip_serializing_if = "Option::is_none")]
    pub user_preferred: Option<String>,
}
