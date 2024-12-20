use std::{collections::HashMap, time::Duration};

use async_recursion::async_recursion;
use futures::future::join_all;
use reqwest::Client;

use crate::errors::{HnCliError, Result};

use super::types::{HnDead, HnDeleted, HnItem, HnItemIdScalar, HnUser};

const HACKER_NEWS_API_BASE_URL: &str = "https://hacker-news.firebaseio.com/v0";

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
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

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum HnStoriesSections {
    Home,
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
            Home => "topstories",
            Ask => "askstories",
            Show => "showstories",
            Jobs => "jobstories",
        }
    }
}

/// Get the resource URL fragment corresponding to the given user ID.
fn get_user_data_resource(id: &str) -> String {
    format!("/user/{}", id)
}

/// The internal Hacker News API client.
pub struct ClassicHnClient {
    /// Base URL of the Hacker News API.
    base_url: &'static str,
    /// `reqwest` client.
    client: Client,
}

/// Flat storage structure for a comments thread.
pub type HnItemComments = HashMap<HnItemIdScalar, HnItem>;

/// Flat storage structure for O(1) check of already fetched comments.
pub type HnStoredItemCommentsIds = HashMap<HnItemIdScalar, ()>;

// TODO: timeouts should be logged and not panic in every case except first ever request (how to track?)
impl ClassicHnClient {
    pub fn new() -> Result<Self> {
        Ok(Self {
            base_url: HACKER_NEWS_API_BASE_URL,
            // TODO: duration from CLI args and/or local configuration
            client: Client::builder().timeout(Duration::from_secs(10)).build()?,
        })
    }

    /// Try to fetch user data from its **case-sensitive** ID (the username).
    ///
    /// NB: as per the [documentation](https://github.com/HackerNews/API#users),
    /// "only users that have public activity (comments or story submissions) on the site are available through the API".
    /// From my own testing, unknown/inaccessible user accounts return the string "null".
    /// In such a case, we return the error `HnCliError::UserNotFound`.
    pub async fn get_user_data(&self, username: &str) -> Result<HnUser> {
        let raw = self
            .client
            .get(&format!(
                "{}/{}.json",
                self.base_url,
                get_user_data_resource(username)
            ))
            .send()
            .await?
            .text()
            .await
            .map_err(HnCliError::HttpError)?;
        // handle null case (not found or no public activity)
        if raw == "null" {
            return Err(HnCliError::UserNotFound(username.into()));
        }
        // general case
        let user: HnUser = serde_json::from_str(&raw).expect(&format!(
            "api.get_user_data: deserialization should work for user with ID {}",
            username
        ));
        Ok(user)
    }

    /// Try to fetch the items of the home page, with the given sorting strategy.
    pub async fn get_home_items(&self, sorting: &HnStoriesSorting) -> Result<Vec<HnItem>> {
        let stories_ids = self.get_home_stories_ids_listing(sorting).await?;
        let stories_ids_cutoff: Vec<HnItemIdScalar> =
            stories_ids.iter().take(100).copied().collect();
        let items = self.get_items(&stories_ids_cutoff[..]).await?;
        Ok(items)
    }

    /// Try to fetch the items of the home page, with the given section option.
    pub async fn get_home_section_items(&self, section: &HnStoriesSections) -> Result<Vec<HnItem>> {
        let stories_ids = self.get_home_section_stories_ids_listing(section).await?;
        let stories_ids_cutoff: Vec<HnItemIdScalar> =
            stories_ids.iter().take(100).copied().collect();
        let items = self.get_items(stories_ids_cutoff.as_slice()).await?;
        Ok(items)
    }

    /// Try to fetch the stories' IDs of the home page (up to 500), with the given sorting strategy.
    pub async fn get_home_stories_ids_listing(
        &self,
        sorting: &HnStoriesSorting,
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

    /// Try to fetch the stories' IDs of the home page for the given section option.
    pub async fn get_home_section_stories_ids_listing(
        &self,
        section: &HnStoriesSections,
    ) -> Result<Vec<HnItemIdScalar>> {
        self.client
            .get(&format!(
                "{}/{}.json",
                self.base_url,
                section.get_resource()
            ))
            .send()
            .await?
            .json()
            .await
            .map_err(HnCliError::HttpError)
    }

    /// Try to fetch the comments of an item, starting from the main descendants.
    #[async_recursion]
    pub async fn get_item_comments(
        &self,
        descendants_ids: &[HnItemIdScalar],
        // TODO: each cached comment should have a timestamp for auto-refresh
        cached_comments_ids: &HnStoredItemCommentsIds,
        hard_refresh: bool,
    ) -> Result<HnItemComments> {
        let filter_comment_id = |id: &HnItemIdScalar| {
            if hard_refresh {
                true
            } else {
                !cached_comments_ids.contains_key(id)
            }
        };

        let main_descendants_ids: Vec<HnItemIdScalar> = descendants_ids
            .iter()
            .filter(|id| filter_comment_id(id))
            .copied()
            .collect();
        let main_descendants = self.get_items(&main_descendants_ids).await?;

        let descendants_ids: Vec<HnItemIdScalar> = main_descendants
            .iter()
            .fold(vec![], |mut acc: Vec<HnItemIdScalar>, descendant| {
                let descendant_kids_ids = descendant.get_kids().unwrap_or(&[]);
                acc.extend(
                    descendant_kids_ids
                        .iter()
                        .filter(|id| filter_comment_id(id)),
                );
                acc
            })
            .iter()
            .copied()
            .collect();
        let descendants = if descendants_ids.is_empty() {
            HnItemComments::new()
        } else {
            self.get_item_comments(&descendants_ids, cached_comments_ids, hard_refresh)
                .await?
        };

        // TODO: this could probably be faster
        let mut item_comments = HnItemComments::new();
        for main_descendant in main_descendants {
            item_comments.insert(main_descendant.get_id(), main_descendant);
        }
        for (descendant_id, descendant_item) in descendants {
            item_comments.insert(descendant_id, descendant_item);
        }

        Ok(item_comments)
    }

    /// Try to fetch the `HnItem` by its given ID.
    pub async fn get_item(&self, id: HnItemIdScalar) -> Result<HnItem> {
        self.client
            .get(&format!("{}/item/{}.json", self.base_url, id))
            .send()
            .await?
            .text()
            .await
            .map(|raw| {
                // handle null case
                if raw == "null" {
                    return HnItem::Null;
                }
                // handle deleted case
                if let Ok(deleted) = serde_json::from_str::<HnDeleted>(&raw) {
                    return HnItem::Deleted(deleted);
                }
                // handle dead case
                if let Ok(dead) = serde_json::from_str::<HnDead>(&raw) {
                    return HnItem::Dead(dead);
                }
                // general case
                serde_json::from_str(&raw).expect(&format!(
                    "api.classic.get_item: deserialization should work for item with ID {}",
                    id
                ))
            })
            .map_err(HnCliError::HttpError)
    }

    /// Try to *concurrently* fetch multiple `HnItem`s by their given IDs.
    pub async fn get_items(&self, ids: &[HnItemIdScalar]) -> Result<Vec<HnItem>> {
        // TODO: can we easily parallelize this over multiple threads for big (500) fetches?
        join_all(ids.iter().map(|id| self.get_item(*id)))
            .await
            .into_iter()
            .filter(|item_result| match item_result {
                Ok(item) => !item.is_null() && !item.is_deleted() && !item.is_dead(),
                Err(_) => false,
            })
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
