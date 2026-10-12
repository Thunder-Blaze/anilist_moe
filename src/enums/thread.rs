use crate::options::SortSpec;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ThreadCommentSort {
    Id,
    IdDesc,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ThreadSort {
    Id,
    IdDesc,
    Title,
    TitleDesc,
    CreatedAt,
    CreatedAtDesc,
    UpdatedAt,
    UpdatedAtDesc,
    RepliedAt,
    RepliedAtDesc,
    ReplyCount,
    ReplyCountDesc,
    ViewCount,
    ViewCountDesc,
    IsSticky,
    SearchMatch,
}

impl SortSpec for ThreadSort {
    fn id_tiebreaker() -> Self {
        Self::Id
    }
}

impl SortSpec for ThreadCommentSort {
    fn id_tiebreaker() -> Self {
        Self::Id
    }
}
