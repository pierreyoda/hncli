use ratatui::{
    layout::{Alignment, Rect},
    style::{
        Color::{self, Blue, Red, Yellow},
        Style,
    },
    text::Line,
    widgets::Paragraph,
};

use super::common::RenderFrame;

#[derive(Clone, Copy, Debug)]
pub enum FlashMessageType {
    /// Info, to be displayed in blue.
    Info,
    /// Warning, to be displayed in yellow.
    Warning,
    /// Error, to be displayed in red.
    Error,
}

impl FlashMessageType {
    fn to_color(&self) -> Color {
        use FlashMessageType::*;
        match self {
            Info => Blue,
            Warning => Yellow,
            Error => Red,
        }
    }
}

/// Global flash message renderer.
///
/// Not a Widget right now since it would require refactoring on some Screens and maybe in the Breakpoints module.
/// TODO: this would require FlashMessageData in the global state.
#[derive(Debug)]
pub struct FlashMessage {
    color: Color,
    duration: usize,
    message_type: FlashMessageType,
}

impl FlashMessage {
    pub fn new(message_type: FlashMessageType, duration: usize) -> Self {
        Self {
            color: message_type.to_color(),
            message_type,
            duration,
        }
    }

    pub fn render(&self, f: &mut RenderFrame, inside: Rect, message: &str) {
        let text = vec![Line::from(message.to_string())];
        let paragraph = Paragraph::new(text)
            .alignment(Alignment::Center)
            .style(Style::default().fg(self.color));
        f.render_widget(paragraph, inside);
    }
}
