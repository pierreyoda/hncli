//! Somewhat inspired from: https://github.com/sayanarijit/tui-input
//!
//! License of `tui-input`:
//!
//! MIT License
//!
//! Copyright (c) 2021 Arijit Basu
//!
//! Permission is hereby granted, free of charge, to any person obtaining a copy
//! of this software and associated documentation files (the "Software"), to deal
//! in the Software without restriction, including without limitation the rights
//! to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
//! copies of the Software, and to permit persons to whom the Software is
//! furnished to do so, subject to the following conditions:
//!
//! The above copyright notice and this permission notice shall be included in all
//! copies or substantial portions of the Software.
//!
//! THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
//! IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
//! FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
//! AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
//! LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
//! OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
//! SOFTWARE.

// TODO: make a separate crate?

use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Widget},
};

use crate::ui::handlers::{ApplicationAction, InputsController};

/// The various interactions with `TextInputState`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TextInputStateAction {
    SetCursorIndex(usize),
    /// The corresponding UTF-8 character on the keyboard.
    InsertCharacter(char),
    /// Typically the left arrow key.
    GoToPreviousCharacter,
    /// Typically the right arrow key.
    GoToNextCharacter,
    /// Typically CTRL + A.
    GoToStart,
    /// Typically CTRL + E.
    GoToEnd,
    /// Typically Backspace.
    DeletePreviousCharacter,
    /// Typically CTRL + U.
    DeleteBeforeCursor,
    /// Typically CTRL + K.
    DeleteAfterCursor,
}

/// "Bridge" between the application's event handling and the corresponding `TextInputStateAction`s.
pub trait TextInputStateActionBridge {
    type ApplicationEvent;

    fn handle_event(&mut self, inputs: &InputsController, event: &Self::ApplicationEvent);
}

/// State for `TextInputWidget`, to be used in the parent structure.
#[derive(Debug, Default)]
pub struct TextInputState {
    value: String,
    cursor_index: usize,
}

pub const TEXT_INPUT_AVAILABLE_ACTIONS: [ApplicationAction; 9] = [
    ApplicationAction::InputSetCursor,
    ApplicationAction::InputInsertCharacter,
    ApplicationAction::InputGoToPreviousCharacter,
    ApplicationAction::InputGoToNextCharacter,
    ApplicationAction::InputGoToStart,
    ApplicationAction::InputGoToEnd,
    ApplicationAction::InputDeletePreviousCharacter,
    ApplicationAction::InputDeleteBeforeCursor,
    ApplicationAction::InputDeleteAfterCursor,
];

impl TextInputStateActionBridge for TextInputState {
    type ApplicationEvent = ApplicationAction;

    fn handle_event(&mut self, inputs: &InputsController, event: &Self::ApplicationEvent) {
        use TextInputStateAction::*;

        let action = match event {
            ApplicationAction::InputInsertCharacter => {
                if let Some((_, char)) = inputs.get_active_input_key() {
                    Some(InsertCharacter(char))
                } else {
                    None
                }
            }
            ApplicationAction::InputGoToPreviousCharacter => Some(GoToPreviousCharacter),
            ApplicationAction::InputGoToNextCharacter => Some(GoToNextCharacter),
            ApplicationAction::InputGoToStart => Some(GoToStart),
            ApplicationAction::InputGoToEnd => Some(GoToEnd),
            ApplicationAction::InputDeletePreviousCharacter => Some(DeletePreviousCharacter),
            ApplicationAction::InputDeleteBeforeCursor => Some(DeletePreviousCharacter),
            ApplicationAction::InputDeleteAfterCursor => Some(DeleteAfterCursor),
            _ => None,
        };
        if let Some(input_action) = action {
            self.handle_action(&input_action);
        }
    }
}

impl TextInputState {
    pub fn from_string(string: &str) -> Self {
        Self {
            value: string.to_string(),
            cursor_index: string.chars().count(),
        }
    }

    pub fn handle_action(&mut self, action: &TextInputStateAction) {
        use TextInputStateAction::*;
        match action {
            SetCursorIndex(index) => {
                let position = *index.min(&self.utf8_len());
                if position != self.cursor_index {
                    self.cursor_index = position;
                }
            }
            InsertCharacter(char) => {
                if self.cursor_index >= self.utf8_len() {
                    self.value.push(*char);
                } else {
                    self.value = self
                        .value
                        .chars()
                        .take(self.cursor_index)
                        .chain(
                            std::iter::once(*char)
                                .chain(self.value.chars().skip(self.cursor_index)),
                        )
                        .collect();
                }
                self.cursor_index += 1;
            }
            GoToPreviousCharacter => {
                if self.cursor_index > 0 {
                    self.cursor_index -= 1;
                }
            }
            GoToNextCharacter => {
                if self.cursor_index != self.utf8_len() {
                    self.cursor_index += 1;
                }
            }
            GoToStart => {
                self.cursor_index = 0;
            }
            GoToEnd => {
                self.cursor_index = self.utf8_len();
            }
            DeletePreviousCharacter => {
                if self.cursor_index == 0 {
                    return;
                }
                self.cursor_index -= 1;
                self.value = self
                    .value
                    .chars()
                    .enumerate()
                    .filter(|(i, _)| *i != self.cursor_index)
                    .map(|(_, char)| char)
                    .collect();
            }
            DeleteBeforeCursor => {
                if self.cursor_index == 0 {
                    return;
                }
                self.value = self.value.chars().take(self.cursor_index).collect();
            }
            DeleteAfterCursor => {
                if self.cursor_index >= self.utf8_len() {
                    return;
                }
                self.value = self.value.chars().skip(self.cursor_index).collect();
            }
        }
    }

    pub fn get_value(&self) -> &String {
        &self.value
    }

    pub fn get_cursor_index(&self) -> usize {
        self.cursor_index
    }

    fn utf8_len(&self) -> usize {
        self.value.chars().count()
    }
}

/// Custom widget for handling text input.
#[derive(Debug)]
pub struct TextInputWidget<'a> {
    /// Persistent state.
    state: &'a TextInputState,
    /// (Optional) Custom rendering style.
    style: Style,
    /// (Optional) Wrapping `tui-rs` Block widget.
    block: Option<Block<'a>>,
}

impl<'a> TextInputWidget<'a> {
    pub fn with_state(state: &'a TextInputState) -> Self {
        Self {
            state,
            style: Style::default(),
            block: None,
        }
    }

    pub fn block(mut self, block: Block<'a>) -> Self {
        self.block = Some(block);
        self
    }

    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }
}

impl<'a> Widget for TextInputWidget<'a> {
    fn render(mut self, area: Rect, buf: &mut Buffer) {
        let text_area = match self.block.take() {
            Some(b) => {
                let inner_area = b.inner(area);
                b.render(area, buf);
                inner_area
            }
            None => area,
        };

        let cursor_position = self
            .state
            .value
            .chars()
            .enumerate()
            .find(|(i, _)| *i == self.state.cursor_index)
            .map(|(index, _)| index as u16);

        buf.set_string(text_area.x, text_area.y, &self.state.value, self.style);
        if let Some(cursor_index) = cursor_position {
            buf.set_string(
                area.x + cursor_index,
                area.y,
                " ",
                Style::default().bg(Color::LightYellow),
            );
        }
    }
}
