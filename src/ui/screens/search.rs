use tui::layout::{Constraint, Direction, Layout, Rect};

use crate::{
    app::{history::AppHistory, state::AppState},
    ui::{
        components::search::{
            algolia_input::ALGOLIA_INPUT_ID, algolia_list::ALGOLIA_LIST_ID,
            algolia_tags::ALGOLIA_TAGS_ID,
        },
        handlers::{ApplicationAction, InputsController},
        router::{AppRoute, AppRouter},
    },
};

use super::{Screen, ScreenComponentsRegistry, ScreenEventResponse};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum SearchScreenPart {
    Filters,
    Input,
    Results,
}

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
        state: &mut AppState,
        _history: &mut AppHistory,
    ) -> (ScreenEventResponse, Option<AppRoute>) {
        let currently_used_algolia_part = state.get_currently_used_algolia_part();
        if currently_used_algolia_part == SearchScreenPart::Results {
            if inputs.is_active(&ApplicationAction::Back) {
                state.set_currently_used_algolia_part(SearchScreenPart::Input);
            }
            (ScreenEventResponse::Caught, None)
        } else if inputs.is_active(&ApplicationAction::ToggleHelp) {
            (ScreenEventResponse::Caught, Some(AppRoute::SearchHelp))
        } else if inputs.is_active(&ApplicationAction::Back) {
            router.pop_navigation_stack();
            (
                ScreenEventResponse::Caught,
                Some(router.get_current_route().clone()),
            )
        } else if inputs.is_active(&ApplicationAction::NavigateUp) {
            state.set_currently_used_algolia_part(match currently_used_algolia_part {
                SearchScreenPart::Filters => SearchScreenPart::Results,
                SearchScreenPart::Input => SearchScreenPart::Filters,
                SearchScreenPart::Results => SearchScreenPart::Input,
            });
            (ScreenEventResponse::Caught, None)
        } else if inputs.is_active(&ApplicationAction::NavigateDown) {
            state.set_currently_used_algolia_part(match currently_used_algolia_part {
                SearchScreenPart::Filters => SearchScreenPart::Input,
                SearchScreenPart::Input => SearchScreenPart::Results,
                SearchScreenPart::Results => SearchScreenPart::Filters,
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
        components_registry.insert(ALGOLIA_LIST_ID, main_layout_chunks[2]);
    }
}

unsafe impl Send for SearchScreen {}
