// Thread queries
pub const FETCH: &str = include_str!("fetch.graphql");
pub const FETCH_ONE: &str = include_str!("fetch_one.graphql");

// Thread comment queries
pub const FETCH_COMMENT: &str = include_str!("fetch_comment.graphql");
pub const FETCH_COMMENT_ONE: &str = include_str!("fetch_comment_one.graphql");
//
// Thread mutations
pub const SAVE: &str = include_str!("save.graphql");
pub const DELETE: &str = include_str!("delete.graphql");

// Comment mutations
pub const SAVE_COMMENT: &str = include_str!("save_comment.graphql");
pub const DELETE_COMMENT: &str = include_str!("delete_comment.graphql");

// Subscription mutations
pub const SUBSCRIPTION: &str = include_str!("subscription.graphql");
