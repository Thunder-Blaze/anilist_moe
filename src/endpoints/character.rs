use crate::client::AniListClient;
use crate::errors::AniListError;
use crate::enums::character::CharacterSort;
use crate::objects::responses::CharacterListResponse;
use serde::Serialize;
use serde_json::{json, Value};
use std::collections::HashMap;

#[derive(Default, Serialize)]
pub struct CharacterSearchOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i32>,
    #[serde(rename = "perPage", skip_serializing_if = "Option::is_none")]
    pub per_page: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "isBirthday", skip_serializing_if = "Option::is_none")]
    pub is_birthday: Option<bool>,
    #[serde(rename = "id_not", skip_serializing_if = "Option::is_none")]
    pub id_not: Option<i32>,
    #[serde(rename = "id_in", skip_serializing_if = "Option::is_none")]
    pub id_in: Option<Vec<i32>>,
    #[serde(rename = "id_not_in", skip_serializing_if = "Option::is_none")]
    pub id_not_in: Option<Vec<i32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<Vec<CharacterSort>>,
}

pub struct CharacterEndpoint(pub(crate) AniListClient);

impl CharacterEndpoint {
    pub fn new(client: AniListClient) -> Self {
        Self(client)
    }

    /// Search characters
    pub async fn search(&self, query: &str, page: i32, per_page: i32) -> Result<CharacterListResponse, AniListError> {
        let options = CharacterSearchOptions {
            search: Some(query.to_string()),
            page: Some(page),
            per_page: Some(per_page),
            ..Default::default()
        };
        self.search_characters(options).await
    }

    /// Get popular characters
    pub async fn get_popular(&self, page: i32, per_page: i32) -> Result<CharacterListResponse, AniListError> {
        let options = CharacterSearchOptions {
            page: Some(page),
            per_page: Some(per_page),
            sort: Some(vec![CharacterSort::Favourites]),
            ..Default::default()
        };
        self.search_characters(options).await
    }

    /// Get trending characters
    pub async fn get_trending(&self, page: i32, per_page: i32) -> Result<CharacterListResponse, AniListError> {
        let options = CharacterSearchOptions {
            page: Some(page),
            per_page: Some(per_page),
            sort: Some(vec![CharacterSort::FavouritesDesc]),
            ..Default::default()
        };
        self.search_characters(options).await
    }

    /// Get character by ID
    pub async fn get_by_id(&self, id: i32) -> Result<CharacterListResponse, AniListError> {
        let options = CharacterSearchOptions {
            id: Some(id),
            ..Default::default()
        };
        self.search_characters(options).await
    }

    /// Get characters with birthdays today
    pub async fn get_birthday_characters(&self, page: i32, per_page: i32) -> Result<CharacterListResponse, AniListError> {
        let options = CharacterSearchOptions {
            page: Some(page),
            per_page: Some(per_page),
            is_birthday: Some(true),
            sort: Some(vec![CharacterSort::Favourites]),
            ..Default::default()
        };
        self.search_characters(options).await
    }

    /// Get most favorited characters
    pub async fn get_most_favorited(&self, page: i32, per_page: i32) -> Result<CharacterListResponse, AniListError> {
        let options = CharacterSearchOptions {
            page: Some(page),
            per_page: Some(per_page),
            sort: Some(vec![CharacterSort::FavouritesDesc]),
            ..Default::default()
        };
        self.search_characters(options).await
    }

    /// Search characters with custom options
    pub async fn search_with_options(&self, options: CharacterSearchOptions) -> Result<CharacterListResponse, AniListError> {
        self.search_characters(options).await
    }

    async fn search_characters(&self, options: CharacterSearchOptions) -> Result<CharacterListResponse, AniListError> {
        let query = include_str!("../queries/character/search_characters.graphql");
        let variables = json!(options);
        let variables_map = self.value_to_hashmap(variables);
        self.0.query_typed(query, Some(&variables_map)).await
    }

    fn value_to_hashmap(&self, value: Value) -> HashMap<String, Value> {
        match value {
            Value::Object(map) => map.into_iter().collect(),
            _ => HashMap::new(),
        }
    }
}
