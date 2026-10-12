use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[non_exhaustive]
pub enum SiteTrendSort {
    Date,
    DateDesc,
    Count,
    CountDesc,
    Change,
    ChangeDesc,
}
