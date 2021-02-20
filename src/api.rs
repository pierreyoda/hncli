use std::time::Duration;

use futures::future::join_all;
use reqwest::Client;
use types::{HnItem, HnItemIdScalar};

use crate::errors::{HnCliError, Result};

pub mod types;

const HACKER_NEWS_API_BASE_URL: &str = "https://hacker-news.firebaseio.com/v0";

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum HnStoriesSorting {
    New,
    Top,
    Best,
}

impl HnStoriesSorting {
    /// Get the corresponding resource URL fragment.
    ///
    /// See [here](https://github.com/HackerNews/API#new-top-and-best-stories).
    pub fn get_resource(&self) -> &str {
        use HnStoriesSorting::*;

        match self {
            New => "newstories",
            Top => "topstories",
            Best => "beststories",
        }
    }
}

#[derive(Debug)]
pub enum HnStoriesSections {
    Ask,
    Show,
    Jobs,
}

impl HnStoriesSections {
    /// Get the corresponding resource URL fragment.
    ///
    /// See [here](https://github.com/HackerNews/API#ask-show-and-job-stories).
    pub fn get_resource(&self) -> &str {
        use HnStoriesSections::*;

        match self {
            Ask => "askstories",
            Show => "showstories",
            Jobs => "jobstories",
        }
    }
}

/// The internal Hacker News API client.
///
/// TODO: add simple caching, with `time`-based invalidation.
/// TODO: integrate error recovery with caching strategy
pub struct HnClient {
    /// Base URL of the Hacker News API.
    base_url: &'static str,
    /// `reqwest` client.
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

    /// Try to fetch the items of the home page, with the given sorting strategy.
    pub async fn get_home_items(&self, sorting: HnStoriesSorting) -> Result<Vec<HnItem>> {
        let stories_ids = self.get_home_stories_ids_listing(sorting).await?;
        let stories_ids_cutoff: Vec<HnItemIdScalar> =
            stories_ids.iter().take(50).copied().collect();
        let items = self.get_items(&stories_ids_cutoff[..]).await?;
        Ok(items)
    }

    /// Try to fetch the stories' IDs of the home page (up to 500), with the given sorting strategy.
    pub async fn get_home_stories_ids_listing(
        &self,
        sorting: HnStoriesSorting,
    ) -> Result<Vec<HnItemIdScalar>> {
        self.client
            .get(&format!(
                "{}/{}.json",
                self.base_url,
                sorting.get_resource()
            ))
            .send()
            .await?
            .json()
            .await
            .map_err(HnCliError::HttpError)
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

    /// Try to *concurrently* fetch multiple `HnItem`s by their given IDs.
    pub async fn get_items(&self, ids: &[HnItemIdScalar]) -> Result<Vec<HnItem>> {
        // TODO: can we easily parallelize this over multiple threads for big (500) fetches?
        join_all(ids.iter().map(|id| self.get_item(*id)))
            .await
            .into_iter()
            .collect()
    }

    /// Try to fetch the ID of the latest `HnItem` inserted into the Firebase store.
    pub async fn get_max_item_id(&self) -> Result<HnItemIdScalar> {
        self.client
            .get(&format!("{}/maxitem.json", self.base_url))
            .send()
            .await?
            .json()
            .await
            .map_err(HnCliError::HttpError)
    }
}
