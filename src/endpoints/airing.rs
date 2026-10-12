use crate::enums::airing::AiringSort;
use crate::errors::AniListError;
use crate::objects::airing::AiringSchedule;
use crate::objects::responses::Page;
use crate::options;
#[cfg(feature = "pagination")]
use crate::pagination::Paginated;
use crate::{client::AniListClient, queries::airing};
use serde::Serialize;
use serde_with::skip_serializing_none;
use std::borrow::Cow;

/// Options for fetching airing schedule information.
#[skip_serializing_none]
#[derive(Default, Debug, Serialize)]
pub struct FetchAiringOptions<'a> {
    pub id: Option<i32>,
    #[serde(rename = "mediaId")]
    pub media_id: Option<i32>,
    pub episode: Option<i32>,
    #[serde(rename = "airingAt")]
    pub airing_at: Option<i32>,
    #[serde(rename = "notYetAired")]
    pub not_yet_aired: Option<bool>,
    #[serde(rename = "id_not")]
    pub id_not: Option<i32>,
    #[serde(rename = "id_in")]
    pub id_in: Option<Cow<'a, [i32]>>,
    #[serde(rename = "id_not_in")]
    pub id_not_in: Option<Cow<'a, [i32]>>,
    #[serde(rename = "mediaId_not")]
    pub media_id_not: Option<i32>,
    #[serde(rename = "mediaId_in")]
    pub media_id_in: Option<Cow<'a, [i32]>>,
    #[serde(rename = "mediaId_not_in")]
    pub media_id_not_in: Option<Cow<'a, [i32]>>,
    #[serde(rename = "episode_not")]
    pub episode_not: Option<i32>,
    #[serde(rename = "episode_in")]
    pub episode_in: Option<Cow<'a, [i32]>>,
    #[serde(rename = "episode_not_in")]
    pub episode_not_in: Option<Cow<'a, [i32]>>,
    #[serde(rename = "episode_greater")]
    pub episode_greater: Option<i32>,
    #[serde(rename = "episode_lesser")]
    pub episode_lesser: Option<i32>,
    #[serde(rename = "airingAt_greater")]
    pub airing_at_greater: Option<i32>,
    #[serde(rename = "airingAt_lesser")]
    pub airing_at_lesser: Option<i32>,
    /// Sort order. The unique key (id) is appended, so paging stays stable;
    /// leaving this unset keeps the order the query document asks for.
    #[serde(
        serialize_with = "crate::options::serialize_sort",
        skip_serializing_if = "Option::is_none"
    )]
    pub sort: Option<Cow<'a, [AiringSort]>>,
    #[serde(rename = "perPage")]
    pub per_page: Option<i32>,
    pub page: Option<i32>,
    // HTML rendering options
    #[serde(rename = "description_as_html")]
    pub description_as_html: Option<bool>,
}

/// Endpoint for airing schedule operations.
pub struct AiringEndpoint<'a> {
    pub client: &'a AniListClient,
}

impl<'a> AiringEndpoint<'a> {
    pub fn new(client: &'a AniListClient) -> Self {
        Self { client }
    }

    pub async fn fetch(
        &self,
        options: &FetchAiringOptions<'_>,
    ) -> Result<Page<Vec<AiringSchedule>>, AniListError> {
        let query = airing::FETCH;
        self.client.fetch(query, Some(&options)).await
    }

    // Convenience functions

    /// Get upcoming airing episodes
    pub async fn get_upcoming(
        &self,
        page: Option<i32>,
        per_page: Option<i32>,
    ) -> Result<Page<Vec<AiringSchedule>>, AniListError> {
        // A clock set before 1970 is not worth a panic: fall back to zero.
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|since_epoch| since_epoch.as_secs() as i32)
            .unwrap_or(0);

        self.fetch(&options!(
            FetchAiringOptions,
            airing_at_greater: Some(now),
            sort: Some(&[AiringSort::Time]),
            page,
            per_page,
        ))
        .await
    }

    /// Get recently aired episodes
    pub async fn get_recent(
        &self,
        page: Option<i32>,
        per_page: Option<i32>,
    ) -> Result<Page<Vec<AiringSchedule>>, AniListError> {
        // A clock set before 1970 is not worth a panic: fall back to zero.
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|since_epoch| since_epoch.as_secs() as i32)
            .unwrap_or(0);

        self.fetch(&options!(
            FetchAiringOptions,
            airing_at_lesser: Some(now),
            sort: Some(&[AiringSort::TimeDesc]),
            page,
            per_page,
        ))
        .await
    }

    /// Get airing schedule for a specific media ID
    pub async fn get_by_media_id(
        &self,
        media_id: i32,
        page: Option<i32>,
        per_page: Option<i32>,
    ) -> Result<Page<Vec<AiringSchedule>>, AniListError> {
        self.fetch(&options!(
            FetchAiringOptions,
            media_id: Some(media_id),
            sort: Some(&[AiringSort::Time]),
            page,
            per_page,
        ))
        .await
    }
}

#[cfg(feature = "pagination")]
impl<'a> Paginated for FetchAiringOptions<'a> {
    type Item = AiringSchedule;

    async fn fetch(
        &self,
        client: &AniListClient,
    ) -> Result<Page<Vec<AiringSchedule>>, AniListError> {
        client.airing().fetch(self).await
    }

    fn set_page(&mut self, page: i32) {
        self.page = Some(page);
    }
}
