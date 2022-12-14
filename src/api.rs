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
    classic_client: ClassicHnClient,
    /// Algolia Hacker News API client.
    ///
    /// Documentation: https://hn.algolia.com/api
    algolia_client: AlgoliaHnClient,
}

impl HnClient {
    pub fn classic(&self) -> &ClassicHnClient {
        &self.classic_client
    }

    pub fn algolia(&self) -> &AlgoliaHnClient {
        &self.algolia_client
    }
}

impl HnClient {
    pub fn new() -> Result<Self> {
        Ok(Self {
            classic_client: ClassicHnClient::new()?,
            algolia_client: AlgoliaHnClient::new()?,
        })
    }
}
