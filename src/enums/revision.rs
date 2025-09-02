use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RevisionHistoryAction {
    #[serde(rename = "CREATE")]
    Create,
    #[serde(rename = "Edit")]
    Edit,
}
