use crate::enums::character::CharacterSort;
use crate::enums::media::MediaSort;
use crate::errors::AniListError;
use crate::objects::character::Character;
use crate::objects::responses::Page;
use crate::{client::AniListClient, queries::character};
use serde::Serialize;
use serde_with::skip_serializing_none;

/// Options for fetching multiple characters with pagination and filters.
#[skip_serializing_none]
#[derive(Default, Debug, Serialize)]
pub struct FetchCharacterOptions<'a> {
    pub page: Option<i32>,
    pub id: Option<i32>,
    #[serde(rename = "perPage")]
    pub per_page: Option<i32>,
    pub search: Option<&'a str>,
    pub sort: Option<&'a [CharacterSort]>,
    #[serde(rename = "isBirthday")]
    pub is_birthday: Option<bool>,
    pub id_not: Option<i32>,
    pub id_in: Option<&'a [i32]>,
    pub id_not_in: Option<&'a [i32]>,
    // Extra
    #[serde(rename = "includeMedia")]
    pub include_media: Option<bool>,
    #[serde(rename = "includeModNotes")]
    pub include_mod_notes: Option<bool>,
    // Sub-pagination variables
    #[serde(rename = "mediaSort")]
    pub media_sort: Option<&'a [MediaSort]>,
    #[serde(rename = "mediaPage")]
    pub media_page: Option<i32>,
    #[serde(rename = "mediaPerPage")]
    pub media_per_page: Option<i32>,
}

/// Options for fetching a single character.
#[skip_serializing_none]
#[derive(Default, Debug, Serialize)]
pub struct FetchCharacterOneOptions<'a> {
    pub id: Option<i32>,
    #[serde(rename = "isBirthday")]
    pub is_birthday: Option<bool>,
    pub search: Option<&'a str>,
    pub id_not: Option<i32>,
    pub id_in: Option<&'a [i32]>,
    pub id_not_in: Option<&'a [i32]>,
    pub sort: Option<&'a [CharacterSort]>,
    pub page: Option<i32>,
    #[serde(rename = "perPage")]
    pub per_page: Option<i32>,
    // Sub-pagination variables
    #[serde(rename = "mediaSort")]
    pub media_sort: Option<&'a [MediaSort]>,
    #[serde(rename = "mediaPage")]
    pub media_page: Option<i32>,
    #[serde(rename = "mediaPerPage")]
    pub media_per_page: Option<i32>,
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
        options: FetchCharacterOptions<'_>,
    ) -> Result<Page<Vec<Character>>, AniListError> {
        let query = character::FETCH;
        self.client.fetch(query, Some(&options)).await
    }

    pub async fn fetch_one(
        &self,
        options: FetchCharacterOneOptions<'_>,
    ) -> Result<Character, AniListError> {
        let query = character::FETCH_ONE;
        self.client.fetch(query, Some(&options)).await
    }

    // Convenience functions

    /// Get popular characters sorted by favorites
    pub async fn get_popular(
        &self,
        page: Option<i32>,
        per_page: Option<i32>,
    ) -> Result<Page<Vec<Character>>, AniListError> {
        self.fetch(FetchCharacterOptions {
            page,
            per_page,
            sort: Some(&[CharacterSort::FavouritesDesc]),
            ..Default::default()
        })
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
        self.fetch(FetchCharacterOptions {
            search: Some(query),
            page,
            per_page,
            sort: Some(&[CharacterSort::SearchMatch]),
            ..Default::default()
        })
        .await
    }

    /// Get character by ID
    pub async fn get_by_id(&self, id: i32) -> Result<Character, AniListError> {
        self.fetch_one(FetchCharacterOneOptions {
            id: Some(id),
            ..Default::default()
        })
        .await
    }

    /// Get characters with birthday today
    pub async fn get_today_birthday(
        &self,
        page: Option<i32>,
        per_page: Option<i32>,
    ) -> Result<Page<Vec<Character>>, AniListError> {
        self.fetch(FetchCharacterOptions {
            is_birthday: Some(true),
            page,
            per_page,
            sort: Some(&[CharacterSort::FavouritesDesc]),
            ..Default::default()
        })
        .await
    }
}
