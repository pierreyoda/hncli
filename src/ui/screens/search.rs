use tui::layout::{Constraint, Direction, Layout, Rect};

use crate::{
    app::{history::AppHistory, state::AppState},
    ui::{
        components::search::{algolia_input::ALGOLIA_INPUT_ID, algolia_tags::ALGOLIA_TAGS_ID},
        handlers::{ApplicationAction, InputsController},
        router::{AppRoute, AppRouter},
    },
};

use super::{Screen, ScreenComponentsRegistry, ScreenEventResponse};

/// The Algolia-based search screen of hncli.
#[derive(Debug)]
pub struct SearchScreen;

impl SearchScreen {
    pub fn new() -> Self {
        Self {}
    }
}

impl Screen for SearchScreen {
    fn handle_inputs(
        &mut self,
        inputs: &InputsController,
        router: &mut AppRouter,
        _state: &mut AppState,
        _history: &mut AppHistory,
    ) -> (ScreenEventResponse, Option<AppRoute>) {
        if inputs.is_active(&ApplicationAction::Back) {
            router.pop_navigation_stack();
            (
                ScreenEventResponse::Caught,
                Some(router.get_current_route().clone()),
            )
        } else {
            (ScreenEventResponse::PassThrough, None)
        }
    }

    fn compute_layout(
        &self,
        frame_size: Rect,
        components_registry: &mut ScreenComponentsRegistry,
        _state: &AppState,
    ) {
        let main_layout_chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(2)
            .constraints(
                [
                    Constraint::Percentage(10),
                    Constraint::Percentage(10),
                    Constraint::Percentage(80),
                ]
                .as_ref(),
            )
            .split(frame_size);

        components_registry.insert(ALGOLIA_TAGS_ID, main_layout_chunks[0]);
        components_registry.insert(ALGOLIA_INPUT_ID, main_layout_chunks[1]);
    }
}

unsafe impl Send for SearchScreen {}
