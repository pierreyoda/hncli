use std::{collections::HashMap, fmt::Debug};

use ratatui::layout::Rect;

use crate::app::{AppContext, state::AppState};

use super::{
    common::UiComponentId,
    handlers::InputsController,
    router::{AppRoute, AppRouter},
};

pub mod help;
pub mod help_search;
pub mod home;
pub mod nested_comments;
pub mod resume;
pub mod search;
pub mod search_help;
pub mod settings;
pub mod story;
pub mod user;

/// Defines layout state by associating each visible component
/// with a defined rendering target `Rect`.
pub type ScreenComponentsRegistry = HashMap<UiComponentId, Rect>;

/// Actions requested by a Screen when handling an input event.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ScreenEventResponse {
    /// Swallow the event, preventing it from bubbling down to the components.
    Caught,
    /// Ignore the event, passing it down to the components.
    PassThrough,
}

/// A Screen is a self-contained state of the application with its own update and rendering logic.
pub trait Screen: Debug + Send + Sync {
    /// Called after instantiation and before mounting the screen.
    fn before_mount(&mut self, _ctx: &mut AppContext) {}

    /// Called before unmounting the screen.
    fn before_unmount(&mut self, _ctx: &mut AppContext) {}

    /// Handle an incoming key event, at the application level.
    ///
    /// Returns the (event_response, new_route_if_navigated) tuple.
    fn handle_inputs(
        &mut self,
        inputs: &InputsController,
        router: &mut AppRouter,
        state: &mut AppState,
    ) -> (ScreenEventResponse, Option<AppRoute>);

    /// Compute the components' layout according to current terminal frame size.
    fn compute_layout(
        &self,
        frame_size: Rect,
        components_registry: &mut ScreenComponentsRegistry,
        state: &AppState,
    );
}

/// Inert `Screen`, standing in for the mounted screen while its lifecycle hooks
/// run, since `AppContext` mutably borrows the currently mounted screen.
#[derive(Debug)]
pub struct PlaceholderScreen;

impl Screen for PlaceholderScreen {
    fn handle_inputs(
        &mut self,
        _inputs: &InputsController,
        _router: &mut AppRouter,
        _state: &mut AppState,
    ) -> (ScreenEventResponse, Option<AppRoute>) {
        (ScreenEventResponse::PassThrough, None)
    }

    fn compute_layout(
        &self,
        _frame_size: Rect,
        _components_registry: &mut ScreenComponentsRegistry,
        _state: &AppState,
    ) {
    }
}
