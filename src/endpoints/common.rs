use crate::enums::external_link::{ExternalLinkMediaType, ExternalLinkType};
use crate::enums::likable::LikeableType;
use crate::errors::AniListError;
use crate::objects::anichart::{AniChartHighlight, AniChartUser};
use crate::objects::common::ParsedMarkdown;
use crate::objects::favourites::Favourites;
use crate::objects::media::{MediaExternalLink, MediaTag};
use crate::objects::responses::Page;
use crate::objects::stats::SiteStatistics;
use crate::objects::user::User;
use crate::unions::likeable::LikeableUnion;
use crate::{client::AniListClient, queries::common};
use serde::Serialize;
use serde_with::skip_serializing_none;
use std::borrow::Cow;
use typed_builder::TypedBuilder;

/// Options for toggling a like on various entities.
#[derive(Serialize, TypedBuilder)]
#[serde(rename_all = "camelCase")]
pub struct ToggleLikeOptions {
    id: u32,
    like_type: LikeableType,
}

/// Options for toggling follow status of a user.
#[derive(Default, Debug, Serialize, TypedBuilder)]
#[serde(rename_all = "camelCase")]
pub struct ToggleFollowOptions {
    user_id: u32,
}

/// Options for adding or removing favourites.
#[skip_serializing_none]
#[derive(Default, Debug, Serialize, TypedBuilder)]
#[builder(field_defaults(
    default,
    setter(strip_option(ignore_invalid, fallback_suffix = "_opt"))
))]
#[serde(rename_all = "camelCase")]
pub struct ToggleFavouriteOptions {
    anime_id: Option<u32>,
    manga_id: Option<u32>,
    character_id: Option<u32>,
    staff_id: Option<u32>,
    studio_id: Option<u32>,
}

/// Options for rendering markdown the way AniList renders it.
#[derive(Debug, Serialize, TypedBuilder)]
#[serde(rename_all = "camelCase")]
pub struct MarkdownOptions<'a> {
    markdown: Cow<'a, str>,
}

/// Options for the external link source list.
#[skip_serializing_none]
#[derive(Default, Debug, Serialize, TypedBuilder)]
#[builder(field_defaults(
    default,
    setter(strip_option(ignore_invalid, fallback_suffix = "_opt"))
))]
#[serde(rename_all = "camelCase")]
pub struct ExternalLinkSourceOptions {
    id: Option<u32>,
    #[serde(rename = "type")]
    link_type: Option<ExternalLinkType>,
    media_type: Option<ExternalLinkMediaType>,
}

/// Options for the AniChart settings.
#[skip_serializing_none]
#[derive(Default, Debug, Serialize, TypedBuilder)]
#[builder(field_defaults(
    default,
    setter(strip_option(ignore_invalid, fallback_suffix = "_opt"))
))]
#[serde(rename_all = "camelCase")]
pub struct UpdateAniChartSettingsOptions<'a> {
    title_language: Option<Cow<'a, str>>,
    outgoing_link_provider: Option<Cow<'a, str>>,
    theme: Option<Cow<'a, str>>,
    sort: Option<Cow<'a, str>>,
}

/// Options for the AniChart highlights.
#[skip_serializing_none]
#[derive(Default, Debug, Serialize, TypedBuilder)]
#[builder(field_defaults(
    default,
    setter(strip_option(ignore_invalid, fallback_suffix = "_opt"))
))]
#[serde(rename_all = "camelCase")]
pub struct UpdateAniChartHighlightsOptions<'a> {
    highlights: Option<Cow<'a, [AniChartHighlight]>>,
}

/// Options for the media tag list. `status` asks for tags above a rank.
#[derive(Default, Debug, Serialize, TypedBuilder)]
#[builder(field_defaults(
    default,
    setter(strip_option(ignore_invalid, fallback_suffix = "_opt"))
))]
#[serde(rename_all = "camelCase")]
pub struct MediaTagsOptions {
    /// Mod only.
    status: Option<i32>,
}

/// Options for listing the users who liked something.
#[skip_serializing_none]
#[derive(Debug, Serialize, TypedBuilder)]
#[builder(field_defaults(
    default,
    setter(strip_option(ignore_invalid, fallback_suffix = "_opt"))
))]
#[serde(rename_all = "camelCase")]
pub struct LikesOptions {
    #[builder(!default)]
    likeable_id: u32,
    #[builder(!default)]
    like_type: LikeableType,
    page: Option<i32>,
    per_page: Option<i32>,
}

/// Options for reordering the entries of a favourites list.
///
/// The `*_ids` arguments name the favourites to order and the `*_order` arguments
/// give the order, as the API describes it: "list of integers which the anime
/// should be ordered by (ascending)".
#[skip_serializing_none]
#[derive(Default, Debug, Serialize, TypedBuilder)]
#[builder(field_defaults(
    default,
    setter(strip_option(ignore_invalid, fallback_suffix = "_opt"))
))]
#[serde(rename_all = "camelCase")]
pub struct UpdateFavouriteOrderOptions<'a> {
    anime_ids: Option<Cow<'a, [u32]>>,
    manga_ids: Option<Cow<'a, [u32]>>,
    character_ids: Option<Cow<'a, [u32]>>,
    staff_ids: Option<Cow<'a, [u32]>>,
    studio_ids: Option<Cow<'a, [u32]>>,
    anime_order: Option<Cow<'a, [u32]>>,
    manga_order: Option<Cow<'a, [u32]>>,
    character_order: Option<Cow<'a, [u32]>>,
    staff_order: Option<Cow<'a, [u32]>>,
    studio_order: Option<Cow<'a, [u32]>>,
}

/// Endpoint for common and general-purpose operations.
pub struct CommonEndpoint<'a> {
    pub client: &'a AniListClient,
}

impl<'a> CommonEndpoint<'a> {
    pub fn new(client: &'a AniListClient) -> Self {
        Self { client }
    }

    /// Every genre AniList knows about.
    ///
    /// The filter options take a genre name, and this is the list of names the
    /// API accepts.
    pub async fn genres(&self) -> Result<Vec<String>, AniListError> {
        let query = common::GENRES;
        self.client.fetch(query, Option::<&()>::None).await
    }

    /// Every media tag AniList knows about, newest first.
    /// The `status` argument is mod only.
    pub async fn media_tags(&self, status: Option<i32>) -> Result<Vec<MediaTag>, AniListError> {
        let query = common::MEDIA_TAGS;
        self.client
            .fetch(query, Some(&MediaTagsOptions { status }))
            .await
    }

    /// The users who liked something, one page at a time.
    pub async fn likes(&self, options: &LikesOptions) -> Result<Page<Vec<User>>, AniListError> {
        let query = common::LIKES;
        self.client.fetch(query, Some(&options)).await
    }

    /// Put a favourites list in the order you want.
    pub async fn update_favourite_order(
        &self,
        options: &UpdateFavouriteOrderOptions<'_>,
    ) -> Result<Favourites, AniListError> {
        let query = common::UPDATE_FAVOURITE_ORDER;
        self.client.fetch(query, Some(&options)).await
    }

    /// Render markdown the way AniList does. Requires a token.
    pub async fn markdown(&self, markdown: &str) -> Result<ParsedMarkdown, AniListError> {
        let query = common::MARKDOWN;
        self.client
            .fetch(
                query,
                Some(&MarkdownOptions {
                    markdown: Cow::Borrowed(markdown),
                }),
            )
            .await
    }

    /// The external link sources AniList knows about, which is what a
    /// `MediaExternalLink` refers to by `site`.
    pub async fn external_link_sources(
        &self,
        options: &ExternalLinkSourceOptions,
    ) -> Result<Vec<MediaExternalLink>, AniListError> {
        let query = common::EXTERNAL_LINK_SOURCES;
        self.client.fetch(query, Some(&options)).await
    }

    /// Site-wide counts, as a series per entity type.
    pub async fn site_statistics(&self) -> Result<SiteStatistics, AniListError> {
        let query = common::SITE_STATISTICS;
        self.client.fetch(query, Option::<&()>::None).await
    }

    /// The authenticated user's AniChart settings and highlights.
    pub async fn ani_chart_user(&self) -> Result<AniChartUser, AniListError> {
        let query = common::ANI_CHART_USER;
        self.client.fetch(query, Option::<&()>::None).await
    }

    /// Save AniChart settings. Requires a token.
    pub async fn update_ani_chart_settings(
        &self,
        options: &UpdateAniChartSettingsOptions<'_>,
    ) -> Result<serde_json::Value, AniListError> {
        let query = common::UPDATE_ANI_CHART_SETTINGS;
        self.client.fetch(query, Some(&options)).await
    }

    /// Save AniChart highlights. Requires a token.
    pub async fn update_ani_chart_highlights(
        &self,
        options: &UpdateAniChartHighlightsOptions<'_>,
    ) -> Result<serde_json::Value, AniListError> {
        let query = common::UPDATE_ANI_CHART_HIGHLIGHTS;
        self.client.fetch(query, Some(&options)).await
    }

    pub async fn toggle_like(
        &self,
        options: &ToggleLikeOptions,
    ) -> Result<LikeableUnion, AniListError> {
        let query = common::TOGGLE_LIKE;
        self.client.fetch(query, Some(&options)).await
    }

    pub async fn toggle_follow(&self, options: &ToggleFollowOptions) -> Result<User, AniListError> {
        let query = common::TOGGLE_FOLLOW;
        self.client.fetch(query, Some(&options)).await
    }

    pub async fn toggle_favourite(
        &self,
        options: &ToggleFavouriteOptions,
    ) -> Result<Favourites, AniListError> {
        let query = common::TOGGLE_FAVOURITE;
        self.client.fetch(query, Some(&options)).await
    }

    // Convenience functions

    /// Like or unlike an activity
    pub async fn like_activity(&self, id: u32) -> Result<LikeableUnion, AniListError> {
        self.toggle_like(&ToggleLikeOptions {
            id,
            like_type: LikeableType::Activity,
        })
        .await
    }

    /// Like or unlike an activity reply
    pub async fn like_activity_reply(&self, id: u32) -> Result<LikeableUnion, AniListError> {
        self.toggle_like(&ToggleLikeOptions {
            id,
            like_type: LikeableType::ActivityReply,
        })
        .await
    }

    /// Like or unlike a thread
    pub async fn like_thread(&self, id: u32) -> Result<LikeableUnion, AniListError> {
        self.toggle_like(&ToggleLikeOptions {
            id,
            like_type: LikeableType::Thread,
        })
        .await
    }

    /// Like or unlike a thread comment
    pub async fn like_thread_comment(&self, id: u32) -> Result<LikeableUnion, AniListError> {
        self.toggle_like(&ToggleLikeOptions {
            id,
            like_type: LikeableType::ThreadComment,
        })
        .await
    }

    /// Follow or unfollow a user
    pub async fn follow_user(&self, user_id: u32) -> Result<User, AniListError> {
        self.toggle_follow(&ToggleFollowOptions::builder().user_id(user_id).build())
            .await
    }

    /// Add or remove anime from favorites
    pub async fn favourite_anime(&self, anime_id: u32) -> Result<Favourites, AniListError> {
        self.toggle_favourite(&ToggleFavouriteOptions::builder().anime_id(anime_id).build())
            .await
    }

    /// Add or remove manga from favorites
    pub async fn favourite_manga(&self, manga_id: u32) -> Result<Favourites, AniListError> {
        self.toggle_favourite(&ToggleFavouriteOptions::builder().manga_id(manga_id).build())
            .await
    }

    /// Add or remove character from favorites
    pub async fn favourite_character(&self, character_id: u32) -> Result<Favourites, AniListError> {
        self.toggle_favourite(
            &ToggleFavouriteOptions::builder()
                .character_id(character_id)
                .build(),
        )
        .await
    }

    /// Add or remove staff from favorites
    pub async fn favourite_staff(&self, staff_id: u32) -> Result<Favourites, AniListError> {
        self.toggle_favourite(&ToggleFavouriteOptions::builder().staff_id(staff_id).build())
            .await
    }

    /// Add or remove studio from favorites
    pub async fn favourite_studio(&self, studio_id: u32) -> Result<Favourites, AniListError> {
        self.toggle_favourite(
            &ToggleFavouriteOptions::builder()
                .studio_id(studio_id)
                .build(),
        )
        .await
    }
}
