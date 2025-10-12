use crate::enums::media::{MediaSort, MediaType};
use crate::enums::media_list::MediaListStatus;
use crate::enums::user::{UserSort, UserStatisticsSort};
use crate::errors::AniListError;
use crate::objects::responses::{GraphQLResponse, Page, ViewerUserData};
use crate::objects::user::User;
use crate::{client::AniListClient, queries::user};
use serde::Serialize;
use serde_json::json;
use serde_with::skip_serializing_none;

/// Options for fetching users.
#[skip_serializing_none]
#[derive(Default, Debug, Serialize)]
pub struct FetchUserOptions {
    pub id: Option<i32>,
    pub name: Option<String>,
    pub search: Option<String>,
    pub sort: Option<Vec<UserSort>>,
    #[serde(rename = "isModerator")]
    pub is_moderator: Option<bool>,
    pub page: Option<i32>,
    #[serde(rename = "perPage")]
    pub per_page: Option<i32>,
}

/// Options for fetching a single user by ID or name.
#[skip_serializing_none]
#[derive(Default, Debug, Serialize)]
pub struct FetchUserOneOptions {
    pub id: Option<i32>,
    pub name: Option<String>,
}

/// Options for fetching a user's followers.
#[skip_serializing_none]
#[derive(Default, Debug, Serialize)]
pub struct FetchUserFollowersOptions {
    #[serde(rename = "userId")]
    pub user_id: i32,
    pub page: Option<i32>,
    #[serde(rename = "perPage")]
    pub per_page: Option<i32>,
}

/// Options for fetching users a user is following.
#[skip_serializing_none]
#[derive(Default, Debug, Serialize)]
pub struct FetchUserFollowingOptions {
    #[serde(rename = "userId")]
    pub user_id: i32,
    pub page: Option<i32>,
    #[serde(rename = "perPage")]
    pub per_page: Option<i32>,
}

/// Options for fetching a user's favorites.
#[skip_serializing_none]
#[derive(Default, Debug, Serialize)]
pub struct FetchUserFavoritesOptions {
    #[serde(rename = "userId")]
    pub user_id: i32,
    // Toggle what to fetch
    #[serde(rename = "fetchAnime")]
    pub fetch_anime: Option<bool>,
    #[serde(rename = "fetchManga")]
    pub fetch_manga: Option<bool>,
    #[serde(rename = "fetchCharacters")]
    pub fetch_characters: Option<bool>,
    #[serde(rename = "fetchStaff")]
    pub fetch_staff: Option<bool>,
    #[serde(rename = "fetchStudios")]
    pub fetch_studios: Option<bool>,
    // Anime pagination
    #[serde(rename = "animePage")]
    pub anime_page: Option<i32>,
    #[serde(rename = "animePerPage")]
    pub anime_per_page: Option<i32>,
    #[serde(rename = "animeSort")]
    pub anime_sort: Option<Vec<MediaSort>>,
    // Manga pagination
    #[serde(rename = "mangaPage")]
    pub manga_page: Option<i32>,
    #[serde(rename = "mangaPerPage")]
    pub manga_per_page: Option<i32>,
    #[serde(rename = "mangaSort")]
    pub manga_sort: Option<Vec<MediaSort>>,
    // Characters pagination
    #[serde(rename = "charactersPage")]
    pub characters_page: Option<i32>,
    #[serde(rename = "charactersPerPage")]
    pub characters_per_page: Option<i32>,
    // Staff pagination
    #[serde(rename = "staffPage")]
    pub staff_page: Option<i32>,
    #[serde(rename = "staffPerPage")]
    pub staff_per_page: Option<i32>,
    // Studios pagination
    #[serde(rename = "studiosPage")]
    pub studios_page: Option<i32>,
    #[serde(rename = "studiosPerPage")]
    pub studios_per_page: Option<i32>,
}

/// Options for fetching a user's media list.
#[skip_serializing_none]
#[derive(Default, Debug, Serialize)]
pub struct FetchUserMediaListOptions {
    #[serde(rename = "userId")]
    pub user_id: Option<i32>,
    pub username: Option<String>,
    #[serde(rename = "type")]
    pub media_type: Option<MediaType>,
    pub status: Option<MediaListStatus>,
    pub notes: Option<String>,
    #[serde(rename = "startedAt")]
    pub started_at: Option<i32>,
    #[serde(rename = "completedAt")]
    pub completed_at: Option<i32>,
    pub sort: Option<Vec<MediaSort>>,
    pub page: Option<i32>,
    #[serde(rename = "perPage")]
    pub per_page: Option<i32>,
}

/// Options for fetching user statistics.
#[skip_serializing_none]
#[derive(Default, Debug, Serialize)]
pub struct FetchUserStatsOptions {
    #[serde(rename = "userId")]
    pub user_id: i32,
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
    pub async fn fetch(&self, options: FetchUserOptions) -> Result<Page<Vec<User>>, AniListError> {
        let query = user::FETCH;
        let variables = json!(options);
        let variables_map = crate::utils::json_to_hashmap(variables);
        let response: Result<GraphQLResponse<Page<Vec<User>>>, AniListError> = self.client.query_typed(query, Some(&variables_map)).await;
        match response {
            Ok(res) => Ok(res.data),
            Err(e) => Err(e),
        }
    }

    /// Fetch a single user with full details
    pub async fn fetch_one(
        &self,
        options: &FetchUserOneOptions,
    ) -> Result<User, AniListError> {
        let query = user::FETCH_ONE;
        let variables = json!(options);
        let variables_map = crate::utils::json_to_hashmap(variables);
        let response: Result<GraphQLResponse<User>, AniListError> = self.client.query_typed(query, Some(&variables_map)).await;
        match response {
            Ok(res) => Ok(res.data),
            Err(e) => Err(e),
        }
    }

    /// Fetch basic user information
    pub async fn fetch_basic(&self) -> Result<ViewerUserData, AniListError> {
        let query = user::BASIC;
        let response: Result<GraphQLResponse<ViewerUserData>, AniListError> = self.client.query_typed(query, None).await;
        match response {
            Ok(res) => Ok(res.data),
            Err(e) => Err(e),
        }
    }

    /// Fetch user followers
    pub async fn followers(
        &self,
        options: &FetchUserFollowersOptions,
    ) -> Result<Page<Vec<User>>, AniListError> {
        let query = user::FOLLOWERS;
        let variables = json!(options);
        let variables_map = crate::utils::json_to_hashmap(variables);
        let response: Result<GraphQLResponse<Page<Vec<User>>>, AniListError> = self.client.query_typed(query, Some(&variables_map)).await;
        match response {
            Ok(res) => Ok(res.data),
            Err(e) => Err(e),
        }
    }

    /// Fetch users that the user is following
    pub async fn following(
        &self,
        options: &FetchUserFollowingOptions,
    ) -> Result<Page<Vec<User>>, AniListError> {
        let query = user::FOLLOWING;
        let variables = json!(options);
        let variables_map = crate::utils::json_to_hashmap(variables);
        let response: Result<GraphQLResponse<Page<Vec<User>>>, AniListError> = self.client.query_typed(query, Some(&variables_map)).await;
        match response {
            Ok(res) => Ok(res.data),
            Err(e) => Err(e),
        }
    }

    /// Fetch user favorites with conditional sections and independent pagination
    pub async fn favorites(
        &self,
        options: &FetchUserFavoritesOptions,
    ) -> Result<Page<Vec<User>>, AniListError> {
        let query = user::FAVORITES;
        let variables = json!(options);
        let variables_map = crate::utils::json_to_hashmap(variables);
        let response: Result<GraphQLResponse<Page<Vec<User>>>, AniListError> = self.client.query_typed(query, Some(&variables_map)).await;
        match response {
            Ok(res) => Ok(res.data),
            Err(e) => Err(e),
        }
    }

    /// Fetch user's media list
    pub async fn media_list(
        &self,
        options: &FetchUserMediaListOptions,
    ) -> Result<Page<Vec<User>>, AniListError> {
        let query = user::MEDIA_LIST;
        let variables = json!(options);
        let variables_map = crate::utils::json_to_hashmap(variables);
        let response: Result<GraphQLResponse<Page<Vec<User>>>, AniListError> = self.client.query_typed(query, Some(&variables_map)).await;
        match response {
            Ok(res) => Ok(res.data),
            Err(e) => Err(e),
        }
    }

    /// Fetch user statistics
    pub async fn stats(
        &self,
        options: &FetchUserStatsOptions,
    ) -> Result<User, AniListError> {
        let query = user::STATS;
        let variables = json!(options);
        let variables_map = crate::utils::json_to_hashmap(variables);
        let response: Result<GraphQLResponse<User>, AniListError> = self.client.query_typed(query, Some(&variables_map)).await;
        match response {
            Ok(res) => Ok(res.data),
            Err(e) => Err(e),
        }
    }

    // Convenience functions

    /// Get current authenticated user
    pub async fn get_current_user(&self) -> Result<User, AniListError> {
        let response = self.client.user().fetch_basic().await?;
        let id = response.id;
        self.fetch_one(&FetchUserOneOptions {
            id: Some(id),
            ..Default::default()
        })
        .await
    }

    /// Get user by ID
    pub async fn get_by_id(&self, id: i32) -> Result<User, AniListError> {
        self.fetch_one(&FetchUserOneOptions {
            id: Some(id),
            ..Default::default()
        })
        .await
    }

    /// Get user by username
    pub async fn get_by_name(&self, name: &str) -> Result<User, AniListError> {
        self.fetch_one(&FetchUserOneOptions {
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
    ) -> Result<Page<Vec<User>>, AniListError> {
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
    ) -> Result<Page<Vec<User>>, AniListError> {
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
    ) -> Result<Page<Vec<User>>, AniListError> {
        self.fetch(FetchUserOptions {
            page,
            per_page,
            sort: Some(vec![UserSort::ChaptersReadDesc]),
            ..Default::default()
        })
        .await
    }
}
