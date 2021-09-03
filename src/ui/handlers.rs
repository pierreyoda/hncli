use crossterm::event::{self, KeyModifiers};
use event::{KeyCode, KeyEvent};

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
    // home screen
    HomeToggleSortingOption,
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
                inputs.modifier == KeyModifier::Control && inputs.key == Key::Char('o')
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
            HomeToggleSortingOption => inputs.key == Key::Char('s'),
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
}

impl InputsController {
    pub fn new() -> Self {
        Self {
            key: Key::None,
            modifier: KeyModifier::None,
        }
    }

    pub fn pump_event(&mut self, event: KeyEvent) {
        self.modifier = match event.modifiers {
            KeyModifiers::CONTROL => KeyModifier::Control,
            _ => KeyModifier::None,
        };
        self.key = Key::from(event);
    }

    pub fn is_active(&self, action: &ApplicationAction) -> bool {
        action.matches_event(self)
    }

    pub fn has_control_modifier(&self) -> bool {
        self.modifier == KeyModifier::Control
    }
}
