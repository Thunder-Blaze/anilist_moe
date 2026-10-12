use crate::enums::character::CharacterSort;
use crate::enums::media::{MediaSort, MediaType};
use crate::enums::media_list::MediaListStatus;
use crate::enums::staff::StaffSort;
use crate::enums::studio::StudioSort;
use crate::enums::user::{UserSort, UserStatisticsSort};
use crate::errors::AniListError;
use crate::objects::responses::Page;
use crate::objects::user::User;
#[cfg(feature = "pagination")]
use crate::pagination::Paginated;
use crate::{client::AniListClient, queries::user};
use serde::Serialize;
use serde_with::skip_serializing_none;
use std::borrow::Cow;
use typed_builder::TypedBuilder;

/// Options for fetching users.
#[skip_serializing_none]
#[derive(Default, Debug, Serialize, TypedBuilder)]
#[builder(field_defaults(default))]
#[serde(rename_all = "camelCase")]
pub struct FetchUserOptions<'a> {
    id: Option<u32>,
    name: Option<Cow<'a, str>>,
    search: Option<Cow<'a, str>>,
    /// Sort order. The unique key (id) is appended, so paging stays stable;
    /// leaving this unset sorts by the same key alone.
    #[serde(
        serialize_with = "crate::options::serialize_sort",
        skip_serializing_if = "crate::options::skip_never"
    )]
    sort: Option<Cow<'a, [UserSort]>>,
    is_moderator: Option<bool>,
    page: Option<i32>,
    per_page: Option<i32>,
    // Extra
    include_favourites: Option<bool>,
    include_statistics: Option<bool>,
    // HTML rendering options
    about_as_html: Option<bool>,
}

/// Options for fetching a single user by ID or name.
#[skip_serializing_none]
#[derive(Default, Debug, Serialize, TypedBuilder)]
#[builder(field_defaults(default))]
#[serde(rename_all = "camelCase")]
pub struct FetchUserOneOptions<'a> {
    id: Option<u32>,
    name: Option<Cow<'a, str>>,
    search: Option<Cow<'a, str>>,
    is_moderator: Option<bool>,
    sort: Option<Cow<'a, [UserSort]>>,
    // HTML rendering options
    about_as_html: Option<bool>,
}

/// Options for fetching a user's followers.
#[skip_serializing_none]
#[derive(Default, Debug, Serialize, TypedBuilder)]
#[builder(field_defaults(default))]
#[serde(rename_all = "camelCase")]
pub struct FetchUserFollowersOptions<'a> {
    user_id: u32,
    sort: Option<Cow<'a, [UserSort]>>,
    page: Option<i32>,
    per_page: Option<i32>,
}

/// Options for fetching users a user is following.
#[skip_serializing_none]
#[derive(Default, Debug, Serialize, TypedBuilder)]
#[builder(field_defaults(default))]
#[serde(rename_all = "camelCase")]
pub struct FetchUserFollowingOptions<'a> {
    user_id: u32,
    sort: Option<Cow<'a, [UserSort]>>,
    page: Option<i32>,
    per_page: Option<i32>,
}

/// Options for fetching a user's favorites.
#[skip_serializing_none]
#[derive(Default, Debug, Serialize, TypedBuilder)]
#[builder(field_defaults(default))]
#[serde(rename_all = "camelCase")]
pub struct FetchUserFavoritesOptions<'a> {
    user_id: u32,
    // Toggle what to fetch
    fetch_anime: Option<bool>,
    fetch_manga: Option<bool>,
    fetch_characters: Option<bool>,
    fetch_staff: Option<bool>,
    fetch_studios: Option<bool>,
    // Anime pagination
    anime_page: Option<i32>,
    anime_per_page: Option<i32>,
    anime_sort: Option<Cow<'a, [MediaSort]>>,
    // Manga pagination
    manga_page: Option<i32>,
    manga_per_page: Option<i32>,
    manga_sort: Option<Cow<'a, [MediaSort]>>,
    // Characters pagination
    characters_page: Option<i32>,
    characters_per_page: Option<i32>,
    // Staff pagination
    staff_page: Option<i32>,
    staff_per_page: Option<i32>,
    // Studios pagination
    studios_page: Option<i32>,
    studios_per_page: Option<i32>,
    characters_sort: Option<Cow<'a, [CharacterSort]>>,
    staff_sort: Option<Cow<'a, [StaffSort]>>,
    studios_sort: Option<Cow<'a, [StudioSort]>>,
}

/// Options for fetching a user's media list.
#[skip_serializing_none]
#[derive(Default, Debug, Serialize, TypedBuilder)]
#[builder(field_defaults(default))]
#[serde(rename_all = "camelCase")]
pub struct FetchUserMediaListOptions<'a> {
    id: Option<u32>,
    user_id: Option<u32>,
    username: Option<Cow<'a, str>>,
    media_type: Option<MediaType>,
    status: Option<MediaListStatus>,
    status_in: Option<Cow<'a, [MediaListStatus]>>,
    status_not: Option<MediaListStatus>,
    status_not_in: Option<Cow<'a, [MediaListStatus]>>,
    media_id: Option<u32>,
    media_id_in: Option<Cow<'a, [u32]>>,
    media_id_not_in: Option<Cow<'a, [u32]>>,
    user_id_in: Option<Cow<'a, [u32]>>,
    is_following: Option<bool>,
    compare_with_auth_list: Option<bool>,
    started_at_exact: Option<i32>,
    started_at_greater: Option<i32>,
    started_at_lesser: Option<i32>,
    completed_at_exact: Option<i32>,
    completed_at_greater: Option<i32>,
    completed_at_lesser: Option<i32>,
    notes: Option<Cow<'a, str>>,
    started_at: Option<Cow<'a, str>>,
    completed_at: Option<Cow<'a, str>>,
    /// Sort order. The unique key (id) is appended, so paging stays stable;
    /// leaving this unset sorts by the same key alone.
    #[serde(
        serialize_with = "crate::options::serialize_sort",
        skip_serializing_if = "crate::options::skip_never"
    )]
    sort: Option<Cow<'a, [MediaSort]>>,
    page: Option<i32>,
    per_page: Option<i32>,
}

/// Options for fetching user statistics.
#[skip_serializing_none]
#[derive(Default, Debug, Serialize, TypedBuilder)]
#[builder(field_defaults(default))]
#[serde(rename_all = "camelCase")]
pub struct FetchUserStatsOptions<'a> {
    user_id: u32,
    /// Sort order. The unique key (id) is appended, so paging stays stable;
    /// leaving this unset sorts by the same key alone.
    #[serde(
        serialize_with = "crate::options::serialize_sort",
        skip_serializing_if = "crate::options::skip_never"
    )]
    sort: Option<Cow<'a, [UserStatisticsSort]>>,
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
        self.fetch_one(&FetchUserOneOptions::builder().id(Some(id)).build())
            .await
    }

    /// Get user by ID
    pub async fn get_by_id(&self, id: u32) -> Result<User, AniListError> {
        self.fetch_one(&FetchUserOneOptions::builder().id(Some(id)).build())
            .await
    }

    /// Get user by username
    pub async fn get_by_name(&self, name: &str) -> Result<User, AniListError> {
        self.fetch_one(
            &FetchUserOneOptions::builder()
                .name(Some(Cow::Borrowed(name)))
                .build(),
        )
        .await
    }

    /// Search users by query
    pub async fn search(
        &self,
        query: &str,
        page: Option<i32>,
        per_page: Option<i32>,
    ) -> Result<Page<Vec<User>>, AniListError> {
        self.fetch(
            &FetchUserOptions::builder()
                .search(Some(Cow::Borrowed(query)))
                .page(page)
                .per_page(per_page)
                .sort(Some(Cow::Borrowed(&[UserSort::SearchMatch])))
                .build(),
        )
        .await
    }

    /// Get users with most anime watched
    pub async fn get_most_anime_watched(
        &self,
        page: Option<i32>,
        per_page: Option<i32>,
    ) -> Result<Page<Vec<User>>, AniListError> {
        self.fetch(
            &FetchUserOptions::builder()
                .page(page)
                .per_page(per_page)
                .sort(Some(Cow::Borrowed(&[UserSort::WatchedTimeDesc])))
                .build(),
        )
        .await
    }

    /// Get users with most manga read
    pub async fn get_most_manga_read(
        &self,
        page: Option<i32>,
        per_page: Option<i32>,
    ) -> Result<Page<Vec<User>>, AniListError> {
        self.fetch(
            &FetchUserOptions::builder()
                .page(page)
                .per_page(per_page)
                .sort(Some(Cow::Borrowed(&[UserSort::ChaptersReadDesc])))
                .build(),
        )
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
