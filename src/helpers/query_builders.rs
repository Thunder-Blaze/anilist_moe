use crate::enums::activity::{ActivitySort, ActivityType};
use crate::enums::character::CharacterSort;
use crate::enums::media::{
    MediaFormat, MediaSeason, MediaSort, MediaSource, MediaStatus, MediaType,
};
use crate::enums::staff::StaffSort;
use crate::enums::studio::StudioSort;
use crate::enums::user::UserSort;
use serde_json::{Value, json};
use std::collections::HashMap;

/// Helper trait for converting types to GraphQL values while filtering out None/null values
pub trait ToGraphQLValue {
    fn to_graphql_value(self) -> Option<Value>;
}

impl<T> ToGraphQLValue for Option<T>
where
    T: ToGraphQLValue,
{
    fn to_graphql_value(self) -> Option<Value> {
        self.and_then(|v| v.to_graphql_value())
    }
}

impl ToGraphQLValue for String {
    fn to_graphql_value(self) -> Option<Value> {
        if self.is_empty() {
            None
        } else {
            Some(Value::String(self))
        }
    }
}

impl ToGraphQLValue for &str {
    fn to_graphql_value(self) -> Option<Value> {
        if self.is_empty() {
            None
        } else {
            Some(Value::String(self.to_string()))
        }
    }
}

impl ToGraphQLValue for i32 {
    fn to_graphql_value(self) -> Option<Value> {
        Some(Value::Number(self.into()))
    }
}

impl ToGraphQLValue for u32 {
    fn to_graphql_value(self) -> Option<Value> {
        Some(Value::Number(self.into()))
    }
}

impl ToGraphQLValue for f64 {
    fn to_graphql_value(self) -> Option<Value> {
        serde_json::Number::from_f64(self).map(Value::Number)
    }
}

impl ToGraphQLValue for bool {
    fn to_graphql_value(self) -> Option<Value> {
        Some(Value::Bool(self))
    }
}

impl<T> ToGraphQLValue for Vec<T>
where
    T: ToGraphQLValue,
{
    fn to_graphql_value(self) -> Option<Value> {
        if self.is_empty() {
            None
        } else {
            let values: Vec<Value> = self
                .into_iter()
                .filter_map(|item| item.to_graphql_value())
                .collect();
            if values.is_empty() {
                None
            } else {
                Some(Value::Array(values))
            }
        }
    }
}

// Implement for common enum types
impl ToGraphQLValue for MediaType {
    fn to_graphql_value(self) -> Option<Value> {
        Some(Value::String(format!("{:?}", self).to_uppercase()))
    }
}

impl ToGraphQLValue for MediaFormat {
    fn to_graphql_value(self) -> Option<Value> {
        Some(Value::String(format!("{:?}", self).to_uppercase()))
    }
}

impl ToGraphQLValue for MediaStatus {
    fn to_graphql_value(self) -> Option<Value> {
        Some(Value::String(format!("{:?}", self).to_uppercase()))
    }
}

impl ToGraphQLValue for MediaSeason {
    fn to_graphql_value(self) -> Option<Value> {
        Some(Value::String(format!("{:?}", self).to_uppercase()))
    }
}

impl ToGraphQLValue for MediaSort {
    fn to_graphql_value(self) -> Option<Value> {
        Some(Value::String(format!("{:?}", self)))
    }
}

impl ToGraphQLValue for MediaSource {
    fn to_graphql_value(self) -> Option<Value> {
        Some(Value::String(format!("{:?}", self).to_uppercase()))
    }
}

impl ToGraphQLValue for ActivityType {
    fn to_graphql_value(self) -> Option<Value> {
        Some(Value::String(format!("{:?}", self).to_uppercase()))
    }
}

impl ToGraphQLValue for ActivitySort {
    fn to_graphql_value(self) -> Option<Value> {
        Some(Value::String(format!("{:?}", self)))
    }
}

impl ToGraphQLValue for UserSort {
    fn to_graphql_value(self) -> Option<Value> {
        Some(Value::String(format!("{:?}", self)))
    }
}

impl ToGraphQLValue for CharacterSort {
    fn to_graphql_value(self) -> Option<Value> {
        Some(Value::String(format!("{:?}", self)))
    }
}

impl ToGraphQLValue for StaffSort {
    fn to_graphql_value(self) -> Option<Value> {
        Some(Value::String(format!("{:?}", self)))
    }
}

impl ToGraphQLValue for StudioSort {
    fn to_graphql_value(self) -> Option<Value> {
        Some(Value::String(format!("{:?}", self)))
    }
}

/// Query types corresponding to our GraphQL files
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QueryType {
    // Media queries
    MediaSearch,           // media/search.graphql
    MediaDetails,          // media/media_details.graphql
    SaveMediaListEntry,    // media/save_media_list_entry.graphql
    DeleteMediaListEntry,  // media/delete_media_list_entry.graphql
    AddToList,            // media/add_to_list.graphql
    ToggleFavorite,       // media/toggle_favorite.graphql

    // Activity queries
    SearchActivities,      // activity/search_activities.graphql
    FetchActivity,        // activity/fetch_activity.graphql
    SaveTextActivity,     // activity/save_text_activity.graphql
    SaveActivityReply,    // activity/save_activity_reply.graphql
    ReplyToActivity,      // activity/reply_to_activity.graphql
    ToggleActivityLike,   // activity/toggle_like.graphql
    DeleteActivity,       // activity/delete_activity.graphql
    DeleteActivityReply,  // activity/delete_activity_reply.graphql

    // User queries
    SearchUsers,          // user/search_users.graphql
    ToggleFollow,         // user/toggle_follow.graphql

    // Character queries
    SearchCharacters,     // character/search_characters.graphql

    // Staff queries
    SearchStaff,         // staff/search_staff.graphql

    // Studio queries
    SearchStudios,       // studio/search_studios.graphql

    // Airing queries
    AiringSchedule,      // airing/airing_schedule.graphql

    // Forum queries
    SearchThreads,       // forum/search_threads.graphql
    SaveThreadComment,   // forum/save_thread_comment.graphql
    CommentOnThread,     // forum/comment_on_thread.graphql
    ToggleThreadLike,    // forum/toggle_thread_like.graphql
    LikeThreadComment,   // forum/like_thread_comment.graphql

    // Review queries
    SearchReviews,       // review/search_reviews.graphql
    SaveReview,          // review/save_review.graphql
    DeleteReview,        // review/delete_review.graphql

    // Recommendation queries
    SaveRecommendation,  // recommendation/save_recommendation.graphql

    // Notification queries
    Notifications,       // notification/notifications.graphql
}

/// Builder for creating GraphQL variable maps
#[derive(Default)]
pub struct QueryVariables {
    variables: HashMap<String, Value>,
}

impl QueryVariables {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
        }
    }

    /// Add a variable if it's not None/null
    pub fn add_var<T: ToGraphQLValue>(mut self, key: &str, value: T) -> Self {
        if let Some(graphql_value) = value.to_graphql_value() {
            self.variables.insert(key.to_string(), graphql_value);
        }
        self
    }

    /// Add a variable with a custom name
    pub fn add_named_var<T: ToGraphQLValue>(mut self, key: &str, value: T) -> Self {
        if let Some(graphql_value) = value.to_graphql_value() {
            self.variables.insert(key.to_string(), graphql_value);
        }
        self
    }

    /// Build the final variables map
    pub fn build(self) -> Value {
        json!(self.variables)
    }

    /// Get the variables as a HashMap
    pub fn into_map(self) -> HashMap<String, Value> {
        self.variables
    }
}

/// Generic query builder that works with any QueryType
pub struct QueryBuilder<T: Copy> {
    query_type: T,
    variables: QueryVariables,
}

/// Type alias for MediaSearch QueryBuilder
pub type MediaSearchQueryBuilder = QueryBuilder<QueryType>;

impl<T: Copy> QueryBuilder<T> {
    pub fn new(query_type: T) -> Self {
        Self {
            query_type,
            variables: QueryVariables::new(),
        }
    }

    pub fn build(self) -> Value {
        self.variables.build()
    }
}

/// Media search specific methods
impl QueryBuilder<QueryType> {
    // Common fields that appear in multiple queries
    pub fn id(self, id: Option<i32>) -> Self {
        Self {
            variables: self.variables.add_var("id", id),
            ..self
        }
    }

    pub fn page(self, page: Option<i32>) -> Self {
        Self {
            variables: self.variables.add_var("page", page),
            ..self
        }
    }

    pub fn per_page(self, per_page: Option<i32>) -> Self {
        Self {
            variables: self.variables.add_var("perPage", per_page),
            ..self
        }
    }

    pub fn search(self, search: Option<String>) -> Self {
        Self {
            variables: self.variables.add_var("search", search),
            ..self
        }
    }
}

/// Methods available for media-related queries
impl QueryBuilder<QueryType> {
    pub fn media_id(self, media_id: Option<i32>) -> Self {
        match self.query_type {
            QueryType::MediaSearch | QueryType::MediaDetails | QueryType::SaveMediaListEntry |
            QueryType::DeleteMediaListEntry | QueryType::AddToList | QueryType::ToggleFavorite => {
                Self {
                    variables: self.variables.add_var("mediaId", media_id),
                    ..self
                }
            },
            _ => self, // Ignore if not applicable
        }
    }

    pub fn id_mal(self, id_mal: Option<i32>) -> Self {
        match self.query_type {
            QueryType::MediaSearch | QueryType::MediaDetails | QueryType::SearchCharacters | QueryType::SearchStaff => {
                Self {
                    variables: self.variables.add_var("idMal", id_mal),
                    ..self
                }
            },
            _ => self,
        }
    }

    pub fn media_type(self, media_type: Option<MediaType>) -> Self {
        match self.query_type {
            QueryType::MediaSearch | QueryType::MediaDetails => {
                Self {
                    variables: self.variables.add_var("type", media_type),
                    ..self
                }
            },
            _ => self,
        }
    }

    pub fn format(self, format: Option<Vec<MediaFormat>>) -> Self {
        match self.query_type {
            QueryType::MediaSearch => {
                Self {
                    variables: self.variables.add_var("format", format),
                    ..self
                }
            },
            _ => self,
        }
    }

    pub fn status(self, status: Option<MediaStatus>) -> Self {
        match self.query_type {
            QueryType::MediaSearch => {
                Self {
                    variables: self.variables.add_var("status", status),
                    ..self
                }
            },
            _ => self,
        }
    }

    pub fn season(self, season: Option<MediaSeason>) -> Self {
        match self.query_type {
            QueryType::MediaSearch => {
                Self {
                    variables: self.variables.add_var("season", season),
                    ..self
                }
            },
            _ => self,
        }
    }

    pub fn season_year(self, season_year: Option<i32>) -> Self {
        match self.query_type {
            QueryType::MediaSearch => {
                Self {
                    variables: self.variables.add_var("seasonYear", season_year),
                    ..self
                }
            },
            _ => self,
        }
    }

    pub fn year(self, year: Option<String>) -> Self {
        match self.query_type {
            QueryType::MediaSearch => {
                Self {
                    variables: self.variables.add_var("year", year),
                    ..self
                }
            },
            _ => self,
        }
    }

    pub fn genre(self, genre: Option<Vec<String>>) -> Self {
        match self.query_type {
            QueryType::MediaSearch => {
                Self {
                    variables: self.variables.add_var("genre", genre),
                    ..self
                }
            },
            _ => self,
        }
    }

    pub fn tag(self, tag: Option<Vec<String>>) -> Self {
        match self.query_type {
            QueryType::MediaSearch => {
                Self {
                    variables: self.variables.add_var("tag", tag),
                    ..self
                }
            },
            _ => self,
        }
    }

    pub fn average_score_greater(self, score: Option<i32>) -> Self {
        match self.query_type {
            QueryType::MediaSearch => {
                Self {
                    variables: self.variables.add_var("averageScore_greater", score),
                    ..self
                }
            },
            _ => self,
        }
    }

    pub fn popularity_greater(self, popularity: Option<i32>) -> Self {
        match self.query_type {
            QueryType::MediaSearch => {
                Self {
                    variables: self.variables.add_var("popularity_greater", popularity),
                    ..self
                }
            },
            _ => self,
        }
    }

    pub fn extended(self, extended: Option<bool>) -> Self {
        match self.query_type {
            QueryType::MediaSearch => {
                Self {
                    variables: self.variables.add_var("extended", extended),
                    ..self
                }
            },
            _ => self,
        }
    }

    pub fn sort_media(self, sort: Option<Vec<MediaSort>>) -> Self {
        match self.query_type {
            QueryType::MediaSearch => {
                Self {
                    variables: self.variables.add_var("sort", sort),
                    ..self
                }
            },
            _ => self,
        }
    }
}

/// Methods for mutation queries (list entries, favorites, etc.)
impl QueryBuilder<QueryType> {
    pub fn list_status(self, status: Option<String>) -> Self {
        match self.query_type {
            QueryType::SaveMediaListEntry | QueryType::AddToList => {
                Self {
                    variables: self.variables.add_var("status", status),
                    ..self
                }
            },
            _ => self,
        }
    }

    pub fn score_raw(self, score_raw: Option<i32>) -> Self {
        match self.query_type {
            QueryType::SaveMediaListEntry | QueryType::AddToList => {
                Self {
                    variables: self.variables.add_var("scoreRaw", score_raw),
                    ..self
                }
            },
            _ => self,
        }
    }

    pub fn progress(self, progress: Option<i32>) -> Self {
        match self.query_type {
            QueryType::SaveMediaListEntry | QueryType::AddToList => {
                Self {
                    variables: self.variables.add_var("progress", progress),
                    ..self
                }
            },
            _ => self,
        }
    }

    pub fn notes(self, notes: Option<String>) -> Self {
        match self.query_type {
            QueryType::SaveMediaListEntry | QueryType::AddToList => {
                Self {
                    variables: self.variables.add_var("notes", notes),
                    ..self
                }
            },
            _ => self,
        }
    }

    pub fn private(self, private: Option<bool>) -> Self {
        match self.query_type {
            QueryType::SaveMediaListEntry | QueryType::AddToList => {
                Self {
                    variables: self.variables.add_var("private", private),
                    ..self
                }
            },
            _ => self,
        }
    }
}

/// Methods for activity queries
impl QueryBuilder<QueryType> {
    pub fn user_id(self, user_id: Option<i32>) -> Self {
        match self.query_type {
            QueryType::SearchActivities | QueryType::SearchUsers | QueryType::ToggleFollow => {
                Self {
                    variables: self.variables.add_var("userId", user_id),
                    ..self
                }
            },
            _ => self,
        }
    }

    pub fn activity_type(self, activity_type: Option<ActivityType>) -> Self {
        match self.query_type {
            QueryType::SearchActivities => {
                Self {
                    variables: self.variables.add_var("type", activity_type),
                    ..self
                }
            },
            _ => self,
        }
    }

    pub fn is_following(self, is_following: Option<bool>) -> Self {
        match self.query_type {
            QueryType::SearchActivities => {
                Self {
                    variables: self.variables.add_var("isFollowing", is_following),
                    ..self
                }
            },
            _ => self,
        }
    }

    pub fn has_replies(self, has_replies: Option<bool>) -> Self {
        match self.query_type {
            QueryType::SearchActivities => {
                Self {
                    variables: self.variables.add_var("hasReplies", has_replies),
                    ..self
                }
            },
            _ => self,
        }
    }

    pub fn text(self, text: Option<String>) -> Self {
        match self.query_type {
            QueryType::SaveTextActivity | QueryType::SaveActivityReply | QueryType::ReplyToActivity => {
                Self {
                    variables: self.variables.add_var("text", text),
                    ..self
                }
            },
            _ => self,
        }
    }

    pub fn activity_id(self, activity_id: Option<i32>) -> Self {
        match self.query_type {
            QueryType::SaveActivityReply | QueryType::ReplyToActivity |
            QueryType::ToggleActivityLike | QueryType::DeleteActivity => {
                Self {
                    variables: self.variables.add_var("activityId", activity_id),
                    ..self
                }
            },
            _ => self,
        }
    }
}

/// Methods for user queries
impl QueryBuilder<QueryType> {
    pub fn name(self, name: Option<String>) -> Self {
        match self.query_type {
            QueryType::SearchUsers => {
                Self {
                    variables: self.variables.add_var("name", name),
                    ..self
                }
            },
            _ => self,
        }
    }

    pub fn is_moderator(self, is_moderator: Option<bool>) -> Self {
        match self.query_type {
            QueryType::SearchUsers => {
                Self {
                    variables: self.variables.add_var("isModerator", is_moderator),
                    ..self
                }
            },
            _ => self,
        }
    }

    pub fn sort_user(self, sort: Option<Vec<UserSort>>) -> Self {
        match self.query_type {
            QueryType::SearchUsers => {
                Self {
                    variables: self.variables.add_var("sort", sort.map(|s| {
                        s.into_iter().map(|sort| format!("{:?}", sort)).collect::<Vec<_>>()
                    })),
                    ..self
                }
            },
            _ => self,
        }
    }
}

/// Methods for character queries
impl QueryBuilder<QueryType> {
    pub fn is_birthday(self, is_birthday: Option<bool>) -> Self {
        match self.query_type {
            QueryType::SearchCharacters | QueryType::SearchStaff => {
                Self {
                    variables: self.variables.add_var("isBirthday", is_birthday),
                    ..self
                }
            },
            _ => self,
        }
    }

    pub fn sort_character(self, sort: Option<Vec<CharacterSort>>) -> Self {
        match self.query_type {
            QueryType::SearchCharacters => {
                Self {
                    variables: self.variables.add_var("sort", sort.map(|s| {
                        s.into_iter().map(|sort| format!("{:?}", sort)).collect::<Vec<_>>()
                    })),
                    ..self
                }
            },
            _ => self,
        }
    }
}

/// Methods for staff queries
impl QueryBuilder<QueryType> {
    pub fn sort_staff(self, sort: Option<Vec<StaffSort>>) -> Self {
        match self.query_type {
            QueryType::SearchStaff => {
                Self {
                    variables: self.variables.add_var("sort", sort.map(|s| {
                        s.into_iter().map(|sort| format!("{:?}", sort)).collect::<Vec<_>>()
                    })),
                    ..self
                }
            },
            _ => self,
        }
    }
}

/// Methods for studio queries
impl QueryBuilder<QueryType> {
    pub fn sort_studio(self, sort: Option<Vec<StudioSort>>) -> Self {
        match self.query_type {
            QueryType::SearchStudios => {
                Self {
                    variables: self.variables.add_var("sort", sort.map(|s| {
                        s.into_iter().map(|sort| format!("{:?}", sort)).collect::<Vec<_>>()
                    })),
                    ..self
                }
            },
            _ => self,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::enums::media::{MediaFormat, MediaSeason, MediaStatus, MediaType};

    #[test]
    fn test_media_search_builder() {
        let variables = QueryBuilder::new(QueryType::MediaSearch)
            .search(Some("Attack on Titan".to_string()))
            .media_type(Some(MediaType::Anime))
            .format(Some(vec![MediaFormat::Tv, MediaFormat::Movie]))
            .status(Some(MediaStatus::Finished))
            .season(Some(MediaSeason::Spring))
            .season_year(Some(2023))
            .genre(Some(vec!["Action".to_string(), "Drama".to_string()]))
            .average_score_greater(Some(80))
            .page(Some(1))
            .per_page(Some(25))
            .extended(Some(true))
            .build();

        println!("{}", serde_json::to_string_pretty(&variables).unwrap());

        // Verify some key fields are present
        let vars = variables.as_object().unwrap();
        assert!(vars.contains_key("search"));
        assert!(vars.contains_key("type"));
        assert!(vars.contains_key("extended"));
    }

    #[test]
    fn test_empty_values_filtered() {
        let variables = QueryBuilder::new(QueryType::MediaSearch)
            .search(None) // Should be filtered out
            .media_type(Some(MediaType::Anime))
            .format(Some(vec![])) // Should be filtered out
            .genre(None) // Should be filtered out
            .page(Some(1))
            .build();

        println!("{}", serde_json::to_string_pretty(&variables).unwrap());

        let vars = variables.as_object().unwrap();
        assert!(!vars.contains_key("search"));
        assert!(!vars.contains_key("format"));
        assert!(!vars.contains_key("genre"));
        assert!(vars.contains_key("type"));
        assert!(vars.contains_key("page"));
    }

    #[test]
    fn test_mutation_builder() {
        let variables = QueryBuilder::new(QueryType::SaveMediaListEntry)
            .media_id(Some(12345))
            .list_status(Some("COMPLETED".to_string()))
            .score_raw(Some(85))
            .progress(Some(12))
            .notes(Some("Great anime!".to_string()))
            .private(Some(false))
            .build();

        println!("{}", serde_json::to_string_pretty(&variables).unwrap());

        let vars = variables.as_object().unwrap();
        assert!(vars.contains_key("mediaId"));
        assert!(vars.contains_key("status"));
        assert!(vars.contains_key("scoreRaw"));
    }

    #[test]
    fn test_activity_search_builder() {
        let variables = QueryBuilder::new(QueryType::SearchActivities)
            .user_id(Some(12345))
            .activity_type(Some(ActivityType::Text))
            .is_following(Some(true))
            .has_replies(Some(true))
            .page(Some(1))
            .per_page(Some(25))
            .build();

        println!("{}", serde_json::to_string_pretty(&variables).unwrap());

        let vars = variables.as_object().unwrap();
        assert!(vars.contains_key("userId"));
        assert!(vars.contains_key("type"));
        assert!(vars.contains_key("isFollowing"));
    }

    #[test]
    fn test_conditional_methods() {
        // Test that media-specific methods don't work on activity queries
        let activity_vars = QueryBuilder::new(QueryType::SearchActivities)
            .media_type(Some(MediaType::Anime)) // This should be ignored
            .user_id(Some(123)) // This should work
            .build();

        let vars = activity_vars.as_object().unwrap();
        assert!(!vars.contains_key("type")); // media_type should be ignored
        assert!(vars.contains_key("userId")); // user_id should work

        // Test that activity-specific methods don't work on media queries
        let media_vars = QueryBuilder::new(QueryType::MediaSearch)
            .has_replies(Some(true)) // This should be ignored
            .media_type(Some(MediaType::Anime)) // This should work
            .build();

        let vars2 = media_vars.as_object().unwrap();
        assert!(!vars2.contains_key("hasReplies")); // activity method should be ignored
        assert!(vars2.contains_key("type")); // media method should work
    }
}
