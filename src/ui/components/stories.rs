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
        types::{HnItemIdScalar, HnStory},
        HnClient,
    },
    app::App,
    errors::{HnCliError, Result},
    ui::{
        common::{UiComponent, UiComponentId, UiTickScalar},
        handlers::Key,
        utils::{datetime_from_hn_time, StatefulList},
    },
};

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
const MEAN_TICKS_BETWEEN_UPDATES: UiTickScalar = 600; // approx. every minute

impl Default for StoriesPanel {
    fn default() -> Self {
        Self {
            ticks_since_last_update: 0,
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

    fn should_update(&mut self, elapsed_ticks: UiTickScalar) -> Result<bool> {
        self.ticks_since_last_update += elapsed_ticks;

        Ok(self.ticks_since_last_update >= MEAN_TICKS_BETWEEN_UPDATES)
    }

    async fn update(&mut self, client: &mut HnClient, app: &mut App) -> Result<()> {
        self.ticks_since_last_update = 0;

        // let stories = client.get_home_stories(HnStoriesSorting::Top).await?;
        // let cut_stories_iter = stories.iter().take(self.list_cutoff);
        // let displayable_stories: Vec<DisplayableHackerNewsStory> = cut_stories_iter
        //     .map(|story| {
        //         DisplayableHackerNewsStory::try_from(story.clone())
        //             .expect("can map DisplayableHackerNewsStory")
        //     })
        //     .collect();
        // self.list_state.replace_items(displayable_stories);

        Ok(())
    }

    fn key_handler(&mut self, key: &Key, app: &mut App) -> Result<bool> {
        Ok(false)
    }

    fn render(&self, f: &mut tui::Frame<CrosstermBackend<Stdout>>, inside: Rect) -> Result<()> {
        let block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .style(Style::default().fg(Color::White))
            .title("Stories");

        // List Items
        let stories = self.list_state.get_items();
        let list_stories_items: Vec<ListItem> = stories
            .iter()
            .map(|story| {
                ListItem::new(Spans::from(vec![Span::styled(
                    story.title.clone(),
                    Style::default(),
                )]))
            })
            .collect();

        // List
        let list_stories = List::new(list_stories_items)
            .block(block)
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
