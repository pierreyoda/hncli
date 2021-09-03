use tui::layout::{Constraint, Direction, Layout, Rect};

use crate::{
    app::AppState,
    ui::{
        components::{navigation::NAVIGATION_ID, settings::SETTINGS_ID},
        handlers::Key,
        router::{AppRoute, AppRouter},
    },
};

use super::{Screen, ScreenComponentsRegistry, ScreenEventResponse};

/// The settings screen of hncli.
#[derive(Debug)]
pub struct SettingsScreen;

impl SettingsScreen {
    pub fn new() -> Self {
        Self {}
    }
}

impl Screen for SettingsScreen {
    fn handle_key_event(
        &mut self,
        key: &Key,
        router: &mut AppRouter,
        _state: &mut AppState,
    ) -> (ScreenEventResponse, Option<AppRoute>) {
        match key {
            Key::Escape => {
                router.pop_navigation_stack();
                (
                    ScreenEventResponse::Caught,
                    Some(router.get_current_route().clone()),
                )
            }
            _ => (ScreenEventResponse::PassThrough, None),
        }
    }

    fn compute_layout(
        &self,
        frame_size: Rect,
        components_registry: &mut ScreenComponentsRegistry,
        _state: &AppState,
    ) {
        // main layout chunks
        let main_layout_chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(2)
            .constraints([Constraint::Percentage(6), Constraint::Percentage(94)].as_ref())
            .split(frame_size);

        components_registry.insert(NAVIGATION_ID, main_layout_chunks[0]);
        components_registry.insert(SETTINGS_ID, main_layout_chunks[1]);
    }
}

unsafe impl Send for SettingsScreen {}
