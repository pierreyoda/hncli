use ratatui::layout::{Constraint, Direction, Layout, Rect};

use crate::{
    app::{history::AppHistory, state::AppState},
    ui::{
        components::{
            search::{
                algolia_input::{ALGOLIA_INPUT_ID, MAX_ALGOLIA_INPUT_LENGTH},
                algolia_list::ALGOLIA_LIST_ID,
                algolia_tags::ALGOLIA_TAGS_ID,
            },
            widgets::text_input::{
                TextInputStateAction, TextInputStateActionBridge, TEXT_INPUT_AVAILABLE_ACTIONS,
            },
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
    /// The Results search screen part, where the stored boolean indicates focus.
    Results(bool),
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
    // TODO: refresh search when changing filter
    fn handle_inputs(
        &mut self,
        inputs: &InputsController,
        router: &mut AppRouter,
        state: &mut AppState,
        _history: &mut AppHistory,
    ) -> (ScreenEventResponse, Option<AppRoute>) {
        let currently_used_algolia_part = state.get_currently_used_algolia_part();

        // help screen toggle
        if inputs.is_active(&ApplicationAction::ToggleHelp) {
            if router.get_current_route().is_search_help() {
                router.pop_navigation_stack();
            } else {
                router.push_navigation_stack(AppRoute::SearchHelp);
            }
            return (
                ScreenEventResponse::Caught,
                Some(router.get_current_route().clone()),
            );
        }

        // the Algolia searchbox input is handled here
        // due to interference with the below commands
        if currently_used_algolia_part == SearchScreenPart::Input {
            if let Some((_, char)) = inputs.get_active_input_key() {
                if state.get_current_algolia_query_state().get_value().len()
                    < MAX_ALGOLIA_INPUT_LENGTH
                {
                    state
                        .get_current_algolia_query_state_mut()
                        .handle_action(&TextInputStateAction::InsertCharacter(char));
                    return (ScreenEventResponse::Caught, None);
                }
            }
            for available_action in TEXT_INPUT_AVAILABLE_ACTIONS {
                if inputs.is_active(&available_action) {
                    state
                        .get_current_algolia_query_state_mut()
                        .handle_event(inputs, &available_action);
                    return (ScreenEventResponse::Caught, None);
                }
            }
        }

        // results part (un)focusing
        if currently_used_algolia_part == SearchScreenPart::Results(true) {
            if inputs.is_active(&ApplicationAction::Back) {
                state.set_currently_used_algolia_part(SearchScreenPart::Input);
                return (ScreenEventResponse::Caught, None);
            }
        } else if currently_used_algolia_part == SearchScreenPart::Results(false)
            && inputs.is_active(&ApplicationAction::ToggleFocusResults)
        {
            state.set_currently_used_algolia_part(SearchScreenPart::Results(true));
            return (ScreenEventResponse::Caught, None);
        }

        if inputs.is_active(&ApplicationAction::ToggleHelp) {
            (ScreenEventResponse::Caught, Some(AppRoute::SearchHelp))
        } else if inputs.is_active(&ApplicationAction::Back) {
            router.pop_navigation_stack();
            (
                ScreenEventResponse::Caught,
                Some(router.get_current_route().clone()),
            )
        } else if inputs.is_active(&ApplicationAction::NavigateUp) {
            state.set_currently_used_algolia_part(match currently_used_algolia_part {
                SearchScreenPart::Filters => SearchScreenPart::Results(false),
                SearchScreenPart::Input => SearchScreenPart::Filters,
                SearchScreenPart::Results(false) => SearchScreenPart::Input,
                SearchScreenPart::Results(true) => SearchScreenPart::Results(true),
            });
            (ScreenEventResponse::Caught, None)
        } else if inputs.is_active(&ApplicationAction::NavigateDown) {
            state.set_currently_used_algolia_part(match currently_used_algolia_part {
                SearchScreenPart::Filters => SearchScreenPart::Input,
                SearchScreenPart::Input => SearchScreenPart::Results(false),
                SearchScreenPart::Results(false) => SearchScreenPart::Filters,
                SearchScreenPart::Results(true) => SearchScreenPart::Results(true),
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
