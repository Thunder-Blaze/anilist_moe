use crate::enums::media::MediaType;
use crate::enums::media_list::{MediaListSort, MediaListStatus};
use crate::errors::AniListError;
use crate::objects::common::{Deleted, FuzzyDate};
use crate::objects::media_list::{MediaList, MediaListCollection};
use crate::objects::responses::Page;
use crate::options;
#[cfg(feature = "pagination")]
use crate::pagination::Paginated;
use crate::{client::AniListClient, queries::medialist};
use serde::Serialize;
use serde_with::skip_serializing_none;
use std::borrow::Cow;

/// Options for fetching media list entries.
#[skip_serializing_none]
#[derive(Default, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FetchMediaListOptions<'a> {
    pub id: Option<u32>,
    pub user_id: Option<u32>,
    pub user_name: Option<Cow<'a, str>>,
    pub media_type: Option<MediaType>,
    pub status: Option<MediaListStatus>,
    pub media_id: Option<u32>,
    pub is_following: Option<bool>,
    pub notes: Option<Cow<'a, str>>,
    pub started_at: Option<i32>,
    pub completed_at: Option<i32>,
    pub compare_with_auth_list: Option<bool>,
    pub user_id_in: Option<Cow<'a, [u32]>>,
    pub status_in: Option<Cow<'a, [MediaListStatus]>>,
    pub status_not_in: Option<Cow<'a, [MediaListStatus]>>,
    pub status_not: Option<MediaListStatus>,
    pub media_id_in: Option<Cow<'a, [u32]>>,
    pub media_id_not_in: Option<Cow<'a, [u32]>>,
    pub notes_like: Option<Cow<'a, str>>,
    pub started_at_greater: Option<i32>,
    pub started_at_lesser: Option<i32>,
    pub started_at_like: Option<Cow<'a, str>>,
    pub completed_at_greater: Option<i32>,
    pub completed_at_lesser: Option<i32>,
    pub completed_at_like: Option<Cow<'a, str>>,
    /// Sort order. The unique key (id) is appended, so paging stays stable;
    /// leaving this unset sorts by the same key alone.
    #[serde(
        serialize_with = "crate::options::serialize_sort",
        skip_serializing_if = "crate::options::skip_never"
    )]
    pub sort: Option<Cow<'a, [MediaListSort]>>,
    pub page: Option<i32>,
    pub per_page: Option<i32>,
}

/// Options for fetching a whole list at once, grouped by status.
///
/// This is the endpoint to reach for when you want everything a user has rather
/// than one page of it: AniList returns the entries grouped into the lists a user
/// sees on the site, with a `chunk`/`perChunk` pair for lists too big for one
/// answer.
///
/// The API insists on a user (id or name) and a [`media_type`](Self::media_type),
/// even though the schema marks both as optional.
#[skip_serializing_none]
#[derive(Default, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FetchMediaListCollectionOptions<'a> {
    pub user_id: Option<u32>,
    pub user_name: Option<Cow<'a, str>>,
    pub media_type: Option<MediaType>,
    pub status: Option<MediaListStatus>,
    pub status_in: Option<Cow<'a, [MediaListStatus]>>,
    pub status_not: Option<MediaListStatus>,
    pub status_not_in: Option<Cow<'a, [MediaListStatus]>>,
    pub notes: Option<Cow<'a, str>>,
    pub notes_like: Option<Cow<'a, str>>,
    pub started_at: Option<i32>,
    pub started_at_greater: Option<i32>,
    pub started_at_lesser: Option<i32>,
    pub started_at_like: Option<Cow<'a, str>>,
    pub completed_at: Option<i32>,
    pub completed_at_greater: Option<i32>,
    pub completed_at_lesser: Option<i32>,
    pub completed_at_like: Option<Cow<'a, str>>,
    pub force_single_completed_list: Option<bool>,
    pub chunk: Option<i32>,
    pub per_chunk: Option<i32>,
    pub sort: Option<Cow<'a, [MediaListSort]>>,
}

/// Options for deleting one of your custom lists.
///
/// Both fields are required: the API rejects a delete without the list's name and
/// the media type it belongs to.
#[skip_serializing_none]
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteCustomListOptions<'a> {
    pub custom_list: Cow<'a, str>,
    pub media_type: MediaType,
}

/// Options for creating or updating a media list entry.
#[skip_serializing_none]
#[derive(Default, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SaveMediaListOptions<'a> {
    pub id: Option<u32>,
    pub media_id: Option<u32>,
    pub status: Option<MediaListStatus>,
    pub score: Option<f64>,
    pub score_raw: Option<i32>,
    pub progress: Option<i32>,
    pub progress_volumes: Option<i32>,
    pub repeat: Option<i32>,
    pub priority: Option<i32>,
    pub private: Option<bool>,
    pub notes: Option<Cow<'a, str>>,
    pub hidden_from_status_lists: Option<bool>,
    pub custom_lists: Option<Cow<'a, [&'a str]>>,
    pub advanced_scores: Option<Cow<'a, [f64]>>,
    pub started_at: Option<FuzzyDate>,
    pub completed_at: Option<FuzzyDate>,
}

/// Options for bulk updating multiple media list entries.
#[skip_serializing_none]
#[derive(Default, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SaveMediaListMultipleOptions<'a> {
    pub status: Option<MediaListStatus>,
    pub score: Option<f64>,
    pub score_raw: Option<i32>,
    pub progress: Option<i32>,
    pub progress_volumes: Option<i32>,
    pub repeat: Option<i32>,
    pub priority: Option<i32>,
    pub private: Option<bool>,
    pub notes: Option<Cow<'a, str>>,
    pub hidden_from_status_lists: Option<bool>,
    pub advanced_scores: Option<Cow<'a, [f64]>>,
    pub started_at: Option<FuzzyDate>,
    pub completed_at: Option<FuzzyDate>,
    pub ids: Cow<'a, [u32]>,
}

/// Options for deleting a media list entry.
#[derive(Default, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteMediaListOptions {
    pub id: u32,
}

/// Endpoint for media list operations.
pub struct MediaListEndpoint<'a> {
    client: &'a AniListClient,
}

impl<'a> MediaListEndpoint<'a> {
    pub fn new(client: &'a AniListClient) -> Self {
        Self { client }
    }

    pub async fn fetch(
        &self,
        options: &FetchMediaListOptions<'_>,
    ) -> Result<Page<Vec<MediaList>>, AniListError> {
        let query = medialist::FETCH;
        self.client.fetch(query, Some(&options)).await
    }

    pub async fn save(
        &self,
        options: &SaveMediaListOptions<'_>,
    ) -> Result<MediaList, AniListError> {
        let query = medialist::SAVE;
        self.client.fetch(query, Some(&options)).await
    }

    /// Fetch one entry by its own id, or by any other filter.
    ///
    /// The options are the ones [`fetch`](Self::fetch) takes; `page` and
    /// `per_page` are ignored, because the API answers with a single entry here.
    pub async fn fetch_one(
        &self,
        options: &FetchMediaListOptions<'_>,
    ) -> Result<MediaList, AniListError> {
        let query = medialist::FETCH_ONE;
        self.client.fetch(query, Some(&options)).await
    }

    /// Fetch a whole list, grouped into the lists a user sees on the site.
    pub async fn fetch_collection(
        &self,
        options: &FetchMediaListCollectionOptions<'_>,
    ) -> Result<MediaListCollection, AniListError> {
        let query = medialist::FETCH_COLLECTION;
        self.client.fetch(query, Some(&options)).await
    }

    /// Delete a custom list, and the list entries in it.
    pub async fn delete_custom_list(
        &self,
        options: &DeleteCustomListOptions<'_>,
    ) -> Result<Deleted, AniListError> {
        let query = medialist::DELETE_CUSTOM_LIST;
        self.client.fetch(query, Some(&options)).await
    }

    pub async fn save_multiple(
        &self,
        options: &SaveMediaListMultipleOptions<'_>,
    ) -> Result<Vec<MediaList>, AniListError> {
        let query = medialist::SAVE_MULTIPLE;
        self.client.fetch(query, Some(&options)).await
    }

    pub async fn delete(&self, options: &DeleteMediaListOptions) -> Result<bool, AniListError> {
        let query = medialist::DELETE;
        let response: Result<Deleted, AniListError> =
            self.client.fetch(query, Some(&options)).await;
        match response {
            Ok(res) => Ok(res.deleted.unwrap_or_default()),
            Err(err) => Err(err),
        }
    }

    // Convenience functions

    /// Get user's anime list by username
    pub async fn get_user_anime_list(
        &self,
        username: &str,
        status: Option<MediaListStatus>,
        page: Option<i32>,
        per_page: Option<i32>,
    ) -> Result<Page<Vec<MediaList>>, AniListError> {
        self.fetch(&options!(
            FetchMediaListOptions,
            user_name: Some(username),
            media_type: Some(MediaType::Anime),
            status,
            sort: Some(&[MediaListSort::UpdatedTimeDesc]),
            page,
            per_page,
        ))
        .await
    }

    /// Get user's manga list by username
    pub async fn get_user_manga_list(
        &self,
        username: &str,
        status: Option<MediaListStatus>,
        page: Option<i32>,
        per_page: Option<i32>,
    ) -> Result<Page<Vec<MediaList>>, AniListError> {
        self.fetch(&options!(
            FetchMediaListOptions,
            user_name: Some(username),
            media_type: Some(MediaType::Manga),
            status,
            sort: Some(&[MediaListSort::UpdatedTimeDesc]),
            page,
            per_page,
        ))
        .await
    }

    /// Get current user's anime list (requires authentication)
    pub async fn get_my_anime_list(
        &self,
        status: Option<MediaListStatus>,
        page: Option<i32>,
        per_page: Option<i32>,
    ) -> Result<Page<Vec<MediaList>>, AniListError> {
        self.fetch(&options!(
            FetchMediaListOptions,
            media_type: Some(MediaType::Anime),
            status,
            sort: Some(&[MediaListSort::UpdatedTimeDesc]),
            page,
            per_page,
        ))
        .await
    }

    /// Get current user's manga list (requires authentication)
    pub async fn get_my_manga_list(
        &self,
        status: Option<MediaListStatus>,
        page: Option<i32>,
        per_page: Option<i32>,
    ) -> Result<Page<Vec<MediaList>>, AniListError> {
        self.fetch(&options!(
            FetchMediaListOptions,
            media_type: Some(MediaType::Manga),
            status,
            sort: Some(&[MediaListSort::UpdatedTimeDesc]),
            page,
            per_page,
        ))
        .await
    }

    /// Get user's currently watching anime
    pub async fn get_watching(
        &self,
        user_name: Option<&str>,
        page: Option<i32>,
        per_page: Option<i32>,
    ) -> Result<Page<Vec<MediaList>>, AniListError> {
        self.fetch(&options!(
            FetchMediaListOptions,
            user_name,
            media_type: Some(MediaType::Anime),
            status: Some(MediaListStatus::Current),
            sort: Some(&[MediaListSort::UpdatedTimeDesc]),
            page,
            per_page,
        ))
        .await
    }

    /// Get user's currently reading manga
    pub async fn get_reading(
        &self,
        user_name: Option<&str>,
        page: Option<i32>,
        per_page: Option<i32>,
    ) -> Result<Page<Vec<MediaList>>, AniListError> {
        self.fetch(&options!(
            FetchMediaListOptions,
            user_name,
            media_type: Some(MediaType::Manga),
            status: Some(MediaListStatus::Current),
            sort: Some(&[MediaListSort::UpdatedTimeDesc]),
            page,
            per_page,
        ))
        .await
    }

    /// Get user's completed anime
    pub async fn get_completed_anime(
        &self,
        user_name: Option<&str>,
        page: Option<i32>,
        per_page: Option<i32>,
    ) -> Result<Page<Vec<MediaList>>, AniListError> {
        self.fetch(&options!(
            FetchMediaListOptions,
            user_name,
            media_type: Some(MediaType::Anime),
            status: Some(MediaListStatus::Completed),
            sort: Some(&[MediaListSort::ScoreDesc]),
            page,
            per_page,
        ))
        .await
    }

    /// Get user's completed manga
    pub async fn get_completed_manga(
        &self,
        user_name: Option<&str>,
        page: Option<i32>,
        per_page: Option<i32>,
    ) -> Result<Page<Vec<MediaList>>, AniListError> {
        self.fetch(&options!(
            FetchMediaListOptions,
            user_name,
            media_type: Some(MediaType::Manga),
            status: Some(MediaListStatus::Completed),
            sort: Some(&[MediaListSort::ScoreDesc]),
            page,
            per_page,
        ))
        .await
    }

    /// Get user's plan to watch list
    pub async fn get_plan_to_watch(
        &self,
        user_name: Option<&str>,
        page: Option<i32>,
        per_page: Option<i32>,
    ) -> Result<Page<Vec<MediaList>>, AniListError> {
        self.fetch(&options!(
            FetchMediaListOptions,
            user_name,
            media_type: Some(MediaType::Anime),
            status: Some(MediaListStatus::Planning),
            sort: Some(&[MediaListSort::UpdatedTimeDesc]),
            page,
            per_page,
        ))
        .await
    }

    /// Get user's plan to read list
    pub async fn get_plan_to_read(
        &self,
        user_name: Option<&str>,
        page: Option<i32>,
        per_page: Option<i32>,
    ) -> Result<Page<Vec<MediaList>>, AniListError> {
        self.fetch(&options!(
            FetchMediaListOptions,
            user_name,
            media_type: Some(MediaType::Manga),
            status: Some(MediaListStatus::Planning),
            sort: Some(&[MediaListSort::UpdatedTimeDesc]),
            page,
            per_page,
        ))
        .await
    }

    /// Add anime to list
    pub async fn add_anime(
        &self,
        media_id: u32,
        status: MediaListStatus,
    ) -> Result<MediaList, AniListError> {
        self.save(&options!(
            SaveMediaListOptions,
            media_id: Some(media_id),
            status: Some(status),
        ))
        .await
    }

    /// Add manga to list
    pub async fn add_manga(
        &self,
        media_id: u32,
        status: MediaListStatus,
    ) -> Result<MediaList, AniListError> {
        self.save(&options!(
            SaveMediaListOptions,
            media_id: Some(media_id),
            status: Some(status),
        ))
        .await
    }

    /// Update progress for an entry
    pub async fn update_progress(
        &self,
        entry_id: u32,
        progress: i32,
    ) -> Result<MediaList, AniListError> {
        self.save(&options!(
            SaveMediaListOptions,
            id: Some(entry_id),
            progress: Some(progress),
        ))
        .await
    }

    /// Update score for an entry
    pub async fn update_score(&self, entry_id: u32, score: f64) -> Result<MediaList, AniListError> {
        self.save(&options!(
            SaveMediaListOptions,
            id: Some(entry_id),
            score: Some(score),
        ))
        .await
    }

    /// Update status for an entry
    pub async fn update_status(
        &self,
        entry_id: u32,
        status: MediaListStatus,
    ) -> Result<MediaList, AniListError> {
        self.save(&options!(
            SaveMediaListOptions,
            id: Some(entry_id),
            status: Some(status),
        ))
        .await
    }

    /// Delete a media list entry
    pub async fn delete_entry(&self, entry_id: u32) -> Result<bool, AniListError> {
        self.delete(&options!(
            DeleteMediaListOptions,
            id => entry_id,
        ))
        .await
    }
}

#[cfg(feature = "pagination")]
impl<'a> Paginated for FetchMediaListOptions<'a> {
    type Item = MediaList;

    async fn fetch(&self, client: &AniListClient) -> Result<Page<Vec<MediaList>>, AniListError> {
        client.medialist().fetch(self).await
    }

    fn set_page(&mut self, page: i32) {
        self.page = Some(page);
    }
}
