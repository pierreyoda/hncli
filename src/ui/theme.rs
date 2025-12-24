use ratatui::style::Color;
use serde::{Deserialize, Serialize};

/** Avoid boilerplate and easily add theme colors and palettes. */
macro_rules! theme_define_color_palettes {
    ($ ( $name: ident : $color_blue: expr, $color_yellow: expr , )* ) => {
    $(
        pub fn $name(&self) -> Color {
            match self {
                Self::Blue => $color_blue,
                Self::Yellow => $color_yellow,
            }
        }
    )*
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum UiTheme {
    Blue,
    Yellow,
}

impl Default for UiTheme {
    fn default() -> Self {
        Self::Yellow
    }
}

impl UiTheme {
    pub fn next_value(&self) -> UiTheme {
        match self {
            Self::Blue => Self::Yellow,
            Self::Yellow => Self::Blue,
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            Self::Blue => "Blue",
            Self::Yellow => "Yellow",
        }
    }

    theme_define_color_palettes! {
        get_main_color: Color::LightBlue, Color::LightYellow,
        get_block_color: Color::LightCyan, Color::White,
        get_accent_color: Color::LightMagenta, Color::Yellow,
    }
}
