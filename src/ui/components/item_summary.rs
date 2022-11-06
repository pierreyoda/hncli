use std::io::Stdout;

use async_trait::async_trait;
use tui::{
    backend::CrosstermBackend,
    layout::{Alignment, Rect},
    style::Style,
    text::Spans,
    widgets::{Block, BorderType, Borders, Paragraph},
};

use crate::{
    api::{types::HnItemIdScalar, HnClient},
    app::AppContext,
    errors::Result,
    ui::common::{UiComponent, UiComponentId, UiTickScalar},
};

use super::common::COMMON_BLOCK_NORMAL_COLOR;

/// Item summary component, intended for when navigating sub-comments.
///
/// Does not do any fetching, everything is pre-cached.
///
/// ```md
/// ___________________________________________
/// |                                         |
/// |                <TITLE>                  |
/// |      <SCORE> POINTS / BY <USERNAME>     |
/// |_________________________________________|
/// ```
#[derive(Debug, Default)]
pub struct ItemSummary {
    id: Option<HnItemIdScalar>,
}

pub const ITEM_SUMMARY_ID: UiComponentId = "item_summary";

#[async_trait]
impl UiComponent for ItemSummary {
    fn id(&self) -> UiComponentId {
        ITEM_SUMMARY_ID
    }

    fn should_update(&mut self, _elapsed_ticks: UiTickScalar, ctx: &AppContext) -> Result<bool> {
        Ok(
            if let Some(item) = ctx.get_state().get_currently_viewed_item() {
                Some(item.id) != self.id
            } else {
                false
            },
        )
    }

    async fn update(&mut self, _client: &mut HnClient, ctx: &mut AppContext) -> Result<()> {
        self.id = ctx
            .get_state()
            .get_currently_viewed_item()
            .map(|item| item.id);
        Ok(())
    }

    fn handle_inputs(&mut self, _ctx: &mut AppContext) -> Result<bool> {
        Ok(false)
    }

    fn render(
        &mut self,
        f: &mut tui::Frame<CrosstermBackend<Stdout>>,
        inside: Rect,
        ctx: &AppContext,
    ) -> Result<()> {
        let viewed_item = if let Some(item) = ctx.get_state().get_currently_viewed_item() {
            item
        } else {
            return Ok(());
        };

        let block = Block::default()
            .style(Style::default().fg(COMMON_BLOCK_NORMAL_COLOR))
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded);

        let item_title = viewed_item.title.clone().unwrap_or_else(|| "".into());
        let text_base = vec![
            Spans::from(item_title.as_str()),
            Spans::from(format!(
                "{} points by {} {}",
                viewed_item.score, viewed_item.by_username, viewed_item.posted_since
            )),
        ];

        let paragraph = Paragraph::new(text_base)
            .block(block)
            .alignment(Alignment::Center);
        f.render_widget(paragraph, inside);

        Ok(())
    }
}
