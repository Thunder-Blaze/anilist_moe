use crate::enums::staff::StaffSort;
use crate::errors::AniListError;
use crate::objects::responses::{StaffListResponse, StaffSingleResponse};
use crate::{client::AniListClient, queries::staff};
use serde::Serialize;
use serde_json::json;
use serde_with::skip_serializing_none;

/// Options for fetching staff members.
#[skip_serializing_none]
#[derive(Default, Serialize)]
pub struct FetchStaffOptions {
    pub id: Option<i32>,
    #[serde(rename = "isBirthday")]
    pub is_birthday: Option<bool>,
    pub search: Option<String>,
    #[serde(rename = "id_not")]
    pub id_not: Option<i32>,
    #[serde(rename = "id_in")]
    pub id_in: Option<Vec<i32>>,
    #[serde(rename = "id_not_in")]
    pub id_not_in: Option<Vec<i32>>,
    pub sort: Option<Vec<StaffSort>>,
}

/// Options for fetching a single staff member by ID.
#[skip_serializing_none]
#[derive(Default, Serialize)]
pub struct FetchStaffOneOptions {
    pub id: Option<i32>,
    #[serde(rename = "isBirthday")]
    pub is_birthday: Option<bool>,
    pub search: Option<String>,
    #[serde(rename = "id_not")]
    pub id_not: Option<i32>,
    #[serde(rename = "id_in")]
    pub id_in: Option<Vec<i32>>,
    #[serde(rename = "id_not_in")]
    pub id_not_in: Option<Vec<i32>>,
    pub sort: Option<Vec<StaffSort>>,
    // Sub-pagination variables
    #[serde(rename = "staffMediaPage")]
    pub staff_media_page: Option<i32>,
    #[serde(rename = "staffMediaPerPage")]
    pub staff_media_per_page: Option<i32>,
    #[serde(rename = "charactersPage")]
    pub characters_page: Option<i32>,
    #[serde(rename = "charactersPerPage")]
    pub characters_per_page: Option<i32>,
    #[serde(rename = "characterMediaPage")]
    pub character_media_page: Option<i32>,
    #[serde(rename = "characterMediaPerPage")]
    pub character_media_per_page: Option<i32>,
}

/// Endpoint for staff member operations.
pub struct StaffEndpoint {
    client: AniListClient,
}

impl StaffEndpoint {
    pub fn new(client: AniListClient) -> Self {
        Self { client }
    }

    pub async fn fetch(
        &self,
        options: FetchStaffOptions,
    ) -> Result<StaffListResponse, AniListError> {
        let query = staff::FETCH;
        let variables = json!(options);
        let variables_map = crate::utils::json_to_hashmap(variables);
        self.client.query_typed(query, Some(&variables_map)).await
    }

    pub async fn fetch_one(
        &self,
        options: FetchStaffOneOptions,
    ) -> Result<StaffSingleResponse, AniListError> {
        let query = staff::FETCH_ONE;
        let variables = json!(options);
        let variables_map = crate::utils::json_to_hashmap(variables);
        self.client.query_typed(query, Some(&variables_map)).await
    }

    // Convenience functions

    /// Get popular staff sorted by favorites
    pub async fn get_popular(
        &self,
        _page: Option<i32>,
        _per_page: Option<i32>,
    ) -> Result<StaffListResponse, AniListError> {
        self.fetch(FetchStaffOptions {
            sort: Some(vec![StaffSort::FavouritesDesc]),
            ..Default::default()
        })
        .await
    }

    /// Get most favorited staff (alias for get_popular)
    pub async fn get_most_favorited(
        &self,
        _page: Option<i32>,
        _per_page: Option<i32>,
    ) -> Result<StaffListResponse, AniListError> {
        self.get_popular(_page, _per_page).await
    }

    /// Search staff by name
    pub async fn search(
        &self,
        query: &str,
        _page: Option<i32>,
        _per_page: Option<i32>,
    ) -> Result<StaffListResponse, AniListError> {
        self.fetch(FetchStaffOptions {
            search: Some(query.to_string()),
            sort: Some(vec![StaffSort::SearchMatch]),
            ..Default::default()
        })
        .await
    }

    /// Get staff by ID
    pub async fn get_by_id(&self, id: i32) -> Result<StaffSingleResponse, AniListError> {
        self.fetch_one(FetchStaffOneOptions {
            id: Some(id),
            ..Default::default()
        })
        .await
    }

    /// Get staff with birthday today
    pub async fn get_today_birthday(
        &self,
        _page: Option<i32>,
        _per_page: Option<i32>,
    ) -> Result<StaffListResponse, AniListError> {
        self.fetch(FetchStaffOptions {
            is_birthday: Some(true),
            sort: Some(vec![StaffSort::FavouritesDesc]),
            ..Default::default()
        })
        .await
    }
}
