use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RevisionHistoryAction {
    Create,
    #[serde(rename = "Edit")]
    Edit,
}
