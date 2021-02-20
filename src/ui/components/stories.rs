//! The stories panel lists all the given Hacker News stories.

use std::{convert::TryFrom, io::Stdout};

use chrono::{DateTime, Utc};

use async_trait::async_trait;
use tui::{
    backend::CrosstermBackend,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, BorderType, Borders, List, ListItem},
};

use crate::{
    api::{
        types::{HnItem, HnItemIdScalar},
        HnClient, HnStoriesSorting,
    },
    app::{App, AppBlock},
    errors::{HnCliError, Result},
    ui::{
        common::{UiComponent, UiComponentId, UiTickScalar},
        handlers::Key,
        utils::{datetime_from_hn_time, StatefulList},
    },
};

use super::common::{COMMON_BLOCK_FOCUS_COLOR, COMMON_BLOCK_NORMAL_COLOR};

/// A display-ready Hacker News story or job posting.
#[derive(Clone, Debug)]
pub struct DisplayableHackerNewsItem {
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

impl TryFrom<HnItem> for DisplayableHackerNewsItem {
    type Error = HnCliError;

    fn try_from(value: HnItem) -> Result<Self> {
        match value {
            HnItem::Story(story) => Ok(Self {
                id: story.id,
                posted_at: datetime_from_hn_time(story.time),
                by_username: story.by,
                title: story.title,
                score: story.score,
                url_hostname: story.url.map(|url| {
                    url::Url::parse(&url[..])
                        .map_err(HnCliError::UrlParsingError)
                        .expect("story URL parsing error") // TODO: avoid expect here
                        .host_str()
                        .expect("story URL must have an hostname")
                        .to_owned()
                }),
            }),
            HnItem::Job(job) => Ok(Self {
                id: job.id,
                posted_at: datetime_from_hn_time(job.time),
                by_username: job.by,
                title: job.title,
                score: job.score,
                url_hostname: job.url.map(|url| {
                    url::Url::parse(&url[..])
                        .map_err(HnCliError::UrlParsingError)
                        .expect("job URL parsing error") // TODO: avoid expect here
                        .host_str()
                        .expect("job URL must have an hostname")
                        .to_owned()
                }),
            }),
            _ => Err(HnCliError::HnItemProcessingError(
                value.get_id().to_string(),
            )),
        }
    }
}

#[derive(Debug)]
pub struct StoriesPanel {
    ticks_since_last_update: u64,
    sorting_type_for_last_update: Option<HnStoriesSorting>,
    list_cutoff: usize,
    list_state: StatefulList<DisplayableHackerNewsItem>,
}

// TODO: load from configuration
const HOME_MAX_DISPLAYED_STORIES: usize = 50;
const MEAN_TICKS_BETWEEN_UPDATES: UiTickScalar = 600; // approx. every minute

impl Default for StoriesPanel {
    fn default() -> Self {
        Self {
            ticks_since_last_update: 0,
            sorting_type_for_last_update: None,
            list_cutoff: HOME_MAX_DISPLAYED_STORIES,
            list_state: StatefulList::with_items(vec![]),
        }
    }
}

pub const STORIES_PANEL_ID: UiComponentId = "panel_stories";

#[async_trait]
impl UiComponent for StoriesPanel {
    fn id(&self) -> UiComponentId {
        STORIES_PANEL_ID
    }

    fn should_update(&mut self, elapsed_ticks: UiTickScalar, app: &App) -> Result<bool> {
        self.ticks_since_last_update += elapsed_ticks;

        Ok(self.ticks_since_last_update >= MEAN_TICKS_BETWEEN_UPDATES
            || match &self.sorting_type_for_last_update {
                Some(last_sorting_type) => last_sorting_type != app.get_main_stories_sorting(),
                None => true, // first fetch
            })
    }

    async fn update(&mut self, client: &mut HnClient, app: &mut App) -> Result<()> {
        self.ticks_since_last_update = 0;

        let sorting_type = app.get_main_stories_sorting().clone();

        // Data fetching
        let stories = client.get_home_items(HnStoriesSorting::Top).await?;
        let cut_stories_iter = stories.iter().take(self.list_cutoff);
        let displayable_stories: Vec<DisplayableHackerNewsItem> = cut_stories_iter
            .cloned()
            .map(|item| {
                DisplayableHackerNewsItem::try_from(item)
                    .expect("can map DisplayableHackerNewsItem")
            })
            .collect();
        self.list_state.replace_items(displayable_stories);

        self.sorting_type_for_last_update = Some(sorting_type);

        Ok(())
    }

    fn key_handler(&mut self, key: &Key, app: &mut App) -> Result<bool> {
        Ok(false)
    }

    fn render(
        &self,
        f: &mut tui::Frame<CrosstermBackend<Stdout>>,
        inside: Rect,
        app: &App,
    ) -> Result<()> {
        let block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .style(
                Style::default().fg(if app.has_current_focus(AppBlock::HomeStories) {
                    COMMON_BLOCK_FOCUS_COLOR
                } else {
                    COMMON_BLOCK_NORMAL_COLOR
                }),
            )
            .title("Stories");

        // List Items
        let stories = self.list_state.get_items();
        let list_stories_items: Vec<ListItem> = stories
            .iter()
            .map(|item| {
                ListItem::new(Spans::from(vec![Span::styled(
                    item.title.clone(),
                    Style::default().fg(Color::White),
                )]))
            })
            .collect();

        // List
        let list_stories = List::new(list_stories_items)
            .block(block)
            .style(Style::default().fg(Color::White))
            .highlight_style(
                Style::default()
                    .bg(Color::Yellow)
                    .fg(Color::Black)
                    .add_modifier(Modifier::BOLD),
            )
            .highlight_symbol(">> ");

        f.render_widget(list_stories, inside);

        Ok(())
    }
}
