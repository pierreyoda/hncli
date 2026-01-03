use async_trait::async_trait;

use ratatui::{
    layout::{Constraint, Direction, HorizontalAlignment, Layout, Rect},
    text::Line,
    widgets::{Block, BorderType, Borders, Paragraph},
};

use crate::{
    api::HnClient,
    app::AppContext,
    config::HNCLI_VERSION,
    errors::Result,
    ui::common::{RenderFrame, UiComponent, UiComponentId, UiTickScalar},
};

/// The About component contains the version number,
/// a short description of the project and some
/// help text for commands and shortcuts.
#[derive(Debug, Default)]
pub struct Help {}

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

    fn handle_inputs(&mut self, _ctx: &mut AppContext) -> Result<bool> {
        Ok(false)
    }

    fn render(&mut self, f: &mut RenderFrame, inside: Rect, _ctx: &AppContext) -> Result<()> {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(10), Constraint::Percentage(90)].as_ref())
            .split(inside);

        Self::render_about_block(f, chunks[0]);
        Self::render_help_block(f, chunks[1]);

        Ok(())
    }
}

impl Help {
    fn render_about_block(f: &mut RenderFrame, inside: Rect) {
        let text = vec![
            Line::from(format!("hncli {HNCLI_VERSION}")),
            Line::from("https://github.com/pierreyoda/hncli"),
            Line::from(
                "A Terminal User Interface-based application for browsing Hacker News, written in 🦀 Rust. ",
            ),
        ];
        let paragraph = Paragraph::new(text)
            .block(Self::get_common_block())
            .alignment(HorizontalAlignment::Center);
        f.render_widget(paragraph, inside);
    }

    fn render_help_block(f: &mut RenderFrame, inside: Rect) {
        let text = vec![
            Line::from(""),
            Line::from("Press 'h' to toggle help."),
            Line::from(""),
            Line::from("Press 'q' to quit."),
            Line::from(""),
            Line::from("Go back with 'escape'."),
            Line::from(""),
            Line::from(
                "Navigate between screens with the left and right arrow keys, or 'j' and 'l'.",
            ),
            Line::from(""),
            Line::from("Navigate between stories with the up and down arrow keys, or 'i' and 'k'."),
            Line::from(""),
            Line::from(
                "Open a tab in your browser for the selected story with 'o'. Open the selected story page with 'enter'.",
            ),
            Line::from(""),
            Line::from(""),
            Line::from("--- On a story page ---"),
            Line::from(""),
            Line::from(
                "Open a tab in your browser for the selected story (or its source) with 'o'.",
            ),
            Line::from(""),
            Line::from("Toggle comments with 'tab'."),
            Line::from(""),
            Line::from(
                "Navigate comments with the up and down arrow keys, and focus a comment with the 'enter' key.",
            ),
            Line::from(""),
            Line::from(""),
            Line::from(""),
            Line::from("--- On a comment ---"),
            Line::from(""),
            Line::from("Open the user profile with 'p'."),
            Line::from(""),
            Line::from(""),
            Line::from("--- On the settings page ---"),
            Line::from(""),
            Line::from(
                "Navigate between settings with the up and down arrow keys, or 'i' and 'k'.",
            ),
            Line::from(""),
            Line::from("Toggle a setting with 'tab'."),
            Line::from(""),
            Line::from("Go back with 'escape'."),
        ];
        let paragraph = Paragraph::new(text)
            .block(Self::get_common_block())
            .alignment(HorizontalAlignment::Center);
        f.render_widget(paragraph, inside);
    }

    fn get_common_block() -> Block<'static> {
        Block::default()
            .border_type(BorderType::Thick)
            .borders(Borders::ALL)
    }
}
