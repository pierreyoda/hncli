use tui::layout::{Constraint, Direction, Layout, Rect};

use crate::{
    app::{AppHandle, AppState},
    ui::{
        components::help::HELP_ID,
        handlers::Key,
        router::{AppRoute, AppRouter},
    },
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
    fn handle_key_event(
        &self,
        key: &Key,
        router: &mut AppRouter,
        _state: &mut AppState,
    ) -> (ScreenEventResponse, Option<AppRoute>) {
        match key {
            Key::Escape | Key::Enter | Key::Char('h') => {
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
            .constraints([Constraint::Length(1)].as_ref())
            .split(frame_size);

        components_registry.insert(HELP_ID, main_layout_chunks[0]);
    }
}

unsafe impl Send for HelpScreen {}
