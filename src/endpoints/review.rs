use crate::client::AniListClient;
use crate::error::AniListError;
use crate::models::social::Review;
use serde_json::json;
use std::collections::HashMap;

pub struct ReviewEndpoint {
    client: AniListClient,
}

impl ReviewEndpoint {
    pub(crate) fn new(client: AniListClient) -> Self {
        Self { client }
    }

    /// Get recent reviews
    pub async fn get_recent_reviews(
        &self,
        page: i32,
        per_page: i32,
    ) -> Result<Vec<Review>, AniListError> {
        let query = r#"
            query ($page: Int, $perPage: Int) {
                Page(page: $page, perPage: $perPage) {
                    reviews(sort: CREATED_AT_DESC) {
                        id
                        userId
                        mediaId
                        mediaType
                        summary
                        body
                        rating
                        ratingAmount
                        userRating
                        score
                        private
                        siteUrl
                        createdAt
                        updatedAt
                        user {
                            id
                            name
                            avatar {
                                large
                                medium
                            }
                        }
                        media {
                            id
                            title {
                                romaji
                                english
                                native
                                userPreferred
                            }
                            coverImage {
                                extraLarge
                                large
                                medium
                                color
                            }
                            bannerImage
                        }
                    }
                }
            }
        "#;

        let mut variables = HashMap::new();
        variables.insert("page".to_string(), json!(page));
        variables.insert("perPage".to_string(), json!(per_page));

        let response = self.client.query(query, Some(variables)).await?;
        let data = response["data"]["Page"]["reviews"].clone();
        let reviews: Vec<Review> = serde_json::from_value(data)?;
        Ok(reviews)
    }

    /// Get reviews by media ID
    pub async fn get_reviews_for_media(
        &self,
        media_id: i32,
        page: i32,
        per_page: i32,
    ) -> Result<Vec<Review>, AniListError> {
        let query = r#"
            query ($mediaId: Int, $page: Int, $perPage: Int) {
                Page(page: $page, perPage: $perPage) {
                    reviews(mediaId: $mediaId, sort: RATING_DESC) {
                        id
                        userId
                        mediaId
                        mediaType
                        summary
                        body
                        rating
                        ratingAmount
                        userRating
                        score
                        private
                        siteUrl
                        createdAt
                        updatedAt
                        user {
                            id
                            name
                            avatar {
                                large
                                medium
                            }
                        }
                        media {
                            id
                            title {
                                romaji
                                english
                                native
                                userPreferred
                            }
                            coverImage {
                                extraLarge
                                large
                                medium
                                color
                            }
                            bannerImage
                        }
                    }
                }
            }
        "#;

        let mut variables = HashMap::new();
        variables.insert("mediaId".to_string(), json!(media_id));
        variables.insert("page".to_string(), json!(page));
        variables.insert("perPage".to_string(), json!(per_page));

        let response = self.client.query(query, Some(variables)).await?;
        let data = response["data"]["Page"]["reviews"].clone();
        let reviews: Vec<Review> = serde_json::from_value(data)?;
        Ok(reviews)
    }

    /// Get reviews by user ID
    pub async fn get_reviews_by_user(
        &self,
        user_id: i32,
        page: i32,
        per_page: i32,
    ) -> Result<Vec<Review>, AniListError> {
        let query = r#"
            query ($userId: Int, $page: Int, $perPage: Int) {
                Page(page: $page, perPage: $perPage) {
                    reviews(userId: $userId, sort: CREATED_AT_DESC) {
                        id
                        userId
                        mediaId
                        mediaType
                        summary
                        body
                        rating
                        ratingAmount
                        userRating
                        score
                        private
                        siteUrl
                        createdAt
                        updatedAt
                        user {
                            id
                            name
                            avatar {
                                large
                                medium
                            }
                        }
                        media {
                            id
                            title {
                                romaji
                                english
                                native
                                userPreferred
                            }
                            coverImage {
                                extraLarge
                                large
                                medium
                                color
                            }
                            bannerImage
                        }
                    }
                }
            }
        "#;

        let mut variables = HashMap::new();
        variables.insert("userId".to_string(), json!(user_id));
        variables.insert("page".to_string(), json!(page));
        variables.insert("perPage".to_string(), json!(per_page));

        let response = self.client.query(query, Some(variables)).await?;
        let data = response["data"]["Page"]["reviews"].clone();
        let reviews: Vec<Review> = serde_json::from_value(data)?;
        Ok(reviews)
    }

    /// Get review by ID
    pub async fn get_review_by_id(&self, id: i32) -> Result<Review, AniListError> {
        let query = r#"
            query ($id: Int) {
                Review(id: $id) {
                    id
                    userId
                    mediaId
                    mediaType
                    summary
                    body
                    rating
                    ratingAmount
                    userRating
                    score
                    private
                    siteUrl
                    createdAt
                    updatedAt
                    user {
                        id
                        name
                        avatar {
                            large
                            medium
                        }
                    }
                    media {
                        id
                        title {
                            romaji
                            english
                            native
                            userPreferred
                        }
                        coverImage {
                            extraLarge
                            large
                            medium
                            color
                        }
                        bannerImage
                    }
                }
            }
        "#;

        let mut variables = HashMap::new();
        variables.insert("id".to_string(), json!(id));

        let response = self.client.query(query, Some(variables)).await?;
        let data = response["data"]["Review"].clone();
        let review: Review = serde_json::from_value(data)?;
        Ok(review)
    }

    /// Create or update a review (requires authentication)
    pub async fn save_review(
        &self,
        media_id: i32,
        body: &str,
        summary: Option<&str>,
        score: Option<i32>,
        private: Option<bool>,
    ) -> Result<Review, AniListError> {
        let query = r#"
            mutation ($mediaId: Int, $body: String, $summary: String, $score: Int, $private: Boolean) {
                SaveReview(mediaId: $mediaId, body: $body, summary: $summary, score: $score, private: $private) {
                    id
                    userId
                    mediaId
                    mediaType
                    summary
                    body
                    rating
                    ratingAmount
                    userRating
                    score
                    private
                    siteUrl
                    createdAt
                    updatedAt
                    user {
                        id
                        name
                        avatar {
                            large
                            medium
                        }
                    }
                    media {
                        id
                        title {
                            romaji
                            english
                            native
                            userPreferred
                        }
                        coverImage {
                            extraLarge
                            large
                            medium
                            color
                        }
                        bannerImage
                    }
                }
            }
        "#;

        let mut variables = HashMap::new();
        variables.insert("mediaId".to_string(), json!(media_id));
        variables.insert("body".to_string(), json!(body));
        if let Some(s) = summary {
            variables.insert("summary".to_string(), json!(s));
        }
        if let Some(sc) = score {
            variables.insert("score".to_string(), json!(sc));
        }
        if let Some(p) = private {
            variables.insert("private".to_string(), json!(p));
        }

        let response = self.client.query(query, Some(variables)).await?;
        let data = response["data"]["SaveReview"].clone();
        let review: Review = serde_json::from_value(data)?;
        Ok(review)
    }

    /// Rate a review (requires authentication)
    pub async fn rate_review(&self, review_id: i32, rating: &str) -> Result<Review, AniListError> {
        let query = r#"
            mutation ($reviewId: Int, $rating: ReviewRating) {
                RateReview(reviewId: $reviewId, rating: $rating) {
                    id
                    rating
                    ratingAmount
                    userRating
                    siteUrl
                }
            }
        "#;

        let mut variables = HashMap::new();
        variables.insert("reviewId".to_string(), json!(review_id));
        variables.insert("rating".to_string(), json!(rating));

        let response = self.client.query(query, Some(variables)).await?;
        let data = response["data"]["RateReview"].clone();
        let review: Review = serde_json::from_value(data)?;
        Ok(review)
    }

    /// Delete a review (requires authentication and ownership)
    pub async fn delete_review(&self, id: i32) -> Result<bool, AniListError> {
        let query = r#"
            mutation ($id: Int) {
                DeleteReview(id: $id) {
                    deleted
                }
            }
        "#;

        let mut variables = HashMap::new();
        variables.insert("id".to_string(), json!(id));

        let response = self.client.query(query, Some(variables)).await?;
        let deleted = response["data"]["DeleteReview"]["deleted"]
            .as_bool()
            .unwrap_or(false);
        Ok(deleted)
    }

    /// Get top rated reviews
    pub async fn get_top_rated_reviews(
        &self,
        page: i32,
        per_page: i32,
    ) -> Result<Vec<Review>, AniListError> {
        let query = r#"
            query ($page: Int, $perPage: Int) {
                Page(page: $page, perPage: $perPage) {
                    reviews(sort: RATING_DESC) {
                        id
                        userId
                        mediaId
                        mediaType
                        summary
                        body
                        rating
                        ratingAmount
                        userRating
                        score
                        private
                        siteUrl
                        createdAt
                        updatedAt
                        user {
                            id
                            name
                            avatar {
                                large
                                medium
                            }
                        }
                        media {
                            id
                            title {
                                romaji
                                english
                                native
                                userPreferred
                            }
                            coverImage {
                                extraLarge
                                large
                                medium
                                color
                            }
                            bannerImage
                        }
                    }
                }
            }
        "#;

        let mut variables = HashMap::new();
        variables.insert("page".to_string(), json!(page));
        variables.insert("perPage".to_string(), json!(per_page));

        let response = self.client.query(query, Some(variables)).await?;
        let data = response["data"]["Page"]["reviews"].clone();
        let reviews: Vec<Review> = serde_json::from_value(data)?;
        Ok(reviews)
    }
}
