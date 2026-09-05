use crate::enums::staff::StaffSort;
use crate::errors::AniListError;
use crate::objects::responses::Page;
use crate::objects::staff::Staff;
use crate::{client::AniListClient, queries::staff};
use serde::Serialize;
use serde_with::skip_serializing_none;

/// Options for fetching staff members.
#[skip_serializing_none]
#[derive(Default, Debug, Serialize)]
pub struct FetchStaffOptions<'a> {
    pub id: Option<i32>,
    #[serde(rename = "isBirthday")]
    pub is_birthday: Option<bool>,
    pub search: Option<&'a str>,
    #[serde(rename = "id_not")]
    pub id_not: Option<i32>,
    #[serde(rename = "id_in")]
    pub id_in: Option<&'a [i32]>,
    #[serde(rename = "id_not_in")]
    pub id_not_in: Option<&'a [i32]>,
    pub sort: Option<&'a [StaffSort]>,
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
#[derive(Default, Debug, Serialize)]
pub struct FetchStaffOneOptions<'a> {
    pub id: Option<i32>,
    #[serde(rename = "isBirthday")]
    pub is_birthday: Option<bool>,
    pub search: Option<&'a str>,
    #[serde(rename = "id_not")]
    pub id_not: Option<i32>,
    #[serde(rename = "id_in")]
    pub id_in: Option<&'a [i32]>,
    #[serde(rename = "id_not_in")]
    pub id_not_in: Option<&'a [i32]>,
    pub sort: Option<&'a [StaffSort]>,
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
pub struct StaffEndpoint<'a> {
    client: &'a AniListClient,
}

impl<'a> StaffEndpoint<'a> {
    pub fn new(client: &'a AniListClient) -> Self {
        Self { client }
    }

    pub async fn fetch(
        &self,
        options: FetchStaffOptions<'a>,
    ) -> Result<Page<Vec<Staff>>, AniListError> {
        let query = staff::FETCH;
        self.client.fetch(query, Some(&options)).await
    }

    pub async fn fetch_one(
        &self,
        options: FetchStaffOneOptions<'_>,
    ) -> Result<Staff, AniListError> {
        let query = staff::FETCH_ONE;
        self.client.fetch(query, Some(&options)).await
    }

    // Convenience functions

    /// Get popular staff sorted by favorites
    pub async fn get_popular(
        &self,
        page: Option<i32>,
        per_page: Option<i32>,
    ) -> Result<Page<Vec<Staff>>, AniListError> {
        self.fetch(FetchStaffOptions {
            sort: Some(&[StaffSort::FavouritesDesc]),
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
        self.fetch(FetchStaffOptions {
            search: Some(query),
            sort: Some(&[StaffSort::SearchMatch]),
            page,
            per_page,
            ..Default::default()
        })
        .await
    }

    /// Get staff by ID
    pub async fn get_by_id(&self, id: i32) -> Result<Staff, AniListError> {
        self.fetch_one(FetchStaffOneOptions {
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
        self.fetch(FetchStaffOptions {
            is_birthday: Some(true),
            sort: Some(&[StaffSort::FavouritesDesc]),
            page,
            per_page,
            ..Default::default()
        })
        .await
    }
}
