use crate::enums::staff::StaffSort;
use crate::errors::AniListError;
use crate::objects::responses::Page;
use crate::objects::staff::Staff;
use crate::{client::AniListClient, queries::staff};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Options for fetching staff members.
#[skip_serializing_none]
#[derive(Default, Debug, Serialize, Deserialize)]
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
    pub page: Option<i32>,
    #[serde(rename = "perPage")]
    pub per_page: Option<i32>,
    // Extra
    #[serde(rename = "includeStaffMedia")]
    pub include_staff_media: Option<bool>,
    #[serde(rename = "includeCharacters")]
    pub include_characters: Option<bool>,
    #[serde(rename = "includeCharacterMedia")]
    pub include_character_media: Option<bool>,
    #[serde(rename = "includeSubmitter")]
    pub include_submitter: Option<bool>,
    #[serde(rename = "includeSubmissionStatus")]
    pub include_submission_status: Option<bool>,
    #[serde(rename = "includeSubmissionNotes")]
    pub include_submission_notes: Option<bool>,
    #[serde(rename = "includeModNotes")]
    pub include_mod_notes: Option<bool>,
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

/// Options for fetching a single staff member by ID.
#[skip_serializing_none]
#[derive(Default, Debug, Serialize, Deserialize)]
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
        options: &FetchStaffOptions,
    ) -> Result<Page<Vec<Staff>>, AniListError> {
        let query = staff::FETCH;
        self.client.fetch(query, Some(options)).await
    }

    pub async fn fetch_one(&self, options: &FetchStaffOneOptions) -> Result<Staff, AniListError> {
        let query = staff::FETCH_ONE;
        self.client.fetch(query, Some(options)).await
    }

    // Convenience functions

    /// Get popular staff sorted by favorites
    pub async fn get_popular(
        &self,
        page: Option<i32>,
        per_page: Option<i32>,
    ) -> Result<Page<Vec<Staff>>, AniListError> {
        self.fetch(&FetchStaffOptions {
            sort: Some(vec![StaffSort::FavouritesDesc]),
            page,
            per_page,
            ..Default::default()
        })
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
        self.fetch(&FetchStaffOptions {
            search: Some(query.to_string()),
            sort: Some(vec![StaffSort::SearchMatch]),
            page,
            per_page,
            ..Default::default()
        })
        .await
    }

    /// Get staff by ID
    pub async fn get_by_id(&self, id: i32) -> Result<Staff, AniListError> {
        self.fetch_one(&FetchStaffOneOptions {
            id: Some(id),
            ..Default::default()
        })
        .await
    }

    /// Get staff with birthday today
    pub async fn get_today_birthday(
        &self,
        page: Option<i32>,
        per_page: Option<i32>,
    ) -> Result<Page<Vec<Staff>>, AniListError> {
        self.fetch(&FetchStaffOptions {
            is_birthday: Some(true),
            sort: Some(vec![StaffSort::FavouritesDesc]),
            page,
            per_page,
            ..Default::default()
        })
        .await
    }
}
