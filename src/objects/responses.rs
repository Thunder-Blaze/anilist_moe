use crate::objects::activity::{ActivityReply, MessageActivity, TextActivity};
use crate::objects::airing::AiringSchedule;
use crate::objects::character::Character;
use crate::objects::common::PageInfo;
use crate::objects::favourites::Favourites;
use crate::objects::media::Media;
use crate::objects::media_list::MediaList;
use crate::objects::recommendation::Recommendation;
use crate::objects::review::Review;
use crate::objects::staff::Staff;
use crate::objects::stats::UserStatisticTypes;
use crate::objects::studio::Studio;
use crate::objects::thread::{Thread, ThreadComment};
use crate::objects::user::{User, UserAvatar};
use crate::unions::activity::ActivityUnion;
use crate::unions::likeable::LikeableUnion;
use crate::unions::notification::NotificationUnion;
use serde::de::{self, Deserializer, MapAccess, Visitor};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use std::fmt;
use std::marker::PhantomData;

/// Top-level GraphQL response wrapper
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GraphQLResponse<T> {
    pub data: T,
}

/// Generic response wrapper for paginated data
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PageResponse<T> {
    #[serde(rename = "Page")]
    pub page: Page<T>,
}

/// Generic page structure
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Page<T> {
    pub page_info: Option<PageInfo>,
    pub data: T,
}

/// Generic viewer response wrapper
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ViewerResponse<T> {
    #[serde(rename = "Viewer")]
    pub viewer: T,
}

/// Specific response data types
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NotificationData {
    pub notifications: Vec<NotificationUnion>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MediaData {
    pub media: Vec<Media>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MediaListData {
    pub media_list: Vec<MediaList>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CharacterData {
    pub characters: Vec<Character>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StaffData {
    pub staff: Vec<Staff>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StudioData {
    pub studios: Vec<Studio>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserData {
    pub users: Vec<User>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivityData {
    pub activities: Vec<ActivityUnion>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivityReplyData {
    pub activity_replies: Vec<ActivityReply>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AiringData {
    pub airing_schedules: Vec<AiringSchedule>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AiringScheduleResponse {
    #[serde(rename = "AiringSchedule")]
    pub airing_schedule: AiringSchedule,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecommendationData {
    pub recommendations: Vec<Recommendation>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReviewData {
    pub reviews: Vec<Review>,
}

/// Single item response wrappers
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MediaResponse {
    #[serde(rename = "Media")]
    pub media: Media,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CharacterResponse {
    #[serde(rename = "Character")]
    pub character: Character,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StaffResponse {
    #[serde(rename = "Staff")]
    pub staff: Staff,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StudioResponse {
    #[serde(rename = "Studio")]
    pub studio: Studio,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserResponse {
    #[serde(rename = "User")]
    pub user: User,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivityResponse {
    #[serde(rename = "Activity")]
    pub activity: ActivityUnion,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReviewResponse {
    #[serde(rename = "Review")]
    pub review: Review,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecommendationResponse {
    #[serde(rename = "Recommendation")]
    pub recommendation: Recommendation,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ThreadData {
    pub threads: Vec<Thread>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ThreadResponse {
    #[serde(rename = "Thread")]
    pub thread: Thread,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ThreadCommentData {
    pub thread_comments: Vec<ThreadComment>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ThreadCommentResponse {
    #[serde(rename = "ThreadComment")]
    pub thread_comment: ThreadComment,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SaveThreadData {
    #[serde(rename = "SaveThread")]
    pub save_thread: Thread,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteThreadData {
    #[serde(rename = "DeleteThread")]
    pub deleted: DeletedResponse,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SaveThreadCommentData {
    #[serde(rename = "SaveThreadComment")]
    pub save_thread_comment: ThreadComment,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteThreadCommentData {
    #[serde(rename = "DeleteThreadComment")]
    pub deleted: DeletedResponse,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ToggleThreadSubscriptionData {
    #[serde(rename = "ToggleThreadSubscription")]
    pub toggle_thread_subscription: Thread,
}

/// Viewer-specific response data types
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ViewerUserData {
    pub id: i32,
    pub name: String,
    pub avatar: Option<UserAvatar>,
    pub statistics: Option<UserStatisticTypes>,
    pub unread_notification_count: i32,
}

/// Type aliases for common response patterns
pub type NotificationResponse = GraphQLResponse<Page<Vec<NotificationUnion>>>;
pub type UserMediaListResponse = GraphQLResponse<Page<Vec<MediaList>>>;
pub type CharacterListResponse = GraphQLResponse<Page<Vec<Character>>>;
pub type StaffListResponse = GraphQLResponse<Page<Vec<Staff>>>;
pub type StudioListResponse = GraphQLResponse<Page<Vec<Studio>>>;
pub type UserListResponse = GraphQLResponse<Page<Vec<User>>>;
pub type ActivityListResponse = GraphQLResponse<Page<Vec<ActivityUnion>>>;
pub type ActivityReplyListResponse = GraphQLResponse<Page<Vec<ActivityReply>>>;
pub type AiringListResponse = GraphQLResponse<Page<Vec<AiringSchedule>>>;
pub type AiringSingleResponse = GraphQLResponse<AiringScheduleResponse>;
pub type RecommendationListResponse = GraphQLResponse<Page<Vec<Recommendation>>>;
pub type ReviewListResponse = GraphQLResponse<Page<Vec<Review>>>;
pub type ViewerFinalResponse = GraphQLResponse<ViewerResponse<ViewerUserData>>;

// Single item response types
pub type CharacterSingleResponse = GraphQLResponse<Character>;
pub type StaffSingleResponse = GraphQLResponse<Staff>;
pub type StudioSingleResponse = GraphQLResponse<Studio>;
pub type UserSingleResponse = GraphQLResponse<User>;
pub type ActivitySingleResponse = GraphQLResponse<ActivityUnion>;
pub type ReviewSingleResponse = GraphQLResponse<Review>;
pub type RecommendationSingleResponse = GraphQLResponse<Recommendation>;

// Activity mutation response types
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SaveTextActivityData {
    #[serde(rename = "SaveTextActivity")]
    pub save_text_activity: TextActivity,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SaveMessageActivityData {
    #[serde(rename = "SaveMessageActivity")]
    pub save_message_activity: MessageActivity,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteActivityData {
    #[serde(rename = "DeleteActivity")]
    pub deleted: DeletedResponse,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteReviewData {
    #[serde(rename = "DeleteReview")]
    pub deleted: bool,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeletedResponse {
    pub deleted: bool,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SaveActivityReplyData {
    #[serde(rename = "SaveActivityReply")]
    pub save_activity_reply: ActivityReply,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteActivityReplyData {
    #[serde(rename = "DeleteActivityReply")]
    pub deleted: DeletedResponse,
}

pub type SaveTextActivityResponse = GraphQLResponse<SaveTextActivityData>;
pub type SaveMessageActivityResponse = GraphQLResponse<SaveMessageActivityData>;
pub type DeleteActivityResponse = GraphQLResponse<DeleteActivityData>;
pub type DeleteReviewResponse = GraphQLResponse<DeleteReviewData>;
pub type SaveActivityReplyResponse = GraphQLResponse<SaveActivityReplyData>;
pub type DeleteActivityReplyResponse = GraphQLResponse<DeleteActivityReplyData>;

// MediaList single item response types
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MediaListEntryData {
    #[serde(rename = "SaveMediaListEntry")]
    pub save_media_list_entry: MediaList,
}

// MediaList multiple item response types
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MultipleMediaListEntryData {
    #[serde(rename = "UpdateMediaListEntries")]
    pub update_media_list_entries: Vec<MediaList>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteMediaListData {
    #[serde(rename = "DeleteMediaListEntry")]
    pub deleted: bool,
}

pub type SaveMediaListEntryResponse = GraphQLResponse<MediaListEntryData>;
pub type SaveMediaListMultipleResponse = GraphQLResponse<MultipleMediaListEntryData>;
pub type DeleteMediaListEntryResponse = GraphQLResponse<DeleteMediaListData>;

// Common endpoint response types
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ToggleLikeData {
    #[serde(rename = "ToggleLikeV2")]
    pub toggle_like_v2: LikeableUnion,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ToggleFollowData {
    #[serde(rename = "ToggleFollow")]
    pub toggle_follow: User,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ToggleFavouriteData {
    #[serde(rename = "ToggleFavourite")]
    pub toggle_favourite: Favourites,
}

pub type ToggleLikeResponse = GraphQLResponse<ToggleLikeData>;
pub type ToggleFollowResponse = GraphQLResponse<ToggleFollowData>;
pub type ToggleFavouriteResponse = GraphQLResponse<ToggleFavouriteData>;

// Thread/Forum endpoint response types
pub type ThreadListResponse = GraphQLResponse<Page<ThreadData>>;
pub type ThreadSingleResponse = GraphQLResponse<ThreadResponse>;
pub type ThreadCommentListResponse = GraphQLResponse<Page<ThreadCommentData>>;
pub type ThreadCommentSingleResponse = GraphQLResponse<ThreadCommentResponse>;
pub type SaveThreadResponse = GraphQLResponse<SaveThreadData>;
pub type DeleteThreadResponse = GraphQLResponse<DeleteThreadData>;
pub type SaveThreadCommentResponse = GraphQLResponse<SaveThreadCommentData>;
pub type DeleteThreadCommentResponse = GraphQLResponse<DeleteThreadCommentData>;
pub type ToggleThreadSubscriptionResponse = GraphQLResponse<ToggleThreadSubscriptionData>;

// The high-performance custom deserialization logic
impl<'de, T> Deserialize<'de> for Page<T>
where
    T: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        // Define a visitor struct to hold the deserialization logic.
        struct PageVisitor<T>(PhantomData<T>);

        impl<'de, T> Visitor<'de> for PageVisitor<T>
        where
            T: Deserialize<'de>,
        {
            type Value = Page<T>; // The final type we want to produce

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("an object with a 'pageInfo' field and one other data field")
            }

            // This method is called by Serde when it encounters a JSON object.
            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: MapAccess<'de>,
            {
                let mut page_info = None;
                let mut data = None;

                // Loop through each key-value pair in the JSON object directly.
                while let Some(key) = map.next_key::<String>()? {
                    match key.as_str() {
                        "pageInfo" => {
                            // Avoid processing the same field twice.
                            if page_info.is_some() {
                                return Err(de::Error::duplicate_field("pageInfo"));
                            }
                            // Deserialize the value directly into the PageInfo struct.
                            page_info = Some(map.next_value()?);
                        }
                        // Any other key is treated as the data field.
                        _ => {
                            if data.is_some() {
                                return Err(de::Error::custom("found more than one data field"));
                            }
                            // Deserialize the value directly into the generic type T.
                            data = Some(map.next_value()?);
                        }
                    }
                }

                let data = data.ok_or_else(|| de::Error::custom("missing data field"))?;

                // Construct the final Page struct.
                Ok(Page { page_info, data })
            }
        }

        // Instruct Serde to use our custom visitor to deserialize the object.
        deserializer.deserialize_map(PageVisitor(PhantomData))
    }
}

// A private helper struct to represent the inner object: {"any_key": [...]}.
// It will have its own custom deserialization logic.
struct InnerData<T> {
    data: T,
}

// First, we teach our helper struct how to deserialize itself from an
// object with a single, dynamically named field.
impl<'de, T> Deserialize<'de> for InnerData<T>
where
    T: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct InnerVisitor<T>(PhantomData<T>);

        impl<'de, T> Visitor<'de> for InnerVisitor<T>
        where
            T: Deserialize<'de>,
        {
            type Value = InnerData<T>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("an object with a single field containing an array")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: MapAccess<'de>,
            {
                // 1. Ignore the key of the single inner field.
                map.next_key::<de::IgnoredAny>()?.ok_or_else(|| {
                    de::Error::custom("expected a single data field, but the object was empty")
                })?;

                // 2. Deserialize the value directly into our final T.
                let data: T = map.next_value()?;

                Ok(InnerData { data })
            }
        }

        deserializer.deserialize_map(InnerVisitor(PhantomData))
    }
}

// Now, we implement Deserialize for our main GraphQLResponse struct.
impl<'de, T> Deserialize<'de> for GraphQLResponse<T>
where
    T: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct OuterVisitor<T>(PhantomData<T>);

        impl<'de, T> Visitor<'de> for OuterVisitor<T>
        where
            T: Deserialize<'de>,
        {
            type Value = GraphQLResponse<T>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("an object with a single 'data' field")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: MapAccess<'de>,
            {
                // 1. Expect the outer key to be "data".
                let key = map.next_key::<String>()?.ok_or_else(|| {
                    de::Error::custom("expected field 'data', but object was empty")
                })?;
                if key != "data" {
                    return Err(de::Error::invalid_value(
                        de::Unexpected::Str(&key),
                        &"the field 'data'",
                    ));
                }

                // 2. Deserialize the value using the custom logic we defined for InnerData.
                let inner: InnerData<T> = map.next_value()?;

                // 3. Construct the final ApiResponse.
                Ok(GraphQLResponse { data: inner.data })
            }
        }

        deserializer.deserialize_map(OuterVisitor(PhantomData))
    }
}
