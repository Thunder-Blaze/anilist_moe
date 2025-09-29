use serde::Serialize;
use crate::client::AniListClient;
use crate::errors::AniListError;
use crate::enums::user::UserSort;
use crate::enums::activity::{ActivitySort, ActivityType};
use crate::enums::media_list::{MediaListSort, MediaListStatus};
use crate::enums::character::CharacterSort;
use crate::enums::staff::StaffSort;
use crate::enums::studio::StudioSort;
use crate::enums::media::{MediaSort, MediaType};
use crate::objects::responses::{UserListResponse, UserSingleResponse, UserResponse, UserMediaListResponse, MediaListResponse, ActivityListResponse};
use serde_json::{json, Value};
use std::collections::HashMap;

#[derive(Default, Serialize)]
pub struct UserSearchOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i32>,
    #[serde(rename = "perPage", skip_serializing_if = "Option::is_none")]
    pub per_page: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<Vec<UserSort>>,
}

pub struct UserEndpoint(pub(crate) AniListClient);

impl UserEndpoint {
    pub fn new(client: AniListClient) -> Self {
        Self(client)
    }

    pub async fn get_by_id(&self, id: i32) -> Result<UserSingleResponse, AniListError> {
        let query = include_str!("../queries/user/get_user_by_id.graphql");
        let mut variables_map = std::collections::HashMap::new();
        variables_map.insert("id".to_string(), serde_json::Value::Number(serde_json::Number::from(id)));

        self.0.query_typed(query, Some(&variables_map)).await
    }

    pub async fn get_by_username(&self, name: &str) -> Result<UserSingleResponse, AniListError> {
        let options = UserSearchOptions {
            search: Some(name.to_string()),
            ..Default::default()
        };
        let result = self.search_users(options).await?;
        let users = &result.data.page.data.users;

        for user in users {
            if user.name.eq_ignore_ascii_case(name) {
                return Ok(UserSingleResponse {
                    data: UserResponse { user: user.clone() }
                });
            }
        }
        Err(AniListError::NotFound)
    }

    pub async fn search(&self, query: &str, page: i32, per_page: i32) -> Result<UserListResponse, AniListError> {
        let options = UserSearchOptions {
            search: Some(query.to_string()),
            page: Some(page),
            per_page: Some(per_page),
            ..Default::default()
        };
        self.search_users(options).await
    }

    pub async fn get_most_anime_watched(&self, page: i32, per_page: i32) -> Result<UserListResponse, AniListError> {
        let options = UserSearchOptions {
            page: Some(page),
            per_page: Some(per_page),
            sort: Some(vec![UserSort::WatchedTimeDesc]),
            ..Default::default()
        };
        self.search_users(options).await
    }

    pub async fn get_most_manga_read(&self, page: i32, per_page: i32) -> Result<UserListResponse, AniListError> {
        let options = UserSearchOptions {
            page: Some(page),
            per_page: Some(per_page),
            sort: Some(vec![UserSort::ChaptersReadDesc]),
            ..Default::default()
        };
        self.search_users(options).await
    }

    async fn search_users(&self, options: UserSearchOptions) -> Result<UserListResponse, AniListError> {
        let query = include_str!("../queries/user/search_users.graphql");
        let variables = json!(options);
        let variables_map = self.value_to_hashmap(variables);
        self.0.query_typed(query, Some(&variables_map)).await
    }

    fn value_to_hashmap(&self, value: Value) -> HashMap<String, Value> {
        match value {
            Value::Object(map) => map.into_iter().collect(),
            _ => HashMap::new(),
        }
    }

    // User Profile Methods

    /// Get user's anime list with pagination and filtering
    pub async fn get_anime_list(
        &self,
        user_id: i32,
        page: Option<i32>,
        per_page: Option<i32>,
        status: Option<MediaListStatus>,
        sort: Option<Vec<MediaListSort>>,
    ) -> Result<UserMediaListResponse, AniListError> {
        let query = include_str!("../queries/user/get_user_anime_list.graphql");
        let mut variables_map = HashMap::new();
        variables_map.insert("userId".to_string(), json!(user_id));
        if let Some(p) = page { variables_map.insert("page".to_string(), json!(p)); }
        if let Some(pp) = per_page { variables_map.insert("perPage".to_string(), json!(pp)); }
        if let Some(s) = status { variables_map.insert("status".to_string(), json!(s)); }
        if let Some(sort_vec) = sort { variables_map.insert("sort".to_string(), json!(sort_vec)); }
        variables_map.insert("type".to_string(), json!(MediaType::Anime));

        self.0.query_typed(query, Some(&variables_map)).await
    }

    /// Get user's manga list with pagination and filtering
    pub async fn get_manga_list(
        &self,
        user_id: i32,
        page: Option<i32>,
        per_page: Option<i32>,
        status: Option<MediaListStatus>,
        sort: Option<Vec<MediaListSort>>,
    ) -> Result<UserMediaListResponse, AniListError> {
        let query = include_str!("../queries/user/get_user_manga_list.graphql");
        let mut variables_map = HashMap::new();
        variables_map.insert("userId".to_string(), json!(user_id));
        if let Some(p) = page { variables_map.insert("page".to_string(), json!(p)); }
        if let Some(pp) = per_page { variables_map.insert("perPage".to_string(), json!(pp)); }
        if let Some(s) = status { variables_map.insert("status".to_string(), json!(s)); }
        if let Some(sort_vec) = sort { variables_map.insert("sort".to_string(), json!(sort_vec)); }
        variables_map.insert("type".to_string(), json!(MediaType::Manga));

        self.0.query_typed(query, Some(&variables_map)).await
    }

    /// Get user's followers with pagination
    pub async fn get_followers(
        &self,
        user_id: i32,
        page: Option<i32>,
        per_page: Option<i32>,
        sort: Option<Vec<UserSort>>,
    ) -> Result<UserListResponse, AniListError> {
        let query = include_str!("../queries/user/get_user_followers.graphql");
        let mut variables_map = HashMap::new();
        variables_map.insert("userId".to_string(), json!(user_id));
        if let Some(p) = page { variables_map.insert("page".to_string(), json!(p)); }
        if let Some(pp) = per_page { variables_map.insert("perPage".to_string(), json!(pp)); }
        if let Some(sort_vec) = sort { variables_map.insert("sort".to_string(), json!(sort_vec)); }

        self.0.query_typed(query, Some(&variables_map)).await
    }

    /// Get user's following with pagination
    pub async fn get_following(
        &self,
        user_id: i32,
        page: Option<i32>,
        per_page: Option<i32>,
        sort: Option<Vec<UserSort>>,
    ) -> Result<UserListResponse, AniListError> {
        let query = include_str!("../queries/user/get_user_following.graphql");
        let mut variables_map = HashMap::new();
        variables_map.insert("userId".to_string(), json!(user_id));
        if let Some(p) = page { variables_map.insert("page".to_string(), json!(p)); }
        if let Some(pp) = per_page { variables_map.insert("perPage".to_string(), json!(pp)); }
        if let Some(sort_vec) = sort { variables_map.insert("sort".to_string(), json!(sort_vec)); }

        self.0.query_typed(query, Some(&variables_map)).await
    }

    /// Get user's favorite anime with pagination
    pub async fn get_favorites_anime(
        &self,
        user_id: i32,
        page: Option<i32>,
        per_page: Option<i32>,
        sort: Option<Vec<MediaSort>>,
    ) -> Result<MediaListResponse, AniListError> {
        let query = include_str!("../queries/user/get_user_favorites_anime.graphql");
        let mut variables_map = HashMap::new();
        variables_map.insert("userId".to_string(), json!(user_id));
        if let Some(p) = page { variables_map.insert("page".to_string(), json!(p)); }
        if let Some(pp) = per_page { variables_map.insert("perPage".to_string(), json!(pp)); }
        if let Some(sort_vec) = sort { variables_map.insert("sort".to_string(), json!(sort_vec)); }

        self.0.query_typed(query, Some(&variables_map)).await
    }

    /// Get user's favorite manga with pagination
    pub async fn get_favorites_manga(
        &self,
        user_id: i32,
        page: Option<i32>,
        per_page: Option<i32>,
        sort: Option<Vec<MediaSort>>,
    ) -> Result<MediaListResponse, AniListError> {
        let query = include_str!("../queries/user/get_user_favorites_manga.graphql");
        let mut variables_map = HashMap::new();
        variables_map.insert("userId".to_string(), json!(user_id));
        if let Some(p) = page { variables_map.insert("page".to_string(), json!(p)); }
        if let Some(pp) = per_page { variables_map.insert("perPage".to_string(), json!(pp)); }
        if let Some(sort_vec) = sort { variables_map.insert("sort".to_string(), json!(sort_vec)); }

        self.0.query_typed(query, Some(&variables_map)).await
    }

    /// Get user's favorite characters with pagination
    pub async fn get_favorites_characters(
        &self,
        user_id: i32,
        page: Option<i32>,
        per_page: Option<i32>,
        sort: Option<Vec<CharacterSort>>,
    ) -> Result<serde_json::Value, AniListError> {
        let query = include_str!("../queries/user/get_user_favorites_characters.graphql");
        let mut variables_map = HashMap::new();
        variables_map.insert("userId".to_string(), json!(user_id));
        if let Some(p) = page { variables_map.insert("page".to_string(), json!(p)); }
        if let Some(pp) = per_page { variables_map.insert("perPage".to_string(), json!(pp)); }
        if let Some(sort_vec) = sort { variables_map.insert("sort".to_string(), json!(sort_vec)); }

        self.0.query(query, Some(&variables_map)).await
    }

    /// Get user's favorite staff with pagination
    pub async fn get_favorites_staff(
        &self,
        user_id: i32,
        page: Option<i32>,
        per_page: Option<i32>,
        sort: Option<Vec<StaffSort>>,
    ) -> Result<serde_json::Value, AniListError> {
        let query = include_str!("../queries/user/get_user_favorites_staff.graphql");
        let mut variables_map = HashMap::new();
        variables_map.insert("userId".to_string(), json!(user_id));
        if let Some(p) = page { variables_map.insert("page".to_string(), json!(p)); }
        if let Some(pp) = per_page { variables_map.insert("perPage".to_string(), json!(pp)); }
        if let Some(sort_vec) = sort { variables_map.insert("sort".to_string(), json!(sort_vec)); }

        self.0.query(query, Some(&variables_map)).await
    }

    /// Get user's favorite studios with pagination
    pub async fn get_favorites_studios(
        &self,
        user_id: i32,
        page: Option<i32>,
        per_page: Option<i32>,
        sort: Option<Vec<StudioSort>>,
    ) -> Result<serde_json::Value, AniListError> {
        let query = include_str!("../queries/user/get_user_favorites_studios.graphql");
        let mut variables_map = HashMap::new();
        variables_map.insert("userId".to_string(), json!(user_id));
        if let Some(p) = page { variables_map.insert("page".to_string(), json!(p)); }
        if let Some(pp) = per_page { variables_map.insert("perPage".to_string(), json!(pp)); }
        if let Some(sort_vec) = sort { variables_map.insert("sort".to_string(), json!(sort_vec)); }

        self.0.query(query, Some(&variables_map)).await
    }

    /// Get user's activities with pagination and filtering
    pub async fn get_activities(
        &self,
        user_id: i32,
        page: Option<i32>,
        per_page: Option<i32>,
        sort: Option<Vec<ActivitySort>>,
        activity_type: Option<ActivityType>,
        has_replies: Option<bool>,
    ) -> Result<ActivityListResponse, AniListError> {
        let query = include_str!("../queries/user/get_user_activities.graphql");
        let mut variables_map = HashMap::new();
        variables_map.insert("userId".to_string(), json!(user_id));
        if let Some(p) = page { variables_map.insert("page".to_string(), json!(p)); }
        if let Some(pp) = per_page { variables_map.insert("perPage".to_string(), json!(pp)); }
        if let Some(sort_vec) = sort { variables_map.insert("sort".to_string(), json!(sort_vec)); }
        if let Some(at) = activity_type { variables_map.insert("type".to_string(), json!(at)); }
        if let Some(hr) = has_replies { variables_map.insert("hasReplies".to_string(), json!(hr)); }

        self.0.query_typed(query, Some(&variables_map)).await
    }

    /// Get user's statistics
    pub async fn get_stats(&self, user_id: i32) -> Result<serde_json::Value, AniListError> {
        let query = include_str!("../queries/user/get_user_stats.graphql");
        let mut variables_map = HashMap::new();
        variables_map.insert("userId".to_string(), json!(user_id));

        self.0.query(query, Some(&variables_map)).await
    }
}
