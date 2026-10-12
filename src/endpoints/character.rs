use crate::enums::character::CharacterSort;
use crate::enums::media::MediaSort;
use crate::errors::AniListError;
use crate::objects::character::Character;
use crate::objects::responses::Page;
#[cfg(feature = "pagination")]
use crate::pagination::Paginated;
use crate::{client::AniListClient, queries::character};
use serde::Serialize;
use serde_with::skip_serializing_none;
use std::borrow::Cow;
use typed_builder::TypedBuilder;

/// Options for fetching multiple characters with pagination and filters.
#[skip_serializing_none]
#[derive(Default, Debug, Serialize, TypedBuilder)]
#[builder(field_defaults(default))]
#[serde(rename_all = "camelCase")]
pub struct FetchCharacterOptions<'a> {
    page: Option<i32>,
    id: Option<u32>,
    per_page: Option<i32>,
    search: Option<Cow<'a, str>>,
    /// Sort order. The unique key (id) is appended, so paging stays stable;
    /// leaving this unset sorts by the same key alone.
    #[serde(
        serialize_with = "crate::options::serialize_sort",
        skip_serializing_if = "crate::options::skip_never"
    )]
    sort: Option<Cow<'a, [CharacterSort]>>,
    is_birthday: Option<bool>,
    id_not: Option<u32>,
    id_in: Option<Cow<'a, [u32]>>,
    id_not_in: Option<Cow<'a, [u32]>>,
    // Extra
    include_media: Option<bool>,
    include_mod_notes: Option<bool>,
    // Sub-pagination variables
    media_sort: Option<Cow<'a, [MediaSort]>>,
    media_page: Option<i32>,
    media_per_page: Option<i32>,
}

/// Options for fetching a single character.
#[skip_serializing_none]
#[derive(Default, Debug, Serialize, TypedBuilder)]
#[builder(field_defaults(default))]
#[serde(rename_all = "camelCase")]
pub struct FetchCharacterOneOptions<'a> {
    id: Option<u32>,
    is_birthday: Option<bool>,
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
    sort: Option<Cow<'a, [CharacterSort]>>,
    // Sub-pagination variables
    media_sort: Option<Cow<'a, [MediaSort]>>,
    media_page: Option<i32>,
    media_per_page: Option<i32>,
}

/// Endpoint for character operations.
pub struct CharacterEndpoint<'a> {
    client: &'a AniListClient,
}

impl<'a> CharacterEndpoint<'a> {
    pub fn new(client: &'a AniListClient) -> Self {
        Self { client }
    }

    pub async fn fetch(
        &self,
        options: &FetchCharacterOptions<'_>,
    ) -> Result<Page<Vec<Character>>, AniListError> {
        let query = character::FETCH;
        self.client.fetch(query, Some(options)).await
    }

    pub async fn fetch_one(
        &self,
        options: &FetchCharacterOneOptions<'_>,
    ) -> Result<Character, AniListError> {
        let query = character::FETCH_ONE;
        self.client.fetch(query, Some(options)).await
    }

    // Convenience functions

    /// Get popular characters sorted by favorites
    pub async fn get_popular(
        &self,
        page: Option<i32>,
        per_page: Option<i32>,
    ) -> Result<Page<Vec<Character>>, AniListError> {
        self.fetch(
            &FetchCharacterOptions::builder()
                .page(page)
                .per_page(per_page)
                .sort(Some(Cow::Borrowed(&[CharacterSort::FavouritesDesc])))
                .build(),
        )
        .await
    }

    /// Get most favorited characters (alias for get_popular)
    pub async fn get_most_favorited(
        &self,
        page: Option<i32>,
        per_page: Option<i32>,
    ) -> Result<Page<Vec<Character>>, AniListError> {
        self.get_popular(page, per_page).await
    }

    /// Search characters by name
    pub async fn search(
        &self,
        query: &str,
        page: Option<i32>,
        per_page: Option<i32>,
    ) -> Result<Page<Vec<Character>>, AniListError> {
        self.fetch(
            &FetchCharacterOptions::builder()
                .search(Some(Cow::Borrowed(query)))
                .page(page)
                .per_page(per_page)
                .sort(Some(Cow::Borrowed(&[CharacterSort::SearchMatch])))
                .build(),
        )
        .await
    }

    /// Get character by ID
    pub async fn get_by_id(&self, id: u32) -> Result<Character, AniListError> {
        self.fetch_one(&FetchCharacterOneOptions::builder().id(Some(id)).build())
            .await
    }

    /// Get characters with birthday today
    pub async fn get_today_birthday(
        &self,
        page: Option<i32>,
        per_page: Option<i32>,
    ) -> Result<Page<Vec<Character>>, AniListError> {
        self.fetch(
            &FetchCharacterOptions::builder()
                .is_birthday(Some(true))
                .page(page)
                .per_page(per_page)
                .sort(Some(Cow::Borrowed(&[CharacterSort::FavouritesDesc])))
                .build(),
        )
        .await
    }
}

#[cfg(feature = "pagination")]
impl<'a> Paginated for FetchCharacterOptions<'a> {
    type Item = Character;

    async fn fetch(&self, client: &AniListClient) -> Result<Page<Vec<Character>>, AniListError> {
        client.character().fetch(self).await
    }

    fn set_page(&mut self, page: i32) {
        self.page = Some(page);
    }
}
