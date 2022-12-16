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

use tui::{
    buffer::Buffer,
    layout::Rect,
    widgets::{Block, Widget},
};

use crate::ui::handlers::ApplicationAction;

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
    GoToPreviousWord,
    GoToNextWord,
    /// Typically CTRL + A.
    GoToStart,
    /// Typically CTRL + E.
    GoToEnd,
    /// Typically Backspace.
    DeletePreviousCharacter,
    DeleteNextCharacter,
    /// Typically CTRL + U.
    DeleteBeforeCursor,
    /// Typically CTRL + K.
    DeleteAfterCursor,
}

/// "Bridge" between the application's event handling and the corresponding `TextInputStateAction`s.
pub trait TextInputStateActionBridge {
    type ApplicationEvent;

    // TODO: can we use a slice and not a Vec? in the context of a future independant crate
    fn available_actions(&self) -> Vec<Self::ApplicationEvent>;

    fn handle_event(&self, event: &Self::ApplicationEvent);
}

impl TextInputStateActionBridge for TextInputState {
    type ApplicationEvent = ApplicationAction;

    fn available_actions(&self) -> Vec<Self::ApplicationEvent> {
        let mut actions = vec![];
        actions
    }

    fn handle_event(&self, event: &Self::ApplicationEvent) {
        todo!()
    }
}

/// State for `TextInputWidget`, to be used in the parent structure.
#[derive(Debug, Default)]
pub struct TextInputState {
    value: String,
    cursor_index: usize,
}

impl TextInputState {
    pub fn from_string(string: &String) -> Self {
        Self {
            value: string.clone(),
            cursor_index: string.chars().count(),
        }
    }

    pub fn handle_action(&mut self, action: &TextInputStateAction) {
        match action {
            _ => todo!(),
        }
    }

    pub fn get_value(&self) -> &String {
        &self.value
    }

    pub fn get_cursor_index(&self) -> usize {
        self.cursor_index
    }
}

/// Custom widget for handling text input.
#[derive(Debug)]
pub struct TextInputWidget<'a> {
    /// Persistent state.
    state: &'a TextInputState,
    /// (Optional) Wrapping `tui-rs` Block widget.
    block: Option<Block<'a>>,
}

impl<'a> TextInputWidget<'a> {
    pub fn with_state(state: &'a TextInputState) -> Self {
        Self { state, block: None }
    }

    pub fn block(mut self, block: Block<'a>) -> Self {
        self.block = Some(block);
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

        todo!()
    }
}
