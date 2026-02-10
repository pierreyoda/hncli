use std::sync::Arc;

use futures::lock::{Mutex, MutexGuard};

use crate::errors::Result;

use self::{algolia_client::AlgoliaHnClient, client::ClassicHnClient};

pub mod algolia_client;
pub mod algolia_types;
pub mod client;
pub mod types;

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
