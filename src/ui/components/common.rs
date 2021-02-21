use tui::style::{Color, Style};

use crate::app::{App, AppBlock};

pub const COMMON_BLOCK_NORMAL_COLOR: Color = Color::White;
pub const COMMON_BLOCK_FOCUS_COLOR: Color = Color::Yellow;

pub fn get_layout_block_style(app: &App, block: AppBlock) -> Style {
    Style::default().fg(if app.has_current_focus(block) {
        COMMON_BLOCK_FOCUS_COLOR
    } else {
        COMMON_BLOCK_NORMAL_COLOR
    })
}

pub const HNCLI_VERSION: &str = env!("CARGO_PKG_VERSION");
