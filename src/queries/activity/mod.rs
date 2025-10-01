// Activity queries and mutations
pub const FETCH: &str = include_str!("fetch.graphql");
pub const FETCH_ONE: &str = include_str!("fetch_one.graphql");
pub const FETCH_REPLIES: &str = include_str!("fetch_replies.graphql");

pub const SAVE_TEXT_ACTIVITY: &str = include_str!("save_text_activity.graphql");
pub const SAVE_MESSAGE_ACTIVITY: &str = include_str!("save_message_activity.graphql");
pub const SAVE_REPLY: &str = include_str!("save_reply.graphql");

pub const SUBSCRIBE: &str = include_str!("subscribe.graphql");

pub const PIN: &str = include_str!("pin.graphql");

pub const DELETE: &str = include_str!("delete.graphql");
pub const DELETE_REPLY: &str = include_str!("delete_reply.graphql");
