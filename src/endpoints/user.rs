use crate::enums::media::{MediaSort, MediaType};
use crate::enums::media_list::MediaListStatus;
use crate::enums::user::{UserSort, UserStatisticsSort};
use crate::errors::AniListError;
use crate::objects::responses::{UserListResponse, UserSingleResponse};
use crate::{client::AniListClient, queries::user};
use serde::Serialize;
use serde_json::json;

/// Options for fetching users.
#[derive(Default, Serialize)]
pub struct FetchUserOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<Vec<UserSort>>,
    #[serde(rename = "isModerator", skip_serializing_if = "Option::is_none")]
    pub is_moderator: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i32>,
    #[serde(rename = "perPage", skip_serializing_if = "Option::is_none")]
    pub per_page: Option<i32>,
}

/// Options for fetching a single user by ID or name.
#[derive(Default, Serialize)]
pub struct FetchUserOneOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// Options for fetching basic user information.
#[derive(Default, Serialize)]
pub struct FetchUserBasicOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// Options for fetching a user's followers.
#[derive(Default, Serialize)]
pub struct FetchUserFollowersOptions {
    #[serde(rename = "userId")]
    pub user_id: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i32>,
    #[serde(rename = "perPage", skip_serializing_if = "Option::is_none")]
    pub per_page: Option<i32>,
}

/// Options for fetching users a user is following.
#[derive(Default, Serialize)]
pub struct FetchUserFollowingOptions {
    #[serde(rename = "userId")]
    pub user_id: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i32>,
    #[serde(rename = "perPage", skip_serializing_if = "Option::is_none")]
    pub per_page: Option<i32>,
}

/// Options for fetching a user's favorites.
#[derive(Default, Serialize)]
pub struct FetchUserFavoritesOptions {
    #[serde(rename = "userId")]
    pub user_id: i32,
    // Toggle what to fetch
    #[serde(rename = "fetchAnime", skip_serializing_if = "Option::is_none")]
    pub fetch_anime: Option<bool>,
    #[serde(rename = "fetchManga", skip_serializing_if = "Option::is_none")]
    pub fetch_manga: Option<bool>,
    #[serde(rename = "fetchCharacters", skip_serializing_if = "Option::is_none")]
    pub fetch_characters: Option<bool>,
    #[serde(rename = "fetchStaff", skip_serializing_if = "Option::is_none")]
    pub fetch_staff: Option<bool>,
    #[serde(rename = "fetchStudios", skip_serializing_if = "Option::is_none")]
    pub fetch_studios: Option<bool>,
    // Anime pagination
    #[serde(rename = "animePage", skip_serializing_if = "Option::is_none")]
    pub anime_page: Option<i32>,
    #[serde(rename = "animePerPage", skip_serializing_if = "Option::is_none")]
    pub anime_per_page: Option<i32>,
    #[serde(rename = "animeSort", skip_serializing_if = "Option::is_none")]
    pub anime_sort: Option<Vec<MediaSort>>,
    // Manga pagination
    #[serde(rename = "mangaPage", skip_serializing_if = "Option::is_none")]
    pub manga_page: Option<i32>,
    #[serde(rename = "mangaPerPage", skip_serializing_if = "Option::is_none")]
    pub manga_per_page: Option<i32>,
    #[serde(rename = "mangaSort", skip_serializing_if = "Option::is_none")]
    pub manga_sort: Option<Vec<MediaSort>>,
    // Characters pagination
    #[serde(rename = "charactersPage", skip_serializing_if = "Option::is_none")]
    pub characters_page: Option<i32>,
    #[serde(rename = "charactersPerPage", skip_serializing_if = "Option::is_none")]
    pub characters_per_page: Option<i32>,
    // Staff pagination
    #[serde(rename = "staffPage", skip_serializing_if = "Option::is_none")]
    pub staff_page: Option<i32>,
    #[serde(rename = "staffPerPage", skip_serializing_if = "Option::is_none")]
    pub staff_per_page: Option<i32>,
    // Studios pagination
    #[serde(rename = "studiosPage", skip_serializing_if = "Option::is_none")]
    pub studios_page: Option<i32>,
    #[serde(rename = "studiosPerPage", skip_serializing_if = "Option::is_none")]
    pub studios_per_page: Option<i32>,
}

/// Options for fetching a user's media list.
#[derive(Default, Serialize)]
pub struct FetchUserMediaListOptions {
    #[serde(rename = "userId", skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub media_type: Option<MediaType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<MediaListStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    #[serde(rename = "startedAt", skip_serializing_if = "Option::is_none")]
    pub started_at: Option<i32>,
    #[serde(rename = "completedAt", skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<Vec<MediaSort>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i32>,
    #[serde(rename = "perPage", skip_serializing_if = "Option::is_none")]
    pub per_page: Option<i32>,
}

/// Options for fetching user statistics.
#[derive(Default, Serialize)]
pub struct FetchUserStatsOptions {
    #[serde(rename = "userId")]
    pub user_id: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<Vec<UserStatisticsSort>>,
}

/// Endpoint for user profile and statistics operations.
pub struct UserEndpoint {
    client: AniListClient,
}

impl UserEndpoint {
    pub fn new(client: AniListClient) -> Self {
        Self { client }
    }

    /// Fetch multiple users with pagination
    pub async fn fetch(&self, options: FetchUserOptions) -> Result<UserListResponse, AniListError> {
        let query = user::FETCH;
        let variables = json!(options);
        let variables_map = crate::utils::json_to_hashmap(variables);
        self.client.query_typed(query, Some(&variables_map)).await
    }

    /// Fetch a single user with full details
    pub async fn fetch_one(
        &self,
        options: FetchUserOneOptions,
    ) -> Result<UserSingleResponse, AniListError> {
        let query = user::FETCH_ONE;
        let variables = json!(options);
        let variables_map = crate::utils::json_to_hashmap(variables);
        self.client.query_typed(query, Some(&variables_map)).await
    }

    /// Fetch basic user information
    pub async fn fetch_basic(
        &self,
        options: FetchUserBasicOptions,
    ) -> Result<UserSingleResponse, AniListError> {
        let query = user::BASIC;
        let variables = json!(options);
        let variables_map = crate::utils::json_to_hashmap(variables);
        self.client.query_typed(query, Some(&variables_map)).await
    }

    /// Fetch user followers
    pub async fn followers(
        &self,
        options: FetchUserFollowersOptions,
    ) -> Result<UserListResponse, AniListError> {
        let query = user::FOLLOWERS;
        let variables = json!(options);
        let variables_map = crate::utils::json_to_hashmap(variables);
        self.client.query_typed(query, Some(&variables_map)).await
    }

    /// Fetch users that the user is following
    pub async fn following(
        &self,
        options: FetchUserFollowingOptions,
    ) -> Result<UserListResponse, AniListError> {
        let query = user::FOLLOWING;
        let variables = json!(options);
        let variables_map = crate::utils::json_to_hashmap(variables);
        self.client.query_typed(query, Some(&variables_map)).await
    }

    /// Fetch user favorites with conditional sections and independent pagination
    pub async fn favorites(
        &self,
        options: FetchUserFavoritesOptions,
    ) -> Result<UserSingleResponse, AniListError> {
        let query = user::FAVORITES;
        let variables = json!(options);
        let variables_map = crate::utils::json_to_hashmap(variables);
        self.client.query_typed(query, Some(&variables_map)).await
    }

    /// Fetch user's media list
    pub async fn media_list(
        &self,
        options: FetchUserMediaListOptions,
    ) -> Result<UserListResponse, AniListError> {
        let query = user::MEDIA_LIST;
        let variables = json!(options);
        let variables_map = crate::utils::json_to_hashmap(variables);
        self.client.query_typed(query, Some(&variables_map)).await
    }

    /// Fetch user statistics
    pub async fn stats(
        &self,
        options: FetchUserStatsOptions,
    ) -> Result<UserSingleResponse, AniListError> {
        let query = user::STATS;
        let variables = json!(options);
        let variables_map = crate::utils::json_to_hashmap(variables);
        self.client.query_typed(query, Some(&variables_map)).await
    }

    // Convenience functions

    /// Get current authenticated user
    pub async fn get_current_user(&self) -> Result<UserSingleResponse, AniListError> {
        self.fetch_one(FetchUserOneOptions {
            ..Default::default()
        })
        .await
    }

    /// Get user by ID
    pub async fn get_by_id(&self, id: i32) -> Result<UserSingleResponse, AniListError> {
        self.fetch_one(FetchUserOneOptions {
            id: Some(id),
            ..Default::default()
        })
        .await
    }

    /// Get user by username
    pub async fn get_by_name(&self, name: &str) -> Result<UserSingleResponse, AniListError> {
        self.fetch_one(FetchUserOneOptions {
            name: Some(name.to_string()),
            ..Default::default()
        })
        .await
    }

    /// Search users by query
    pub async fn search(
        &self,
        query: &str,
        page: Option<i32>,
        per_page: Option<i32>,
    ) -> Result<UserListResponse, AniListError> {
        self.fetch(FetchUserOptions {
            search: Some(query.to_string()),
            page,
            per_page,
            sort: Some(vec![UserSort::SearchMatch]),
            ..Default::default()
        })
        .await
    }

    /// Get users with most anime watched
    pub async fn get_most_anime_watched(
        &self,
        page: Option<i32>,
        per_page: Option<i32>,
    ) -> Result<UserListResponse, AniListError> {
        self.fetch(FetchUserOptions {
            page,
            per_page,
            sort: Some(vec![UserSort::WatchedTimeDesc]),
            ..Default::default()
        })
        .await
    }

    /// Get users with most manga read
    pub async fn get_most_manga_read(
        &self,
        page: Option<i32>,
        per_page: Option<i32>,
    ) -> Result<UserListResponse, AniListError> {
        self.fetch(FetchUserOptions {
            page,
            per_page,
            sort: Some(vec![UserSort::ChaptersReadDesc]),
            ..Default::default()
        })
        .await
    }
}
