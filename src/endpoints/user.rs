use crate::enums::media::{MediaSort, MediaType};
use crate::enums::media_list::MediaListStatus;
use crate::enums::user::{UserSort, UserStatisticsSort};
use crate::errors::AniListError;
use crate::objects::responses::Page;
use crate::objects::user::User;
use crate::options;
#[cfg(feature = "pagination")]
use crate::pagination::Paginated;
use crate::{client::AniListClient, queries::user};
use serde::Serialize;
use serde_with::skip_serializing_none;
use std::borrow::Cow;

/// Options for fetching users.
#[skip_serializing_none]
#[derive(Default, Debug, Serialize)]
pub struct FetchUserOptions<'a> {
    pub id: Option<i32>,
    pub name: Option<Cow<'a, str>>,
    pub search: Option<Cow<'a, str>>,
    /// Sort order. The unique key (id) is appended, so paging stays stable;
    /// leaving this unset sorts by the same key alone.
    #[serde(
        serialize_with = "crate::options::serialize_sort",
        skip_serializing_if = "crate::options::skip_never"
    )]
    pub sort: Option<Cow<'a, [UserSort]>>,
    #[serde(rename = "isModerator")]
    pub is_moderator: Option<bool>,
    pub page: Option<i32>,
    #[serde(rename = "perPage")]
    pub per_page: Option<i32>,
    // Extra
    #[serde(rename = "includeFavourites")]
    pub include_favourites: Option<bool>,
    #[serde(rename = "includeStatistics")]
    pub include_statistics: Option<bool>,
    // HTML rendering options
    #[serde(rename = "about_as_html")]
    pub about_as_html: Option<bool>,
}

/// Options for fetching a single user by ID or name.
#[skip_serializing_none]
#[derive(Default, Debug, Serialize)]
pub struct FetchUserOneOptions<'a> {
    pub id: Option<i32>,
    pub name: Option<Cow<'a, str>>,
    // HTML rendering options
    #[serde(rename = "about_as_html")]
    pub about_as_html: Option<bool>,
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
pub struct FetchUserFavoritesOptions<'a> {
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
    pub anime_sort: Option<Cow<'a, [MediaSort]>>,
    // Manga pagination
    #[serde(rename = "mangaPage")]
    pub manga_page: Option<i32>,
    #[serde(rename = "mangaPerPage")]
    pub manga_per_page: Option<i32>,
    #[serde(rename = "mangaSort")]
    pub manga_sort: Option<Cow<'a, [MediaSort]>>,
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
pub struct FetchUserMediaListOptions<'a> {
    #[serde(rename = "userId")]
    pub user_id: Option<i32>,
    pub username: Option<Cow<'a, str>>,
    #[serde(rename = "type")]
    pub media_type: Option<MediaType>,
    pub status: Option<MediaListStatus>,
    pub notes: Option<Cow<'a, str>>,
    #[serde(rename = "startedAt")]
    pub started_at: Option<i32>,
    #[serde(rename = "completedAt")]
    pub completed_at: Option<i32>,
    /// Sort order. The unique key (id) is appended, so paging stays stable;
    /// leaving this unset sorts by the same key alone.
    #[serde(
        serialize_with = "crate::options::serialize_sort",
        skip_serializing_if = "crate::options::skip_never"
    )]
    pub sort: Option<Cow<'a, [MediaSort]>>,
    pub page: Option<i32>,
    #[serde(rename = "perPage")]
    pub per_page: Option<i32>,
}

/// Options for fetching user statistics.
#[skip_serializing_none]
#[derive(Default, Debug, Serialize)]
pub struct FetchUserStatsOptions<'a> {
    #[serde(rename = "userId")]
    pub user_id: i32,
    /// Sort order. The unique key (id) is appended, so paging stays stable;
    /// leaving this unset sorts by the same key alone.
    #[serde(
        serialize_with = "crate::options::serialize_sort",
        skip_serializing_if = "crate::options::skip_never"
    )]
    pub sort: Option<Cow<'a, [UserStatisticsSort]>>,
}

/// Endpoint for user profile and statistics operations.
pub struct UserEndpoint<'a> {
    client: &'a AniListClient,
}

impl<'a> UserEndpoint<'a> {
    pub fn new(client: &'a AniListClient) -> Self {
        Self { client }
    }

    /// Fetch multiple users with pagination
    pub async fn fetch(
        &self,
        options: &FetchUserOptions<'_>,
    ) -> Result<Page<Vec<User>>, AniListError> {
        let query = user::FETCH;
        self.client.fetch(query, Some(&options)).await
    }

    /// Fetch a single user with full details
    pub async fn fetch_one(&self, options: &FetchUserOneOptions<'_>) -> Result<User, AniListError> {
        let query = user::FETCH_ONE;
        self.client.fetch(query, Some(&options)).await
    }

    /// Fetch basic user information
    pub async fn fetch_basic(&self) -> Result<User, AniListError> {
        let query = user::BASIC;
        self.client.fetch(query, Option::<&()>::None).await
    }

    /// Fetch user followers
    pub async fn followers(
        &self,
        options: &FetchUserFollowersOptions,
    ) -> Result<Page<Vec<User>>, AniListError> {
        let query = user::FOLLOWERS;
        self.client.fetch(query, Some(&options)).await
    }

    /// Fetch users that the user is following
    pub async fn following(
        &self,
        options: &FetchUserFollowingOptions,
    ) -> Result<Page<Vec<User>>, AniListError> {
        let query = user::FOLLOWING;
        self.client.fetch(query, Some(&options)).await
    }

    /// Fetch user favorites with conditional sections and independent pagination
    pub async fn favorites(
        &self,
        options: &FetchUserFavoritesOptions<'_>,
    ) -> Result<Page<Vec<User>>, AniListError> {
        let query = user::FAVORITES;
        self.client.fetch(query, Some(&options)).await
    }

    /// Fetch user's media list
    pub async fn media_list(
        &self,
        options: &FetchUserMediaListOptions<'_>,
    ) -> Result<Page<Vec<User>>, AniListError> {
        let query = user::MEDIA_LIST;
        self.client.fetch(query, Some(&options)).await
    }

    /// Fetch user statistics
    pub async fn stats(&self, options: &FetchUserStatsOptions<'_>) -> Result<User, AniListError> {
        let query = user::STATS;
        self.client.fetch(query, Some(&options)).await
    }

    // Convenience functions

    /// Get current authenticated user
    pub async fn get_current_user(&self) -> Result<User, AniListError> {
        let response = self.client.user().fetch_basic().await?;
        let id = response.id;
        self.fetch_one(&options!(
            FetchUserOneOptions,
            id: Some(id),
        ))
        .await
    }

    /// Get user by ID
    pub async fn get_by_id(&self, id: i32) -> Result<User, AniListError> {
        self.fetch_one(&options!(
            FetchUserOneOptions,
            id: Some(id),
        ))
        .await
    }

    /// Get user by username
    pub async fn get_by_name(&self, name: &str) -> Result<User, AniListError> {
        self.fetch_one(&options!(
            FetchUserOneOptions,
            name: Some(name),
        ))
        .await
    }

    /// Search users by query
    pub async fn search(
        &self,
        query: &str,
        page: Option<i32>,
        per_page: Option<i32>,
    ) -> Result<Page<Vec<User>>, AniListError> {
        self.fetch(&options!(
            FetchUserOptions,
            search: Some(query),
            page,
            per_page,
            sort: Some(&[UserSort::SearchMatch]),
        ))
        .await
    }

    /// Get users with most anime watched
    pub async fn get_most_anime_watched(
        &self,
        page: Option<i32>,
        per_page: Option<i32>,
    ) -> Result<Page<Vec<User>>, AniListError> {
        self.fetch(&options!(
            FetchUserOptions,
            page,
            per_page,
            sort: Some(&[UserSort::WatchedTimeDesc]),
        ))
        .await
    }

    /// Get users with most manga read
    pub async fn get_most_manga_read(
        &self,
        page: Option<i32>,
        per_page: Option<i32>,
    ) -> Result<Page<Vec<User>>, AniListError> {
        self.fetch(&options!(
            FetchUserOptions,
            page,
            per_page,
            sort: Some(&[UserSort::ChaptersReadDesc]),
        ))
        .await
    }
}

#[cfg(feature = "pagination")]
impl<'a> Paginated for FetchUserOptions<'a> {
    type Item = User;

    async fn fetch(&self, client: &AniListClient) -> Result<Page<Vec<User>>, AniListError> {
        client.user().fetch(self).await
    }

    fn set_page(&mut self, page: i32) {
        self.page = Some(page);
    }
}

#[cfg(feature = "pagination")]
impl Paginated for FetchUserFollowersOptions {
    type Item = User;

    async fn fetch(&self, client: &AniListClient) -> Result<Page<Vec<User>>, AniListError> {
        client.user().followers(self).await
    }

    fn set_page(&mut self, page: i32) {
        self.page = Some(page);
    }
}

#[cfg(feature = "pagination")]
impl Paginated for FetchUserFollowingOptions {
    type Item = User;

    async fn fetch(&self, client: &AniListClient) -> Result<Page<Vec<User>>, AniListError> {
        client.user().following(self).await
    }

    fn set_page(&mut self, page: i32) {
        self.page = Some(page);
    }
}

#[cfg(feature = "pagination")]
impl<'a> Paginated for FetchUserMediaListOptions<'a> {
    type Item = User;

    async fn fetch(&self, client: &AniListClient) -> Result<Page<Vec<User>>, AniListError> {
        client.user().media_list(self).await
    }

    fn set_page(&mut self, page: i32) {
        self.page = Some(page);
    }
}
