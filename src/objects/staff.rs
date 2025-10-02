use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::objects::{
    character::CharacterConnection,
    common::{FuzzyDate, PageInfo},
    media::MediaConnection,
    user::User,
};

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Staff {
    pub id: i32,
    pub name: Option<StaffName>,
    pub language_v2: Option<String>,
    pub image: Option<StaffImage>,
    pub description: Option<String>,
    pub primary_occupations: Option<Vec<String>>,
    pub gender: Option<String>,
    pub date_of_birth: Option<FuzzyDate>,
    pub date_of_death: Option<FuzzyDate>,
    pub age: Option<i32>,
    pub years_active: Option<Vec<i32>>,
    pub home_town: Option<String>,
    pub blood_type: Option<String>,
    pub is_favourite: Option<bool>,
    pub is_favourite_blocked: Option<bool>,
    pub site_url: Option<String>,
    pub staff_media: Option<MediaConnection>,
    pub characters: Option<CharacterConnection>,
    pub character_media: Option<MediaConnection>,
    pub staff: Box<Option<Staff>>,
    pub submitter: Option<User>,
    pub submission_status: Option<i32>,
    pub submission_notes: Option<String>,
    pub favourites: Option<i32>,
    pub mod_notes: Option<String>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StaffConnection {
    pub edges: Option<Vec<StaffEdge>>,
    pub nodes: Option<Vec<Staff>>,
    pub page_info: Option<PageInfo>,
    pub total_count: Option<i32>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StaffEdge {
    pub node: Option<Staff>,
    pub id: Option<i32>,
    pub role: Option<String>,
    pub favourite_order: Option<i32>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StaffImage {
    pub large: Option<String>,
    pub medium: Option<String>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StaffName {
    pub first: Option<String>,
    pub middle: Option<String>,
    pub last: Option<String>,
    pub full: Option<String>,
    pub native: Option<String>,
    pub alternative: Option<Vec<String>>,
    pub user_preferred: Option<String>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StaffRoleType {
    pub voice_actor: Option<Staff>,
    pub role_notes: Option<String>,
    pub dub_group: Option<String>,
}
