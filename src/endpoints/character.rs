use crate::enums::media::MediaSort;
use crate::{client::AniListClient, queries::character};
use crate::errors::AniListError;
use crate::enums::character::CharacterSort;
use crate::objects::responses::{CharacterListResponse, CharacterSingleResponse};
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
}
