use crate::enums::character::CharacterSort;
use crate::enums::media::{MediaSort, MediaType};
use crate::enums::media_list::MediaListStatus;
use crate::enums::staff::StaffSort;
use crate::enums::studio::StudioSort;
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
#[serde(rename_all = "camelCase")]
pub struct FetchUserOptions<'a> {
    pub id: Option<u32>,
    pub name: Option<Cow<'a, str>>,
    pub search: Option<Cow<'a, str>>,
    /// Sort order. The unique key (id) is appended, so paging stays stable;
    /// leaving this unset sorts by the same key alone.
    #[serde(
        serialize_with = "crate::options::serialize_sort",
        skip_serializing_if = "crate::options::skip_never"
    )]
    pub sort: Option<Cow<'a, [UserSort]>>,
    pub is_moderator: Option<bool>,
    pub page: Option<i32>,
    pub per_page: Option<i32>,
    // Extra
    pub include_favourites: Option<bool>,
    pub include_statistics: Option<bool>,
    // HTML rendering options
    pub about_as_html: Option<bool>,
}

/// Options for fetching a single user by ID or name.
#[skip_serializing_none]
#[derive(Default, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FetchUserOneOptions<'a> {
    pub id: Option<u32>,
    pub name: Option<Cow<'a, str>>,
    pub search: Option<Cow<'a, str>>,
    pub is_moderator: Option<bool>,
    pub sort: Option<Cow<'a, [UserSort]>>,
    // HTML rendering options
    pub about_as_html: Option<bool>,
}

/// Options for fetching a user's followers.
#[skip_serializing_none]
#[derive(Default, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FetchUserFollowersOptions<'a> {
    pub user_id: u32,
    pub sort: Option<Cow<'a, [UserSort]>>,
    pub page: Option<i32>,
    pub per_page: Option<i32>,
}

/// Options for fetching users a user is following.
#[skip_serializing_none]
#[derive(Default, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FetchUserFollowingOptions<'a> {
    pub user_id: u32,
    pub sort: Option<Cow<'a, [UserSort]>>,
    pub page: Option<i32>,
    pub per_page: Option<i32>,
}

/// Options for fetching a user's favorites.
#[skip_serializing_none]
#[derive(Default, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FetchUserFavoritesOptions<'a> {
    pub user_id: u32,
    // Toggle what to fetch
    pub fetch_anime: Option<bool>,
    pub fetch_manga: Option<bool>,
    pub fetch_characters: Option<bool>,
    pub fetch_staff: Option<bool>,
    pub fetch_studios: Option<bool>,
    // Anime pagination
    pub anime_page: Option<i32>,
    pub anime_per_page: Option<i32>,
    pub anime_sort: Option<Cow<'a, [MediaSort]>>,
    // Manga pagination
    pub manga_page: Option<i32>,
    pub manga_per_page: Option<i32>,
    pub manga_sort: Option<Cow<'a, [MediaSort]>>,
    // Characters pagination
    pub characters_page: Option<i32>,
    pub characters_per_page: Option<i32>,
    // Staff pagination
    pub staff_page: Option<i32>,
    pub staff_per_page: Option<i32>,
    // Studios pagination
    pub studios_page: Option<i32>,
    pub studios_per_page: Option<i32>,
    pub characters_sort: Option<Cow<'a, [CharacterSort]>>,
    pub staff_sort: Option<Cow<'a, [StaffSort]>>,
    pub studios_sort: Option<Cow<'a, [StudioSort]>>,
}

/// Options for fetching a user's media list.
#[skip_serializing_none]
#[derive(Default, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FetchUserMediaListOptions<'a> {
    pub id: Option<u32>,
    pub user_id: Option<u32>,
    pub username: Option<Cow<'a, str>>,
    pub media_type: Option<MediaType>,
    pub status: Option<MediaListStatus>,
    pub status_in: Option<Cow<'a, [MediaListStatus]>>,
    pub status_not: Option<MediaListStatus>,
    pub status_not_in: Option<Cow<'a, [MediaListStatus]>>,
    pub media_id: Option<u32>,
    pub media_id_in: Option<Cow<'a, [u32]>>,
    pub media_id_not_in: Option<Cow<'a, [u32]>>,
    pub user_id_in: Option<Cow<'a, [u32]>>,
    pub is_following: Option<bool>,
    pub compare_with_auth_list: Option<bool>,
    pub started_at_exact: Option<i32>,
    pub started_at_greater: Option<i32>,
    pub started_at_lesser: Option<i32>,
    pub completed_at_exact: Option<i32>,
    pub completed_at_greater: Option<i32>,
    pub completed_at_lesser: Option<i32>,
    pub notes: Option<Cow<'a, str>>,
    pub started_at: Option<Cow<'a, str>>,
    pub completed_at: Option<Cow<'a, str>>,
    /// Sort order. The unique key (id) is appended, so paging stays stable;
    /// leaving this unset sorts by the same key alone.
    #[serde(
        serialize_with = "crate::options::serialize_sort",
        skip_serializing_if = "crate::options::skip_never"
    )]
    pub sort: Option<Cow<'a, [MediaSort]>>,
    pub page: Option<i32>,
    pub per_page: Option<i32>,
}

/// Options for fetching user statistics.
#[skip_serializing_none]
#[derive(Default, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FetchUserStatsOptions<'a> {
    pub user_id: u32,
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
        options: &FetchUserFollowersOptions<'_>,
    ) -> Result<Page<Vec<User>>, AniListError> {
        let query = user::FOLLOWERS;
        self.client.fetch(query, Some(&options)).await
    }

    /// Fetch users that the user is following
    pub async fn following(
        &self,
        options: &FetchUserFollowingOptions<'_>,
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
    pub async fn get_by_id(&self, id: u32) -> Result<User, AniListError> {
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
impl Paginated for FetchUserFollowersOptions<'_> {
    type Item = User;

    async fn fetch(&self, client: &AniListClient) -> Result<Page<Vec<User>>, AniListError> {
        client.user().followers(self).await
    }

    fn set_page(&mut self, page: i32) {
        self.page = Some(page);
    }
}

#[cfg(feature = "pagination")]
impl Paginated for FetchUserFollowingOptions<'_> {
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
