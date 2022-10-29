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
    api::{HnClient, HnItemComments},
    app::AppContext,
    errors::Result,
    ui::{
        common::{UiComponent, UiComponentId, UiTickScalar},
        components::common::COMMON_BLOCK_NORMAL_COLOR,
        displayable_item::{DisplayableHackerNewsItem, DisplayableHackerNewsItemComments},
    },
};

/// Item comments component.
#[derive(Debug, Default)]
pub struct ItemComments {
    ticks_since_last_update: u64,
    comments: DisplayableHackerNewsItemComments,
}

const MEAN_TICKS_BETWEEN_UPDATES: UiTickScalar = 1800; // approx. every 3 minutes

pub const ITEM_COMMENTS_ID: UiComponentId = "item_comments";

#[async_trait]
impl UiComponent for ItemComments {
    fn id(&self) -> UiComponentId {
        ITEM_COMMENTS_ID
    }

    fn should_update(&mut self, elapsed_ticks: UiTickScalar, _ctx: &AppContext) -> Result<bool> {
        // TODO: should also update when comments are dirty
        self.ticks_since_last_update += elapsed_ticks;
        Ok(self.ticks_since_last_update >= MEAN_TICKS_BETWEEN_UPDATES)
    }

    async fn update(&mut self, client: &mut HnClient, ctx: &mut AppContext) -> Result<()> {
        self.ticks_since_last_update = 0;

        let viewed_item = match ctx.get_state().get_currently_viewed_item() {
            Some(item) => item,
            None => return Ok(()),
        };
        let viewed_item_kids = match &viewed_item.kids {
            Some(kids) => kids,
            None => return Ok(()),
        };

        let comments_raw = client.get_item_comments(&viewed_item_kids).await?;
        self.comments = DisplayableHackerNewsItem::transform_comments(comments_raw)?;

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
        if let Some(item) = ctx.get_state().get_currently_viewed_item() {
            let block = Block::default()
                .style(Style::default().fg(COMMON_BLOCK_NORMAL_COLOR))
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded);

            let text = vec![Spans::from(""), Spans::from("TODO: COMMENTS SECTION")];
            let paragraph = Paragraph::new(text)
                .block(block)
                .alignment(Alignment::Center);
            f.render_widget(paragraph, inside);
        }

        Ok(())
    }
}
