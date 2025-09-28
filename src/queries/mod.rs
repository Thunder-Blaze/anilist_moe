// Media queries and mutations
pub const MEDIA_SEARCH: &str = include_str!("media/search.graphql");
pub const MEDIA_DETAILS: &str = include_str!("media/media_details.graphql");
pub const SAVE_MEDIA_LIST_ENTRY: &str = include_str!("media/save_media_list_entry.graphql");
pub const DELETE_MEDIA_LIST_ENTRY: &str = include_str!("media/delete_media_list_entry.graphql");
pub const ADD_TO_LIST: &str = include_str!("media/add_to_list.graphql");
pub const TOGGLE_FAVORITE: &str = include_str!("media/toggle_favorite.graphql");

// Activity queries and mutations
pub const SEARCH_ACTIVITIES: &str = include_str!("activity/search_activities.graphql");
pub const FETCH_ACTIVITY: &str = include_str!("activity/fetch_activity.graphql");
pub const SAVE_TEXT_ACTIVITY: &str = include_str!("activity/save_text_activity.graphql");
pub const SAVE_ACTIVITY_REPLY: &str = include_str!("activity/save_activity_reply.graphql");
pub const REPLY_TO_ACTIVITY: &str = include_str!("activity/reply_to_activity.graphql");
pub const TOGGLE_ACTIVITY_LIKE: &str = include_str!("activity/toggle_like.graphql");
pub const DELETE_ACTIVITY: &str = include_str!("activity/delete_activity.graphql");
pub const DELETE_ACTIVITY_REPLY: &str = include_str!("activity/delete_activity_reply.graphql");

// User queries and mutations
pub const SEARCH_USERS: &str = include_str!("user/search_users.graphql");
pub const TOGGLE_FOLLOW: &str = include_str!("user/toggle_follow.graphql");

// Character queries
pub const SEARCH_CHARACTERS: &str = include_str!("character/search_characters.graphql");

// Staff queries
pub const SEARCH_STAFF: &str = include_str!("staff/search_staff.graphql");

// Studio queries
pub const SEARCH_STUDIOS: &str = include_str!("studio/search_studios.graphql");

// Airing queries
pub const AIRING_SCHEDULE: &str = include_str!("airing/airing_schedule.graphql");

// Forum queries and mutations
pub const SEARCH_THREADS: &str = include_str!("forum/search_threads.graphql");
pub const SAVE_THREAD_COMMENT: &str = include_str!("forum/save_thread_comment.graphql");
pub const COMMENT_ON_THREAD: &str = include_str!("forum/comment_on_thread.graphql");
pub const TOGGLE_THREAD_LIKE: &str = include_str!("forum/toggle_thread_like.graphql");
pub const LIKE_THREAD_COMMENT: &str = include_str!("forum/like_thread_comment.graphql");

// Review queries and mutations
pub const SEARCH_REVIEWS: &str = include_str!("review/search_reviews.graphql");
pub const SAVE_REVIEW: &str = include_str!("review/save_review.graphql");
pub const DELETE_REVIEW: &str = include_str!("review/delete_review.graphql");

// Recommendation mutations
pub const SAVE_RECOMMENDATION: &str = include_str!("recommendation/save_recommendation.graphql");

// Notification queries
pub const NOTIFICATIONS: &str = include_str!("notification/notifications.graphql");
