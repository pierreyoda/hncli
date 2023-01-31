use async_trait::async_trait;
use tui::{
    layout::{Alignment, Rect},
    style::Style,
    text::Spans,
    widgets::{Block, BorderType, Borders, Paragraph},
};

use crate::{
    api::HnClient,
    app::AppContext,
    errors::Result,
    ui::{
        common::{RenderFrame, UiComponent, UiComponentId, UiTickScalar},
        handlers::ApplicationAction,
    },
};

use super::common::COMMON_BLOCK_NORMAL_COLOR;

/// Search input component, for filtering the stories list.
#[derive(Debug)]
pub struct Search {
    query: String,
}

impl Default for Search {
    fn default() -> Self {
        Self { query: "".into() }
    }
}

pub const SEARCH_ID: UiComponentId = "search";

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
        Ok(if inputs.is_active(&ApplicationAction::InputDelete) {
            self.query.pop();
            ctx.get_state_mut()
                .set_main_search_mode_query(Some(self.query.clone()));
            true
        } else if inputs.is_active(&ApplicationAction::InputClear) {
            self.query.clear();
            ctx.get_state_mut()
                .set_main_search_mode_query(Some(self.query.clone()));
            true
        } else if inputs.is_active(&ApplicationAction::Back) {
            ctx.get_state_mut().set_main_search_mode_query(None);
            self.query.clear();
            true
        } else if let Some((_, input_key)) = inputs.get_active_input_key() {
            self.query.push(input_key);
            ctx.get_state_mut()
                .set_main_search_mode_query(Some(self.query.clone()));
            true
        } else {
            false
        })
    }

    fn render(&mut self, f: &mut RenderFrame, inside: Rect, _ctx: &AppContext) -> Result<()> {
        let block = Block::default()
            .style(Style::default().fg(COMMON_BLOCK_NORMAL_COLOR))
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .title("Search input");
        let text = vec![Spans::from(self.query.as_str())];
        let paragraph = Paragraph::new(text).block(block).alignment(Alignment::Left);
        f.render_widget(paragraph, inside);

        Ok(())
    }
}
