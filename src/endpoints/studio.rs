use crate::enums::studio::StudioSort;
use crate::errors::AniListError;
use crate::objects::responses::Page;
use crate::objects::studio::Studio;
#[cfg(feature = "pagination")]
use crate::pagination::Paginated;
use crate::{client::AniListClient, queries::studio};
use serde::Serialize;
use serde_with::skip_serializing_none;
use std::borrow::Cow;
use typed_builder::TypedBuilder;

/// Options for fetching studios.
#[skip_serializing_none]
#[derive(Default, Debug, Serialize, TypedBuilder)]
#[builder(field_defaults(default))]
#[serde(rename_all = "camelCase")]
pub struct FetchStudioOptions<'a> {
    page: Option<i32>,
    per_page: Option<i32>,
    id: Option<u32>,
    search: Option<Cow<'a, str>>,
    id_not: Option<u32>,
    id_in: Option<Cow<'a, [u32]>>,
    id_not_in: Option<Cow<'a, [u32]>>,
    /// Sort order. The unique key (id) is appended, so paging stays stable;
    /// leaving this unset sorts by the same key alone.
    #[serde(
        serialize_with = "crate::options::serialize_sort",
        skip_serializing_if = "crate::options::skip_never"
    )]
    sort: Option<Cow<'a, [StudioSort]>>,
    // Extra
    include_media: Option<bool>,
    // Sub-pagination variables
    media_page: Option<i32>,
    media_per_page: Option<i32>,
}

/// Options for fetching a single studio by ID.
#[skip_serializing_none]
#[derive(Default, Debug, Serialize, TypedBuilder)]
#[builder(field_defaults(default))]
#[serde(rename_all = "camelCase")]
pub struct FetchStudioOneOptions<'a> {
    id: Option<u32>,
    search: Option<Cow<'a, str>>,
    id_not: Option<u32>,
    id_in: Option<Cow<'a, [u32]>>,
    id_not_in: Option<Cow<'a, [u32]>>,
    /// Sort order. The unique key (id) is appended, so paging stays stable;
    /// leaving this unset sorts by the same key alone.
    #[serde(
        serialize_with = "crate::options::serialize_sort",
        skip_serializing_if = "crate::options::skip_never"
    )]
    sort: Option<Cow<'a, [StudioSort]>>,
    // Sub-pagination variables
    media_page: Option<i32>,
    media_per_page: Option<i32>,
}

/// Endpoint for studio operations.
pub struct StudioEndpoint<'a> {
    client: &'a AniListClient,
}

impl<'a> StudioEndpoint<'a> {
    pub fn new(client: &'a AniListClient) -> Self {
        Self { client }
    }

    pub async fn fetch(
        &self,
        options: &FetchStudioOptions<'_>,
    ) -> Result<Page<Vec<Studio>>, AniListError> {
        let query = studio::FETCH;
        self.client.fetch(query, Some(&options)).await
    }

    pub async fn fetch_one(
        &self,
        options: &FetchStudioOneOptions<'_>,
    ) -> Result<Studio, AniListError> {
        let query = studio::FETCH_ONE;
        self.client.fetch(query, Some(&options)).await
    }

    // Convenience functions

    /// Get popular studios sorted by favorites
    pub async fn get_popular(
        &self,
        page: Option<i32>,
        per_page: Option<i32>,
    ) -> Result<Page<Vec<Studio>>, AniListError> {
        self.fetch(
            &FetchStudioOptions::builder()
                .page(page)
                .per_page(per_page)
                .sort(Some(Cow::Borrowed(&[StudioSort::FavouritesDesc])))
                .build(),
        )
        .await
    }

    /// Search studios by name
    pub async fn search(
        &self,
        query: &str,
        page: Option<i32>,
        per_page: Option<i32>,
    ) -> Result<Page<Vec<Studio>>, AniListError> {
        self.fetch(
            &FetchStudioOptions::builder()
                .search(Some(Cow::Borrowed(query)))
                .page(page)
                .per_page(per_page)
                .sort(Some(Cow::Borrowed(&[StudioSort::SearchMatch])))
                .build(),
        )
        .await
    }

    /// Get studio by ID

    pub async fn get_by_id(&self, id: u32) -> Result<Studio, AniListError> {
        self.fetch_one(&FetchStudioOneOptions::builder().id(Some(id)).build())
            .await
    }
}

#[cfg(feature = "pagination")]
impl<'a> Paginated for FetchStudioOptions<'a> {
    type Item = Studio;

    async fn fetch(&self, client: &AniListClient) -> Result<Page<Vec<Studio>>, AniListError> {
        client.studio().fetch(self).await
    }

    fn set_page(&mut self, page: i32) {
        self.page = Some(page);
    }
}
