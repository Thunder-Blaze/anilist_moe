use serde::{Deserialize, Serialize};

#[derive(Debug,   Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SiteTrendSort {
    Date,
    DateDesc,
    Count,
    CountDesc,
    Change,
    ChangeDesc,
}
