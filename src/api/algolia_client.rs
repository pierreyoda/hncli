use std::time::Duration;

use reqwest::Client;

use crate::errors::Result;

const ALGOLIA_HACKER_NEWS_API_BASE_URL: &str = "http://hn.algolia.com/api/v1";

/// The internal Algolia Hacker News API client.
///
/// Documentation: https://hn.algolia.com/api
pub struct AlgoliaHnClient {
    /// Base URL of the Algolia Hacker News API.
    base_url: &'static str,
    /// `reqwest`client.
    client: Client,
}

impl AlgoliaHnClient {
    pub fn new() -> Result<Self> {
        Ok(Self {
            base_url: ALGOLIA_HACKER_NEWS_API_BASE_URL,
            // TODO: duration from CLI args and/or local configuration
            client: Client::builder().timeout(Duration::from_secs(10)).build()?,
        })
    }
}
