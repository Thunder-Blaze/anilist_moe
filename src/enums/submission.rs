use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[non_exhaustive]
pub enum SubmissionSort {
    Id,
    IdDesc,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[non_exhaustive]
pub enum SubmissionStatus {
    Pending,
    Rejected,
    PartiallyAccepted,
    Approved,
    /// A value the API returned that this version of the crate does not know yet.
    ///
    /// Only ever produced by deserialization; never send it as input.
    #[serde(other)]
    Unknown,
}
