use tui::layout::{Constraint, Direction, Layout, Rect};

use crate::{
    app::AppHandle,
    ui::{components::help::HELP_ID, handlers::Key},
};

use super::{Screen, ScreenComponentsRegistry, ScreenEventResponse};

/// The Help screen of hncli.
#[derive(Debug)]
pub struct HelpScreen;

impl HelpScreen {
    pub fn new() -> Self {
        Self {}
    }
}

impl Screen for HelpScreen {
    fn handle_key_event(&mut self, key: &Key, app: &mut AppHandle) -> ScreenEventResponse {
        match key {
            Key::Escape | Key::Enter | Key::Char('h') => {
                app.router_pop_navigation_stack();
                ScreenEventResponse::Caught
            }
            _ => ScreenEventResponse::PassThrough,
        }
    }

    fn compute_layout(
        &mut self,
        frame_size: Rect,
        components_registry: &mut ScreenComponentsRegistry,
        _app: &AppHandle,
    ) {
        // main layout chunks
        let main_layout_chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(2)
            .constraints([Constraint::Length(1)].as_ref())
            .split(frame_size);

        components_registry.insert(HELP_ID, main_layout_chunks[0]);
    }
}

unsafe impl Send for HelpScreen {}
