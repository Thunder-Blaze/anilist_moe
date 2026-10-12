use crate::options::SortSpec;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[non_exhaustive]
pub enum ThreadCommentSort {
    Id,
    IdDesc,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[non_exhaustive]
pub enum ThreadSort {
    Id,
    IdDesc,
    Title,
    TitleDesc,
    #[deprecated(note = "the API deprecated it: use `Id` instead")]
    CreatedAt,
    #[deprecated(note = "the API deprecated it: use `IdDesc` instead")]
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
