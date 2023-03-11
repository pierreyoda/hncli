use tui::layout::{Constraint, Direction, Layout, Rect};

use crate::{
    app::{history::AppHistory, state::AppState},
    ui::{
        components::search::algolia_help::ALGOLIA_HELP_ID,
        handlers::{ApplicationAction, InputsController},
        router::{AppRoute, AppRouter},
    },
};

use super::{Screen, ScreenComponentsRegistry, ScreenEventResponse};

#[derive(Debug)]
pub struct SearchHelpScreen;

impl SearchHelpScreen {
    pub fn new() -> Self {
        Self {}
    }
}

impl Screen for SearchHelpScreen {
    fn handle_inputs(
        &mut self,
        inputs: &InputsController,
        router: &mut AppRouter,
        _state: &mut AppState,
        _history: &mut AppHistory,
    ) -> (ScreenEventResponse, Option<AppRoute>) {
        if inputs.is_active(&ApplicationAction::Back)
            || inputs.is_active(&ApplicationAction::ToggleHelp)
        {
            router.pop_navigation_stack();
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
            .constraints([Constraint::Length(1)].as_ref())
            .split(frame_size);

        components_registry.insert(ALGOLIA_HELP_ID, main_layout_chunks[0]);
    }
}
