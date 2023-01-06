use std::io::Stdout;

use tui::{
    backend::CrosstermBackend,
    layout::{Alignment, Rect},
    style::{Color, Style},
    text::Spans,
    widgets::{Block, BorderType, Borders, Paragraph},
};

pub const COMMON_BLOCK_NORMAL_COLOR: Color = Color::White;

pub fn render_text_message(
    f: &mut tui::Frame<CrosstermBackend<Stdout>>,
    inside: Rect,
    message: &str,
) {
    let block = Block::default()
        .style(Style::default().fg(COMMON_BLOCK_NORMAL_COLOR))
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded);

    let text = vec![Spans::from(""), Spans::from(message.to_string())];
    let paragraph = Paragraph::new(text)
        .block(block)
        .alignment(Alignment::Center);
    f.render_widget(paragraph, inside);
}
