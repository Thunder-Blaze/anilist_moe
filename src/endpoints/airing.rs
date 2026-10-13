use crate::enums::airing::AiringSort;
use crate::errors::AniListError;
use crate::objects::airing::AiringSchedule;
use crate::objects::responses::Page;
#[cfg(feature = "pagination")]
use crate::pagination::Paginated;
use crate::{client::AniListClient, queries::airing};
use serde::Serialize;
use serde_with::skip_serializing_none;
use std::borrow::Cow;
use typed_builder::TypedBuilder;

/// Options for fetching airing schedule information.
#[skip_serializing_none]
#[derive(Default, Debug, Serialize, TypedBuilder)]
#[builder(field_defaults(
    default,
    setter(strip_option(ignore_invalid, fallback_suffix = "_opt"))
))]
#[serde(rename_all = "camelCase")]
pub struct FetchAiringOptions<'a> {
    id: Option<u32>,
    media_id: Option<u32>,
    episode: Option<i32>,
    airing_at: Option<i32>,
    not_yet_aired: Option<bool>,
    id_not: Option<u32>,
    id_in: Option<Cow<'a, [u32]>>,
    id_not_in: Option<Cow<'a, [u32]>>,
    media_id_not: Option<u32>,
    media_id_in: Option<Cow<'a, [u32]>>,
    media_id_not_in: Option<Cow<'a, [u32]>>,
    episode_not: Option<i32>,
    episode_in: Option<Cow<'a, [i32]>>,
    episode_not_in: Option<Cow<'a, [i32]>>,
    episode_greater: Option<i32>,
    episode_lesser: Option<i32>,
    airing_at_greater: Option<i32>,
    airing_at_lesser: Option<i32>,
    /// Sort order. The unique key (id) is appended, so paging stays stable;
    /// leaving this unset keeps the order the query document asks for.
    #[serde(
        serialize_with = "crate::options::serialize_sort",
        skip_serializing_if = "Option::is_none"
    )]
    sort: Option<Cow<'a, [AiringSort]>>,
    per_page: Option<i32>,
    page: Option<i32>,
    // HTML rendering options
    description_as_html: Option<bool>,
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

    /// Fetch one airing schedule by id, or by any other filter.
    ///
    /// The options are the ones [`fetch`](Self::fetch) takes; `page` and
    /// `per_page` are ignored, because the API answers with a single schedule here.
    pub async fn fetch_one(
        &self,
        options: &FetchAiringOptions<'_>,
    ) -> Result<AiringSchedule, AniListError> {
        let query = airing::FETCH_ONE;
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

        self.fetch(
            &FetchAiringOptions::builder()
                .airing_at_greater(now)
                .sort(Cow::Borrowed(&[AiringSort::Time]))
                .page_opt(page)
                .per_page_opt(per_page)
                .build(),
        )
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

        self.fetch(
            &FetchAiringOptions::builder()
                .airing_at_lesser(now)
                .sort(Cow::Borrowed(&[AiringSort::TimeDesc]))
                .page_opt(page)
                .per_page_opt(per_page)
                .build(),
        )
        .await
    }

    /// Get airing schedule for a specific media ID
    pub async fn get_by_media_id(
        &self,
        media_id: u32,
        page: Option<i32>,
        per_page: Option<i32>,
    ) -> Result<Page<Vec<AiringSchedule>>, AniListError> {
        self.fetch(
            &FetchAiringOptions::builder()
                .media_id(media_id)
                .sort(Cow::Borrowed(&[AiringSort::Time]))
                .page_opt(page)
                .per_page_opt(per_page)
                .build(),
        )
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
