use ratatui::{
    layout::{Alignment, Rect},
    style::{Color, Style},
    text::Line,
    widgets::Paragraph,
};

use super::common::RenderFrame;

/// Global flash message renderer.
#[derive(Default)]
pub struct FlashMessage {}

impl FlashMessage {
    pub fn render(&self, f: &mut RenderFrame, inside: Rect, message: &str) {
        let text = vec![Line::from(message.to_string())];
        let paragraph = Paragraph::new(text)
            .alignment(Alignment::Center)
            .style(Style::default().fg(Color::Yellow));
        f.render_widget(paragraph, inside);
    }
}
