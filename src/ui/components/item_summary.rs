use async_trait::async_trait;
use ratatui::{
    layout::{Alignment, Rect},
    style::Style,
    text::Line,
    widgets::{Block, BorderType, Borders, Paragraph},
};

use crate::{
    api::{HnClient, types::HnItemIdScalar},
    app::{AppContext, state::AppState},
    errors::Result,
    ui::common::{RenderFrame, UiComponent, UiComponentId, UiTickScalar},
};

/// Item summary component, intended for when navigating sub-comments.
///
/// Does not do any fetching, everything is pre-cached.
///
/// ```md
/// ___________________________________________
/// |                                         |
/// |         <PARENT COMMENT USERNAME>       |
/// |           <SUB-COMMENTS LEVEL>          |
/// |_________________________________________|
/// ```
#[derive(Debug, Default)]
pub struct ItemSummary {
    /// HackerNews ID of the parent comment, cached for efficiency.
    parent_comment_id: Option<HnItemIdScalar>,
}

pub const ITEM_SUMMARY_ID: UiComponentId = "item_summary";

#[async_trait]
impl UiComponent for ItemSummary {
    fn id(&self) -> UiComponentId {
        ITEM_SUMMARY_ID
    }

    fn should_update(&mut self, _elapsed_ticks: UiTickScalar, ctx: &AppContext) -> Result<bool> {
        Ok(self.get_parent_comment_id(ctx.get_state()) != self.parent_comment_id)
    }

    async fn update(&mut self, _client: &mut HnClient, ctx: &mut AppContext) -> Result<()> {
        self.parent_comment_id = self.get_parent_comment_id(ctx.get_state());
        Ok(())
    }

    fn handle_inputs(&mut self, _ctx: &mut AppContext) -> Result<bool> {
        Ok(false)
    }

    fn render(&mut self, f: &mut RenderFrame, inside: Rect, ctx: &AppContext) -> Result<()> {
        let comments = ctx
            .get_state()
            .get_currently_viewed_item_comments()
            .expect("item_summary expects comments to be cached");
        let parent_comment = if let Some(comment_id) = self.parent_comment_id {
            if let Some(comment) = comments.get(&comment_id) {
                comment
            } else {
                return Ok(());
            }
        } else {
            return Ok(());
        };

        let theme = ctx.get_theme();

        let block = Block::default()
            .style(Style::default().fg(theme.get_block_color()))
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded);

        let text_base = vec![
            Line::from(format!("Parent comment by: {}", parent_comment.by_username)),
            Line::from(format!(
                "Sub-comment level: {}",
                ctx.get_state()
                    .get_currently_viewed_item_comments_chain()
                    .len()
            )),
        ];

        let paragraph = Paragraph::new(text_base)
            .block(block)
            .alignment(Alignment::Center);
        f.render_widget(paragraph, inside);

        Ok(())
    }
}

impl ItemSummary {
    fn get_parent_comment_id(&self, state: &AppState) -> Option<HnItemIdScalar> {
        let comments_chain = state.get_currently_viewed_item_comments_chain();
        comments_chain
            .len()
            .checked_sub(2)
            .map(|parent_comment_index| comments_chain[parent_comment_index])
    }
}
