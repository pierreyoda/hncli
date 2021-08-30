use std::io::Stdout;

use tui::{
    backend::CrosstermBackend,
    layout::{Alignment, Rect},
    text::Spans,
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};

use crate::ui::components::common::HNCLI_VERSION;

use super::router::AppRoute;

/// Global contextual block displaying controls help relevant to the current route.
pub struct ContextualHelper {}

impl ContextualHelper {
    /// Renderer.
    pub fn render(
        &self,
        f: &mut Frame<CrosstermBackend<Stdout>>,
        inside: Rect,
        for_route: &AppRoute,
    ) {
        match for_route {
            AppRoute::Home => self.render_home_help(f, inside),
            AppRoute::Help => self.render_help_about(f, inside),
        }
    }

    fn render_home_help(&self, f: &mut Frame<CrosstermBackend<Stdout>>, inside: Rect) {}

    fn render_help_about(&self, f: &mut Frame<CrosstermBackend<Stdout>>, inside: Rect) {
        let version_text = vec![Spans::from(format!("hncli {}", HNCLI_VERSION))];
        let version_paragraph = Paragraph::new(version_text)
            .block(Self::build_common_block())
            .alignment(Alignment::Center);
        f.render_widget(version_paragraph, inside);
    }

    fn build_common_block<'a>() -> Block<'a> {
        Block::default()
            .border_type(BorderType::Thick)
            .borders(Borders::ALL)
    }
}
