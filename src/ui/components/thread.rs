use std::io::Stdout;

use async_trait::async_trait;
use html2text::from_read;
use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect},
    style::Style,
    widgets::{Block, BorderType, Borders},
    Frame,
};

use crate::{
    api::HnClient,
    app::AppHandle,
    errors::Result,
    ui::{
        common::{UiComponent, UiComponentId, UiTickScalar},
        handlers::Key,
    },
};

use super::{
    common::COMMON_BLOCK_NORMAL_COLOR, stories::DisplayableHackerNewsItem,
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

    fn should_update(&mut self, elapsed_ticks: UiTickScalar, app: &AppHandle) -> Result<bool> {
        self.ticks_since_last_update += elapsed_ticks;

        Ok(self.ticks_since_last_update >= MEAN_TICKS_BETWEEN_UPDATES
            || match &self.item_details {
                None => true,
                Some(item) => match app.get_state().get_currently_viewed_item() {
                    Some(app_item) => item.id != app_item.id,
                    None => false,
                },
            })
    }

    async fn update(&mut self, _client: &mut HnClient, app: &mut AppHandle) -> Result<()> {
        self.ticks_since_last_update = 0;
        self.item_details = app.get_state().get_currently_viewed_item().clone();

        Ok(())
    }

    fn key_handler(&mut self, key: &Key, _app: &mut AppHandle) -> Result<bool> {
        Ok(match key {
            _ => false,
        })
    }

    fn render(
        &self,
        f: &mut Frame<CrosstermBackend<Stdout>>,
        inside: Rect,
        _app: &AppHandle,
    ) -> Result<()> {
        // Layout
        let block = Block::default()
            .style(Style::default().fg(COMMON_BLOCK_NORMAL_COLOR))
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
