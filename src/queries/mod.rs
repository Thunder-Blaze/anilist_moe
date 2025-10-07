//! # GraphQL Queries
//!
//! This module contains all GraphQL queries used by the AniList API wrapper.
//! Queries are organized by endpoint category and loaded from separate .graphql files.

/// Anime-related GraphQL queries
pub mod anime {
    /// Get popular anime query
    pub const GET_POPULAR: &str = include_str!("anime/get_popular.graphql");

    /// Get trending anime query
    pub const GET_TRENDING: &str = include_str!("anime/get_trending.graphql");

    /// Search anime query
    pub const SEARCH: &str = include_str!("anime/search.graphql");
}

/// User-related GraphQL queries
pub mod user {
    /// Toggle follow/unfollow user mutation
    pub const TOGGLE_FOLLOW: &str = include_str!("user/toggle_follow.graphql");

    /// Toggle favorite anime/manga mutation
    pub const TOGGLE_FAVORITE: &str = include_str!("user/toggle_favorite.graphql");
}

/// Activity-related GraphQL queries
pub mod activity {
    /// Toggle like on activity mutation
    pub const TOGGLE_LIKE: &str = include_str!("activity/toggle_like.graphql");

    /// Reply to activity mutation
    pub const REPLY_TO_ACTIVITY: &str = include_str!("activity/reply_to_activity.graphql");
}

/// Forum-related GraphQL queries
pub mod forum {
    /// Comment on thread mutation
    pub const COMMENT_ON_THREAD: &str = include_str!("forum/comment_on_thread.graphql");

    /// Like thread comment mutation
    pub const LIKE_THREAD_COMMENT: &str = include_str!("forum/like_thread_comment.graphql");
}
