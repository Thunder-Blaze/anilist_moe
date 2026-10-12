use crate::enums::media::MediaType;
use crate::enums::media_list::{MediaListSort, MediaListStatus};
use crate::errors::AniListError;
use crate::objects::common::{Deleted, FuzzyDate};
use crate::objects::media_list::{MediaList, MediaListCollection};
use crate::objects::responses::Page;
#[cfg(feature = "pagination")]
use crate::pagination::Paginated;
use crate::{client::AniListClient, queries::medialist};
use serde::Serialize;
use serde_with::skip_serializing_none;
use std::borrow::Cow;
use typed_builder::TypedBuilder;

/// Options for fetching media list entries.
#[skip_serializing_none]
#[derive(Default, Debug, Serialize, TypedBuilder)]
#[builder(field_defaults(default))]
#[serde(rename_all = "camelCase")]
pub struct FetchMediaListOptions<'a> {
    id: Option<u32>,
    user_id: Option<u32>,
    user_name: Option<Cow<'a, str>>,
    media_type: Option<MediaType>,
    status: Option<MediaListStatus>,
    media_id: Option<u32>,
    is_following: Option<bool>,
    notes: Option<Cow<'a, str>>,
    started_at: Option<i32>,
    completed_at: Option<i32>,
    /// Limits the answer to entries also on the authenticated user's list.
    ///
    /// The API requires a user id or name alongside it.
    compare_with_auth_list: Option<bool>,
    user_id_in: Option<Cow<'a, [u32]>>,
    status_in: Option<Cow<'a, [MediaListStatus]>>,
    status_not_in: Option<Cow<'a, [MediaListStatus]>>,
    status_not: Option<MediaListStatus>,
    media_id_in: Option<Cow<'a, [u32]>>,
    media_id_not_in: Option<Cow<'a, [u32]>>,
    notes_like: Option<Cow<'a, str>>,
    started_at_greater: Option<i32>,
    started_at_lesser: Option<i32>,
    started_at_like: Option<Cow<'a, str>>,
    completed_at_greater: Option<i32>,
    completed_at_lesser: Option<i32>,
    completed_at_like: Option<Cow<'a, str>>,
    /// Sort order. The unique key (id) is appended, so paging stays stable;
    /// leaving this unset sorts by the same key alone.
    #[serde(
        serialize_with = "crate::options::serialize_sort",
        skip_serializing_if = "crate::options::skip_never"
    )]
    sort: Option<Cow<'a, [MediaListSort]>>,
    page: Option<i32>,
    per_page: Option<i32>,
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
#[derive(Default, Debug, Serialize, TypedBuilder)]
#[builder(field_defaults(default))]
#[serde(rename_all = "camelCase")]
pub struct FetchMediaListCollectionOptions<'a> {
    user_id: Option<u32>,
    user_name: Option<Cow<'a, str>>,
    media_type: Option<MediaType>,
    status: Option<MediaListStatus>,
    status_in: Option<Cow<'a, [MediaListStatus]>>,
    status_not: Option<MediaListStatus>,
    status_not_in: Option<Cow<'a, [MediaListStatus]>>,
    notes: Option<Cow<'a, str>>,
    notes_like: Option<Cow<'a, str>>,
    started_at: Option<i32>,
    started_at_greater: Option<i32>,
    started_at_lesser: Option<i32>,
    started_at_like: Option<Cow<'a, str>>,
    completed_at: Option<i32>,
    completed_at_greater: Option<i32>,
    completed_at_lesser: Option<i32>,
    completed_at_like: Option<Cow<'a, str>>,
    force_single_completed_list: Option<bool>,
    chunk: Option<i32>,
    per_chunk: Option<i32>,
    sort: Option<Cow<'a, [MediaListSort]>>,
}

/// Options for deleting one of your custom lists.
///
/// Both fields are required: the API rejects a delete without the list's name and
/// the media type it belongs to.
#[skip_serializing_none]
#[derive(Debug, Serialize, TypedBuilder)]
#[serde(rename_all = "camelCase")]
pub struct DeleteCustomListOptions<'a> {
    #[builder(default)]
    custom_list: Cow<'a, str>,
    media_type: MediaType,
}

/// Options for creating or updating a media list entry.
#[skip_serializing_none]
#[derive(Default, Debug, Serialize, TypedBuilder)]
#[builder(field_defaults(default))]
#[serde(rename_all = "camelCase")]
pub struct SaveMediaListOptions<'a> {
    id: Option<u32>,
    media_id: Option<u32>,
    status: Option<MediaListStatus>,
    score: Option<f64>,
    score_raw: Option<i32>,
    progress: Option<i32>,
    progress_volumes: Option<i32>,
    repeat: Option<i32>,
    priority: Option<i32>,
    private: Option<bool>,
    notes: Option<Cow<'a, str>>,
    hidden_from_status_lists: Option<bool>,
    custom_lists: Option<Cow<'a, [&'a str]>>,
    advanced_scores: Option<Cow<'a, [f64]>>,
    started_at: Option<FuzzyDate>,
    completed_at: Option<FuzzyDate>,
}

/// Options for bulk updating multiple media list entries.
#[skip_serializing_none]
#[derive(Default, Debug, Serialize, TypedBuilder)]
#[builder(field_defaults(default))]
#[serde(rename_all = "camelCase")]
pub struct SaveMediaListMultipleOptions<'a> {
    status: Option<MediaListStatus>,
    score: Option<f64>,
    score_raw: Option<i32>,
    progress: Option<i32>,
    progress_volumes: Option<i32>,
    repeat: Option<i32>,
    priority: Option<i32>,
    private: Option<bool>,
    notes: Option<Cow<'a, str>>,
    hidden_from_status_lists: Option<bool>,
    advanced_scores: Option<Cow<'a, [f64]>>,
    started_at: Option<FuzzyDate>,
    completed_at: Option<FuzzyDate>,
    ids: Cow<'a, [u32]>,
}

/// Options for deleting a media list entry.
#[derive(Default, Debug, Serialize, TypedBuilder)]
#[builder(field_defaults(default))]
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
        self.fetch(
            &FetchMediaListOptions::builder()
                .user_name(Some(Cow::Borrowed(username)))
                .media_type(Some(MediaType::Anime))
                .status(status)
                .sort(Some(Cow::Borrowed(&[MediaListSort::UpdatedTimeDesc])))
                .page(page)
                .per_page(per_page)
                .build(),
        )
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
        self.fetch(
            &FetchMediaListOptions::builder()
                .user_name(Some(Cow::Borrowed(username)))
                .media_type(Some(MediaType::Manga))
                .status(status)
                .sort(Some(Cow::Borrowed(&[MediaListSort::UpdatedTimeDesc])))
                .page(page)
                .per_page(per_page)
                .build(),
        )
        .await
    }

    /// Get current user's anime list (requires authentication)
    pub async fn get_my_anime_list(
        &self,
        status: Option<MediaListStatus>,
        page: Option<i32>,
        per_page: Option<i32>,
    ) -> Result<Page<Vec<MediaList>>, AniListError> {
        self.fetch(
            &FetchMediaListOptions::builder()
                .media_type(Some(MediaType::Anime))
                .status(status)
                .sort(Some(Cow::Borrowed(&[MediaListSort::UpdatedTimeDesc])))
                .page(page)
                .per_page(per_page)
                .build(),
        )
        .await
    }

    /// Get current user's manga list (requires authentication)
    pub async fn get_my_manga_list(
        &self,
        status: Option<MediaListStatus>,
        page: Option<i32>,
        per_page: Option<i32>,
    ) -> Result<Page<Vec<MediaList>>, AniListError> {
        self.fetch(
            &FetchMediaListOptions::builder()
                .media_type(Some(MediaType::Manga))
                .status(status)
                .sort(Some(Cow::Borrowed(&[MediaListSort::UpdatedTimeDesc])))
                .page(page)
                .per_page(per_page)
                .build(),
        )
        .await
    }

    /// Get user's currently watching anime
    pub async fn get_watching(
        &self,
        user_name: Option<&str>,
        page: Option<i32>,
        per_page: Option<i32>,
    ) -> Result<Page<Vec<MediaList>>, AniListError> {
        self.fetch(
            &FetchMediaListOptions::builder()
                .user_name(user_name.map(Cow::Borrowed))
                .media_type(Some(MediaType::Anime))
                .status(Some(MediaListStatus::Current))
                .sort(Some(Cow::Borrowed(&[MediaListSort::UpdatedTimeDesc])))
                .page(page)
                .per_page(per_page)
                .build(),
        )
        .await
    }

    /// Get user's currently reading manga
    pub async fn get_reading(
        &self,
        user_name: Option<&str>,
        page: Option<i32>,
        per_page: Option<i32>,
    ) -> Result<Page<Vec<MediaList>>, AniListError> {
        self.fetch(
            &FetchMediaListOptions::builder()
                .user_name(user_name.map(Cow::Borrowed))
                .media_type(Some(MediaType::Manga))
                .status(Some(MediaListStatus::Current))
                .sort(Some(Cow::Borrowed(&[MediaListSort::UpdatedTimeDesc])))
                .page(page)
                .per_page(per_page)
                .build(),
        )
        .await
    }

    /// Get user's completed anime
    pub async fn get_completed_anime(
        &self,
        user_name: Option<&str>,
        page: Option<i32>,
        per_page: Option<i32>,
    ) -> Result<Page<Vec<MediaList>>, AniListError> {
        self.fetch(
            &FetchMediaListOptions::builder()
                .user_name(user_name.map(Cow::Borrowed))
                .media_type(Some(MediaType::Anime))
                .status(Some(MediaListStatus::Completed))
                .sort(Some(Cow::Borrowed(&[MediaListSort::ScoreDesc])))
                .page(page)
                .per_page(per_page)
                .build(),
        )
        .await
    }

    /// Get user's completed manga
    pub async fn get_completed_manga(
        &self,
        user_name: Option<&str>,
        page: Option<i32>,
        per_page: Option<i32>,
    ) -> Result<Page<Vec<MediaList>>, AniListError> {
        self.fetch(
            &FetchMediaListOptions::builder()
                .user_name(user_name.map(Cow::Borrowed))
                .media_type(Some(MediaType::Manga))
                .status(Some(MediaListStatus::Completed))
                .sort(Some(Cow::Borrowed(&[MediaListSort::ScoreDesc])))
                .page(page)
                .per_page(per_page)
                .build(),
        )
        .await
    }

    /// Get user's plan to watch list
    pub async fn get_plan_to_watch(
        &self,
        user_name: Option<&str>,
        page: Option<i32>,
        per_page: Option<i32>,
    ) -> Result<Page<Vec<MediaList>>, AniListError> {
        self.fetch(
            &FetchMediaListOptions::builder()
                .user_name(user_name.map(Cow::Borrowed))
                .media_type(Some(MediaType::Anime))
                .status(Some(MediaListStatus::Planning))
                .sort(Some(Cow::Borrowed(&[MediaListSort::UpdatedTimeDesc])))
                .page(page)
                .per_page(per_page)
                .build(),
        )
        .await
    }

    /// Get user's plan to read list
    pub async fn get_plan_to_read(
        &self,
        user_name: Option<&str>,
        page: Option<i32>,
        per_page: Option<i32>,
    ) -> Result<Page<Vec<MediaList>>, AniListError> {
        self.fetch(
            &FetchMediaListOptions::builder()
                .user_name(user_name.map(Cow::Borrowed))
                .media_type(Some(MediaType::Manga))
                .status(Some(MediaListStatus::Planning))
                .sort(Some(Cow::Borrowed(&[MediaListSort::UpdatedTimeDesc])))
                .page(page)
                .per_page(per_page)
                .build(),
        )
        .await
    }

    /// Add anime to list
    pub async fn add_anime(
        &self,
        media_id: u32,
        status: MediaListStatus,
    ) -> Result<MediaList, AniListError> {
        self.save(
            &SaveMediaListOptions::builder()
                .media_id(Some(media_id))
                .status(Some(status))
                .build(),
        )
        .await
    }

    /// Add manga to list
    pub async fn add_manga(
        &self,
        media_id: u32,
        status: MediaListStatus,
    ) -> Result<MediaList, AniListError> {
        self.save(
            &SaveMediaListOptions::builder()
                .media_id(Some(media_id))
                .status(Some(status))
                .build(),
        )
        .await
    }

    /// Update progress for an entry
    pub async fn update_progress(
        &self,
        entry_id: u32,
        progress: i32,
    ) -> Result<MediaList, AniListError> {
        self.save(
            &SaveMediaListOptions::builder()
                .id(Some(entry_id))
                .progress(Some(progress))
                .build(),
        )
        .await
    }

    /// Update score for an entry
    pub async fn update_score(&self, entry_id: u32, score: f64) -> Result<MediaList, AniListError> {
        self.save(
            &SaveMediaListOptions::builder()
                .id(Some(entry_id))
                .score(Some(score))
                .build(),
        )
        .await
    }

    /// Update status for an entry
    pub async fn update_status(
        &self,
        entry_id: u32,
        status: MediaListStatus,
    ) -> Result<MediaList, AniListError> {
        self.save(
            &SaveMediaListOptions::builder()
                .id(Some(entry_id))
                .status(Some(status))
                .build(),
        )
        .await
    }

    /// Delete a media list entry
    pub async fn delete_entry(&self, entry_id: u32) -> Result<bool, AniListError> {
        self.delete(&DeleteMediaListOptions::builder().id(entry_id).build())
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
