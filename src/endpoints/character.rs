use crate::enums::character::CharacterSort;
use crate::enums::media::MediaSort;
use crate::errors::AniListError;
use crate::objects::responses::{CharacterListResponse, CharacterSingleResponse};
use crate::{client::AniListClient, queries::character};
use serde::Serialize;
use serde_json::json;
use serde_with::skip_serializing_none;

/// Options for fetching multiple characters with pagination and filters.
#[skip_serializing_none]
#[derive(Default, Serialize)]
pub struct FetchCharacterOptions {
    pub page: Option<i32>,
    pub id: Option<i32>,
    #[serde(rename = "perPage")]
    pub per_page: Option<i32>,
    pub search: Option<String>,
    pub sort: Option<Vec<CharacterSort>>,
    #[serde(rename = "isBirthday")]
    pub is_birthday: Option<bool>,
    pub id_not: Option<i32>,
    pub id_in: Option<Vec<i32>>,
    pub id_not_in: Option<Vec<i32>>,
}

/// Options for fetching a single character.
#[skip_serializing_none]
#[derive(Default, Serialize)]
pub struct FetchCharacterOneOptions {
    pub id: Option<i32>,
    #[serde(rename = "isBirthday")]
    pub is_birthday: Option<bool>,
    pub search: Option<String>,
    pub id_not: Option<i32>,
    pub id_in: Option<Vec<i32>>,
    pub id_not_in: Option<Vec<i32>>,
    pub sort: Option<Vec<CharacterSort>>,
    pub page: Option<i32>,
    #[serde(rename = "perPage")]
    pub per_page: Option<i32>,
    // Sub-pagination variables
    #[serde(rename = "mediaSort")]
    pub media_sort: Option<Vec<MediaSort>>,
    #[serde(rename = "mediaPage")]
    pub media_page: Option<i32>,
    #[serde(rename = "mediaPerPage")]
    pub media_per_page: Option<i32>,
}

/// Endpoint for character operations.
pub struct CharacterEndpoint {
    client: AniListClient,
}

impl CharacterEndpoint {
    pub fn new(client: AniListClient) -> Self {
        Self { client }
    }

    pub async fn fetch(
        &self,
        options: FetchCharacterOptions,
    ) -> Result<CharacterListResponse, AniListError> {
        let query = character::FETCH;
        let variables = json!(options);
        let variables_map = crate::utils::json_to_hashmap(variables);
        self.client.query_typed(query, Some(&variables_map)).await
    }

    pub async fn fetch_one(
        &self,
        options: FetchCharacterOneOptions,
    ) -> Result<CharacterSingleResponse, AniListError> {
        let query = character::FETCH_ONE;
        let variables = json!(options);
        let variables_map = crate::utils::json_to_hashmap(variables);
        self.client.query_typed(query, Some(&variables_map)).await
    }

    // Convenience functions

    /// Get popular characters sorted by favorites
    pub async fn get_popular(
        &self,
        page: Option<i32>,
        per_page: Option<i32>,
    ) -> Result<CharacterListResponse, AniListError> {
        self.fetch(FetchCharacterOptions {
            page,
            per_page,
            sort: Some(vec![CharacterSort::FavouritesDesc]),
            ..Default::default()
        })
        .await
    }

    /// Get most favorited characters (alias for get_popular)
    pub async fn get_most_favorited(
        &self,
        page: Option<i32>,
        per_page: Option<i32>,
    ) -> Result<CharacterListResponse, AniListError> {
        self.get_popular(page, per_page).await
    }

    /// Search characters by name
    pub async fn search(
        &self,
        query: &str,
        page: Option<i32>,
        per_page: Option<i32>,
    ) -> Result<CharacterListResponse, AniListError> {
        self.fetch(FetchCharacterOptions {
            search: Some(query.to_string()),
            page,
            per_page,
            sort: Some(vec![CharacterSort::SearchMatch]),
            ..Default::default()
        })
        .await
    }

    /// Get character by ID
    pub async fn get_by_id(&self, id: i32) -> Result<CharacterSingleResponse, AniListError> {
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
    ) -> Result<CharacterListResponse, AniListError> {
        self.fetch(FetchCharacterOptions {
            is_birthday: Some(true),
            page,
            per_page,
            sort: Some(vec![CharacterSort::FavouritesDesc]),
            ..Default::default()
        })
        .await
    }
}
