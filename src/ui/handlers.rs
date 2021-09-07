use crossterm::event::{self, KeyModifiers};
use event::{KeyCode, KeyEvent};

use crate::app::AppState;

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
            Tab => "⇥ (tabulation)".into(),
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

pub enum ApplicationAction {
    // general
    OpenExternalOrHackerNewsLink,
    OpenHackerNewsLink,
    SelectItem,
    ToggleHelp,
    Back,
    Quit,
    // navigation
    NavigateUp,
    NavigateDown,
    NavigateLeft,
    NavigateRight,
    // input
    InputClear,
    InputDelete,
    // home screen
    HomeToggleSortingOption,
    HomeToggleSearchMode,
    // item screen
    ItemToggleComments,
    // settings screen
    SettingsToggleControl,
}

impl ApplicationAction {
    pub fn matches_event(&self, inputs: &InputsController) -> bool {
        use ApplicationAction::*;
        match self {
            OpenExternalOrHackerNewsLink => inputs.key == Key::Char('o'),
            OpenHackerNewsLink => {
                inputs.modifier == KeyModifier::Shift && inputs.key == Key::Char('o')
            }
            SelectItem => inputs.key == Key::Enter,
            ToggleHelp => inputs.key == Key::Char('h'),
            Back => inputs.key == Key::Escape,
            Quit => {
                inputs.key == Key::Char('q')
                    || (inputs.modifier == KeyModifier::Control && inputs.key == Key::Char('c'))
            }
            NavigateUp => inputs.key == Key::Up || inputs.key == Key::Char('i'),
            NavigateDown => inputs.key == Key::Down || inputs.key == Key::Char('k'),
            NavigateLeft => inputs.key == Key::Left || inputs.key == Key::Char('j'),
            NavigateRight => inputs.key == Key::Right || inputs.key == Key::Char('l'),
            InputClear => inputs.modifier == KeyModifier::Control && inputs.key == Key::Char('u'),
            InputDelete => inputs.key == Key::Backspace,
            HomeToggleSortingOption => inputs.key == Key::Char('s'),
            HomeToggleSearchMode => inputs.key == Key::Char('f'),
            ItemToggleComments => inputs.key == Key::Tab,
            SettingsToggleControl => inputs.key == Key::Tab,
        }
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
            active_input_mode: false,
        }
    }

    pub fn pump_event(&mut self, event: KeyEvent, state: &AppState) {
        // TODO: somehow make the Control modifier work properly
        self.modifier = match event.modifiers {
            KeyModifiers::CONTROL => KeyModifier::Control,
            KeyModifiers::SHIFT => KeyModifier::Shift,
            _ => KeyModifier::None,
        };
        self.active_input_mode = state.get_main_search_mode_query().is_some();
        self.key = if self.active_input_mode {
            match Key::from(event) {
                Key::Char(_) => Key::None,
                other => other,
            }
        } else {
            Key::from(event)
        };
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
