use std::{collections::HashMap, fmt::Debug};

use tui::layout::Rect;

use crate::app::AppHandle;

use super::{common::UiComponentId, handlers::Key};

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
    fn handle_key_event(&mut self, key: &Key, app: &mut AppHandle) -> ScreenEventResponse;

    /// Compute the components' layout according to current terminal frame size.
    fn compute_layout(
        &mut self,
        frame_size: Rect,
        components_registry: &mut ScreenComponentsRegistry,
        app: &AppHandle,
    );
}
