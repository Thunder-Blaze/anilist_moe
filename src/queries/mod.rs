// Media queries and mutations
pub mod media;

// MediaList queries and mutations
pub mod medialist;

// Activity queries and mutations
pub mod activity;

// Common Mutations
pub mod common;

// User queries and mutations
pub const SEARCH_USERS: &str = include_str!("user/search_users.graphql");

// Character queries
pub const SEARCH_CHARACTERS: &str = include_str!("character/search_characters.graphql");

// Staff queries
pub const SEARCH_STAFF: &str = include_str!("staff/search_staff.graphql");

// Studio queries
pub const SEARCH_STUDIOS: &str = include_str!("studio/search_studios.graphql");

// Airing queries
pub mod airing;

// Forum queries and mutations
pub const SEARCH_THREADS: &str = include_str!("forum/search_threads.graphql");
pub const SAVE_THREAD_COMMENT: &str = include_str!("forum/save_thread_comment.graphql");
pub const COMMENT_ON_THREAD: &str = include_str!("forum/comment_on_thread.graphql");
pub const TOGGLE_THREAD_LIKE: &str = include_str!("forum/toggle_thread_like.graphql");
pub const LIKE_THREAD_COMMENT: &str = include_str!("forum/like_thread_comment.graphql");

// Review queries and mutations
pub mod review;

// Recommendation mutations
pub mod recommendation;

// Notification queries
pub mod notification;
