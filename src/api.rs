use std::{time::Duration, todo};

use reqwest::Client;
use types::{HnItem, HnItemIdScalar};

use crate::errors::{HnCliError, Result};

pub mod types;

const HACKER_NEWS_API_BASE_URL: &str = "https://hacker-news.firebaseio.com/v0";

/// The internal Hacker News API client.
pub struct HnClient {
    base_url: &'static str,
    client: Client,
}

impl HnClient {
    pub fn new() -> Result<Self> {
        Ok(Self {
            base_url: HACKER_NEWS_API_BASE_URL,
            // TODO: duration from CLI args and/or local configuration
            client: Client::builder().timeout(Duration::from_secs(10)).build()?,
        })
    }

    /// Try to fetch the `HnItem` by its given ID.
    pub async fn get_item(&self, id: HnItemIdScalar) -> Result<HnItem> {
        self.client
            .get(&format!("{}/item/{}.json", self.base_url, id))
            .send()
            .await?
            .json()
            .await
            .map_err(HnCliError::HttpError)
    }
}
