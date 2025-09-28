use ratatui::{
    layout::{Alignment, Rect},
    style::{
        Color::{self, Blue, Red, Yellow},
        Style,
    },
    text::Line,
    widgets::Paragraph,
};
use std::cmp;

use crate::ui::common::UiTickScalar;

use super::common::RenderFrame;

pub type FlashMessageDurationType = UiTickScalar;

pub const FLASH_MESSAGE_DEFAULT_DURATION_MS: FlashMessageDurationType = 4000;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
#[derive(Debug)]
pub struct FlashMessage {
    /// Cached for the rendering function.
    color: Color,
    message: String,
    starting_duration_ms: FlashMessageDurationType,
    spent_duration_ms: FlashMessageDurationType,
}

impl Default for FlashMessage {
    fn default() -> Self {
        Self {
            color: FlashMessageType::Info.to_color(),
            message: "".into(),
            starting_duration_ms: FLASH_MESSAGE_DEFAULT_DURATION_MS,
            spent_duration_ms: 0,
        }
    }
}

impl FlashMessage {
    pub fn new<S: Into<String>>(
        message: S,
        message_type: FlashMessageType,
        duration_ms: FlashMessageDurationType,
    ) -> Self {
        Self {
            color: message_type.to_color(),
            message: message.into(),
            starting_duration_ms: duration_ms,
            spent_duration_ms: 0,
        }
    }

    pub fn update(&mut self, elapsed_ticks: UiTickScalar) {
        self.spent_duration_ms = cmp::min(
            self.spent_duration_ms
                .wrapping_add(Self::ui_ticks_to_ms(elapsed_ticks)),
            self.starting_duration_ms,
        );
    }

    /// Returns true if the flash message has expired.
    pub fn is_active(&self) -> bool {
        self.spent_duration_ms < self.starting_duration_ms
    }

    pub fn render(&self, f: &mut RenderFrame, inside: Rect) {
        let text = vec![Line::from(self.message.clone())];
        let paragraph = Paragraph::new(text)
            .alignment(Alignment::Center)
            .style(Style::default().fg(self.color));
        f.render_widget(paragraph, inside);
    }

    fn ui_ticks_to_ms(ticks: UiTickScalar) -> FlashMessageDurationType {
        // a UI tick is as close as possible to 100ms
        ticks * (100 as UiTickScalar)
    }
}

#[cfg(test)]
mod tests {
    use crate::ui::flash::{FLASH_MESSAGE_DEFAULT_DURATION_MS, FlashMessage, FlashMessageType};

    #[test]
    fn test_flash_default() {
        let flash_message = FlashMessage::default();
        assert_eq!(flash_message.color, FlashMessageType::Info.to_color());
        assert_eq!(
            flash_message.starting_duration_ms,
            FLASH_MESSAGE_DEFAULT_DURATION_MS
        );
        assert_eq!(flash_message.spent_duration_ms, 0);
    }

    #[test]
    fn test_flash_update() {
        use FlashMessageType::*;

        let mut flash1 = FlashMessage::new("test1", Info, 500);
        assert_eq!(flash1.starting_duration_ms, 500);
        flash1.update(4);
        assert_eq!(flash1.starting_duration_ms, 500);
        assert_eq!(flash1.spent_duration_ms, 400);
        assert!(flash1.is_active());
        flash1.update(1);
        assert_eq!(flash1.starting_duration_ms, 500);
        assert_eq!(flash1.spent_duration_ms, 500);
        assert!(!flash1.is_active());

        let mut flash2 = FlashMessage::new("test2", Warning, 1000);
        assert_eq!(flash2.starting_duration_ms, 1000);
        flash2.update(2);
        assert_eq!(flash2.starting_duration_ms, 1000);
        assert_eq!(flash2.spent_duration_ms, 200);
        assert!(flash2.is_active());

        let mut flash3 = FlashMessage::new("test3", Error, 500);
        assert_eq!(flash3.starting_duration_ms, 500);
        flash3.update(6);
        assert_eq!(flash3.spent_duration_ms, 500);
        assert!(!flash3.is_active());
    }

    #[test]
    fn test_flash_ui_ticks_to_ms() {
        assert_eq!(FlashMessage::ui_ticks_to_ms(0), 0);
        assert_eq!(FlashMessage::ui_ticks_to_ms(5), 500);
        assert_eq!(FlashMessage::ui_ticks_to_ms(15), 1500);
    }
}
