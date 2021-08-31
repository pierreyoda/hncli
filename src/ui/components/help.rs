use std::io::Stdout;

use async_trait::async_trait;

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
    app::AppContext,
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

    fn should_update(&mut self, _elapsed_ticks: UiTickScalar, _ctx: &AppContext) -> Result<bool> {
        Ok(false)
    }

    async fn update(&mut self, _client: &mut HnClient, _ctx: &mut AppContext) -> Result<()> {
        Ok(())
    }

    fn key_handler(&mut self, _key: &Key, _ctx: &mut AppContext) -> Result<bool> {
        Ok(false)
    }

    fn render(
        &self,
        f: &mut Frame<CrosstermBackend<Stdout>>,
        inside: Rect,
        _ctx: &AppContext,
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
