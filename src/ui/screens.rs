use std::{collections::HashMap, fmt::Debug};

use tui::layout::Rect;

use crate::app::AppState;

use super::{
    common::UiComponentId,
    handlers::Key,
    router::{AppRoute, AppRouter},
};

pub mod help;
pub mod home;

/// Defines layout state by associating each visible component
/// with a defined target `Rect`.
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
pub trait Screen: Debug + Send {
    /// Handle an incoming key event, at the application level. Returns true if
    /// the event is to be captured (swallowed) and not passed down to components.
    ///
    /// Returns the (event_response, new_route_if_navigated) tuple.
    fn handle_key_event(
        &self,
        key: &Key,
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
