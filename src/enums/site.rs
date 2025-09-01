use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SiteTrendSort {
    #[serde(rename = "DATE")]
    Date,
    #[serde(rename = "DATE_DESC")]
    DateDesc,
    #[serde(rename = "COUNT")]
    Count,
    #[serde(rename = "COUNT_DESC")]
    CountDesc,
    #[serde(rename = "CHANGE")]
    Change,
    #[serde(rename = "CHANGE_DESC")]
    ChangeDesc,
}