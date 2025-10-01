use serde::Serialize;
use crate::endpoints::Vth;
use crate::{client::AniListClient, queries::staff};
use crate::errors::AniListError;
use crate::enums::staff::StaffSort;
use crate::objects::responses::StaffListResponse;
use serde_json::json;

#[derive(Default, Serialize)]
pub struct FetchStaffOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "isBirthday", skip_serializing_if = "Option::is_none")]
    pub is_birthday: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search: Option<String>,
    #[serde(rename = "id_not", skip_serializing_if = "Option::is_none")]
    pub id_not: Option<i32>,
    #[serde(rename = "id_in", skip_serializing_if = "Option::is_none")]
    pub id_in: Option<Vec<i32>>,
    #[serde(rename = "id_not_in", skip_serializing_if = "Option::is_none")]
    pub id_not_in: Option<Vec<i32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<Vec<StaffSort>>,
}

#[derive(Default, Serialize)]
pub struct FetchStaffOneOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "isBirthday", skip_serializing_if = "Option::is_none")]
    pub is_birthday: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search: Option<String>,
    #[serde(rename = "id_not", skip_serializing_if = "Option::is_none")]
    pub id_not: Option<i32>,
    #[serde(rename = "id_in", skip_serializing_if = "Option::is_none")]
    pub id_in: Option<Vec<i32>>,
    #[serde(rename = "id_not_in", skip_serializing_if = "Option::is_none")]
    pub id_not_in: Option<Vec<i32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<Vec<StaffSort>>,
    // Sub-pagination variables
    #[serde(rename = "staffMediaPage", skip_serializing_if = "Option::is_none")]
    pub staff_media_page: Option<i32>,
    #[serde(rename = "staffMediaPerPage", skip_serializing_if = "Option::is_none")]
    pub staff_media_per_page: Option<i32>,
    #[serde(rename = "charactersPage", skip_serializing_if = "Option::is_none")]
    pub characters_page: Option<i32>,
    #[serde(rename = "charactersPerPage", skip_serializing_if = "Option::is_none")]
    pub characters_per_page: Option<i32>,
    #[serde(rename = "characterMediaPage", skip_serializing_if = "Option::is_none")]
    pub character_media_page: Option<i32>,
    #[serde(rename = "characterMediaPerPage", skip_serializing_if = "Option::is_none")]
    pub character_media_per_page: Option<i32>,
}

pub struct StaffEndpoint(pub(crate) AniListClient);

impl StaffEndpoint {
    pub fn new(client: AniListClient) -> Self {
        Self(client)
    }

    pub async fn fetch(
        &self,
        options: FetchStaffOptions,
    ) -> Result<StaffListResponse, AniListError> {
        let query = staff::FETCH;
        let variables = json!(options);
        let variables_map = self.value_to_hashmap(variables);
        self.0.query_typed(query, Some(&variables_map)).await
    }

    pub async fn fetch_one(
        &self,
        options: FetchStaffOneOptions,
    ) -> Result<StaffListResponse, AniListError> {
        let query = staff::FETCH_ONE;
        let variables = json!(options);
        let variables_map = self.value_to_hashmap(variables);
        self.0.query_typed(query, Some(&variables_map)).await
    }
}

impl Vth for StaffEndpoint {}


