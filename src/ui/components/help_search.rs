use async_trait::async_trait;
use ratatui::{
    layout::{Alignment::Center, Constraint, Direction, Layout, Rect},
    text::Line,
    widgets::{Block, BorderType, Borders, Paragraph},
};

use crate::{
    api::HnClient,
    app::AppContext,
    errors::Result,
    ui::common::{RenderFrame, UiComponent, UiComponentId, UiTickScalar},
};

/// The Algolia search help component displays instructions on how to use it.
#[derive(Debug, Default)]
pub struct AlgoliaHelp {}

pub const ALGOLIA_HELP_ID: UiComponentId = "algolia_help";

#[async_trait]
impl UiComponent for AlgoliaHelp {
    fn id(&self) -> UiComponentId {
        ALGOLIA_HELP_ID
    }

    fn should_update(&mut self, _elapsed_ticks: UiTickScalar, _ctx: &AppContext) -> Result<bool> {
        Ok(false)
    }

    async fn update(&mut self, _client: &mut HnClient, _ctx: &mut AppContext) -> Result<()> {
        Ok(())
    }

    fn handle_inputs(&mut self, _ctx: &mut AppContext) -> Result<bool> {
        Ok(false)
    }

    fn render(&mut self, f: &mut RenderFrame, inside: Rect, _ctx: &AppContext) -> Result<()> {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(10), Constraint::Percentage(90)].as_ref())
            .split(inside);

        Self::render_help_block(f, chunks[1]);

        Ok(())
    }
}

impl AlgoliaHelp {
    fn render_help_block(f: &mut RenderFrame, inside: Rect) {
        let text = vec![
            Line::from("=== Algolia Search Help ==="),
            Line::from(""),
            Line::from("Press 'h' to toggle help."),
            Line::from(""),
            Line::from("Press 'Escape' to return to the search screen."),
            Line::from(""),
            Line::from("Navigate the sections with the up and down arrow keys."),
            Line::from(""),
            Line::from("--- Results sections ---"),
            Line::from(""),
            Line::from("Press 'Enter' to focus, and 'Escape' to unfocus."),
            Line::from(""),
            Line::from("Navigate the results with the up and down arrow keys."),
            Line::from(""),
            Line::from(
                "On a result entry, press 'o' to visit the link, and 'l' for the Hacker News comments.",
            ),
        ];
        let paragraph = Paragraph::new(text)
            .block(Self::get_common_block())
            .alignment(Center);
        f.render_widget(paragraph, inside);
    }

    fn get_common_block() -> Block<'static> {
        Block::default()
            .border_type(BorderType::Thick)
            .borders(Borders::ALL)
    }
}
