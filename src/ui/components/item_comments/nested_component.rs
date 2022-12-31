use std::io::Stdout;

use async_trait::async_trait;
use log::warn;
use tui::{backend::CrosstermBackend, layout::Rect, Frame};

use crate::{
    api::{types::HnItemIdScalar, HnClient},
    app::{state::AppState, AppContext},
    errors::Result,
    ui::{
        common::{UiComponent, UiComponentId, UiTickScalar},
        displayable_item::DisplayableHackerNewsItem,
        handlers::ApplicationAction,
        router::AppRoute,
    },
};

use super::common::ItemCommentsCommon;

const MEAN_TICKS_BETWEEN_UPDATES: UiTickScalar = 1800; // approx. every 3 minutes

pub const COMMENT_ITEM_NESTED_COMMENTS_ID: UiComponentId = "item_nested_comments";

/// Sub-main level (= nested) comments component.
#[derive(Debug, Default)]
pub struct CommentItemNestedComments {
    common: ItemCommentsCommon,
    /// Cached parent comment ID.
    parent_comment_id: Option<HnItemIdScalar>,
}

// TODO: fix behavior when stuck (ALL inputs not working) in nested comments (probably due to update's awaits)
#[async_trait]
impl UiComponent for CommentItemNestedComments {
    fn id(&self) -> UiComponentId {
        COMMENT_ITEM_NESTED_COMMENTS_ID
    }

    fn should_update(&mut self, elapsed_ticks: UiTickScalar, ctx: &AppContext) -> Result<bool> {
        self.common.ticks_since_last_update += elapsed_ticks;
        self.common.inputs_debouncer.tick(elapsed_ticks);

        let should_update = self.common.ticks_since_last_update >= MEAN_TICKS_BETWEEN_UPDATES
            || Self::get_parent_comment_id(ctx.get_state()) != self.parent_comment_id;

        if should_update {
            self.common.loading = true;
        }

        Ok(should_update)
    }

    async fn update(&mut self, client: &mut HnClient, ctx: &mut AppContext) -> Result<()> {
        self.common.loading = true;

        self.parent_comment_id = Self::get_parent_comment_id(ctx.get_state());
        if self.parent_comment_id.is_none() {
            warn!("CommentItemNestedComments.update: no parent comment ID available.");
            return Ok(());
        }

        // Parent comment handling
        let parent_comment_kids = if let Some(kids) = Self::get_parent_comment_kids(ctx.get_state())
        {
            kids
        } else {
            return Ok(());
        };

        // Comments fetching
        let comments_raw = client
            .get_item_comments(parent_comment_kids.as_slice())
            .await?;
        let comments = DisplayableHackerNewsItem::transform_comments(comments_raw)?;
        ctx.get_state_mut()
            .set_currently_viewed_item_comments(Some(comments));

        self.common.loading = false;

        // Widget state
        let viewed_item_comments =
            if let Some(cached_comments) = ctx.get_state().get_currently_viewed_item_comments() {
                cached_comments
            } else {
                return Ok(());
            };
        self.common
            .widget_state
            .update(viewed_item_comments, parent_comment_kids.as_slice());

        // Latest focused comment, if applicable
        if let Some(restored_comment_id) = ctx.get_state().get_previously_viewed_comment_id() {
            self.common
                .widget_state
                .restore_focused_comment_id(restored_comment_id, parent_comment_kids.as_slice());
            ctx.get_state_mut().set_previously_viewed_comment_id(None);
        }

        Ok(())
    }

    fn handle_inputs(&mut self, ctx: &mut AppContext) -> Result<bool> {
        if ctx.get_inputs().is_active(&ApplicationAction::Back) {
            // TODO: this should be handled at screen level but seems to be needed somehow
            ctx.router_pop_navigation_stack();
            return Ok(true);
        }

        if self.common.loading || !self.common.inputs_debouncer.is_action_allowed() {
            return Ok(false);
        }

        let parent_comment_kids = if let Some(kids) = Self::get_parent_comment_kids(ctx.get_state())
        {
            kids
        } else {
            return Ok(false);
        };

        let inputs = ctx.get_inputs();
        // TODO: refactor with top component usage as much as possible
        Ok(if inputs.is_active(&ApplicationAction::NavigateUp) {
            let new_focused_id = self
                .common
                .widget_state
                .previous_main_comment(parent_comment_kids.as_slice());
            ctx.get_state_mut()
                .replace_latest_in_currently_viewed_item_comments_chain(new_focused_id);
            true
        } else if inputs.is_active(&ApplicationAction::NavigateDown) {
            let new_focused_id = self
                .common
                .widget_state
                .next_main_comment(parent_comment_kids.as_slice());
            ctx.get_state_mut()
                .replace_latest_in_currently_viewed_item_comments_chain(new_focused_id);
            true
        } else if inputs.is_active(&ApplicationAction::ItemExpandFocusedComment) {
            let focused_comment_id =
                if let Some(comment_id) = self.common.widget_state.get_focused_comment_id() {
                    comment_id
                } else {
                    return Ok(false);
                };
            let focused_comment = ctx
                .get_state()
                .get_currently_viewed_item_comments()
                .expect("comments should be cached in the global state")
                .get(&focused_comment_id)
                .expect("focused comment should be present in the global state")
                .clone();
            if focused_comment
                .kids
                .as_ref()
                .map_or(true, |kids| kids.is_empty())
            {
                // a comment with no sub-comments cannot be focused
                return Ok(false);
            }
            ctx.router_push_navigation_stack(AppRoute::ItemNestedComments(focused_comment));
            true
        } else {
            false
        })
    }

    fn render(
        &mut self,
        f: &mut Frame<CrosstermBackend<Stdout>>,
        inside: Rect,
        ctx: &AppContext,
    ) -> Result<()> {
        // TODO: handle specific errors that may arise
        self.common.render(f, inside, ctx, || None)
    }
}

impl CommentItemNestedComments {
    fn get_parent_comment_kids(state: &AppState) -> Option<Vec<HnItemIdScalar>> {
        let comments_cache = state.get_currently_viewed_item_comments()?;

        let parent_comment_id = if let Some(id) = Self::get_parent_comment_id(state) {
            id
        } else {
            warn!("CommentItemNestedComments: cannot retrieve parent comment ID.");
            return None;
        };

        let parent_comment = if let Some(comment) = comments_cache.get(&parent_comment_id) {
            comment
        } else {
            warn!(
                "CommentItemNestedComments: cannot find parent comment with ID '{}'",
                parent_comment_id
            );
            return None;
        };

        Some(
            parent_comment
                .kids
                .as_ref()
                .map_or(vec![], |kids| kids.to_vec()),
        )
    }

    fn get_parent_comment_id(state: &AppState) -> Option<HnItemIdScalar> {
        let comments_chain = state.get_currently_viewed_item_comments_chain();
        comments_chain
            .len()
            .checked_sub(2)
            .map(|parent_comment_index| comments_chain[parent_comment_index])
    }
}
