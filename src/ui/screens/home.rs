use tui::layout::{Direction, Layout, Rect};

use crate::{
    api::client::HnStoriesSections,
    app::{history::AppHistory, state::AppState},
    config::AppConfiguration,
    ui::{
        components::{navigation::NAVIGATION_ID, options::OPTIONS_ID, stories::STORIES_PANEL_ID},
        handlers::InputsController,
        router::{AppRoute, AppRouter},
        utils::breakpoints::Breakpoints,
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
    section: HnStoriesSections,
    breakpoints: Breakpoints,
}

impl HomeScreen {
    pub fn new(section: HnStoriesSections) -> Self {
        Self {
            section,
            breakpoints: Breakpoints::new("home_screen", &[20, 65, 15])
                .breakpoint(25, &[15, 75, 10])
                .breakpoint(45, &[7, 88, 5]),
        }
    }
}

impl Screen for HomeScreen {
    fn before_mount(&mut self, state: &mut AppState, _config: &AppConfiguration) {
        state.set_main_stories_section(self.section);
    }

    fn handle_inputs(
        &mut self,
        _inputs: &InputsController,
        _router: &mut AppRouter,
        _state: &mut AppState,
        _history: &mut AppHistory,
    ) -> (ScreenEventResponse, Option<AppRoute>) {
        (ScreenEventResponse::PassThrough, None)
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
            .constraints(self.breakpoints.to_constraints(frame_size.height))
            .split(frame_size);
        components_registry.insert(NAVIGATION_ID, main_layout_chunks[0]);
        components_registry.insert(STORIES_PANEL_ID, main_layout_chunks[1]);
        components_registry.insert(OPTIONS_ID, main_layout_chunks[2]);
    }
}

unsafe impl Send for HomeScreen {}
