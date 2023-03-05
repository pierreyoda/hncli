use async_trait::async_trait;
use tui::{
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, BorderType, Borders},
};

use crate::{
    api::HnClient,
    app::AppContext,
    errors::Result,
    ui::{
        common::{RenderFrame, UiComponent, UiComponentId, UiTickScalar},
        components::widgets::text_input::{TextInputState, TextInputWidget},
        screens::search::SearchScreenPart,
    },
};

/// The input controlling the Hacker News Algolia search.
#[derive(Debug)]
pub struct AlgoliaInput {
    input_state: TextInputState,
}

impl Default for AlgoliaInput {
    fn default() -> Self {
        Self {
            input_state: TextInputState::default(),
        }
    }
}

pub const ALGOLIA_INPUT_ID: UiComponentId = "algolia_input";

#[async_trait]
impl UiComponent for AlgoliaInput {
    fn id(&self) -> UiComponentId {
        ALGOLIA_INPUT_ID
    }

    fn should_update(&mut self, _elapsed_ticks: UiTickScalar, _ctx: &AppContext) -> Result<bool> {
        Ok(false)
    }

    async fn update(&mut self, _client: &mut HnClient, _ctx: &mut AppContext) -> Result<()> {
        Ok(())
    }

    fn handle_inputs(&mut self, _ctx: &mut AppContext) -> Result<bool> {
        Ok(false)
    }

    fn render(&mut self, f: &mut RenderFrame, inside: Rect, ctx: &AppContext) -> Result<()> {
        let input_widget_border_style = if matches!(
            ctx.get_state().get_currently_used_algolia_part(),
            SearchScreenPart::Input
        ) {
            Style::default().fg(Color::Yellow)
        } else {
            Style::default()
        };
        let input_widget = TextInputWidget::with_state(&self.input_state).block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Thick)
                .border_style(input_widget_border_style)
                .title("Search input"),
        );
        f.render_widget(input_widget, inside);

        Ok(())
    }
}
