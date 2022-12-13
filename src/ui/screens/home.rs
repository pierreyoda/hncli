use tui::layout::{Constraint, Direction, Layout, Rect};

use crate::{
    api::HnStoriesSections,
    app::{history::AppHistory, AppState},
    config::AppConfiguration,
    ui::{
        components::{
            navigation::NAVIGATION_ID, options::OPTIONS_ID, search::SEARCH_ID,
            stories::STORIES_PANEL_ID,
        },
        handlers::{ApplicationAction, InputsController},
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
    section: HnStoriesSections,
}

impl HomeScreen {
    pub fn new(section: HnStoriesSections) -> Self {
        Self { section }
    }
}

impl Screen for HomeScreen {
    fn before_mount(&mut self, state: &mut AppState, _config: &AppConfiguration) {
        state.set_main_stories_section(self.section);
    }

    fn handle_inputs(
        &mut self,
        inputs: &InputsController,
        _router: &mut AppRouter,
        state: &mut AppState,
        _history: &mut AppHistory,
    ) -> (ScreenEventResponse, Option<AppRoute>) {
        if inputs.is_active(&ApplicationAction::HomeToggleSearchMode) {
            state.set_main_search_mode_query(if state.get_main_search_mode_query().is_some() {
                None
            } else {
                Some("".into())
            });
            (ScreenEventResponse::Caught, None)
        } else {
            (ScreenEventResponse::PassThrough, None)
        }
    }

    fn compute_layout(
        &self,
        frame_size: Rect,
        components_registry: &mut ScreenComponentsRegistry,
        state: &AppState,
    ) {
        // main layout chunks
        let main_layout_chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(2)
            .constraints(
                [
                    Constraint::Percentage(6),
                    Constraint::Percentage(89),
                    Constraint::Percentage(5),
                ]
                .as_ref(),
            )
            .split(frame_size);

        components_registry.insert(
            if state.get_main_search_mode_query().is_some() {
                SEARCH_ID
            } else {
                NAVIGATION_ID
            },
            main_layout_chunks[0],
        );
        components_registry.insert(STORIES_PANEL_ID, main_layout_chunks[1]);
        components_registry.insert(OPTIONS_ID, main_layout_chunks[2]);
    }
}

unsafe impl Send for HomeScreen {}
