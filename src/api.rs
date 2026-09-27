use std::{future::Future, sync::Arc, time::Duration};

use futures::lock::{Mutex, MutexGuard};

use crate::errors::Result;

use self::{algolia_client::AlgoliaHnClient, client::ClassicHnClient};

pub mod algolia_client;
pub mod algolia_types;
pub mod client;
pub mod types;

/// Maximum number of attempts for a single retryable request, before giving up.
const RETRY_MAX_ATTEMPTS: u8 = 3;
/// Base delay between retry attempts (multiplied by the attempt count).
const RETRY_BASE_DELAY: Duration = Duration::from_millis(250);

/// Retry the given fallible async operation up to `RETRY_MAX_ATTEMPTS` times, with a
/// linearly increasing delay between attempts.
///
/// Intended for transient network/HTTP failures: the closure should only wrap the
/// actual request, not business-logic error branches (e.g. an item legitimately not
/// existing), which would otherwise be retried uselessly.
pub(crate) async fn with_retries<T, F, Fut>(operation: F) -> Result<T>
where
    F: Fn() -> Fut,
    Fut: Future<Output = Result<T>>,
{
    let mut last_error = None;
    for attempt in 0..RETRY_MAX_ATTEMPTS {
        if attempt > 0 {
            tokio::time::sleep(RETRY_BASE_DELAY * attempt as u32).await;
        }
        match operation().await {
            Ok(value) => return Ok(value),
            Err(err) => last_error = Some(err),
        }
    }
    Err(last_error.expect("with_retries: at least one attempt was made"))
}

/// The exposed Hacker News API client, wrapping two sources: official API and Algolia-based API.
pub struct HnClient {
    /// Original Hacker News API client.
    ///
    /// Documentation: https://github.com/HackerNews/API
    classic_client: Arc<Mutex<ClassicHnClient>>,
    /// Algolia Hacker News API client.
    ///
    /// Documentation: https://hn.algolia.com/api
    algolia_client: Arc<Mutex<AlgoliaHnClient>>,
}

impl HnClient {
    pub async fn classic(&self) -> MutexGuard<'_, ClassicHnClient> {
        self.classic_client.lock().await
    }

    pub fn classic_non_blocking(&self) -> Arc<Mutex<ClassicHnClient>> {
        Arc::clone(&self.classic_client)
    }

    pub async fn algolia(&self) -> MutexGuard<'_, AlgoliaHnClient> {
        self.algolia_client.lock().await
    }
}

impl HnClient {
    pub fn new() -> Result<Self> {
        Ok(Self {
            classic_client: Arc::new(Mutex::new(ClassicHnClient::new()?)),
            algolia_client: Arc::new(Mutex::new(AlgoliaHnClient::new()?)),
        })
    }
}
