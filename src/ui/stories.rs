//! The stories panel lists all the given Hacker News stories.

use std::convert::TryFrom;

use chrono::{DateTime, Utc};

use async_trait::async_trait;

use crate::{
    api::{
        types::{HnItemIdScalar, HnStory},
        HnClient, HnStoriesSorting,
    },
    app::App,
    errors::{HnCliError, Result},
};

use super::{
    common::{UiComponent, UiTickScalar},
    handlers::Key,
    utils::{datetime_from_hn_time, StatefulList},
};

// pub mod handler;
// pub mod renderer;

/// A display-ready Hacker News story.
#[derive(Clone, Debug)]
pub struct DisplayableHackerNewsStory {
    /// Unique ID.
    id: HnItemIdScalar,
    /// Posted at.
    posted_at: DateTime<Utc>,
    /// Username of the poster.
    by_username: String,
    /// Title.
    title: String,
    /// Score.
    score: u32,
    /// Hostname of the URL, if any.
    url_hostname: Option<String>,
}

impl TryFrom<HnStory> for DisplayableHackerNewsStory {
    type Error = HnCliError;

    fn try_from(value: HnStory) -> Result<Self> {
        Ok(Self {
            id: value.id,
            posted_at: datetime_from_hn_time(value.time),
            by_username: value.by,
            title: value.title,
            score: value.score,
            url_hostname: value.url.map(|url| {
                url::Url::parse(&url[..])
                    .map_err(HnCliError::UrlParsingError)
                    .expect("story URL parsing error") // TODO: avoid expect here
                    .host_str()
                    .expect("story URL must have an hostname")
                    .to_owned()
            }),
        })
    }
}

#[derive(Debug)]
pub struct StoriesPanel {
    ticks_since_last_update: u64,
    list_cutoff: usize,
    list_state: StatefulList<DisplayableHackerNewsStory>,
}
// TODO: load from configuration
const HOME_MAX_DISPLAYED_STORIES: usize = 20;
const MEAN_TICKS_BETWEEN_UPDATES: UiTickScalar = 60;

impl Default for StoriesPanel {
    fn default() -> Self {
        Self {
            ticks_since_last_update: 0,
            list_cutoff: HOME_MAX_DISPLAYED_STORIES,
            list_state: StatefulList::with_items(vec![]),
        }
    }
}

const STORIES_PANEL_ID: &str = "panel_stories";
#[async_trait]
impl UiComponent for StoriesPanel {
    fn id(&self) -> &'static str {
        STORIES_PANEL_ID
    }

    fn should_update(&mut self, elapsed_ticks: UiTickScalar) -> Result<bool> {
        self.ticks_since_last_update += elapsed_ticks;

        Ok(self.ticks_since_last_update >= MEAN_TICKS_BETWEEN_UPDATES)
    }

    async fn update(&mut self, client: &mut HnClient, app: &mut App) -> Result<()> {
        self.ticks_since_last_update = 0;

        let stories = client.get_home_stories(HnStoriesSorting::Top).await?;
        let cut_stories_iter = stories.iter().take(self.list_cutoff);
        let displayable_stories: Vec<DisplayableHackerNewsStory> = cut_stories_iter
            .map(|story| {
                DisplayableHackerNewsStory::try_from(story.clone())
                    .expect("can map DisplayableHackerNewsStory")
            })
            .collect();

        Ok(())
    }

    fn key_handler(&mut self, key: &Key, app: &mut App) -> Result<bool> {
        dbg!(key);

        Ok(false)
    }
}
