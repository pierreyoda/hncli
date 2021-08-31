use tui::layout::{Constraint, Direction, Layout, Rect};

use crate::{
    app::AppState,
    ui::{
        components::{navigation::NAVIGATION_ID, options::OPTIONS_ID, stories::STORIES_PANEL_ID},
        handlers::Key,
        router::{AppRoute, AppRouter},
    },
};

use super::{Screen, ScreenComponentsRegistry, ScreenEventResponse};

/// The Home screen of hncli.
///
/// The current layout is as following:
///
/// ```md
/// ------------------------------------------
/// |              navigation                |
/// ------------------------------------------
/// |                                        |
/// |                                        |
/// |               stories                  |
/// |                                        |
/// |                                        |
/// ------------------------------------------
/// |          options (eg. sorting)         |
/// ------------------------------------------
/// ```
#[derive(Debug)]
pub struct HomeScreen {
    // TODO: add story type parameter? ("home", "ask HN", "show HN", "jobs")
}

impl HomeScreen {
    pub fn new() -> Self {
        Self {}
    }
}

impl Screen for HomeScreen {
    fn handle_key_event(
        &self,
        key: &Key,
        router: &mut AppRouter,
        _state: &mut AppState,
    ) -> (ScreenEventResponse, Option<AppRoute>) {
        match key {
            Key::Char('h') => {
                router.push_navigation_stack(AppRoute::Help);
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
            .constraints(
                [
                    Constraint::Length(3),
                    Constraint::Min(2),
                    Constraint::Length(3),
                ]
                .as_ref(),
            )
            .split(frame_size);

        components_registry.insert(NAVIGATION_ID, main_layout_chunks[0]);
        components_registry.insert(STORIES_PANEL_ID, main_layout_chunks[1]);
        components_registry.insert(OPTIONS_ID, main_layout_chunks[2]);
    }
}

unsafe impl Send for HomeScreen {}
