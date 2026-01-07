use async_trait::async_trait;
use ratatui::{Frame, layout::Rect};

use crate::{api::HnClient, app::AppContext, errors::Result};

/// A `tick` is a UI update, in the order of the hundred milliseconds.
pub type UiTickScalar = u16;

/// A hashable type for application-unique component IDs.
pub type UiComponentId = &'static str;

pub type RenderFrame<'a> = Frame<'a>;

/// A `Component` in this Terminal UI context is a self-contained
/// widget or group of widgets with each their own updating,
/// events handling and rendering logic.
#[async_trait]
pub trait UiComponent {
    /// Must return a constant, **application-unique** component ID.
    fn id(&self) -> UiComponentId;

    /// Called at instantiation, before any update or render pass.
    fn before_mount(&mut self, _ctx: &mut AppContext) {}

    /// Called after un-mounting from a Screen.
    fn before_unmount(&mut self) {}

    /// Must return `true` if the state should update itself.
    async fn should_update(
        &mut self,
        elapsed_ticks: UiTickScalar,
        ctx: &AppContext,
    ) -> Result<bool>;

    /// Update the state from various sources.
    async fn update(&mut self, client: &mut HnClient, ctx: &mut AppContext) -> Result<()>;

    /// Inputs handler for the component.
    ///
    /// Returns true if the active event is to be captured, that is swallowed
    /// and no longer passed to other components.
    async fn handle_inputs(&mut self, ctx: &mut AppContext) -> Result<bool>;

    /// Renderer for the component.
    fn render(&mut self, f: &mut RenderFrame, inside: Rect, ctx: &AppContext) -> Result<()>;
}
