//! Offline endpoint tests.
//!
//! Every request is answered by a scripted [`Transport`], so these run without a
//! network and assert what the crate actually puts on the wire.

use anilist_moe::endpoints::media::{FetchMediaOneOptions, FetchMediaOptions};
use anilist_moe::enums::media::{MediaSort, MediaType};
use anilist_moe::options;
use anilist_moe::transport::{HttpRequest, HttpResponse, Transport};
use anilist_moe::utils::RetryConfig;
use anilist_moe::{AniListClient, AniListError};
use parking_lot::Mutex;
use serde_json::Value;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

/// Answers each request with the next queued response, recording what it saw.
#[derive(Debug, Clone, Default)]
struct ScriptedTransport {
    inner: Arc<ScriptedState>,
}

#[derive(Debug, Default)]
struct ScriptedState {
    responses: Mutex<Vec<HttpResponse>>,
    requests: Mutex<Vec<HttpRequest>>,
}

impl ScriptedTransport {
    fn new(responses: Vec<HttpResponse>) -> Self {
        Self {
            inner: Arc::new(ScriptedState {
                responses: Mutex::new(responses),
                requests: Mutex::new(Vec::new()),
            }),
        }
    }

    fn json(body: &str) -> HttpResponse {
        HttpResponse::new(200, body.as_bytes().to_vec())
    }

    fn requests(&self) -> Vec<HttpRequest> {
        self.inner.requests.lock().clone()
    }

    /// The variables of the single request that was sent.
    fn sent_variables(&self) -> Value {
        let request = self.sent_request();
        let body: Value = serde_json::from_slice(&request.body).expect("json body");
        body["variables"].clone()
    }

    fn sent_request(&self) -> HttpRequest {
        let requests = self.requests();
        assert_eq!(requests.len(), 1, "expected exactly one request");
        requests[0].clone()
    }
}

impl Transport for ScriptedTransport {
    fn execute(
        &self,
        request: HttpRequest,
    ) -> Pin<Box<dyn Future<Output = Result<HttpResponse, AniListError>> + Send + '_>> {
        Box::pin(async move {
            self.inner.requests.lock().push(request);
            let mut responses = self.inner.responses.lock();
            assert!(!responses.is_empty(), "no queued response left");
            Ok(responses.remove(0))
        })
    }
}

fn client(transport: &ScriptedTransport) -> AniListClient {
    AniListClient::builder()
        .transport(transport.clone())
        .retry_config(RetryConfig {
            max_retries: 0,
            base_delay_ms: 1,
            exponential_backoff: false,
            max_delay_ms: 1,
            jitter: false,
            retry_transient_errors: true,
        })
        .build()
        .expect("a supplied transport cannot fail to build")
}

/// One media list page, in the shape the API returns it.
const MEDIA_LIST_PAGE: &str = r#"{
    "data": {
        "Page": {
            "mediaList": [
                {"id": 1, "mediaId": 101, "status": "COMPLETED", "score": 10.0},
                {"id": 2, "mediaId": 102, "status": "COMPLETED", "score": 9.0}
            ],
            "pageInfo": {"hasNextPage": true, "total": 40, "currentPage": 1, "perPage": 2}
        }
    }
}"#;

#[tokio::test]
async fn completed_anime_is_sorted_deterministically() {
    let transport = ScriptedTransport::new(vec![ScriptedTransport::json(MEDIA_LIST_PAGE)]);
    let client = client(&transport);

    client
        .medialist()
        .get_completed_anime(Some("someone"), Some(2), Some(50))
        .await
        .expect("the scripted response parses");

    let variables = transport.sent_variables();
    assert_eq!(variables["userName"], "someone");
    assert_eq!(variables["mediaType"], "ANIME");
    assert_eq!(variables["status"], "COMPLETED");
    assert_eq!(variables["page"], 2);
    assert_eq!(variables["perPage"], 50);
    assert_eq!(
        variables["sort"],
        serde_json::json!(["SCORE_DESC", "MEDIA_ID"]),
        "score alone is not unique, so the media id breaks ties (issue #19)"
    );
}

#[tokio::test]
async fn a_watch_list_is_sorted_deterministically() {
    let transport = ScriptedTransport::new(vec![ScriptedTransport::json(MEDIA_LIST_PAGE)]);
    let client = client(&transport);

    client
        .medialist()
        .get_watching(Some("someone"), Some(1), Some(50))
        .await
        .expect("the scripted response parses");

    let variables = transport.sent_variables();
    assert_eq!(variables["status"], "CURRENT");
    assert_eq!(
        variables["sort"],
        serde_json::json!(["UPDATED_TIME_DESC", "MEDIA_ID"]),
        "updated time alone is not unique, so the media id breaks ties (issue #19)"
    );
}

#[tokio::test]
async fn the_page_is_parsed_into_the_returned_type() {
    let transport = ScriptedTransport::new(vec![ScriptedTransport::json(MEDIA_LIST_PAGE)]);
    let client = client(&transport);

    let page = client
        .medialist()
        .get_completed_anime(Some("someone"), Some(1), Some(2))
        .await
        .expect("the scripted response parses");

    assert_eq!(page.data.len(), 2);
    assert_eq!(page.data[0].media_id, Some(101));
    assert_eq!(page.data[1].media_id, Some(102));
    assert_eq!(
        page.page_info.and_then(|info| info.has_next_page),
        Some(true)
    );
}

#[tokio::test]
async fn media_options_carry_the_include_flags() {
    const MEDIA_PAGE: &str = r#"{"data":{"Page":{"media":[{"id":1,"title":{"romaji":"Cowboy Bebop"}}],"pageInfo":{"hasNextPage":false}}}}"#;
    let transport = ScriptedTransport::new(vec![ScriptedTransport::json(MEDIA_PAGE)]);
    let client = client(&transport);

    let options = options!(
        FetchMediaOptions,
        media_type: MediaType::Anime,
        sort: &[MediaSort::PopularityDesc],
        include_characters: true,
        include_staff: true,
        page: 1,
        per_page: 5,
    );
    let page = client.media().fetch(&options).await.expect("parses");

    assert_eq!(page.data.len(), 1);
    let variables = transport.sent_variables();
    assert_eq!(variables["includeCharacters"], true);
    assert_eq!(variables["includeStaff"], true);
    assert_eq!(variables["mediaType"], "ANIME");
    assert_eq!(
        variables["sort"],
        serde_json::json!(["POPULARITY_DESC", "ID"]),
        "the id is appended so paging stays stable"
    );
    assert_eq!(
        variables.get("includeReviews"),
        None,
        "unset options are left out of the payload"
    );
}

#[tokio::test]
async fn authenticated_requests_carry_the_bearer_token() {
    let transport = ScriptedTransport::new(vec![ScriptedTransport::json(MEDIA_LIST_PAGE)]);
    let client = AniListClient::builder()
        .transport(transport.clone())
        .token("secret-token")
        .build()
        .expect("a supplied transport cannot fail to build");

    client
        .medialist()
        .get_completed_anime(Some("someone"), Some(1), Some(2))
        .await
        .expect("the scripted response parses");

    let request = transport.sent_request();
    assert_eq!(
        request.header_value("Authorization"),
        Some("Bearer secret-token")
    );
    assert_eq!(
        request.header_value("Content-Type"),
        Some("application/json")
    );
    assert_eq!(request.url, "https://graphql.anilist.co");
}

#[tokio::test]
async fn a_not_found_response_surfaces_as_not_found() {
    let transport = ScriptedTransport::new(vec![HttpResponse::new(404, "missing")]);
    let client = client(&transport);

    let options = options!(FetchMediaOneOptions, id: 999_999_999);
    let error = client
        .media()
        .fetch_one(&options)
        .await
        .expect_err("a 404 must fail");

    assert!(matches!(error, AniListError::NotFound), "got {error:?}");
}
#[tokio::test]
async fn user_favorites_parses_response_as_user() {
    let transport = ScriptedTransport::new(vec![ScriptedTransport::json(
        r#"{"data":{"User":{"id":42,"name":"Alice","favourites":{"anime":{"pageInfo":{"total":0},"nodes":[]}}}}}"#,
    )]);
    let client = client(&transport);

    let options = anilist_moe::endpoints::user::FetchUserFavoritesOptions {
        user_id: 42,
        fetch_anime: Some(true),
        ..Default::default()
    };
    let user = client
        .user()
        .favorites(&options)
        .await
        .expect("user favorites should parse into User");

    assert_eq!(user.id, 42);
    assert_eq!(user.name.as_deref(), Some("Alice"));
    assert!(user.favourites.is_some());
}

#[tokio::test]
async fn user_media_list_sends_media_list_sort_and_returns_media_list_items() {
    let transport = ScriptedTransport::new(vec![ScriptedTransport::json(
        r#"{"data":{"Page":{"pageInfo":{"hasNextPage":false},"mediaList":[{"id":100,"userId":42,"mediaId":500,"status":"CURRENT","score":8.5,"progress":12}]}}}"#,
    )]);
    let client = client(&transport);

    let options = anilist_moe::endpoints::user::FetchUserMediaListOptions {
        user_id: Some(42),
        sort: Some(std::borrow::Cow::Borrowed(&[
            anilist_moe::enums::media_list::MediaListSort::ScoreDesc,
        ])),
        ..Default::default()
    };
    let page = client
        .user()
        .media_list(&options)
        .await
        .expect("user media_list should parse into Page<Vec<MediaList>>");

    assert_eq!(page.data.len(), 1);
    let entry = &page.data[0];
    assert_eq!(entry.id, 100);
    assert_eq!(entry.user_id, Some(42));
    assert_eq!(entry.media_id, Some(500));
    assert_eq!(entry.progress, Some(12));

    // The wire payload should contain the MediaListSort value
    let vars = transport.sent_variables();
    assert_eq!(vars["sort"], serde_json::json!(["SCORE_DESC", "MEDIA_ID"]));
}

#[tokio::test]
async fn activity_pin_and_subscribe_parse_with_typename() {
    let transport = ScriptedTransport::new(vec![
        ScriptedTransport::json(
            r#"{"data":{"ToggleActivityPin":{"__typename":"TextActivity","id":10,"type":"TEXT","isPinned":true}}}"#,
        ),
        ScriptedTransport::json(
            r#"{"data":{"ToggleActivitySubscription":{"__typename":"ListActivity","id":20,"type":"ANIME_LIST","isSubscribed":true}}}"#,
        ),
    ]);
    let client = client(&transport);

    let pin_opts = anilist_moe::endpoints::activity::PinActivityOptions {
        id: 10,
        pinned: true,
    };
    let pin_res = client
        .activity()
        .pin(&pin_opts)
        .await
        .expect("activity pin should parse into ActivityUnion");
    match pin_res {
        anilist_moe::unions::activity::ActivityUnion::TextActivity(t) => {
            assert_eq!(t.id, 10);
            assert_eq!(t.is_pinned, Some(true));
        }
        other => panic!("expected TextActivity, got {other:?}"),
    }

    let sub_opts = anilist_moe::endpoints::activity::SubscribeActivityOptions {
        activity_id: 20,
        subscribe: true,
    };
    let sub_res = client
        .activity()
        .subscribe(&sub_opts)
        .await
        .expect("activity subscribe should parse into ActivityUnion");
    match sub_res {
        anilist_moe::unions::activity::ActivityUnion::ListActivity(l) => {
            assert_eq!(l.id, 20);
            assert_eq!(l.is_subscribed, Some(true));
        }
        other => panic!("expected ListActivity, got {other:?}"),
    }
}

#[tokio::test]
async fn toggle_thread_subscription_reads_the_state_then_sends_the_opposite() {
    // The API requires `subscribe` on every call, so a toggle has to look at the
    // thread first: this scripted thread is not subscribed, so the mutation must
    // ask for `subscribe: true`.
    let transport = ScriptedTransport::new(vec![
        ScriptedTransport::json(r#"{"data":{"Thread":{"id":77,"isSubscribed":false}}}"#),
        ScriptedTransport::json(
            r#"{"data":{"ToggleThreadSubscription":{"id":77,"title":"Anime Discussion","isSubscribed":true}}}"#,
        ),
    ]);
    let client = client(&transport);

    let thread = client
        .forum()
        .toggle_thread_subscription(77)
        .await
        .expect("toggle thread subscription should succeed");
    assert_eq!(thread.id, 77);

    let requests = transport.requests();
    assert_eq!(
        requests.len(),
        2,
        "a toggle reads the thread, then writes it"
    );

    // second request: the mutation, with the value the API insists on
    let body: Value = serde_json::from_slice(&requests[1].body).expect("json body");
    assert_eq!(body["variables"]["threadId"], serde_json::json!(77));
    assert_eq!(
        body["variables"]["subscribe"],
        serde_json::json!(true),
        "the state has to be sent, not omitted"
    );
    let document = body["query"].as_str().expect("a query string");
    assert!(
        document.contains("$subscribe: Boolean!"),
        "the document must declare the argument it always sends"
    );
}

#[tokio::test]
async fn subscribing_and_unsubscribing_send_an_explicit_state() {
    let transport = ScriptedTransport::new(vec![
        ScriptedTransport::json(
            r#"{"data":{"ToggleThreadSubscription":{"id":3,"isSubscribed":true}}}"#,
        ),
        ScriptedTransport::json(
            r#"{"data":{"ToggleThreadSubscription":{"id":3,"isSubscribed":false}}}"#,
        ),
    ]);
    let client = client(&transport);

    client
        .forum()
        .subscribe_to_thread(3)
        .await
        .expect("subscribe");
    client
        .forum()
        .unsubscribe_from_thread(3)
        .await
        .expect("unsubscribe");

    let requests = transport.requests();
    let first: Value = serde_json::from_slice(&requests[0].body).unwrap();
    let second: Value = serde_json::from_slice(&requests[1].body).unwrap();
    assert_eq!(first["variables"]["subscribe"], serde_json::json!(true));
    assert_eq!(second["variables"]["subscribe"], serde_json::json!(false));
}
