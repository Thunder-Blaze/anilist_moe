//! Offline tests for the page walk, behind the `pagination` feature.

#![cfg(feature = "pagination")]

use anilist_moe::endpoints::media::FetchMediaOptions;
use anilist_moe::enums::media::{MediaSort, MediaType};
use anilist_moe::transport::{HttpRequest, HttpResponse, Transport};
use anilist_moe::utils::RetryConfig;
use anilist_moe::{AniListClient, AniListError};
use parking_lot::Mutex;
use serde_json::Value;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

/// Answers with the queued responses, then either errors or repeats the last one.
#[derive(Debug, Clone)]
struct ScriptedTransport {
    inner: Arc<ScriptedState>,
}

#[derive(Debug)]
struct ScriptedState {
    responses: Mutex<Vec<HttpResponse>>,
    requests: Mutex<Vec<HttpRequest>>,
    repeat_last: bool,
}

impl ScriptedTransport {
    fn new(responses: Vec<HttpResponse>) -> Self {
        Self::with_mode(responses, false)
    }

    fn repeating(responses: Vec<HttpResponse>) -> Self {
        Self::with_mode(responses, true)
    }

    fn with_mode(responses: Vec<HttpResponse>, repeat_last: bool) -> Self {
        Self {
            inner: Arc::new(ScriptedState {
                responses: Mutex::new(responses),
                requests: Mutex::new(Vec::new()),
                repeat_last,
            }),
        }
    }

    fn requests(&self) -> Vec<HttpRequest> {
        self.inner.requests.lock().clone()
    }

    /// The `page` variable of every request that was sent.
    fn requested_pages(&self) -> Vec<Value> {
        self.requests()
            .iter()
            .map(|request| {
                let body: Value = serde_json::from_slice(&request.body).expect("json body");
                body["variables"]["page"].clone()
            })
            .collect()
    }
}

impl Transport for ScriptedTransport {
    fn execute(
        &self,
        request: HttpRequest,
    ) -> Pin<Box<dyn Future<Output = Result<HttpResponse, AniListError>> + Send + '_>> {
        Box::pin(async move {
            let mut responses = self.inner.responses.lock();
            self.inner.requests.lock().push(request);

            if responses.is_empty() {
                if self.inner.repeat_last {
                    // Answers the same page forever, like a server that lies.
                    return Ok(HttpResponse::new(200, PAGE_OF_TWO.as_bytes().to_vec()));
                }
                return Err(AniListError::Transport {
                    message: "no queued response".to_string(),
                    source: None,
                });
            }

            Ok(responses.remove(0))
        })
    }
}

const PAGE_OF_TWO: &str = r#"{"data":{"Page":{"media":[{"id":1},{"id":2}],"pageInfo":{"hasNextPage":true,"currentPage":1,"lastPage":5}}}}"#;

/// A page of `count` entries with the given pagination info.
fn page(page: i32, count: i32, has_next: bool) -> HttpResponse {
    let media: Vec<String> = (0..count)
        .map(|index| format!("{{\"id\":{}}}", page * 100 + index))
        .collect();

    HttpResponse::new(
        200,
        format!(
            r#"{{"data":{{"Page":{{"media":[{}],"pageInfo":{{"hasNextPage":{has_next},"currentPage":{page},"lastPage":5}}}}}}}}"#,
            media.join(",")
        )
        .into_bytes(),
    )
}

fn make_client(transport: &ScriptedTransport) -> AniListClient {
    AniListClient::builder()
        .transport(transport.clone())
        .retry_config(RetryConfig::default())
        .build()
        .expect("a supplied transport cannot build fail")
}

fn media_options() -> FetchMediaOptions<'static> {
    FetchMediaOptions::builder()
        .media_type(MediaType::Anime)
        .per_page(2)
        .build()
}

#[tokio::test]
async fn it_walks_every_page_once_and_then_stops() {
    let transport =
        ScriptedTransport::new(vec![page(1, 2, true), page(2, 2, true), page(3, 1, false)]);
    let client = make_client(&transport);

    let mut pages = client.paginate(media_options());
    let mut seen = 0;

    while let Some(result) = pages.next_page().await {
        let page = result.expect("the scripted pages parse");
        seen += page.data.len();
    }

    assert_eq!(seen, 5);
    assert_eq!(
        transport.requested_pages(),
        vec![Value::from(1), Value::from(2), Value::from(3)],
        "each request asks for the next page, starting at one"
    );
    assert_eq!(pages.next_page_number(), 4);
    assert!(pages.is_finished());
}

#[tokio::test]
async fn it_hands_over_entries_one_by_one() {
    let transport = ScriptedTransport::new(vec![page(1, 2, true), page(2, 1, false)]);
    let client = make_client(&transport);

    let mut items = client.paginate(media_options());
    let mut ids = Vec::new();

    while let Some(result) = items.next_item().await {
        ids.push(result.expect("the scripted pages parse").id);
    }

    assert_eq!(ids, vec![100, 101, 200]);
}

#[tokio::test]
async fn collect_all_gathers_the_whole_walk() {
    let transport = ScriptedTransport::new(vec![page(1, 2, true), page(2, 2, false)]);
    let client = make_client(&transport);

    let all = client
        .paginate(media_options())
        .collect_all()
        .await
        .expect("the walk succeeds");

    assert_eq!(all.len(), 4);
}

#[tokio::test]
async fn the_options_reach_every_request() {
    let transport = ScriptedTransport::new(vec![page(1, 1, false)]);
    let client = make_client(&transport);

    let options = FetchMediaOptions::builder()
        .media_type(MediaType::Anime)
        .sort(&[MediaSort::PopularityDesc])
        .per_page(2)
        .build();
    let _ = client.paginate(options).collect_all().await.unwrap();

    let body: Value = serde_json::from_slice(&transport.requests()[0].body).unwrap();
    assert_eq!(body["variables"]["mediaType"], "ANIME");
    assert_eq!(body["variables"]["perPage"], 2);
    assert_eq!(
        body["variables"]["sort"],
        serde_json::json!(["POPULARITY_DESC", "ID"]),
        "the sort still ends in the tiebreaker"
    );
}

#[tokio::test]
async fn an_empty_page_ends_the_walk() {
    let transport = ScriptedTransport::new(vec![page(1, 0, true)]);
    let client = make_client(&transport);

    let all = client
        .paginate(media_options())
        .collect_all()
        .await
        .unwrap();

    assert!(all.is_empty());
    assert_eq!(
        transport.requests().len(),
        1,
        "an empty page is the end, however hasNextPage is answered"
    );
}

#[tokio::test]
async fn an_error_ends_the_walk_and_is_handed_over() {
    let transport = ScriptedTransport::new(vec![
        page(1, 2, true),
        page(2, 0, true),
        HttpResponse::new(429, ""),
    ]);
    let client = make_client(&transport);

    let mut pages = client.paginate(media_options());

    assert!(pages.next_page().await.unwrap().is_ok());
    assert!(pages.next_page().await.unwrap().is_ok());
    assert!(
        pages.next_page().await.is_none(),
        "the empty page ended the walk"
    );

    // A failing query ends the walk rather than being retried forever.
    let failing =
        ScriptedTransport::new(vec![page(1, 2, true), HttpResponse::new(429, "slow down")]);
    let failing_client = make_client(&failing);
    let mut pages = failing_client.paginate(media_options());

    assert!(pages.next_page().await.unwrap().is_ok());
    assert!(matches!(
        pages.next_page().await.unwrap(),
        Err(AniListError::RateLimitSimple)
    ));
    assert!(pages.next_page().await.is_none());
    assert_eq!(failing.requests().len(), 2, "the failure is not retried");
}

#[tokio::test]
async fn a_server_that_never_stops_is_capped() {
    // Every answer claims another page, so only the cap ends the walk.
    let transport = ScriptedTransport::repeating(vec![]);
    let walker = make_client(&transport);

    let pages = walker
        .paginate(media_options())
        .with_max_pages(3)
        .collect_all()
        .await
        .unwrap();

    assert_eq!(pages.len(), 6, "three pages of two entries");
    assert_eq!(transport.requests().len(), 3);
}
