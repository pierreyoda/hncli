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

pub type FlashMessageDurationType = usize;

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

/// Global flash message data.
#[derive(Debug)]
pub struct FlashMessage {
    color: Color,
    text: &'static str,
    duration: FlashMessageDurationType,
}

impl FlashMessage {
    pub fn empty() -> Self {
        Self {
            text: "",
            duration: 0,
            color: FlashMessageType::Info.to_color(),
        }
    }

    pub fn from_data(
        r#type: FlashMessageType,
        text: &'static str,
        duration: FlashMessageDurationType,
    ) -> Self {
        Self {
            text,
            duration,
            color: r#type.to_color(),
        }
    }

    pub fn update(&mut self, r#type: FlashMessageType, text: &'static str) {
        self.text = text;
        self.color = r#type.to_color();
    }

    pub fn clear(&mut self) {
        self.duration = 0;
    }

    pub fn render(&self, f: &mut RenderFrame, inside: Rect, message: &str) {
        if self.text.is_empty() {
            return;
        }

        let text = vec![Line::from(message.to_string())];
        let paragraph = Paragraph::new(text)
            .alignment(Alignment::Center)
            .style(Style::default().fg(self.color));
        f.render_widget(paragraph, inside);
    }
}
