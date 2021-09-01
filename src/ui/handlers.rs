use crossterm::event;
use event::{KeyCode, KeyEvent};

/// Abstraction over a key event.
///
/// Used to abstract over tui's backend, and to facilitate user configuration.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Key {
    /// Escape key.
    Escape,
    /// Enter/Return and Numpad Enter.
    Enter,
    /// Tabulation key.
    Tab,
    /// Up arrow.
    Up,
    /// Down arrow.
    Down,
    /// Left arrow.
    Left,
    /// Right arrow.
    Right,
    /// Keyboard character.
    Char(char),
    /// Unhandled.
    Other,
}

impl Key {
    /// Returns the character(s) representing the key.
    ///
    /// Does not support `Other`.
    pub fn get_representation(&self) -> String {
        use Key::*;
        match self {
            Escape => "⎋ (escape)".into(),
            Enter => "↵ (enter)".into(),
            Tab => "⇥ (tabulation)".into(),
            Up => "⬆️ (up)".into(),
            Down => "⬇️ (down)".into(),
            Left => "⬅️ (left)".into(),
            Right => "➡️ (right)".into(),
            Char(char) => format!("'{}'", char),
            Other => unreachable!(),
        }
    }
}

impl From<KeyEvent> for Key {
    fn from(event: KeyEvent) -> Self {
        use Key::*;

        match event.code {
            KeyCode::Esc => Escape,
            KeyCode::Enter => Enter,
            KeyCode::Tab => Tab,
            KeyCode::Up => Up,
            KeyCode::Down => Down,
            KeyCode::Left => Left,
            KeyCode::Right => Right,
            KeyCode::Char(c) => Char(c),
            _ => Other,
        }
    }
}
