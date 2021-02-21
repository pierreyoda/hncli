use std::io::Stdout;

use async_trait::async_trait;

use app::App;
use handlers::Key;
use tui::{
    backend::CrosstermBackend,
    layout::{Alignment::Center, Rect},
    text::Spans,
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};

use super::common::HNCLI_VERSION;
use crate::{
    api::HnClient,
    app,
    errors::Result,
    ui::{
        common::{UiComponent, UiComponentId, UiTickScalar},
        handlers,
    },
};

/// The About component contains the version number,
/// a short description of the project and some
/// help text for commands and shortcuts.
#[derive(Debug)]
pub struct Help {}

impl Default for Help {
    fn default() -> Self {
        Self {}
    }
}

pub const HELP_ID: UiComponentId = "about";

#[async_trait]
impl UiComponent for Help {
    fn id(&self) -> UiComponentId {
        HELP_ID
    }

    fn should_update(&mut self, _elapsed_ticks: UiTickScalar, _app: &App) -> Result<bool> {
        Ok(false)
    }

    async fn update(&mut self, _client: &mut HnClient, _app: &mut App) -> Result<()> {
        Ok(())
    }

    fn key_handler(&mut self, key: &Key, app: &mut App) -> Result<bool> {
        Ok(match key {
            Key::Escape | Key::Enter | Key::Char('h') => {
                app.pop_navigation_stack();
                true
            }
            _ => false,
        })
    }

    fn render(
        &self,
        f: &mut Frame<CrosstermBackend<Stdout>>,
        inside: Rect,
        _app: &App,
    ) -> Result<()> {
        let block = Block::default()
            .border_type(BorderType::Thick)
            .borders(Borders::ALL);

        let text = vec![
            Spans::from(format!("hncli {}", HNCLI_VERSION)),
            Spans::from(""),
        ];

        let paragraph = Paragraph::new(text).block(block).alignment(Center);

        f.render_widget(paragraph, inside);

        Ok(())
    }
}
