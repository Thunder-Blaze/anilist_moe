//! # GraphQL Queries
//!
//! This module contains all GraphQL queries used by the AniList API wrapper.
//! Queries are organized by endpoint category and loaded from separate .graphql files.

pub mod anime {
    pub const GET_POPULAR: &str = include_str!("anime/get_popular.graphql");

    pub const GET_TRENDING: &str = include_str!("anime/get_trending.graphql");

    pub const SEARCH: &str = include_str!("anime/search.graphql");
}

pub mod manga {
    pub const SEARCH: &str = include_str!("manga/search.graphql");
}

pub mod user {
    pub const TOGGLE_FOLLOW: &str = include_str!("user/toggle_follow.graphql");

    pub const TOGGLE_FAVORITE: &str = include_str!("user/toggle_favorite.graphql");
}

pub mod activity {
    pub const TOGGLE_LIKE: &str = include_str!("activity/toggle_like.graphql");

    pub const REPLY_TO_ACTIVITY: &str = include_str!("activity/reply_to_activity.graphql");
}

pub mod forum {
    pub const COMMENT_ON_THREAD: &str = include_str!("forum/comment_on_thread.graphql");

    pub const LIKE_THREAD_COMMENT: &str = include_str!("forum/like_thread_comment.graphql");
}
