use std::io::Stdout;

use async_trait::async_trait;
use log::info;
use tui::{
    backend::CrosstermBackend,
    layout::{Alignment, Rect},
    style::Style,
    text::Spans,
    widgets::{Block, BorderType, Borders, Paragraph},
};

use crate::{
    api::{types::HnItemIdScalar, HnClient},
    app::{AppContext, AppState},
    errors::Result,
    ui::{
        common::{UiComponent, UiComponentId, UiTickScalar},
        components::common::COMMON_BLOCK_NORMAL_COLOR,
        displayable_item::DisplayableHackerNewsItem,
        handlers::ApplicationAction,
        router::AppRoute,
        utils::debouncer::Debouncer,
    },
};

use self::widget::{ItemCommentsWidget, ItemCommentsWidgetState};

mod comment_widget;
mod widget;

/// Item comments component.
#[derive(Debug)]
pub struct ItemComments {
    ticks_since_last_update: u64,
    inputs_debouncer: Debouncer,
    loading: bool,
    widget_state: ItemCommentsWidgetState,
}

impl Default for ItemComments {
    fn default() -> Self {
        Self {
            ticks_since_last_update: 0,
            inputs_debouncer: Debouncer::new(5), // approx. 500ms
            loading: true,
            widget_state: ItemCommentsWidgetState::default(),
        }
    }
}

const MEAN_TICKS_BETWEEN_UPDATES: UiTickScalar = 1800; // approx. every 3 minutes

pub const ITEM_COMMENTS_ID: UiComponentId = "item_comments";

// TODO: fix bug where inputs are no longer responsive on some comments
#[async_trait]
impl UiComponent for ItemComments {
    fn id(&self) -> UiComponentId {
        ITEM_COMMENTS_ID
    }

    fn should_update(&mut self, elapsed_ticks: UiTickScalar, ctx: &AppContext) -> Result<bool> {
        if ctx.get_state().get_currently_viewed_item_switched() {
            // update comments on viewed item switch
            // TODO: try to find a not too contrived way of forcing the loading screen to display on switching between items
            self.inputs_debouncer.reset();
            self.loading = true;
            return Ok(true);
        }

        self.ticks_since_last_update += elapsed_ticks;
        self.inputs_debouncer.tick(elapsed_ticks);

        let currently_viewed_item_or_comment = Self::get_viewed_item_or_comment(ctx.get_state());

        let should_update = self.ticks_since_last_update >= MEAN_TICKS_BETWEEN_UPDATES
            || ctx
                .get_state()
                .get_currently_viewed_item_comments()
                .is_none()
            || currently_viewed_item_or_comment.map_or(false, |item| {
                !item.is_comment
                    && item.kids.as_ref().map_or(0, |kids| kids.len())
                        != self.widget_state.get_focused_same_level_comments_count()
            })
            || currently_viewed_item_or_comment.map_or(false, |comment| {
                comment.is_comment && Some(comment.id) != self.widget_state.get_focused_comment_id()
            });

        if should_update {
            self.loading = true;
        }

        Ok(should_update)
    }

    async fn update(&mut self, client: &mut HnClient, ctx: &mut AppContext) -> Result<()> {
        self.loading = true;
        self.ticks_since_last_update = 0;

        // Viewed item handling
        let viewed_item = if let Some(item_or_comment) =
            Self::get_viewed_item_or_parent_comment(ctx.get_state())
        {
            item_or_comment.clone()
        } else {
            self.loading = true;
            return Ok(());
        };
        let viewed_item_kids: &[HnItemIdScalar] = viewed_item
            .kids
            .as_ref()
            .map_or(&[], |kids| kids.as_slice());

        // Comments fetching
        let comments_raw = client.get_item_comments(viewed_item_kids).await?;
        let comments = DisplayableHackerNewsItem::transform_comments(comments_raw)?;
        ctx.get_state_mut()
            .set_currently_viewed_item_comments(Some(comments));

        // Widget state
        let viewed_item_comments =
            if let Some(cached_comments) = ctx.get_state().get_currently_viewed_item_comments() {
                cached_comments
            } else {
                self.loading = false;
                return Ok(());
            };
        self.widget_state
            .update(viewed_item_comments, viewed_item_kids);

        // Latest focused comment, if applicable
        if let Some(restored_comment_id) = ctx.get_state().get_previously_viewed_comment_id() {
            self.widget_state
                .restore_focused_comment_id(restored_comment_id, viewed_item_kids);
            ctx.get_state_mut().set_previously_viewed_comment_id(None);
        }

        self.loading = false;

        Ok(())
    }

    fn handle_inputs(&mut self, ctx: &mut AppContext) -> Result<bool> {
        if self.loading || !self.inputs_debouncer.is_action_allowed() {
            return Ok(false);
        }

        info!("handle_inputs.before_check");

        let viewed_item_kids = if let Some(item_or_comment) =
            Self::get_viewed_item_or_parent_comment(ctx.get_state())
        {
            match item_or_comment.kids.as_ref() {
                Some(kids) => kids.as_slice(),
                None => &[],
            }
        } else {
            return Ok(false);
        };

        info!(
            "handle_inputs.viewed_item_id={:?}",
            ctx.get_state()
                .get_currently_viewed_item()
                .map(|item| item.id)
        );
        info!("handle_inputs.viewed_item_kids = {:?}", viewed_item_kids);

        info!(
            "handle_inputs, chain={:?}",
            ctx.get_state().get_currently_viewed_item_comments_chain()
        );
        let inputs = ctx.get_inputs();
        Ok(if inputs.is_active(&ApplicationAction::NavigateUp) {
            let new_focused_id = self.widget_state.previous_main_comment(viewed_item_kids);
            ctx.get_state_mut()
                .replace_latest_in_currently_viewed_item_comments_chain(new_focused_id);
            true
        } else if inputs.is_active(&ApplicationAction::NavigateDown) {
            let new_focused_id = self.widget_state.next_main_comment(viewed_item_kids);
            ctx.get_state_mut()
                .replace_latest_in_currently_viewed_item_comments_chain(new_focused_id);
            true
        } else if inputs.is_active(&ApplicationAction::ItemExpandFocusedComment) {
            let focused_comment_id =
                if let Some(comment_id) = self.widget_state.get_focused_comment_id() {
                    comment_id
                } else {
                    return Ok(false);
                };
            let focused_comment = ctx
                .get_state()
                .get_currently_viewed_item_comments()
                .expect("comments should be cached in the global state")
                .get(&focused_comment_id)
                .expect("comment should be present in the global state")
                .clone();
            if focused_comment
                .kids
                .as_ref()
                .map_or(true, |kids| kids.is_empty())
            {
                return Ok(false);
            }
            ctx.router_push_navigation_stack(AppRoute::ItemSubComments(focused_comment));
            true
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
        // (Initial) loading case
        if self.loading {
            return Self::render_text_message(f, inside, "Loading...");
        }

        // Are we rendering a sub-comment?
        let rendering_nested_comment = Self::get_viewed_item_or_parent_comment(ctx.get_state())
            .map_or(false, |item| item.is_comment);

        // Unavailable comments cache
        let viewed_item_comments =
            if let Some(cached_comments) = ctx.get_state().get_currently_viewed_item_comments() {
                cached_comments
            } else {
                return Self::render_text_message(
                    f,
                    inside,
                    "Comments fetching issue. Please retry later.",
                );
            };

        // Invalid viewed item cases
        if ctx
            .get_state()
            .get_currently_viewed_item_comments_chain()
            .len()
            > 1
        {
            if let Some(item_or_comment) = if rendering_nested_comment {
                Self::get_viewed_item_or_parent_comment(ctx.get_state())
            } else {
                Self::get_viewed_item_or_comment(ctx.get_state())
            } {
                // No comments case
                if item_or_comment.kids.is_none() {
                    return Self::render_text_message(f, inside, "No comments yet.");
                }
                // Main-level comment case
                item_or_comment
            } else {
                return Self::render_text_message(f, inside, "Cannot display this item.");
            }
        } else if let Some(parent_item_or_comment) =
            Self::get_viewed_item_or_parent_comment(ctx.get_state())
        {
            // Sub-comment case
            parent_item_or_comment
        } else {
            return Self::render_text_message(f, inside, "Cannot render this item.");
        };

        // Rendering
        let widget = ItemCommentsWidget::with_comments(&self.widget_state, viewed_item_comments);
        f.render_widget(widget, inside);

        Ok(())
    }
}

impl ItemComments {
    /// Get the currently viewed main HackerNews item, which can be the current comment in this case.
    fn get_viewed_item_or_comment(state: &AppState) -> Option<&DisplayableHackerNewsItem> {
        let comments_chain = state.get_currently_viewed_item_comments_chain();
        info!("get_viewed_item_or_comment:chain={:?}", comments_chain);
        info!(
            "\tget_viewed_item_or_comment:available comments IDs={:?}",
            state
                .get_currently_viewed_item_comments()
                .map(|cached| cached.keys())
        );
        if let Some(latest_comment_id) = comments_chain.last() {
            match state.get_currently_viewed_item_comments() {
                Some(cached_comments) => cached_comments.get(latest_comment_id),
                None => None,
            }
        } else {
            state.get_currently_viewed_item()
        }
    }

    /// Get the currently viewed main HackerNews item, which can be the parent comment in this case.
    fn get_viewed_item_or_parent_comment(state: &AppState) -> Option<&DisplayableHackerNewsItem> {
        let comments_chain = state.get_currently_viewed_item_comments_chain();
        if let Some(parent_comment_id) = comments_chain
            .len()
            .checked_sub(2)
            .map(|parent_comment_index| comments_chain[parent_comment_index])
        {
            match state.get_currently_viewed_item_comments() {
                Some(cached_comments) => cached_comments.get(&parent_comment_id),
                None => None,
            }
        } else {
            state.get_currently_viewed_item()
        }
    }

    fn render_text_message(
        f: &mut tui::Frame<CrosstermBackend<Stdout>>,
        inside: Rect,
        message: &str,
    ) -> Result<()> {
        let block = Block::default()
            .style(Style::default().fg(COMMON_BLOCK_NORMAL_COLOR))
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded);

        let text = vec![Spans::from(""), Spans::from(message.to_string())];
        let paragraph = Paragraph::new(text)
            .block(block)
            .alignment(Alignment::Center);
        f.render_widget(paragraph, inside);
        Ok(())
    }
}
