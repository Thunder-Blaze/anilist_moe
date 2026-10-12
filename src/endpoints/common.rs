use crate::enums::likable::LikeableType;
use crate::errors::AniListError;
use crate::objects::favourites::Favourites;
use crate::objects::media::MediaTag;
use crate::objects::responses::Page;
use crate::objects::user::User;
use crate::options;
use crate::unions::likeable::LikeableUnion;
use crate::{client::AniListClient, queries::common};
use serde::Serialize;
use serde_with::skip_serializing_none;
use std::borrow::Cow;

/// Options for toggling a like on various entities.
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ToggleLikeOptions {
    pub id: u32,
    pub like_type: LikeableType,
}

/// Options for toggling follow status of a user.
#[derive(Default, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ToggleFollowOptions {
    pub user_id: u32,
}

/// Options for adding or removing favourites.
#[skip_serializing_none]
#[derive(Default, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ToggleFavouriteOptions {
    pub anime_id: Option<u32>,
    pub manga_id: Option<u32>,
    pub character_id: Option<u32>,
    pub staff_id: Option<u32>,
    pub studio_id: Option<u32>,
}

/// Options for the media tag list. `status` asks for tags above a rank.
#[derive(Default, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MediaTagsOptions {
    pub status: Option<i32>,
}

/// Options for listing the users who liked something.
#[skip_serializing_none]
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LikesOptions {
    pub likeable_id: u32,
    pub like_type: LikeableType,
    pub page: Option<i32>,
    pub per_page: Option<i32>,
}

/// Options for reordering the entries of a favourites list.
///
/// The `*_ids` arguments name the favourites to order and the `*_order` arguments
/// give the order, as the API describes it: "list of integers which the anime
/// should be ordered by (ascending)".
#[skip_serializing_none]
#[derive(Default, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateFavouriteOrderOptions<'a> {
    pub anime_ids: Option<Cow<'a, [u32]>>,
    pub manga_ids: Option<Cow<'a, [u32]>>,
    pub character_ids: Option<Cow<'a, [u32]>>,
    pub staff_ids: Option<Cow<'a, [u32]>>,
    pub studio_ids: Option<Cow<'a, [u32]>>,
    pub anime_order: Option<Cow<'a, [u32]>>,
    pub manga_order: Option<Cow<'a, [u32]>>,
    pub character_order: Option<Cow<'a, [u32]>>,
    pub staff_order: Option<Cow<'a, [u32]>>,
    pub studio_order: Option<Cow<'a, [u32]>>,
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
        self.toggle_follow(&options!(
            ToggleFollowOptions,
            user_id => user_id,
        ))
        .await
    }

    /// Add or remove anime from favorites
    pub async fn favourite_anime(&self, anime_id: u32) -> Result<Favourites, AniListError> {
        self.toggle_favourite(&options!(
            ToggleFavouriteOptions,
            anime_id: Some(anime_id),
        ))
        .await
    }

    /// Add or remove manga from favorites
    pub async fn favourite_manga(&self, manga_id: u32) -> Result<Favourites, AniListError> {
        self.toggle_favourite(&options!(
            ToggleFavouriteOptions,
            manga_id: Some(manga_id),
        ))
        .await
    }

    /// Add or remove character from favorites
    pub async fn favourite_character(&self, character_id: u32) -> Result<Favourites, AniListError> {
        self.toggle_favourite(&options!(
            ToggleFavouriteOptions,
            character_id: Some(character_id),
        ))
        .await
    }

    /// Add or remove staff from favorites
    pub async fn favourite_staff(&self, staff_id: u32) -> Result<Favourites, AniListError> {
        self.toggle_favourite(&options!(
            ToggleFavouriteOptions,
            staff_id: Some(staff_id),
        ))
        .await
    }

    /// Add or remove studio from favorites
    pub async fn favourite_studio(&self, studio_id: u32) -> Result<Favourites, AniListError> {
        self.toggle_favourite(&options!(
            ToggleFavouriteOptions,
            studio_id: Some(studio_id),
        ))
        .await
    }
}
