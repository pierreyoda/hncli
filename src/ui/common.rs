use async_trait::async_trait;

use crate::{api::HnClient, app::App, errors::Result};

use super::handlers::Key;

/// A `tick` is a UI update, in the order of the hundred milliseconds.
pub type UiTickScalar = u64;

/// A `Component` in this Terminal UI context is a self-contained
/// widget or group of widgets with each their own updating and
/// rendering logic.
#[async_trait]
pub trait UiComponent {
    /// Returns a constant, **application-unique** component ID.
    fn id(&self) -> &'static str;

    /// Returns `true` if the state should update itself.
    fn should_update(&mut self, elapsed_ticks: UiTickScalar) -> Result<bool>;

    /// Update the state from various sources.
    async fn update(&mut self, client: &mut HnClient, app: &mut App) -> Result<()>;

    /// Key event handler for the component.
    ///
    /// Returns true if the event is to be captured, that is swallowed
    /// and no longer passed to other components.
    fn key_handler(&mut self, key: &Key, app: &mut App) -> Result<bool>;
}
