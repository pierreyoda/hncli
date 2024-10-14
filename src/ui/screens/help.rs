use ratatui::layout::{Constraint, Direction, Layout, Rect};

use crate::{
    app::{history::AppHistory, state::AppState},
    ui::{
        components::help::HELP_ID,
        handlers::{ApplicationAction, InputsController},
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

// TODO: when navigating to help screen through navbar, redirect back to home if needed
impl Screen for HelpScreen {
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
        // main layout chunks
        let main_layout_chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(2)
            .constraints([Constraint::Percentage(100)].as_ref())
            .split(frame_size);

        components_registry.insert(HELP_ID, main_layout_chunks[0]);
    }
}
