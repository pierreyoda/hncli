use std::{
    convert::{TryFrom, TryInto},
    io::Stdout,
};

use async_trait::async_trait;
use html2text::from_read;
use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Span, Spans},
    widgets::{Block, BorderType, Borders, Tabs},
    Frame,
};

use crate::{
    api::{HnClient, HnStoriesSorting},
    app::{App, AppBlock},
    errors::{HnCliError, Result},
    ui::{
        common::{UiComponent, UiComponentId, UiTickScalar},
        handlers::Key,
    },
};

use super::{
    common::get_layout_block_style, stories::DisplayableHackerNewsItem,
    widgets::story_header::StoryHeader,
};

/// The Thread component provides context-dependent options
/// for the current active component.
#[derive(Debug)]
pub struct Thread {
    ticks_since_last_update: UiTickScalar,
    item_details: Option<DisplayableHackerNewsItem>,
    cached_item_corpus: Option<String>,
}

// TODO: load from configuration
const MEAN_TICKS_BETWEEN_UPDATES: UiTickScalar = 600; // approx. every minute

impl Default for Thread {
    fn default() -> Self {
        Self {
            ticks_since_last_update: 0,
            item_details: None,
            cached_item_corpus: None,
        }
    }
}

pub const THREAD_ID: UiComponentId = "thread";

#[async_trait]
impl UiComponent for Thread {
    fn id(&self) -> UiComponentId {
        THREAD_ID
    }

    fn should_update(&mut self, elapsed_ticks: UiTickScalar, app: &App) -> Result<bool> {
        self.ticks_since_last_update += elapsed_ticks;

        // TODO: only compare IDs?
        Ok(self.ticks_since_last_update >= MEAN_TICKS_BETWEEN_UPDATES
            || app.get_currently_viewed_item() != &self.item_details)
    }

    async fn update(&mut self, _client: &mut HnClient, app: &mut App) -> Result<()> {
        self.item_details = app.get_currently_viewed_item().clone();

        Ok(())
    }

    fn key_handler(&mut self, key: &Key, app: &mut App) -> Result<bool> {
        Ok(match key {
            _ => false,
        })
    }

    fn render(
        &self,
        f: &mut Frame<CrosstermBackend<Stdout>>,
        inside: Rect,
        app: &App,
    ) -> Result<()> {
        // Layout
        let block = Block::default()
            .style(get_layout_block_style(app, AppBlock::ItemThread))
            .border_type(BorderType::Thick)
            .borders(Borders::ALL)
            .title("Thread");

        let vertical_chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(2)
            .constraints([Constraint::Length(2)].as_ref())
            .split(inside);

        if let Some(ref item) = self.item_details {
            // Header
            let header = StoryHeader::new(item.clone());
            f.render_widget(header, vertical_chunks[0])
        }

        Ok(())
    }
}
