//! Offline response-parsing tests.
//!
//! Each endpoint gets a realistic response body in the exact shape the API
//! returns it, answered by a scripted [`Transport`]. The assertions pin the
//! deserialised Rust shape, so a serde regression is caught without a network.

use anilist_moe::endpoints::activity::FetchActivityOptions;
use anilist_moe::endpoints::airing::FetchAiringOptions;
use anilist_moe::endpoints::character::FetchCharacterOptions;
use anilist_moe::endpoints::forum::{FetchThreadCommentOptions, FetchThreadOptions};
use anilist_moe::endpoints::media::FetchMediaOptions;
use anilist_moe::endpoints::medialist::FetchMediaListOptions;
use anilist_moe::endpoints::notification::NotificationSearchOptions;
use anilist_moe::endpoints::recommendation::FetchRecommendationOptions;
use anilist_moe::endpoints::review::FetchReviewOptions;
use anilist_moe::endpoints::staff::FetchStaffOptions;
use anilist_moe::endpoints::studio::FetchStudioOptions;
use anilist_moe::endpoints::user::FetchUserOptions;
use anilist_moe::enums::activity::ActivityType;
use anilist_moe::enums::character::CharacterRole;
use anilist_moe::enums::media::{MediaFormat, MediaStatus, MediaType};
use anilist_moe::enums::media_list::MediaListStatus;
use anilist_moe::enums::moderators::ModRole;
use anilist_moe::enums::recommendation::RecommendationRating;
use anilist_moe::transport::{HttpRequest, HttpResponse, Transport};
use anilist_moe::unions::activity::ActivityUnion;
use anilist_moe::unions::notification::NotificationUnion;
use anilist_moe::{AniListClient, AniListError};
use parking_lot::Mutex;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

/// Answers each request with the next queued response.
#[derive(Debug, Clone)]
struct ScriptedTransport {
    responses: Arc<Mutex<Vec<HttpResponse>>>,
}

impl ScriptedTransport {
    fn new(responses: Vec<HttpResponse>) -> Self {
        Self {
            responses: Arc::new(Mutex::new(responses)),
        }
    }

    fn json(body: &str) -> HttpResponse {
        HttpResponse::new(200, body.as_bytes().to_vec())
    }
}

impl Transport for ScriptedTransport {
    fn execute(
        &self,
        _request: HttpRequest,
    ) -> Pin<Box<dyn Future<Output = Result<HttpResponse, AniListError>> + Send + '_>> {
        Box::pin(async move {
            let mut responses = self.responses.lock();
            assert!(!responses.is_empty(), "no queued response left");
            Ok(responses.remove(0))
        })
    }
}

fn client(transport: &ScriptedTransport) -> AniListClient {
    AniListClient::builder()
        .transport(transport.clone())
        .build()
        .expect("a supplied transport cannot fail to build")
}

#[tokio::test]
async fn media_page_parses_the_requested_fields() {
    const BODY: &str = r#"{
        "data": {"Page": {
            "pageInfo": {"total": 2, "currentPage": 1, "lastPage": 1, "hasNextPage": false, "perPage": 2},
            "media": [
                {"id": 1, "title": {"romaji": "Cowboy Bebop", "english": "Cowboy Bebop"},
                 "type": "ANIME", "format": "TV", "status": "FINISHED"},
                {"id": 2}
            ]
        }}
    }"#;
    let transport = ScriptedTransport::new(vec![ScriptedTransport::json(BODY)]);
    let client = client(&transport);

    let page = client
        .media()
        .fetch(
            &FetchMediaOptions::builder()
                .media_type(Some(MediaType::Anime))
                .page(Some(1))
                .per_page(Some(2))
                .build(),
        )
        .await
        .expect("the scripted response parses");

    assert_eq!(page.data.len(), 2);
    assert_eq!(page.data[0].id, 1);
    assert_eq!(page.data[1].id, 2);
    assert_eq!(
        page.data[0]
            .title
            .as_ref()
            .and_then(|t| t.romaji.as_deref()),
        Some("Cowboy Bebop")
    );
    assert!(matches!(page.data[0].media_type, Some(MediaType::Anime)));
    assert!(matches!(page.data[0].format, Some(MediaFormat::Tv)));
    assert!(
        page.data[1].title.is_none(),
        "a missing optional field is None"
    );
    let info = page.page_info.expect("the query asks for pageInfo");
    assert_eq!(info.total, Some(2));
    assert_eq!(info.has_next_page, Some(false));
}

#[tokio::test]
async fn media_list_page_parses_the_requested_fields() {
    const BODY: &str = r#"{
        "data": {"Page": {
            "pageInfo": {"hasNextPage": true},
            "mediaList": [
                {"id": 10, "mediaId": 101, "status": "COMPLETED", "score": 9.5,
                 "media": {"id": 101, "title": {"romaji": "Steins;Gate"},
                           "type": "ANIME", "format": "TV", "status": "FINISHED"}},
                {"id": 11}
            ]
        }}
    }"#;
    let transport = ScriptedTransport::new(vec![ScriptedTransport::json(BODY)]);
    let client = client(&transport);

    let page = client
        .medialist()
        .fetch(
            &FetchMediaListOptions::builder()
                .user_name(Some(std::borrow::Cow::Borrowed("someone")))
                .status(Some(MediaListStatus::Completed))
                .build(),
        )
        .await
        .expect("the scripted response parses");

    assert_eq!(page.data.len(), 2);
    assert_eq!(page.data[0].id, 10);
    assert_eq!(page.data[1].id, 11);
    assert_eq!(page.data[0].media_id, Some(101));
    assert!(matches!(
        page.data[0].status,
        Some(MediaListStatus::Completed)
    ));
    assert_eq!(
        page.data[0]
            .media
            .as_ref()
            .and_then(|m| m.title.as_ref())
            .and_then(|t| t.romaji.as_deref()),
        Some("Steins;Gate")
    );
    assert!(page.data[1].status.is_none());
    assert_eq!(
        page.page_info.and_then(|info| info.has_next_page),
        Some(true)
    );
}

#[tokio::test]
async fn user_page_parses_the_requested_fields() {
    const BODY: &str = r#"{
        "data": {"Page": {
            "pageInfo": {"total": 1, "currentPage": 1, "lastPage": 1, "hasNextPage": false, "perPage": 1},
            "users": [
                {"id": 7, "name": "someone", "avatar": {"large": "l.png", "medium": "m.png"},
                 "moderatorRoles": ["ADMIN"]},
                {"id": 8}
            ]
        }}
    }"#;
    let transport = ScriptedTransport::new(vec![ScriptedTransport::json(BODY)]);
    let client = client(&transport);

    let page = client
        .user()
        .fetch(
            &FetchUserOptions::builder()
                .search(Some(std::borrow::Cow::Borrowed("someone")))
                .build(),
        )
        .await
        .expect("the scripted response parses");

    assert_eq!(page.data.len(), 2);
    assert_eq!(page.data[0].id, 7);
    assert_eq!(page.data[1].id, 8);
    assert_eq!(page.data[0].name.as_deref(), Some("someone"));
    assert_eq!(
        page.data[0]
            .avatar
            .as_ref()
            .and_then(|a| a.large.as_deref()),
        Some("l.png")
    );
    assert!(matches!(
        page.data[0].moderator_roles.as_deref(),
        Some([ModRole::Admin])
    ));
    assert!(page.data[1].name.is_none());
    assert_eq!(page.page_info.and_then(|info| info.total), Some(1));
}

#[tokio::test]
async fn character_page_parses_the_requested_fields() {
    const BODY: &str = r#"{
        "data": {"Page": {
            "pageInfo": {"total": 1, "currentPage": 1, "lastPage": 1, "hasNextPage": false, "perPage": 1},
            "characters": [
                {"id": 20, "name": {"full": "Spike Spiegel", "native": "スパイク"},
                 "image": {"large": "c.png"},
                 "media": {"edges": [
                     {"id": 200, "characterRole": "MAIN",
                      "node": {"id": 1, "title": {"romaji": "Cowboy Bebop"}}}
                 ]}},
                {"id": 21}
            ]
        }}
    }"#;
    let transport = ScriptedTransport::new(vec![ScriptedTransport::json(BODY)]);
    let client = client(&transport);

    let page = client
        .character()
        .fetch(
            &FetchCharacterOptions::builder()
                .search(Some(std::borrow::Cow::Borrowed("spike")))
                .include_media(Some(true))
                .build(),
        )
        .await
        .expect("the scripted response parses");

    assert_eq!(page.data.len(), 2);
    assert_eq!(page.data[0].id, 20);
    assert_eq!(page.data[1].id, 21);
    assert_eq!(
        page.data[0].name.as_ref().and_then(|n| n.full.as_deref()),
        Some("Spike Spiegel")
    );
    assert_eq!(
        page.data[0].image.as_ref().and_then(|i| i.large.as_deref()),
        Some("c.png")
    );
    let edge = &page.data[0]
        .media
        .as_ref()
        .and_then(|m| m.edges.as_ref())
        .expect("media edges are requested")[0];
    assert!(matches!(edge.character_role, Some(CharacterRole::Main)));
    assert_eq!(
        edge.node
            .as_ref()
            .and_then(|n| n.title.as_ref())
            .and_then(|t| t.romaji.as_deref()),
        Some("Cowboy Bebop")
    );
    assert!(page.data[1].name.is_none());
    assert_eq!(page.page_info.and_then(|info| info.total), Some(1));
}

#[tokio::test]
async fn staff_page_parses_the_requested_fields() {
    const BODY: &str = r#"{
        "data": {"Page": {
            "pageInfo": {"total": 1, "currentPage": 1, "lastPage": 1, "hasNextPage": true, "perPage": 1},
            "staff": [
                {"id": 30, "name": {"full": "Shinichiro Watanabe"},
                 "staffMedia": {"edges": [
                     {"id": 300, "staffRole": "Director",
                      "node": {"id": 1, "title": {"romaji": "Cowboy Bebop"},
                               "format": "TV", "status": "FINISHED"}}
                 ]}},
                {"id": 31}
            ]
        }}
    }"#;
    let transport = ScriptedTransport::new(vec![ScriptedTransport::json(BODY)]);
    let client = client(&transport);

    let page = client
        .staff()
        .fetch(
            &FetchStaffOptions::builder()
                .search(Some(std::borrow::Cow::Borrowed("watanabe")))
                .include_staff_media(Some(true))
                .build(),
        )
        .await
        .expect("the scripted response parses");

    assert_eq!(page.data.len(), 2);
    assert_eq!(page.data[0].id, 30);
    assert_eq!(page.data[1].id, 31);
    assert_eq!(
        page.data[0].name.as_ref().and_then(|n| n.full.as_deref()),
        Some("Shinichiro Watanabe")
    );
    let edge = &page.data[0]
        .staff_media
        .as_ref()
        .and_then(|m| m.edges.as_ref())
        .expect("staffMedia edges are requested")[0];
    assert!(matches!(
        edge.node.as_ref().and_then(|n| n.status),
        Some(MediaStatus::Finished)
    ));
    assert!(page.data[1].name.is_none());
    assert_eq!(
        page.page_info.and_then(|info| info.has_next_page),
        Some(true)
    );
}

#[tokio::test]
async fn studio_page_parses_the_requested_fields() {
    const BODY: &str = r#"{
        "data": {"Page": {
            "pageInfo": {"hasNextPage": false},
            "studios": [
                {"id": 40, "name": "Bones", "isAnimationStudio": true,
                 "media": {"edges": [
                     {"id": 400, "isMainStudio": true,
                      "node": {"id": 1, "title": {"romaji": "Cowboy Bebop"}, "format": "TV"}}
                 ]}},
                {"id": 41}
            ]
        }}
    }"#;
    let transport = ScriptedTransport::new(vec![ScriptedTransport::json(BODY)]);
    let client = client(&transport);

    let page = client
        .studio()
        .fetch(
            &FetchStudioOptions::builder()
                .search(Some(std::borrow::Cow::Borrowed("bones")))
                .include_media(Some(true))
                .build(),
        )
        .await
        .expect("the scripted response parses");

    assert_eq!(page.data.len(), 2);
    assert_eq!(page.data[0].id, Some(40));
    assert_eq!(page.data[1].id, Some(41));
    assert_eq!(page.data[0].name.as_deref(), Some("Bones"));
    assert_eq!(page.data[0].is_animation_studio, Some(true));
    let edge = &page.data[0]
        .media
        .as_ref()
        .and_then(|m| m.edges.as_ref())
        .expect("media edges are requested")[0];
    assert!(matches!(
        edge.node.as_ref().and_then(|n| n.format),
        Some(MediaFormat::Tv)
    ));
    assert!(page.data[1].name.is_none());
    assert_eq!(
        page.page_info.and_then(|info| info.has_next_page),
        Some(false)
    );
}

#[tokio::test]
async fn forum_thread_page_parses_the_requested_fields() {
    const BODY: &str = r#"{
        "data": {"Page": {
            "pageInfo": {"total": 1, "currentPage": 1, "lastPage": 1, "hasNextPage": false, "perPage": 1},
            "threads": [
                {"id": 50, "title": "Weekly discussion", "body": "<p>hello</p>", "replyCount": 3,
                 "user": {"id": 7, "name": "someone", "moderatorRoles": ["ADMIN"]}},
                {"id": 51}
            ]
        }}
    }"#;
    let transport = ScriptedTransport::new(vec![ScriptedTransport::json(BODY)]);
    let client = client(&transport);

    let page = client
        .forum()
        .fetch(
            &FetchThreadOptions::builder()
                .search(Some(std::borrow::Cow::Borrowed("discussion")))
                .include_body(Some(true))
                .include_user(Some(true))
                .build(),
        )
        .await
        .expect("the scripted response parses");

    assert_eq!(page.data.len(), 2);
    assert_eq!(page.data[0].id, 50);
    assert_eq!(page.data[1].id, 51);
    assert_eq!(page.data[0].title.as_deref(), Some("Weekly discussion"));
    assert_eq!(page.data[0].body.as_deref(), Some("<p>hello</p>"));
    let user = page.data[0].user.as_ref().expect("the user is requested");
    assert_eq!(user.name.as_deref(), Some("someone"));
    assert!(matches!(
        user.moderator_roles.as_deref(),
        Some([ModRole::Admin])
    ));
    assert!(page.data[1].title.is_none());
    assert_eq!(page.page_info.and_then(|info| info.total), Some(1));
}

#[tokio::test]
async fn forum_comment_page_parses_the_requested_fields() {
    const BODY: &str = r#"{
        "data": {"Page": {
            "pageInfo": {"total": 1, "currentPage": 1, "lastPage": 1, "hasNextPage": false, "perPage": 1},
            "threadComments": [
                {"id": 60, "threadId": 50, "comment": "nice", "likeCount": 2,
                 "user": {"id": 7, "name": "someone", "moderatorRoles": ["ADMIN"]}},
                {"id": 61}
            ]
        }}
    }"#;
    let transport = ScriptedTransport::new(vec![ScriptedTransport::json(BODY)]);
    let client = client(&transport);

    let page = client
        .forum()
        .fetch_comments(
            &FetchThreadCommentOptions::builder()
                .thread_id(Some(50))
                .build(),
        )
        .await
        .expect("the scripted response parses");

    assert_eq!(page.data.len(), 2);
    assert_eq!(page.data[0].id, 60);
    assert_eq!(page.data[1].id, 61);
    assert_eq!(page.data[0].thread_id, Some(50));
    assert_eq!(page.data[0].comment.as_deref(), Some("nice"));
    let user = page.data[0].user.as_ref().expect("the user is requested");
    assert!(matches!(
        user.moderator_roles.as_deref(),
        Some([ModRole::Admin])
    ));
    assert!(page.data[1].comment.is_none());
    assert_eq!(page.page_info.and_then(|info| info.total), Some(1));
}

#[tokio::test]
async fn activity_page_parses_the_requested_fields() {
    const BODY: &str = r#"{
        "data": {"Page": {
            "pageInfo": {"hasNextPage": true},
            "activities": [
                {"__typename": "ListActivity", "id": 70, "type": "ANIME_LIST", "status": "CURRENT",
                 "progress": "3", "user": {"id": 7, "name": "someone", "avatar": {"large": "u.png"}}},
                {"__typename": "TextActivity", "id": 71}
            ]
        }}
    }"#;
    let transport = ScriptedTransport::new(vec![ScriptedTransport::json(BODY)]);
    let client = client(&transport);

    let page = client
        .activity()
        .fetch(
            &FetchActivityOptions::builder()
                .page(Some(1))
                .per_page(Some(2))
                .build(),
        )
        .await
        .expect("the scripted response parses");

    assert_eq!(page.data.len(), 2);
    match &page.data[0] {
        ActivityUnion::ListActivity(list) => {
            assert_eq!(list.id, 70);
            assert!(matches!(list.activity_type, Some(ActivityType::AnimeList)));
            assert_eq!(list.status.as_deref(), Some("CURRENT"));
            assert_eq!(
                list.user.as_ref().and_then(|u| u.name.as_deref()),
                Some("someone")
            );
        }
        other => panic!("expected a ListActivity, got {other:?}"),
    }
    match &page.data[1] {
        ActivityUnion::TextActivity(text) => {
            assert_eq!(text.id, 71);
            assert!(text.activity_type.is_none());
            assert!(text.text.is_none());
        }
        other => panic!("expected a TextActivity, got {other:?}"),
    }
    assert_eq!(
        page.page_info.and_then(|info| info.has_next_page),
        Some(true)
    );
}

#[tokio::test]
async fn airing_page_parses_the_requested_fields() {
    const BODY: &str = r#"{
        "data": {"Page": {
            "pageInfo": {"hasNextPage": false},
            "airingSchedules": [
                {"id": 80, "airingAt": 1700000000, "timeUntilAiring": 100, "episode": 5, "mediaId": 101,
                 "media": {"id": 101, "title": {"romaji": "Cowboy Bebop"},
                           "format": "TV", "status": "RELEASING"}},
                {"id": 81}
            ]
        }}
    }"#;
    let transport = ScriptedTransport::new(vec![ScriptedTransport::json(BODY)]);
    let client = client(&transport);

    let page = client
        .airing()
        .fetch(&FetchAiringOptions::builder().media_id(Some(101)).build())
        .await
        .expect("the scripted response parses");

    assert_eq!(page.data.len(), 2);
    assert_eq!(page.data[0].id, Some(80));
    assert_eq!(page.data[1].id, Some(81));
    assert_eq!(page.data[0].episode, Some(5));
    let media = page.data[0].media.as_ref().expect("media is requested");
    assert_eq!(
        media.title.as_ref().and_then(|t| t.romaji.as_deref()),
        Some("Cowboy Bebop")
    );
    assert!(matches!(media.format, Some(MediaFormat::Tv)));
    assert!(matches!(media.status, Some(MediaStatus::Releasing)));
    assert!(page.data[1].media.is_none());
    assert_eq!(
        page.page_info.and_then(|info| info.has_next_page),
        Some(false)
    );
}

#[tokio::test]
async fn review_page_parses_the_requested_fields() {
    const BODY: &str = r#"{
        "data": {"Page": {
            "pageInfo": {"hasNextPage": true},
            "reviews": [
                {"id": 90, "mediaId": 101, "mediaType": "ANIME", "summary": "Great", "score": 92,
                 "user": {"id": 7, "name": "someone"},
                 "media": {"id": 101, "title": {"romaji": "Cowboy Bebop"},
                           "format": "TV", "status": "FINISHED"}},
                {"id": 91}
            ]
        }}
    }"#;
    let transport = ScriptedTransport::new(vec![ScriptedTransport::json(BODY)]);
    let client = client(&transport);

    let page = client
        .review()
        .fetch(&FetchReviewOptions::builder().media_id(Some(101)).build())
        .await
        .expect("the scripted response parses");

    assert_eq!(page.data.len(), 2);
    assert_eq!(page.data[0].id, 90);
    assert_eq!(page.data[1].id, 91);
    assert_eq!(page.data[0].media_type.as_deref(), Some("ANIME"));
    assert_eq!(page.data[0].summary.as_deref(), Some("Great"));
    assert_eq!(
        page.data[0].user.as_ref().and_then(|u| u.name.as_deref()),
        Some("someone")
    );
    assert!(matches!(
        page.data[0].media.as_ref().and_then(|m| m.status),
        Some(MediaStatus::Finished)
    ));
    assert!(page.data[1].media.is_none());
    assert_eq!(
        page.page_info.and_then(|info| info.has_next_page),
        Some(true)
    );
}

#[tokio::test]
async fn recommendation_page_parses_the_requested_fields() {
    const BODY: &str = r#"{
        "data": {"Page": {
            "pageInfo": {"hasNextPage": false},
            "recommendations": [
                {"id": 100, "rating": 12, "userRating": "RATE_UP",
                 "media": {"id": 101, "title": {"romaji": "Cowboy Bebop"}, "format": "TV"},
                 "mediaRecommendation": {"id": 102, "title": {"romaji": "Samurai Champloo"},
                                         "status": "FINISHED"}},
                {"id": 103}
            ]
        }}
    }"#;
    let transport = ScriptedTransport::new(vec![ScriptedTransport::json(BODY)]);
    let client = client(&transport);

    let page = client
        .recommendation()
        .fetch(
            &FetchRecommendationOptions::builder()
                .media_id(Some(101))
                .build(),
        )
        .await
        .expect("the scripted response parses");

    assert_eq!(page.data.len(), 2);
    assert_eq!(page.data[0].id, 100);
    assert_eq!(page.data[1].id, 103);
    assert_eq!(page.data[0].rating, Some(12));
    assert!(matches!(
        page.data[0].user_rating,
        Some(RecommendationRating::RateUp)
    ));
    let recommendation = page.data[0]
        .media_recommendation
        .as_ref()
        .expect("mediaRecommendation is requested");
    assert_eq!(
        recommendation
            .title
            .as_ref()
            .and_then(|t| t.romaji.as_deref()),
        Some("Samurai Champloo")
    );
    assert!(matches!(recommendation.status, Some(MediaStatus::Finished)));
    assert!(page.data[1].media_recommendation.is_none());
    assert_eq!(
        page.page_info.and_then(|info| info.has_next_page),
        Some(false)
    );
}

#[tokio::test]
async fn notification_page_parses_the_requested_fields() {
    const BODY: &str = r#"{
        "data": {"Page": {
            "pageInfo": {"hasNextPage": true},
            "notifications": [
                {"type": "AIRING", "id": 110, "animeId": 101, "episode": 5,
                 "media": {"id": 101, "title": {"romaji": "Cowboy Bebop"}}},
                {"type": "FOLLOWING", "id": 111, "userId": 7, "context": "started following you",
                 "user": {"id": 7, "name": "someone", "avatar": {"large": "u.png"}}}
            ]
        }}
    }"#;
    let transport = ScriptedTransport::new(vec![ScriptedTransport::json(BODY)]);
    let client = client(&transport);

    let page = client
        .notification()
        .fetch(
            &NotificationSearchOptions::builder()
                .page(Some(1))
                .per_page(Some(2))
                .build(),
        )
        .await
        .expect("the scripted response parses");

    assert_eq!(page.data.len(), 2);
    // The notification `type` discriminates the union; the variant itself is the
    // enum value that round-trips.
    assert!(matches!(&page.data[0], NotificationUnion::Airing(_)));
    match &page.data[0] {
        NotificationUnion::Airing(airing) => {
            assert_eq!(airing.id, 110);
            assert_eq!(airing.anime_id, Some(101));
            assert!(airing.contexts.is_none());
            assert_eq!(
                airing
                    .media
                    .as_ref()
                    .and_then(|m| m.title.as_ref())
                    .and_then(|t| t.romaji.as_deref()),
                Some("Cowboy Bebop")
            );
        }
        other => panic!("expected an Airing notification, got {other:?}"),
    }
    match &page.data[1] {
        NotificationUnion::Following(following) => {
            assert_eq!(following.id, 111);
            assert_eq!(
                following.user.as_ref().and_then(|u| u.name.as_deref()),
                Some("someone")
            );
        }
        other => panic!("expected a Following notification, got {other:?}"),
    }
    assert_eq!(
        page.page_info.and_then(|info| info.has_next_page),
        Some(true)
    );
}
