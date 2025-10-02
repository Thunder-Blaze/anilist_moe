use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{
    enums::{character::CharacterRole, submission::SubmissionStatus},
    objects::{character::Character, common::PageInfo, staff::Staff, user::User},
};

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CharacterSubmission {
    pub id: i32,
    pub character: Option<Character>,
    pub submission: Option<Character>,
    pub submitter: Option<User>,
    pub assignee: Option<User>,
    pub status: Option<SubmissionStatus>,
    pub notes: Option<String>,
    pub source: Option<String>,
    pub locked: Option<bool>,
    pub created_at: Option<i32>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CharacterSubmissionConnection {
    pub edges: Option<Vec<CharacterSubmissionEdge>>,
    pub nodes: Option<Vec<CharacterSubmission>>,
    pub page_info: Option<PageInfo>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CharacterSubmissionEdge {
    pub node: Option<CharacterSubmission>,
    pub role: Option<CharacterRole>,
    pub voice_actors: Option<Vec<Staff>>,
    pub submitted_voice_actors: Option<Vec<StaffSubmission>>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StaffSubmission {
    pub id: i32,
    pub staff: Option<Staff>,
    pub submission: Option<Staff>,
    pub submitter: Option<User>,
    pub assignee: Option<User>,
    pub status: Option<SubmissionStatus>,
    pub notes: Option<String>,
    pub source: Option<String>,
    pub locked: Option<bool>,
    pub created_at: Option<i32>,
}
