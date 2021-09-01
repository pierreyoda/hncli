use tui::layout::Rect;

use crate::{
    app::AppState,
    ui::{
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
        state: &AppState,
    ) {
    }
}

unsafe impl Send for SettingsScreen {}
