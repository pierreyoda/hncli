use crossterm::event::{self, KeyModifiers};
use event::{KeyCode, KeyEvent};

use crate::app::state::AppState;

/// Abstraction over a key event.
///
/// Used to abstract over tui's backend, and to facilitate user configuration.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Key {
    None,
    /// Escape key.
    Escape,
    /// Enter/Return and Numpad Enter.
    Enter,
    /// Backspace key.
    Backspace,
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
    /// Does not support `Other` and `None`.
    pub fn get_representation(&self) -> String {
        use Key::*;
        match self {
            Escape => "⎋ (escape)".into(),
            Enter => "↵ (enter)".into(),
            Backspace => "← (backspace)".into(),
            Tab => "⇥ (tab)".into(),
            Up => "⬆️ (up)".into(),
            Down => "⬇️ (down)".into(),
            Left => "⬅️ (left)".into(),
            Right => "➡️ (right)".into(),
            Char(char) => format!("'{}'", char),
            Other => unreachable!(),
            None => unreachable!(),
        }
    }
}

impl From<KeyEvent> for Key {
    fn from(event: KeyEvent) -> Self {
        use Key::*;

        match event.code {
            KeyCode::Esc => Escape,
            KeyCode::Enter => Enter,
            KeyCode::Backspace => Backspace,
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

/// Abstraction over a key event modifier.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum KeyModifier {
    None,
    Shift,
    Control,
}

#[derive(Debug)]
pub enum ApplicationAction {
    // general
    OpenExternalOrHackerNewsLink,
    OpenHackerNewsLink,
    SelectItem,
    ToggleHelp,
    Back,
    Quit,
    QuitShortcut,
    // navigation
    NavigateUp,
    NavigateDown,
    NavigateLeft,
    NavigateRight,
    // input
    InputSetCursor,
    InputInsertCharacter,
    InputGoToPreviousCharacter,
    InputGoToNextCharacter,
    InputGoToStart,
    InputGoToEnd,
    InputDeletePreviousCharacter,
    InputDeleteBeforeCursor,
    InputDeleteAfterCursor,
    // home screen
    HomeToggleSortingOption,
    // item screen
    ItemToggleComments,
    ItemExpandFocusedComment,
    FocusedCommentViewUserProfile,
    ItemUpvote,
    ItemDownvote,
    // user profile screen
    OpenHackerNewsProfile,
    // search screen
    ToggleFocusResults,
    // settings screen
    SettingsToggleControl,
}

impl ApplicationAction {
    pub fn matches_event(&self, inputs: &InputsController) -> bool {
        use ApplicationAction::*;
        match self {
            // general
            OpenExternalOrHackerNewsLink => inputs.key == Key::Char('o'),
            OpenHackerNewsLink => inputs.key == Key::Char('l'),
            SelectItem => inputs.key == Key::Enter,
            ToggleHelp => inputs.key == Key::Char('h'),
            Back => inputs.key == Key::Escape,
            Quit => inputs.modifier == KeyModifier::Control && inputs.key == Key::Char('c'),
            QuitShortcut => inputs.key == Key::Char('q'),
            // navigation
            NavigateUp => inputs.key == Key::Up,
            NavigateDown => inputs.key == Key::Down,
            NavigateLeft => inputs.key == Key::Left,
            NavigateRight => inputs.key == Key::Right,
            // input
            InputSetCursor => {
                inputs.modifier == KeyModifier::None
                    && (inputs.key == Key::Left || inputs.key == Key::Right)
            }
            InputInsertCharacter => {
                inputs.modifier == KeyModifier::None && Self::is_key_char(&inputs.key)
            }
            InputGoToPreviousCharacter => inputs.key == Key::Left,
            InputGoToNextCharacter => inputs.key == Key::Right,
            InputGoToStart => {
                inputs.modifier == KeyModifier::Control && inputs.key == Key::Char('a')
            }
            InputGoToEnd => inputs.modifier == KeyModifier::Control && inputs.key == Key::Char('e'),
            InputDeletePreviousCharacter => inputs.key == Key::Backspace,
            InputDeleteBeforeCursor => {
                inputs.modifier == KeyModifier::Control && inputs.key == Key::Char('u')
            }
            InputDeleteAfterCursor => {
                inputs.modifier == KeyModifier::Control && inputs.key == Key::Char('k')
            }
            // home screen
            HomeToggleSortingOption => inputs.key == Key::Char('s'),
            // item screen
            ItemToggleComments => inputs.key == Key::Tab,
            ItemExpandFocusedComment => inputs.key == Key::Enter,
            FocusedCommentViewUserProfile => inputs.key == Key::Char('p'),
            ItemUpvote => inputs.key == Key::Char('u'),
            ItemDownvote => inputs.key == Key::Char('d'),
            // user profile screen
            OpenHackerNewsProfile => inputs.key == Key::Char('o'),
            // search screen
            ToggleFocusResults => inputs.key == Key::Enter,
            // settings screen
            SettingsToggleControl => inputs.key == Key::Tab,
        }
    }

    fn is_key_char(key: &Key) -> bool {
        matches!(key, Key::Char(_))
    }
}

/// Application inputs controller.
///
/// Bridges between raw inputs and application-level events.
#[derive(Debug)]
pub struct InputsController {
    key: Key,
    modifier: KeyModifier,
    active_input_key: Key,
    active_input_mode: bool,
}

impl InputsController {
    pub fn new() -> Self {
        Self {
            key: Key::None,
            modifier: KeyModifier::None,
            active_input_key: Key::None,
            active_input_mode: true,
        }
    }

    pub fn pump_event(&mut self, event: KeyEvent, _state: &AppState) {
        // TODO: somehow make the modifiers work properly in all circumstances
        self.modifier = match event.modifiers {
            KeyModifiers::CONTROL => KeyModifier::Control,
            KeyModifiers::SHIFT => KeyModifier::Shift,
            _ => KeyModifier::None,
        };
        self.key = Key::from(event);
        self.active_input_key = if self.active_input_mode {
            match Key::from(event) {
                Key::Char(c) => Key::Char(c),
                _ => Key::None,
            }
        } else {
            Key::None
        };
    }

    pub fn is_active(&self, action: &ApplicationAction) -> bool {
        action.matches_event(self)
    }

    pub fn has_ctrl_modifier(&self) -> bool {
        self.modifier == KeyModifier::Control
    }

    pub fn has_shift_modifier(&self) -> bool {
        self.modifier == KeyModifier::Shift
    }

    pub fn get_active_input_key(&self) -> Option<(Key, char)> {
        for c in b'A'..=b'z' {
            let key = Key::Char(c as char);
            if key == self.active_input_key {
                return Some((key, c as char));
            }
        }
        None
    }
}
