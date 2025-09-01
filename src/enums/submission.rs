use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SubmissionSort {
    #[serde(rename = "ID")]
    Id,
    #[serde(rename = "ID_DESC")]
    IdDesc,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SubmissionStatus {
    #[serde(rename = "PENDING")]
    Pending,
    #[serde(rename = "REJECTED")]
    Rejected,
    #[serde(rename = "PARTIALLY_ACCEPTED")]
    PartiallyAccepted,
    #[serde(rename = "APPROVED")]
    Approved,
}