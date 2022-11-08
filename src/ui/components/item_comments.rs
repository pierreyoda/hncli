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
    initial_loading: bool,
    loading: bool,
    viewed_item_id: HnItemIdScalar,
    viewed_item_kids: Vec<HnItemIdScalar>,
    latest_focused_comment_id: Option<HnItemIdScalar>,
    widget_state: ItemCommentsWidgetState,
}

impl Default for ItemComments {
    fn default() -> Self {
        Self {
            ticks_since_last_update: 0,
            inputs_debouncer: Debouncer::new(5), // approx. 500ms
            initial_loading: true,
            loading: true,
            viewed_item_id: 0,
            viewed_item_kids: vec![],
            latest_focused_comment_id: None,
            widget_state: ItemCommentsWidgetState::default(),
        }
    }
}

const MEAN_TICKS_BETWEEN_UPDATES: UiTickScalar = 1800; // approx. every 3 minutes

pub const ITEM_COMMENTS_ID: UiComponentId = "item_comments";

#[async_trait]
impl UiComponent for ItemComments {
    fn id(&self) -> UiComponentId {
        ITEM_COMMENTS_ID
    }

    fn should_update(&mut self, elapsed_ticks: UiTickScalar, ctx: &AppContext) -> Result<bool> {
        self.ticks_since_last_update += elapsed_ticks;
        self.inputs_debouncer.tick(elapsed_ticks);

        // TODO: should also update when comments are dirty?
        Ok(self.initial_loading
            || self.ticks_since_last_update >= MEAN_TICKS_BETWEEN_UPDATES
            || ctx
                .get_state()
                .get_currently_viewed_item()
                .map(|item| item.id)
                != Some(self.viewed_item_id))
    }

    // TODO: fix instability in currently focused sub-comment
    async fn update(&mut self, client: &mut HnClient, ctx: &mut AppContext) -> Result<()> {
        self.loading = true;
        self.ticks_since_last_update = 0;

        ctx.get_state_mut().set_currently_viewed_item_comments(None);

        // Viewed item handling
        let viewed_item = match ctx.get_state().get_currently_viewed_item() {
            Some(item) => item,
            None => return Ok(()),
        };
        self.viewed_item_id = viewed_item.id;
        self.viewed_item_kids = match &viewed_item.kids {
            Some(kids) => kids.clone(),
            None => return Ok(()),
        };

        // Comments fetching
        let comments_raw = client.get_item_comments(&self.viewed_item_kids).await?;
        let comments = DisplayableHackerNewsItem::transform_comments(comments_raw)?;
        ctx.get_state_mut()
            .set_currently_viewed_item_comments(Some(comments));

        self.initial_loading = false;
        self.loading = false;

        // Widget state
        let viewed_item_comments =
            if let Some(cached_comments) = ctx.get_state().get_currently_viewed_item_comments() {
                cached_comments
            } else {
                return Ok(());
            };
        self.widget_state
            .update(viewed_item_comments, &self.viewed_item_kids);

        // Latest focused comment, if applicable
        if let Some(comment_id) = self.latest_focused_comment_id {
            self.widget_state
                .restore_focused_comment_id(comment_id, &self.viewed_item_kids);
        }

        // Comments chain update
        if let Some(focused_comment_id) = self.widget_state.get_focused_comment_id() {
            ctx.get_state_mut()
                .push_currently_viewed_item_comments_chain(focused_comment_id);
        }

        Ok(())
    }

    fn handle_inputs(&mut self, ctx: &mut AppContext) -> Result<bool> {
        if self.initial_loading || self.loading || !self.inputs_debouncer.is_action_allowed() {
            return Ok(false);
        }

        let inputs = ctx.get_inputs();
        Ok(if inputs.is_active(&ApplicationAction::NavigateUp) {
            self.widget_state
                .previous_main_comment(&self.viewed_item_kids);
            true
        } else if inputs.is_active(&ApplicationAction::NavigateDown) {
            self.widget_state.next_main_comment(&self.viewed_item_kids);
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
            self.latest_focused_comment_id = self.widget_state.get_focused_comment_id();
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
        if self.initial_loading || self.loading {
            return Self::render_text_message(f, inside, "Loading...");
        }

        // Invalid viewed item case
        match ctx.get_state().get_currently_viewed_item() {
            Some(viewed_item) if viewed_item.can_have_comments() => (),
            _ => {
                return Self::render_text_message(
                    f,
                    inside,
                    "Cannot display comments for this item.",
                )
            }
        };

        // No comments case
        if self.viewed_item_kids.is_empty() {
            return Self::render_text_message(f, inside, "No comments yet.");
        }

        // General case
        let viewed_item_comments =
            if let Some(cached_comments) = ctx.get_state().get_currently_viewed_item_comments() {
                cached_comments
            } else {
                return Ok(());
            };

        let viewed_comment = if let Some(comment_id) = self.widget_state.get_focused_comment_id() {
            viewed_item_comments.get(&comment_id)
        } else {
            None
        };
        let viewed_comment_kids_count = viewed_comment.map_or(0, |comment| {
            comment.kids.as_ref().map_or(0, |kids| kids.len())
        });
        let widget = ItemCommentsWidget::with_comments(
            self.viewed_item_id,
            &self.viewed_item_kids,
            viewed_item_comments,
            viewed_comment_kids_count,
            &self.widget_state,
        );
        f.render_widget(widget, inside);

        Ok(())
    }
}

impl ItemComments {
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
