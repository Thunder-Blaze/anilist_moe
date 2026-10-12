use crate::enums::media::MediaType;
use crate::enums::review::{ReviewRating, ReviewSort};
use crate::errors::AniListError;
use crate::objects::common::Deleted;
use crate::objects::responses::Page;
use crate::objects::review::Review;
#[cfg(feature = "pagination")]
use crate::pagination::Paginated;
use crate::{client::AniListClient, queries::review};
use serde::Serialize;
use serde_with::skip_serializing_none;
use std::borrow::Cow;
use typed_builder::TypedBuilder;

/// Options for fetching reviews.
#[skip_serializing_none]
#[derive(Default, Debug, Serialize, TypedBuilder)]
#[builder(field_defaults(default))]
#[serde(rename_all = "camelCase")]
pub struct FetchReviewOptions<'a> {
    page: Option<i32>,
    per_page: Option<i32>,
    id: Option<u32>,
    media_id: Option<u32>,
    user_id: Option<u32>,
    media_type: Option<MediaType>,
    /// Sort order. The unique key (id) is appended, so paging stays stable;
    /// leaving this unset sorts by the same key alone.
    #[serde(
        serialize_with = "crate::options::serialize_sort",
        skip_serializing_if = "crate::options::skip_never"
    )]
    sort: Option<Cow<'a, [ReviewSort]>>,
    // HTML rendering options
    body_as_html: Option<bool>,
    description_as_html: Option<bool>,
}

/// Options for creating or updating a review.
#[skip_serializing_none]
#[derive(Default, Debug, Serialize, TypedBuilder)]
#[builder(field_defaults(default))]
#[serde(rename_all = "camelCase")]
pub struct SaveReviewOptions<'a> {
    id: Option<u32>,
    media_id: u32,
    score: Option<i32>,
    summary: Option<Cow<'a, str>>,
    body: Option<Cow<'a, str>>,
    private: Option<bool>,
    // HTML rendering options
    body_as_html: Option<bool>,
}

/// Options for deleting a review.
#[derive(Default, Debug, Serialize, TypedBuilder)]
#[builder(field_defaults(default))]
#[serde(rename_all = "camelCase")]
pub struct DeleteReviewOptions {
    id: u32,
}

/// Options for rating a review.
#[derive(Default, Debug, Serialize, TypedBuilder)]
#[builder(field_defaults(default))]
#[serde(rename_all = "camelCase")]
pub struct RateReviewOptions {
    review_id: u32,
    rating: ReviewRating,
}

/// Endpoint for review operations.
pub struct ReviewEndpoint<'a> {
    pub client: &'a AniListClient,
}

impl<'a> ReviewEndpoint<'a> {
    pub fn new(client: &'a AniListClient) -> Self {
        Self { client }
    }

    pub async fn fetch(
        &self,
        options: &FetchReviewOptions<'_>,
    ) -> Result<Page<Vec<Review>>, AniListError> {
        let query = review::FETCH;
        self.client.fetch(query, Some(&options)).await
    }

    pub async fn save(&self, options: &SaveReviewOptions<'_>) -> Result<Review, AniListError> {
        let query = review::SAVE;
        self.client.fetch(query, Some(&options)).await
    }

    pub async fn delete(&self, options: &DeleteReviewOptions) -> Result<bool, AniListError> {
        let query = review::DELETE;
        let response: Result<Deleted, AniListError> =
            self.client.fetch(query, Some(&options)).await;
        match response {
            Ok(res) => Ok(res.deleted.unwrap_or_default()),
            Err(err) => Err(err),
        }
    }

    pub async fn rate(&self, options: &RateReviewOptions) -> Result<Review, AniListError> {
        let query = review::RATE;
        self.client.fetch(query, Some(&options)).await
    }

    // Convenience functions

    /// Get reviews for a specific media
    pub async fn get_by_media_id(
        &self,
        media_id: u32,
        page: Option<i32>,
        per_page: Option<i32>,
    ) -> Result<Page<Vec<Review>>, AniListError> {
        self.fetch(
            &FetchReviewOptions::builder()
                .media_id(Some(media_id))
                .page(page)
                .per_page(per_page)
                .sort(Some(Cow::Borrowed(&[ReviewSort::RatingDesc])))
                .build(),
        )
        .await
    }

    /// Get reviews by a specific user
    pub async fn get_by_user_id(
        &self,
        user_id: u32,
        page: Option<i32>,
        per_page: Option<i32>,
    ) -> Result<Page<Vec<Review>>, AniListError> {
        self.fetch(
            &FetchReviewOptions::builder()
                .user_id(Some(user_id))
                .page(page)
                .per_page(per_page)
                .sort(Some(Cow::Borrowed(&[ReviewSort::IdDesc])))
                .build(),
        )
        .await
    }

    /// Get recent reviews
    pub async fn get_recent(
        &self,
        page: Option<i32>,
        per_page: Option<i32>,
    ) -> Result<Page<Vec<Review>>, AniListError> {
        self.fetch(
            &FetchReviewOptions::builder()
                .page(page)
                .per_page(per_page)
                .sort(Some(Cow::Borrowed(&[ReviewSort::IdDesc])))
                .build(),
        )
        .await
    }

    /// Get review by ID
    pub async fn get_by_id(&self, id: u32) -> Result<Review, AniListError> {
        let mut response = self
            .fetch(&FetchReviewOptions::builder().id(Some(id)).build())
            .await?;

        if response.data.is_empty() {
            return Err(AniListError::NotFound);
        }

        Ok(response.data.swap_remove(0))
    }

    /// Create a new review
    pub async fn create(
        &self,
        media_id: u32,
        score: i32,
        summary: &str,
        body: &str,
        private: Option<bool>,
    ) -> Result<Review, AniListError> {
        self.save(
            &SaveReviewOptions::builder()
                .media_id(media_id)
                .score(Some(score))
                .summary(Some(Cow::Borrowed(summary)))
                .body(Some(Cow::Borrowed(body)))
                .private(private)
                .build(),
        )
        .await
    }

    /// Delete a review
    pub async fn delete_review(&self, id: u32) -> Result<bool, AniListError> {
        self.delete(&DeleteReviewOptions::builder().id(id).build())
            .await
    }

    /// Rate a review
    pub async fn rate_review(
        &self,
        review_id: u32,
        rating: ReviewRating,
    ) -> Result<Review, AniListError> {
        self.rate(
            &RateReviewOptions::builder()
                .review_id(review_id)
                .rating(rating)
                .build(),
        )
        .await
    }
}

#[cfg(feature = "pagination")]
impl<'a> Paginated for FetchReviewOptions<'a> {
    type Item = Review;

    async fn fetch(&self, client: &AniListClient) -> Result<Page<Vec<Review>>, AniListError> {
        client.review().fetch(self).await
    }

    fn set_page(&mut self, page: i32) {
        self.page = Some(page);
    }
}
