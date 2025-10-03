use crate::enums::character::CharacterSort;
use crate::enums::media::MediaSort;
use crate::errors::AniListError;
use crate::objects::responses::{CharacterListResponse, CharacterSingleResponse};
use crate::{client::AniListClient, queries::character};
use serde::Serialize;
use serde_json::json;

#[derive(Default, Serialize)]
pub struct FetchCharacterOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "perPage", skip_serializing_if = "Option::is_none")]
    pub per_page: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<Vec<CharacterSort>>,
    #[serde(rename = "isBirthday", skip_serializing_if = "Option::is_none")]
    pub is_birthday: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_not: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_in: Option<Vec<i32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_not_in: Option<Vec<i32>>,
}

#[derive(Default, Serialize)]
pub struct FetchCharacterOneOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "isBirthday", skip_serializing_if = "Option::is_none")]
    pub is_birthday: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_not: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_in: Option<Vec<i32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_not_in: Option<Vec<i32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<Vec<CharacterSort>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i32>,
    #[serde(rename = "perPage", skip_serializing_if = "Option::is_none")]
    pub per_page: Option<i32>,
    // Sub-pagination variables
    #[serde(rename = "mediaSort", skip_serializing_if = "Option::is_none")]
    pub media_sort: Option<Vec<MediaSort>>,
    #[serde(rename = "mediaPage", skip_serializing_if = "Option::is_none")]
    pub media_page: Option<i32>,
    #[serde(rename = "mediaPerPage", skip_serializing_if = "Option::is_none")]
    pub media_per_page: Option<i32>,
}

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
