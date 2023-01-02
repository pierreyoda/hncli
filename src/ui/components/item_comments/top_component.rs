use std::{io::Stdout, vec};

use async_trait::async_trait;
use log::warn;
use tui::{backend::CrosstermBackend, layout::Rect};

use crate::{
    api::{types::HnItemIdScalar, HnClient},
    app::AppContext,
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

pub const ITEM_TOP_LEVEL_COMMENTS_ID: UiComponentId = "item_top_comments";

/// Top-level comments component.
#[derive(Debug, Default)]
pub struct ItemTopLevelComments {
    common: ItemCommentsCommon,
}

// TODO: fix behavior when stuck (ALL inputs not working) in top/nested comments (probably due to update's awaits)...
// TODO: ...maybe perform updates in another thread?
#[async_trait]
impl UiComponent for ItemTopLevelComments {
    fn id(&self) -> UiComponentId {
        ITEM_TOP_LEVEL_COMMENTS_ID
    }

    fn should_update(&mut self, elapsed_ticks: UiTickScalar, ctx: &AppContext) -> Result<bool> {
        if ctx.get_state().get_currently_viewed_item_switched() {
            // update comments on viewed item switch
            self.common.inputs_debouncer.reset();
            self.common.loading = true;

            // history navigation handling
            let currently_viewed_item =
                if let Some(item) = ctx.get_state().get_currently_viewed_item() {
                    item
                } else {
                    return Ok(true);
                };
            if let Some(restored_comment_id) = ctx
                .get_history()
                .restored_top_level_comment_id_for_story(currently_viewed_item.id)
            {
                self.common
                    .widget_state
                    .history_prepare_focus_on_comment_id(restored_comment_id);
            }

            return Ok(true);
        }

        self.common.ticks_since_last_update += elapsed_ticks;
        self.common.inputs_debouncer.tick(elapsed_ticks);

        let currently_viewed_item = if let Some(item) = ctx.get_state().get_currently_viewed_item()
        {
            item
        } else {
            return Ok(false);
        };

        let should_update = self.common.ticks_since_last_update >= MEAN_TICKS_BETWEEN_UPDATES
            || ctx
                .get_state()
                .get_currently_viewed_item_comments()
                .is_none()
            || currently_viewed_item
                .kids
                .as_ref()
                .map_or(0, |kids| kids.len())
                != self
                    .common
                    .widget_state
                    .get_focused_same_level_comments_count();

        if should_update {
            self.common.loading = true;
        }

        Ok(should_update)
    }

    async fn update(&mut self, client: &mut HnClient, ctx: &mut AppContext) -> Result<()> {
        self.common.loading = true;

        // Parent item handling
        let parent_item = if let Some(item_or_comment) = ctx.get_state().get_currently_viewed_item()
        {
            item_or_comment
        } else {
            return Ok(());
        };
        let parent_item_kids: Vec<HnItemIdScalar> = parent_item
            .kids
            .as_ref()
            .map_or(vec![], |kids| kids.to_vec()); // TODO: can we avoid the Vec here?

        // Comments fetching
        let comments_raw = client
            .get_item_comments(parent_item_kids.as_slice())
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
            .update(viewed_item_comments, parent_item_kids.as_slice());

        self.common.loading = false;

        Ok(())
    }

    fn handle_inputs(&mut self, ctx: &mut AppContext) -> Result<bool> {
        if ctx.get_inputs().is_active(&ApplicationAction::Back) {
            // TODO: this should be handled at screen level but seems to be needed sometimes
            ctx.router_pop_navigation_stack();
            return Ok(true);
        }

        if self.common.loading || !self.common.inputs_debouncer.is_action_allowed() {
            return Ok(false);
        }

        let parent_item = if let Some(item) = ctx.get_state().get_currently_viewed_item() {
            item
        } else {
            warn!("ItemTopLevelComments.handle_inputs: no parent item.");
            return Ok(false);
        };
        let parent_item_kids: &[HnItemIdScalar] = parent_item
            .kids
            .as_ref()
            .map_or(&[], |kids| kids.as_slice());

        let inputs = ctx.get_inputs();
        Ok(if inputs.is_active(&ApplicationAction::NavigateUp) {
            let new_focused_id = self
                .common
                .widget_state
                .previous_main_comment(parent_item_kids);
            ctx.get_state_mut()
                .replace_latest_in_currently_viewed_item_comments_chain(new_focused_id);
            true
        } else if inputs.is_active(&ApplicationAction::NavigateDown) {
            let new_focused_id = self.common.widget_state.next_main_comment(parent_item_kids);
            ctx.get_state_mut()
                .replace_latest_in_currently_viewed_item_comments_chain(new_focused_id);
            true
        } else if inputs.is_active(&ApplicationAction::ItemExpandFocusedComment) {
            if let Some(focused_comment) = self.common.get_focused_comment(ctx.get_state()) {
                if focused_comment
                    .kids
                    .as_ref()
                    .map_or(true, |kids| kids.is_empty())
                {
                    // a comment with no sub-comments cannot be focused
                    return Ok(false);
                }
                ctx.router_push_navigation_stack(AppRoute::ItemNestedComments(
                    focused_comment.clone(),
                ));
                true
            } else {
                false
            }
        } else {
            false
        })
    }

    fn render(
        &mut self,
        f: &mut tui::Frame<CrosstermBackend<Stdout>>,
        inside: Rect,
        ctx: &AppContext,
    ) -> Result<()> {
        self.common.render(f, inside, ctx, || None)
    }
}
