use tui::layout::Rect;

use crate::{
    app::{history::AppHistory, state::AppState},
    config::AppConfiguration,
    ui::{
        handlers::InputsController,
        router::{AppRoute, AppRouter},
    },
};

use super::{Screen, ScreenComponentsRegistry, ScreenEventResponse};

/// The sign in to an HackerNews account screen.
#[derive(Debug)]
pub struct LoginScreen {}

impl Screen for LoginScreen {
    fn before_mount(&mut self, state: &mut AppState, _config: &AppConfiguration) {
        state.set_flash_message("Your credentials will not be store.", None);
    }

    fn before_unmount(&mut self, state: &mut AppState) {
        state.clear_flash_message();
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
    }
}
