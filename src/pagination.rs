//! Walking the pages of a list query.
//!
//! Gated behind the `pagination` feature. The cursor asks for one page at a time,
//! so the caller decides how fast to walk: nothing is prefetched, and a walk that
//! is dropped mid-page simply stops asking.
//!
//! ```rust
//! # #[cfg(feature = "pagination")]
//! # async fn example() -> Result<(), anilist_moe::AniListError> {
//! use anilist_moe::endpoints::media::FetchMediaOptions;
//! use anilist_moe::enums::media::MediaType;
//! use anilist_moe::AniListClient;
//!
//! let client = AniListClient::new();
//!
//! // Every entry, page by page.
//! let options = FetchMediaOptions::builder()
//!     .media_type(MediaType::Anime)
//!     .per_page(50)
//!     .build();
//! let mut items = client.paginate(options);
//!
//! while let Some(anime) = items.next_item().await {
//!     let anime = anime?;
//!     println!("{:?}", anime.title);
//! }
//! # Ok(())
//! # }
//! ```
//!
//! [`Paginator::next_page`] hands over whole pages instead, which keeps the page
//! number of a failure with the entries that were already delivered.

use crate::client::AniListClient;
use crate::errors::AniListError;
use crate::objects::responses::Page;
use std::collections::VecDeque;
use std::future::Future;

/// How many pages a walk asks for before it stops, unless changed.
///
/// A guard rather than a limit that a server can talk its way past: if a query
/// keeps answering `hasNextPage: true`, the walk still ends.
pub const MAX_PAGES: u32 = 1_000;

/// A list query that can be pointed at another page and run again.
///
/// Implemented for the options struct of every paginated endpoint, so
/// [`AniListClient::paginate`](crate::client::AniListClient::paginate) accepts
/// them directly.
pub trait Paginated {
    /// One entry of a page.
    type Item;

    /// Runs the query as the options currently stand.
    fn fetch(
        &self,
        client: &AniListClient,
    ) -> impl Future<Output = Result<Page<Vec<Self::Item>>, AniListError>> + Send;

    /// Points the query at `page`, which is one based.
    fn set_page(&mut self, page: i32);
}

/// Walks the pages of a list query.
///
/// Built by [`AniListClient::paginate`](crate::client::AniListClient::paginate).
/// It never retries and never sleeps between pages: whether a `429` is worth
/// waiting out is the caller's call, as everywhere else in the crate.
pub struct Paginator<Q: Paginated> {
    client: AniListClient,
    options: Q,
    next_page: i32,
    buffer: VecDeque<Q::Item>,
    finished: bool,
    max_pages: u32,
}

impl<Q: Paginated> Paginator<Q> {
    /// Starts a walk at page one.
    #[must_use]
    pub fn new(client: AniListClient, options: Q) -> Self {
        Self {
            client,
            options,
            next_page: 1,
            buffer: VecDeque::new(),
            finished: false,
            max_pages: MAX_PAGES,
        }
    }

    /// Stops the walk after `max_pages` pages, for callers that want a smaller cap.
    #[must_use]
    pub fn with_max_pages(mut self, max_pages: u32) -> Self {
        self.max_pages = max_pages;
        self
    }

    /// The page the next call will ask for, one based.
    #[must_use]
    pub fn next_page_number(&self) -> i32 {
        self.next_page
    }

    /// Whether the walk has handed over everything.
    #[must_use]
    pub fn is_finished(&self) -> bool {
        self.finished && self.buffer.is_empty()
    }

    /// The next page, or `None` once there is none.
    ///
    /// An error is handed over as it is and ends the walk, so a caller cannot
    /// spin on a failing query. A drop in the middle of a request cancels that
    /// request and leaves the walk where it was, which is safe to drop.
    pub async fn next_page(&mut self) -> Option<Result<Page<Vec<Q::Item>>, AniListError>> {
        if self.finished {
            return None;
        }
        if self.next_page > self.max_pages as i32 {
            self.finished = true;
            return None;
        }

        self.options.set_page(self.next_page);

        match self.options.fetch(&self.client).await {
            Ok(page) => {
                if !has_more(&page) {
                    self.finished = true;
                }
                self.next_page += 1;
                Some(Ok(page))
            }
            Err(error) => {
                self.finished = true;
                Some(Err(error))
            }
        }
    }

    /// The next entry, walking on to the next page when this one runs out.
    pub async fn next_item(&mut self) -> Option<Result<Q::Item, AniListError>> {
        loop {
            if let Some(item) = self.buffer.pop_front() {
                return Some(Ok(item));
            }
            if self.finished {
                return None;
            }

            match self.next_page().await? {
                Ok(page) => self.buffer.extend(page.data),
                Err(error) => return Some(Err(error)),
            }
        }
    }

    /// Everything that is left, or the first error that came up.
    ///
    /// # Errors
    ///
    /// The failure that ended the walk, with the entries gathered so far dropped.
    /// Use [`Paginator::next_item`] when a partial result matters.
    pub async fn collect_all(&mut self) -> Result<Vec<Q::Item>, AniListError> {
        let mut all = Vec::new();

        loop {
            match self.next_page().await {
                Some(Ok(page)) => all.extend(page.data),
                Some(Err(error)) => return Err(error),
                None => return Ok(all),
            }
        }
    }
}

/// Whether the server gave any reason to ask for another page.
fn has_more<T>(page: &Page<Vec<T>>) -> bool {
    if page.data.is_empty() {
        return false;
    }

    match page.page_info.as_ref() {
        Some(info) => match info.has_next_page {
            Some(has_next_page) => has_next_page,
            // No word either way: trust the page number and the count when they
            // are there, and otherwise keep asking until a page comes back empty.
            None => info
                .last_page
                .zip(info.current_page)
                .is_none_or(|(last, current)| current < last),
        },
        None => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::objects::common::PageInfo;

    fn page(items: usize, info: Option<PageInfo>) -> Page<Vec<u32>> {
        Page {
            page_info: info,
            data: (0..items as u32).collect(),
        }
    }

    fn info(has_next_page: Option<bool>, current: Option<i32>, last: Option<i32>) -> PageInfo {
        PageInfo {
            total: None,
            per_page: None,
            current_page: current,
            last_page: last,
            has_next_page,
        }
    }

    #[test]
    fn an_explicit_has_next_page_decides() {
        assert!(has_more(&page(2, Some(info(Some(true), Some(1), Some(4))))));
        assert!(!has_more(&page(
            2,
            Some(info(Some(false), Some(4), Some(4)))
        )));
    }

    #[test]
    fn an_empty_page_always_ends_the_walk() {
        assert!(!has_more(&page(
            0,
            Some(info(Some(true), Some(1), Some(4)))
        )));
    }

    #[test]
    fn without_a_flag_the_page_numbers_decide() {
        assert!(has_more(&page(2, Some(info(None, Some(1), Some(4))))));
        assert!(!has_more(&page(2, Some(info(None, Some(4), Some(4))))));
        // No counts either: keep asking, the walk is capped anyway.
        assert!(has_more(&page(2, Some(info(None, None, None)))));
    }

    #[test]
    fn without_pagination_info_there_is_nothing_to_follow() {
        assert!(!has_more(&page(20, None)));
    }
}
