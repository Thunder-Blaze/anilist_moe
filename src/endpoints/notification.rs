use crate::enums::notification::NotificationType;
use crate::errors::AniListError;
use crate::objects::responses::Page;
#[cfg(feature = "pagination")]
use crate::pagination::Paginated;
use crate::unions::notification::NotificationUnion;
use crate::{client::AniListClient, queries::notification};
use serde::Serialize;
use serde_with::skip_serializing_none;
use std::borrow::Cow;
use typed_builder::TypedBuilder;

/// Options for searching and filtering notifications.
#[skip_serializing_none]
#[derive(Default, Debug, Serialize, TypedBuilder)]
#[builder(field_defaults(default))]
#[serde(rename_all = "camelCase")]
pub struct NotificationSearchOptions<'a> {
    page: Option<i32>,
    per_page: Option<i32>,
    notification_type: Option<NotificationType>,
    type_in: Option<Cow<'a, [NotificationType]>>,
    reset_notification_count: Option<bool>,
    // HTML rendering options
    message_as_html: Option<bool>,
    text_as_html: Option<bool>,
    comment_as_html: Option<bool>,
}

/// Endpoint for notification operations.
pub struct NotificationEndpoint<'a> {
    pub client: &'a AniListClient,
}

impl<'a> NotificationEndpoint<'a> {
    pub fn new(client: &'a AniListClient) -> Self {
        Self { client }
    }

    pub async fn fetch(
        &self,
        options: &NotificationSearchOptions<'a>,
    ) -> Result<Page<Vec<NotificationUnion>>, AniListError> {
        let query = notification::FETCH;
        self.client.fetch(query, Some(&options)).await
    }

    // Convenience functions

    /// Get all notifications (requires authentication)
    pub async fn get_all(
        &self,
        page: Option<i32>,
        per_page: Option<i32>,
    ) -> Result<Page<Vec<NotificationUnion>>, AniListError> {
        self.fetch(
            &NotificationSearchOptions::builder()
                .page(page)
                .per_page(per_page)
                .build(),
        )
        .await
    }

    /// Get unread notifications and mark them as read
    pub async fn get_and_mark_read(
        &self,
        page: Option<i32>,
        per_page: Option<i32>,
    ) -> Result<Page<Vec<NotificationUnion>>, AniListError> {
        self.fetch(
            &NotificationSearchOptions::builder()
                .page(page)
                .per_page(per_page)
                .reset_notification_count(Some(true))
                .build(),
        )
        .await
    }

    /// Get notifications of a specific type
    pub async fn get_by_type(
        &self,
        notification_type: NotificationType,
        page: Option<i32>,
        per_page: Option<i32>,
    ) -> Result<Page<Vec<NotificationUnion>>, AniListError> {
        self.fetch(
            &NotificationSearchOptions::builder()
                .notification_type(Some(notification_type))
                .page(page)
                .per_page(per_page)
                .build(),
        )
        .await
    }
}

#[cfg(feature = "pagination")]
impl<'a> Paginated for NotificationSearchOptions<'a> {
    type Item = NotificationUnion;

    async fn fetch(
        &self,
        client: &AniListClient,
    ) -> Result<Page<Vec<NotificationUnion>>, AniListError> {
        client.notification().fetch(self).await
    }

    fn set_page(&mut self, page: i32) {
        self.page = Some(page);
    }
}
