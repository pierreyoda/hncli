use tui::{
    layout::{Alignment, Rect},
    style::{Color, Style},
    text::Spans,
    widgets::{Block, BorderType, Borders, Paragraph},
};

use super::common::RenderFrame;

/// Global flash message renderer.
#[derive(Default)]
pub struct FlashMessage {}

impl FlashMessage {
    pub fn render(&self, f: &mut RenderFrame, inside: Rect, message: &String) {
        let text = vec![Spans::from(message.clone())];
        let paragraph = Paragraph::new(text)
            .block(
                Block::default()
                    .border_type(BorderType::Thick)
                    .border_style(Style::default().fg(Color::Yellow))
                    .borders(Borders::ALL),
            )
            .alignment(Alignment::Center)
            .style(Style::default().fg(Color::Yellow));
        f.render_widget(paragraph, inside);
    }
}
