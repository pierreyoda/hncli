use async_trait::async_trait;
use tui::layout::Rect;

use crate::{
    api::HnClient,
    app::AppContext,
    errors::Result,
    ui::common::{RenderFrame, UiComponent, UiComponentId, UiTickScalar},
};

use super::widgets::text_input::{TextInputState, TextInputStateActionBridge, TextInputWidget};

pub mod algolia_help;
pub mod algolia_input;
pub mod algolia_list;
pub mod algolia_tags;

/// Search input component, for filtering the stories list.
#[derive(Debug)]
pub struct Search {
    input_state: TextInputState,
}

impl Default for Search {
    fn default() -> Self {
        Self {
            input_state: Default::default(),
        }
    }
}

pub const SEARCH_ID: UiComponentId = "stories-search";

// TODO: rename from Search to StoriesSearch or ItemsSearch
#[async_trait]
impl UiComponent for Search {
    fn id(&self) -> UiComponentId {
        SEARCH_ID
    }

    fn should_update(&mut self, _elapsed_ticks: UiTickScalar, _ctx: &AppContext) -> Result<bool> {
        Ok(false)
    }

    async fn update(&mut self, _client: &mut HnClient, _ctx: &mut AppContext) -> Result<()> {
        Ok(())
    }

    fn handle_inputs(&mut self, ctx: &mut AppContext) -> Result<bool> {
        let inputs = ctx.get_inputs();
        for input_available_action in self.input_state.available_actions() {
            if inputs.is_active(&input_available_action) {
                // ctx.get_state_mut()
                //     .set_main_search_mode_query(Some(self.input_state.get_value().clone()));
                return Ok(true);
            }
        }

        Ok(false)
    }

    fn render(&mut self, f: &mut RenderFrame, inside: Rect, _ctx: &AppContext) -> Result<()> {
        // let block = Block::default()
        //     .style(Style::default().fg(COMMON_BLOCK_NORMAL_COLOR))
        //     .borders(Borders::ALL)
        //     .border_type(BorderType::Rounded)
        //     .title("Search input");

        let input_widget = TextInputWidget::with_state(&self.input_state);
        f.render_widget(input_widget, inside);

        Ok(())
    }
}
