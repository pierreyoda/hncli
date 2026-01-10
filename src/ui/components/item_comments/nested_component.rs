use std::{sync::Arc, thread};

use async_trait::async_trait;
use log::warn;
use ratatui::layout::Rect;

use crate::{
    api::{HnClient, types::HnItemIdScalar},
    app::{AppContext, state::AppState},
    errors::{HnCliError, Result},
    ui::{
        common::{RenderFrame, UiComponent, UiComponentId, UiTickScalar},
        displayable_item::{
            CachedHackerNewsItemCommentsIds, DisplayableHackerNewsItem,
            DisplayableHackerNewsItemComments,
        },
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

    async fn should_update(
        &mut self,
        elapsed_ticks: UiTickScalar,
        ctx: &AppContext,
    ) -> Result<bool> {
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

        // Parent comments handling
        let parent_comment_kids =
            if let Some(kids) = Self::get_parent_comment_kids(ctx.get_state()).await {
                kids
            } else {
                return Ok(());
            };

        // Comments fetching
        let cached_comments_ids = ctx
            .get_state()
            .use_currently_viewed_item_comments(|cached_comments| {
                cached_comments
                    .unwrap_or(&DisplayableHackerNewsItemComments::new())
                    .to_cached_ids()
            })
            .await;
        let fetching = Arc::clone(&self.common.fetching);
        let fetched_comments = Arc::clone(&self.common.fetched_comments);
        let fetching_client = client.classic_non_blocking();
        // fetching in a separate thread to avoid blocking the async runtime
        thread::spawn(async move || {
            if *fetching.lock().await {
                return Ok(());
            }
            *fetching.lock().await = true;
            let comments_raw = fetching_client
                .lock()
                .await
                .get_item_comments(&parent_comment_kids, &cached_comments_ids, false) // TODO: avoid .clone()
                .await?;
            *fetching.lock().await = false;
            let comments = DisplayableHackerNewsItem::transform_comments(comments_raw)?;
            *fetched_comments.lock().await = Some(comments);
            Ok::<(), HnCliError>(())
        });

        // Widget state
        if let Some(fetched_comments) = self.common.fetched_comments.lock().await.take() {
            ctx.get_state_mut()
                .update_currently_viewed_item_comments(Some(fetched_comments))
                .await;
            // TODO: avoid cloning
            let cached_comments = Some(
                ctx.get_state()
                    .use_currently_viewed_item_comments(|comments| {
                        comments
                            .unwrap_or(&DisplayableHackerNewsItemComments::new())
                            .clone()
                    })
                    .await,
            );
            self.common.widget_state.update(
                &cached_comments
                    .as_ref()
                    .unwrap_or(&DisplayableHackerNewsItemComments::new()),
                &Self::get_parent_comment_kids(ctx.get_state())
                    .await
                    .unwrap_or(vec![]),
            );
        }

        self.common.loading = false;

        // Latest focused comment, if applicable
        if let Some(restored_comment_id) = ctx.get_state().get_previously_viewed_comment_id() {
            self.common.widget_state.restore_focused_comment_id(
                restored_comment_id,
                &Self::get_parent_comment_kids(ctx.get_state())
                    .await
                    .unwrap_or(vec![]),
            );
            ctx.get_state_mut().set_previously_viewed_comment_id(None);
        }

        Ok(())
    }

    async fn handle_inputs(&mut self, ctx: &mut AppContext) -> Result<bool> {
        if ctx.get_inputs().is_active(&ApplicationAction::Back) {
            // TODO: this should be handled at screen level but seems to be needed somehow
            ctx.router_pop_navigation_stack();
            return Ok(true);
        }

        if self.common.loading || !self.common.inputs_debouncer.is_action_allowed() {
            return Ok(false);
        }

        let parent_comment_kids =
            if let Some(kids) = Self::get_parent_comment_kids(ctx.get_state()).await {
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
            if let Some(focused_comment) = self.common.get_focused_comment(ctx.get_state()).await {
                if focused_comment
                    .kids
                    .as_ref()
                    .is_none_or(|kids| kids.is_empty())
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
        } else if inputs.is_active(&ApplicationAction::FocusedCommentViewUserProfile) {
            if let Some(focused_comment) = self.common.get_focused_comment(ctx.get_state()).await {
                ctx.router_push_navigation_stack(AppRoute::UserProfile(
                    focused_comment.by_username.clone(),
                ));
                true
            } else {
                false
            }
        } else {
            false
        })
    }

    fn render(&mut self, f: &mut RenderFrame, inside: Rect, ctx: &AppContext) -> Result<()> {
        // TODO: handle specific errors that may arise
        self.common.render(f, inside, ctx, || None)
    }
}

impl CommentItemNestedComments {
    async fn get_parent_comment_kids(state: &AppState) -> Option<Vec<HnItemIdScalar>> {
        state.use_currently_viewed_item_comments(|comments| {
            let comments_cache = if let Some(cache) = comments.as_ref() {
                cache
            } else {
                warn!("CommentItemNestedComments: no comments cache available.");
                return None;
            };
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
                    "CommentItemNestedComments: cannot find parent comment with ID '{parent_comment_id}'"
                );
                return None;
            };

            Some(parent_comment.kids.as_ref().map_or(vec![], |kids| kids.to_vec()))
        }).await
    }

    fn get_parent_comment_id(state: &AppState) -> Option<HnItemIdScalar> {
        let comments_chain = state.get_currently_viewed_item_comments_chain();
        comments_chain
            .len()
            .checked_sub(2)
            .map(|parent_comment_index| comments_chain[parent_comment_index])
    }
}
