//! The stories panel lists all the given Hacker News stories.

use std::{convert::TryFrom, io::Stdout};

use async_trait::async_trait;
use chrono::{DateTime, Utc};

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
    app::AppHandle,
    errors::{HnCliError, Result},
    ui::{
        common::{UiComponent, UiComponentId, UiTickScalar},
        handlers::Key,
        utils::{datetime_from_hn_time, StatefulList},
    },
};

use super::common::COMMON_BLOCK_NORMAL_COLOR;

/// A display-ready Hacker News story or job posting.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DisplayableHackerNewsItem {
    /// Unique ID.
    pub id: HnItemIdScalar,
    /// Posted at.
    pub posted_at: DateTime<Utc>,
    /// Posted since, formatted for display.
    pub posted_since: String,
    /// Username of the poster.
    pub by_username: String,
    /// Title.
    pub title: String,
    /// Score.
    pub score: u32,
    /// Hostname of the URL, if any.
    pub url_hostname: Option<String>,
}

const MINUTES_PER_DAY: i64 = 24 * 60;

impl DisplayableHackerNewsItem {
    pub fn formatted_posted_since(posted_at: &DateTime<Utc>) -> String {
        let now = Utc::now();
        let minutes = (now - *posted_at).num_minutes();
        match minutes {
            _ if minutes >= MINUTES_PER_DAY => {
                format!("{} ago", Self::pluralized(minutes / MINUTES_PER_DAY, "day"))
            }
            _ if minutes >= 60 => format!("{} ago", Self::pluralized(minutes / 60, "hour")),
            _ => format!("{} ago", Self::pluralized(minutes, "minute")),
        }
    }

    fn pluralized(value: i64, word: &str) -> String {
        if value > 1 {
            format!("{} {}s", value, word)
        } else {
            format!("{} {}", value, word)
        }
    }
}

impl TryFrom<HnItem> for DisplayableHackerNewsItem {
    type Error = HnCliError;

    fn try_from(value: HnItem) -> Result<Self> {
        match value {
            HnItem::Story(story) => {
                let posted_at = datetime_from_hn_time(story.time);
                Ok(Self {
                    id: story.id,
                    posted_at,
                    posted_since: Self::formatted_posted_since(&posted_at),
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
                })
            }
            HnItem::Job(job) => {
                let posted_at = datetime_from_hn_time(job.time);
                Ok(Self {
                    id: job.id,
                    posted_at,
                    posted_since: Self::formatted_posted_since(&posted_at),
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
                })
            }
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

    fn should_update(&mut self, elapsed_ticks: UiTickScalar, app: &AppHandle) -> Result<bool> {
        self.ticks_since_last_update += elapsed_ticks;

        Ok(self.ticks_since_last_update >= MEAN_TICKS_BETWEEN_UPDATES
            || match &self.sorting_type_for_last_update {
                Some(last_sorting_type) => {
                    last_sorting_type != app.get_state().get_main_stories_sorting()
                }
                None => true, // first fetch
            })
    }

    async fn update(&mut self, client: &mut HnClient, app: &mut AppHandle) -> Result<()> {
        self.ticks_since_last_update = 0;

        let sorting_type = *app.get_state().get_main_stories_sorting();

        // Data fetching
        let stories = client.get_home_items(sorting_type).await?;
        let cut_stories_iter = stories.iter().take(self.list_cutoff);
        let displayable_stories: Vec<DisplayableHackerNewsItem> = cut_stories_iter
            .cloned()
            .map(|item| {
                DisplayableHackerNewsItem::try_from(item)
                    .expect("can map DisplayableHackerNewsItem")
            })
            .collect();

        // TODO: temp, for testing
        app.get_state_mut()
            .set_currently_viewed_item(Some(displayable_stories[0].clone()));

        self.list_state.replace_items(displayable_stories);

        self.sorting_type_for_last_update = Some(sorting_type);

        Ok(())
    }

    fn key_handler(&mut self, key: &Key, app: &mut AppHandle) -> Result<bool> {
        let selected = self.list_state.get_state().selected();
        Ok(match key {
            Key::Up | Key::Char('i') => {
                self.list_state.previous();
                true
            }
            Key::Down | Key::Char('k') => {
                self.list_state.next();
                true
            }
            Key::Enter if selected.is_some() => {
                let items = self.list_state.get_items();
                app.get_state_mut()
                    .set_currently_viewed_item(Some(items[selected.unwrap()].clone()));
                true
            }
            _ => false,
        })
    }

    fn render(
        &self,
        f: &mut tui::Frame<CrosstermBackend<Stdout>>,
        inside: Rect,
        _app: &AppHandle,
    ) -> Result<()> {
        let block = Block::default()
            .style(Style::default().fg(COMMON_BLOCK_NORMAL_COLOR))
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
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
            .highlight_symbol(">> ")
            .highlight_style(Style::default().fg(Color::Yellow));

        f.render_widget(list_stories, inside);

        Ok(())
    }
}
