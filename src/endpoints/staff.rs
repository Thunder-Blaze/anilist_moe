use crate::enums::staff::StaffSort;
use crate::errors::AniListError;
use crate::objects::responses::Page;
use crate::objects::staff::Staff;
#[cfg(feature = "pagination")]
use crate::pagination::Paginated;
use crate::{client::AniListClient, queries::staff};
use serde::Serialize;
use serde_with::skip_serializing_none;
use std::borrow::Cow;
use typed_builder::TypedBuilder;

/// Options for fetching staff members.
#[skip_serializing_none]
#[derive(Default, Debug, Serialize, TypedBuilder)]
#[builder(field_defaults(
    default,
    setter(strip_option(ignore_invalid, fallback_suffix = "_opt"))
))]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct FetchStaffOptions<'a> {
    pub id: Option<u32>,
    pub is_birthday: Option<bool>,
    #[builder(setter(into))]
    pub search: Option<Cow<'a, str>>,
    pub id_not: Option<u32>,
    #[builder(setter(into))]
    pub id_in: Option<Cow<'a, [u32]>>,
    #[builder(setter(into))]
    pub id_not_in: Option<Cow<'a, [u32]>>,
    /// Sort order. The unique key (id) is appended, so paging stays stable;
    /// leaving this unset sorts by the same key alone.
    #[serde(
        serialize_with = "crate::options::serialize_sort",
        skip_serializing_if = "crate::options::skip_never"
    )]
    #[builder(setter(into))]
    pub sort: Option<Cow<'a, [StaffSort]>>,
    pub page: Option<i32>,
    pub per_page: Option<i32>,
    // Extra
    pub include_staff_media: Option<bool>,
    pub include_characters: Option<bool>,
    pub include_character_media: Option<bool>,
    pub include_submitter: Option<bool>,
    pub include_submission_status: Option<bool>,
    pub include_submission_notes: Option<bool>,
    pub include_mod_notes: Option<bool>,
    // Sub-pagination variables
    pub staff_media_page: Option<i32>,
    pub staff_media_per_page: Option<i32>,
    pub characters_page: Option<i32>,
    pub characters_per_page: Option<i32>,
    pub character_media_page: Option<i32>,
    pub character_media_per_page: Option<i32>,
}

/// Options for fetching a single staff member by ID.
#[skip_serializing_none]
#[derive(Default, Debug, Serialize, TypedBuilder)]
#[builder(field_defaults(
    default,
    setter(strip_option(ignore_invalid, fallback_suffix = "_opt"))
))]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct FetchStaffOneOptions<'a> {
    pub id: Option<u32>,
    pub is_birthday: Option<bool>,
    #[builder(setter(into))]
    pub search: Option<Cow<'a, str>>,
    pub id_not: Option<u32>,
    #[builder(setter(into))]
    pub id_in: Option<Cow<'a, [u32]>>,
    #[builder(setter(into))]
    pub id_not_in: Option<Cow<'a, [u32]>>,
    /// Sort order. The unique key (id) is appended, so paging stays stable;
    /// leaving this unset sorts by the same key alone.
    #[serde(
        serialize_with = "crate::options::serialize_sort",
        skip_serializing_if = "crate::options::skip_never"
    )]
    #[builder(setter(into))]
    pub sort: Option<Cow<'a, [StaffSort]>>,
    // Sub-pagination variables
    pub staff_media_page: Option<i32>,
    pub staff_media_per_page: Option<i32>,
    pub characters_page: Option<i32>,
    pub characters_per_page: Option<i32>,
    pub character_media_page: Option<i32>,
    pub character_media_per_page: Option<i32>,
}

/// Endpoint for staff member operations.
pub struct StaffEndpoint<'a> {
    client: &'a AniListClient,
}

impl<'a> StaffEndpoint<'a> {
    pub fn new(client: &'a AniListClient) -> Self {
        Self { client }
    }

    pub async fn fetch(
        &self,
        options: &FetchStaffOptions<'a>,
    ) -> Result<Page<Vec<Staff>>, AniListError> {
        let query = staff::FETCH;
        self.client.fetch(query, Some(&options)).await
    }

    pub async fn fetch_one(
        &self,
        options: &FetchStaffOneOptions<'_>,
    ) -> Result<Staff, AniListError> {
        let query = staff::FETCH_ONE;
        self.client.fetch(query, Some(&options)).await
    }

    // Convenience functions

    /// Get popular staff sorted by favorites
    pub async fn get_popular(
        &self,
        page: Option<i32>,
        per_page: Option<i32>,
    ) -> Result<Page<Vec<Staff>>, AniListError> {
        self.fetch(
            &FetchStaffOptions::builder()
                .sort(&[StaffSort::FavouritesDesc])
                .page_opt(page)
                .per_page_opt(per_page)
                .build(),
        )
        .await
    }

    /// Get most favorited staff (alias for get_popular)
    pub async fn get_most_favorited(
        &self,
        page: Option<i32>,
        per_page: Option<i32>,
    ) -> Result<Page<Vec<Staff>>, AniListError> {
        self.get_popular(page, per_page).await
    }

    /// Search staff by name
    pub async fn search(
        &self,
        query: &str,
        page: Option<i32>,
        per_page: Option<i32>,
    ) -> Result<Page<Vec<Staff>>, AniListError> {
        self.fetch(
            &FetchStaffOptions::builder()
                .search(Cow::Borrowed(query))
                .sort(&[StaffSort::SearchMatch])
                .page_opt(page)
                .per_page_opt(per_page)
                .build(),
        )
        .await
    }

    /// Get staff by ID
    pub async fn get_by_id(&self, id: u32) -> Result<Staff, AniListError> {
        self.fetch_one(&FetchStaffOneOptions::builder().id(id).build())
            .await
    }

    /// Get staff with birthday today
    pub async fn get_today_birthday(
        &self,
        page: Option<i32>,
        per_page: Option<i32>,
    ) -> Result<Page<Vec<Staff>>, AniListError> {
        self.fetch(
            &FetchStaffOptions::builder()
                .is_birthday(true)
                .sort(&[StaffSort::FavouritesDesc])
                .page_opt(page)
                .per_page_opt(per_page)
                .build(),
        )
        .await
    }
}

#[cfg(feature = "pagination")]
impl<'a> Paginated for FetchStaffOptions<'a> {
    type Item = Staff;

    async fn fetch(&self, client: &AniListClient) -> Result<Page<Vec<Staff>>, AniListError> {
        client.staff().fetch(self).await
    }

    fn set_page(&mut self, page: i32) {
        self.page = Some(page);
    }
}
