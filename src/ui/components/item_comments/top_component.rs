use std::{sync::Arc, thread};

use async_trait::async_trait;
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

pub const ITEM_TOP_LEVEL_COMMENTS_ID: UiComponentId = "item_top_comments";

/// Top-level comments component.
#[derive(Debug, Default)]
pub struct ItemTopLevelComments {
    common: ItemCommentsCommon,
}

#[async_trait]
impl UiComponent for ItemTopLevelComments {
    fn id(&self) -> UiComponentId {
        ITEM_TOP_LEVEL_COMMENTS_ID
    }

    fn before_unmount(&mut self) {
        self.common.loader.stop();
    }

    async fn should_update(
        &mut self,
        elapsed_ticks: UiTickScalar,
        ctx: &AppContext,
    ) -> Result<bool> {
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
                .use_currently_viewed_item_comments(|comments| comments.is_none())
                .await
            || currently_viewed_item
                .kids
                .as_ref()
                .map_or(0, |kids| kids.len())
                != self
                    .common
                    .widget_state
                    .get_focused_same_level_comments_count()
            || self.common.fetched_comments.lock().await.is_some();

        self.common.loader.update();

        if should_update {
            self.common.loading = true;
        }

        Ok(should_update)
    }

    async fn update(&mut self, client: &mut HnClient, ctx: &mut AppContext) -> Result<()> {
        self.common.loading = true;

        // Comments fetching
        let parent_item_kids = Self::get_parent_item_kids(ctx.get_state())?;
        if parent_item_kids.is_empty() {
            return Ok(());
        }
        let cached_comments_ids = ctx
            .get_state()
            .use_currently_viewed_item_comments(|comments| {
                comments
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
                .get_item_comments(&parent_item_kids, &cached_comments_ids, false) // TODO: avoid .clone()
                .await?;
            *fetching.lock().await = false;
            let comments = DisplayableHackerNewsItem::transform_comments(comments_raw)?;
            *fetched_comments.lock().await = Some(comments);
            Ok::<(), HnCliError>(())
        });
        if let Some(fetched_comments) = self.common.fetched_comments.lock().await.take() {
            ctx.get_state_mut()
                .update_currently_viewed_item_comments(Some(fetched_comments))
                .await;
            // TODO: avoid cloning
            ctx.get_state()
                .use_currently_viewed_item_comments(|comments| {
                    self.common.cached_comments = comments.cloned();
                    self.common.widget_state.update(
                        &self
                            .common
                            .cached_comments
                            .as_ref()
                            .unwrap_or(&DisplayableHackerNewsItemComments::new()),
                        &Self::get_parent_item_kids(ctx.get_state())?,
                    );
                    Ok::<(), HnCliError>(())
                })
                .await?;
        }

        self.common.loading = false;

        Ok(())
    }

    async fn handle_inputs(&mut self, ctx: &mut AppContext) -> Result<bool> {
        if ctx.get_inputs().is_active(&ApplicationAction::Back) {
            // TODO: this should be handled at screen level but seems to be needed sometimes
            ctx.router_pop_navigation_stack();
            return Ok(true);
        }

        if self.common.loading || !self.common.inputs_debouncer.is_action_allowed() {
            return Ok(false);
        }

        let parent_item_kids = Self::get_parent_item_kids(ctx.get_state())?;
        let inputs = ctx.get_inputs();
        Ok(if inputs.is_active(&ApplicationAction::NavigateUp) {
            let new_focused_id = self
                .common
                .widget_state
                .previous_main_comment(&parent_item_kids);
            ctx.get_state_mut()
                .replace_latest_in_currently_viewed_item_comments_chain(new_focused_id);
            true
        } else if inputs.is_active(&ApplicationAction::NavigateDown) {
            let new_focused_id = self
                .common
                .widget_state
                .next_main_comment(&parent_item_kids);
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
        self.common.render(f, inside, ctx, || None)
    }
}

impl ItemTopLevelComments {
    fn get_parent_item_kids(state: &AppState) -> Result<Vec<HnItemIdScalar>> {
        let parent_item = state.get_currently_viewed_item().ok_or_else(|| {
            HnCliError::UiError(
                "ItemTopLevelComments: cannot retrieve currently viewed item.".into(),
            )
        })?;
        Ok(
            parent_item
                .kids
                .as_ref()
                .map_or(vec![], |kids| kids.to_vec()), // TODO: can we avoid the Vec here?
        )
    }
}
