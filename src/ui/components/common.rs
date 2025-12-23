use ratatui::{
    layout::{Alignment, Rect},
    style::Style,
    text::Line,
    widgets::{Block, BorderType, Borders, Paragraph},
};

use crate::ui::{common::RenderFrame, theme::UiTheme};

pub fn render_text_message(f: &mut RenderFrame, inside: Rect, message: &str, theme: &UiTheme) {
    let block = Block::default()
        .style(Style::default().fg(theme.get_block_color()))
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded);

    let text = vec![Line::from(""), Line::from(message.to_string())];
    let paragraph = Paragraph::new(text)
        .block(block)
        .alignment(Alignment::Center);
    f.render_widget(paragraph, inside);
}
