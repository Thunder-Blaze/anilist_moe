use serde::{Deserialize, Serialize};

use crate::{
    enums::{character::CharacterRole, submission::SubmissionStatus},
    objects::{character::Character, common::PageInfo, staff::Staff, user::User},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterSubmission {
    pub id: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub character: Option<Character>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submission: Option<Character>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submitter: Option<User>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assignee: Option<User>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<SubmissionStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locked: Option<bool>,
    #[serde(rename = "createdAt", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterSubmissionConnection {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edges: Option<Vec<CharacterSubmissionEdge>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nodes: Option<Vec<CharacterSubmission>>,
    #[serde(rename = "pageInfo", skip_serializing_if = "Option::is_none")]
    pub page_info: Option<PageInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterSubmissionEdge {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node: Option<CharacterSubmission>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<CharacterRole>,
    #[serde(rename = "voiceActors", skip_serializing_if = "Option::is_none")]
    pub voice_actors: Option<Vec<Staff>>,
    #[serde(rename = "submittedVoiceActors", skip_serializing_if = "Option::is_none")]
    pub submitted_voice_actors: Option<Vec<StaffSubmission>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StaffSubmission {
    pub id: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub staff: Option<Staff>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submission: Option<Staff>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submitter: Option<User>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assignee: Option<User>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<SubmissionStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locked: Option<bool>,
    #[serde(rename = "createdAt", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<i32>,
}
