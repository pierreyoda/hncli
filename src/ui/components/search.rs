use async_trait::async_trait;
use ratatui::layout::Rect;

use crate::{
    api::HnClient,
    app::AppContext,
    errors::Result,
    ui::common::{RenderFrame, UiComponent, UiComponentId, UiTickScalar},
};

use super::widgets::text_input::TextInputWidget;

pub mod algolia_help;
pub mod algolia_input;
pub mod algolia_list;
pub mod algolia_tags;

/// Search input component, for filtering the stories list.
#[derive(Debug, Default)]
pub struct Search {}

pub const SEARCH_ID: UiComponentId = "stories-search";

// TODO: rename from Search to StoriesSearch or ItemsSearch
#[async_trait]
impl UiComponent for Search {
    fn id(&self) -> UiComponentId {
        SEARCH_ID
    }

    async fn should_update(
        &mut self,
        _elapsed_ticks: UiTickScalar,
        _ctx: &AppContext,
    ) -> Result<bool> {
        Ok(false)
    }

    async fn update(&mut self, _client: &mut HnClient, _ctx: &mut AppContext) -> Result<()> {
        Ok(())
    }

    async fn handle_inputs(&mut self, _ctx: &mut AppContext) -> Result<bool> {
        Ok(false)
    }

    fn render(&mut self, f: &mut RenderFrame, inside: Rect, ctx: &AppContext) -> Result<()> {
        let input_widget =
            TextInputWidget::with_state(ctx.get_state().get_current_algolia_query_state());
        f.render_widget(input_widget, inside);

        Ok(())
    }
}
