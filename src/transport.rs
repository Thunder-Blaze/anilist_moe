//! The HTTP seam.
//!
//! Every request the client makes goes through a [`Transport`]. The crate ships
//! [`ReqwestTransport`], and a caller can supply its own implementation through
//! [`AniListClientBuilder::transport`](crate::client::AniListClientBuilder::transport)
//! to route requests elsewhere, or to answer them without a network at all.
//!
//! ```rust
//! use anilist_moe::transport::{HttpRequest, HttpResponse, Transport};
//! use anilist_moe::AniListError;
//! use std::future::Future;
//! use std::pin::Pin;
//!
//! /// Answers every request with a canned body.
//! #[derive(Debug)]
//! struct Canned(&'static str);
//!
//! impl Transport for Canned {
//!     fn execute(
//!         &self,
//!         _request: HttpRequest,
//!     ) -> Pin<Box<dyn Future<Output = Result<HttpResponse, AniListError>> + Send + '_>> {
//!         Box::pin(async move {
//!             Ok(HttpResponse::new(200, self.0.as_bytes().to_vec()))
//!         })
//!     }
//! }
//! ```

use crate::errors::AniListError;
use std::future::Future;
use std::pin::Pin;

/// An outgoing request, already serialised.
#[derive(Debug, Clone)]
pub struct HttpRequest {
    /// Absolute URL the request is sent to.
    pub url: String,
    /// Headers to send, in the order they were added.
    pub headers: Vec<(String, String)>,
    /// Request body, serialised JSON.
    pub body: Vec<u8>,
}

impl HttpRequest {
    /// Builds a request with an empty header list.
    #[must_use]
    pub fn new(url: impl Into<String>, body: impl Into<Vec<u8>>) -> Self {
        Self {
            url: url.into(),
            headers: Vec::new(),
            body: body.into(),
        }
    }

    /// Adds a header, replacing any existing one with the same name.
    #[must_use]
    pub fn header(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        let name = name.into();
        let value = value.into();
        match self
            .headers
            .iter_mut()
            .find(|(existing, _)| existing.eq_ignore_ascii_case(&name))
        {
            Some((_, existing)) => *existing = value,
            None => self.headers.push((name, value)),
        }
        self
    }

    /// Returns a header value, ignoring case.
    #[must_use]
    pub fn header_value(&self, name: &str) -> Option<&str> {
        self.headers
            .iter()
            .find(|(existing, _)| existing.eq_ignore_ascii_case(name))
            .map(|(_, value)| value.as_str())
    }
}

/// The answer to an [`HttpRequest`].
#[derive(Debug, Clone)]
pub struct HttpResponse {
    /// HTTP status code.
    pub status: u16,
    /// Response headers.
    pub headers: Vec<(String, String)>,
    /// Response body.
    pub body: Vec<u8>,
}

impl HttpResponse {
    /// Builds a response with no headers.
    #[must_use]
    pub fn new(status: u16, body: impl Into<Vec<u8>>) -> Self {
        Self {
            status,
            headers: Vec::new(),
            body: body.into(),
        }
    }

    /// Adds a header, replacing any existing one with the same name.
    #[must_use]
    pub fn header(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        let name = name.into();
        let value = value.into();
        match self
            .headers
            .iter_mut()
            .find(|(existing, _)| existing.eq_ignore_ascii_case(&name))
        {
            Some((_, existing)) => *existing = value,
            None => self.headers.push((name, value)),
        }
        self
    }

    /// Returns a header value, ignoring case.
    #[must_use]
    pub fn header_value(&self, name: &str) -> Option<&str> {
        self.headers
            .iter()
            .find(|(existing, _)| existing.eq_ignore_ascii_case(name))
            .map(|(_, value)| value.as_str())
    }

    /// The body as text, lossily decoded when it is not valid UTF-8.
    #[must_use]
    pub fn text(&self) -> std::borrow::Cow<'_, str> {
        String::from_utf8_lossy(&self.body)
    }

    /// Whether the status code is in the 2xx range.
    #[must_use]
    pub fn is_success(&self) -> bool {
        (200..300).contains(&self.status)
    }
}

/// Sends [`HttpRequest`]s.
///
/// Implementations must be safe to share across threads; the client holds one in
/// an `Arc` and calls it concurrently.
pub trait Transport: Send + Sync + std::fmt::Debug {
    /// Sends `request` and returns its response.
    ///
    /// A failure to reach the server at all is reported as
    /// [`AniListError::Transport`]; HTTP status codes that the server did answer
    /// with are *not* errors here, they are returned in [`HttpResponse::status`].
    fn execute(
        &self,
        request: HttpRequest,
    ) -> Pin<Box<dyn Future<Output = Result<HttpResponse, AniListError>> + Send + '_>>;
}

/// User-Agent header for identifying this library.
const USER_AGENT: &str = concat!("anilist-moe/", env!("CARGO_PKG_VERSION"), " (Rust)");

/// Default request timeout.
const DEFAULT_TIMEOUT: std::time::Duration = std::time::Duration::from_secs(30);

/// Sends requests with `reqwest`.
#[derive(Debug, Clone)]
pub struct ReqwestTransport {
    client: reqwest::Client,
}

impl ReqwestTransport {
    /// A transport that builds its own `reqwest` client with the crate's defaults:
    /// a 30s timeout, up to 10 idle connections per host, and `TCP_NODELAY`.
    #[must_use]
    pub fn new() -> Self {
        Self::with_options(None, None).expect("the default reqwest client is well formed")
    }

    /// A transport around an existing `reqwest` client.
    #[must_use]
    pub fn with_client(client: reqwest::Client) -> Self {
        Self { client }
    }

    /// Builds a `reqwest` client, overriding the timeout and the user agent when given.
    ///
    /// # Errors
    ///
    /// [`AniListError::Transport`] when `reqwest` refuses the configuration.
    pub fn with_options(
        timeout: Option<std::time::Duration>,
        user_agent: Option<&str>,
    ) -> Result<Self, AniListError> {
        let client = reqwest::Client::builder()
            .user_agent(user_agent.unwrap_or(USER_AGENT))
            .timeout(timeout.unwrap_or(DEFAULT_TIMEOUT))
            .pool_max_idle_per_host(10)
            .tcp_nodelay(true)
            .build()?;

        Ok(Self { client })
    }
}

impl Default for ReqwestTransport {
    fn default() -> Self {
        Self::new()
    }
}

impl Transport for ReqwestTransport {
    fn execute(
        &self,
        request: HttpRequest,
    ) -> Pin<Box<dyn Future<Output = Result<HttpResponse, AniListError>> + Send + '_>> {
        Box::pin(async move {
            let mut builder = self.client.post(&request.url);
            for (name, value) in &request.headers {
                builder = builder.header(name, value);
            }

            let response = builder.body(request.body).send().await?;

            let status = response.status().as_u16();
            let headers = response
                .headers()
                .iter()
                .filter_map(|(name, value)| {
                    value
                        .to_str()
                        .ok()
                        .map(|value| (name.as_str().to_string(), value.to_string()))
                })
                .collect();
            let body = response.bytes().await?.to_vec();

            Ok(HttpResponse {
                status,
                headers,
                body,
            })
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn header_lookup_ignores_case() {
        let request = HttpRequest::new("https://example.test", "{}").header("content-type", "x");
        assert_eq!(request.header_value("Content-Type"), Some("x"));
        assert_eq!(request.header_value("CONTENT-TYPE"), Some("x"));
        assert_eq!(request.header_value("absent"), None);
    }

    #[test]
    fn headers_replace_instead_of_duplicating() {
        let request = HttpRequest::new("https://example.test", "{}")
            .header("Authorization", "Bearer first")
            .header("authorization", "Bearer second");
        assert_eq!(
            request.header_value("Authorization"),
            Some("Bearer second"),
            "the later value must win"
        );
        assert_eq!(request.headers.len(), 1);
    }

    #[test]
    fn response_reports_success_and_text() {
        let ok = HttpResponse::new(200, "{\"data\":null}");
        assert!(ok.is_success());
        assert_eq!(ok.text(), "{\"data\":null}");

        let error = HttpResponse::new(503, "down");
        assert!(!error.is_success());
        assert_eq!(error.header_value("Retry-After"), None);
    }

    #[test]
    fn response_headers_are_readable_ignoring_case() {
        let response = HttpResponse::new(429, "").header("retry-after", "30");
        assert_eq!(response.header_value("Retry-After"), Some("30"));
    }
}
