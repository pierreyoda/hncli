use std::time::Duration;

use reqwest::Client;

use crate::{
    api::algolia_types::AlgoliaHnCommentsHits,
    errors::{HnCliError, Result},
};

use super::algolia_types::{AlgoliaHnFilter, AlgoliaHnSearchTag, AlgoliaHnStoriesHits};

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

    /// Perform a full-text query search with (optionally) filtering tags that will combine as AND.
    ///
    /// Returns the most recent Hacker News items first.
    pub async fn search_stories(
        &self,
        query: &str,
        tags: &[AlgoliaHnSearchTag],
    ) -> Result<AlgoliaHnStoriesHits> {
        // TODO: handle pagination? default page size seems to be 20 items

        // query params handling
        let tags = tags.iter().fold(String::new(), |acc, tag| {
            if acc.is_empty() {
                tag.to_query()
            } else {
                format!("{},{}", acc, tag.to_query())
            }
        });
        let url = format!(
            "{}/search_by_date?query={}{}",
            self.base_url,
            query,
            if tags.is_empty() {
                "".into()
            } else {
                format!("&tags={}", tags)
            }
        );

        // request
        let result: AlgoliaHnStoriesHits = self
            .client
            .get(url)
            .send()
            .await?
            .text()
            .await
            .map(|raw| {
                serde_json::from_str(&raw).expect("api.algolia.search: deserialization should work")
            })
            .map_err(HnCliError::HttpError)?;

        Ok(result)
    }

    /// Perform a full-text query search on Hacker News comments.
    pub async fn search_comments(&self, query: &str) -> Result<AlgoliaHnCommentsHits> {
        let url = format!("{}/search?query={}=&tags=comment", self.base_url, query);

        let result: AlgoliaHnCommentsHits = self
            .client
            .get(url)
            .send()
            .await?
            .text()
            .await
            .map(|raw| {
                serde_json::from_str(&raw)
                    .expect("api.algolia.searxh_xomments: deserialization should work")
            })
            .map_err(HnCliError::HttpError)?;

        Ok(result)
    }
}
