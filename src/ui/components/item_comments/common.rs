use std::io::Stdout;

use tui::{
    backend::CrosstermBackend,
    layout::{Alignment, Rect},
    style::Style,
    text::Spans,
    widgets::{Block, BorderType, Borders, Paragraph},
};

use crate::{
    app::{state::AppState, AppContext},
    errors::Result,
    ui::{
        common::UiTickScalar,
        components::common::{render_text_message, COMMON_BLOCK_NORMAL_COLOR},
        displayable_item::DisplayableHackerNewsItem,
        utils::debouncer::Debouncer,
    },
};

use super::comment_widget::{ItemCommentsWidget, ItemCommentsWidgetState};

/// Common (meta-)data between top and nested components.
#[derive(Debug)]
pub struct ItemCommentsCommon {
    pub ticks_since_last_update: u64,
    pub inputs_debouncer: Debouncer,
    pub loading: bool,
    pub widget_state: ItemCommentsWidgetState,
}

const INPUTS_DEBOUNCER_THROTTLING_TIME: UiTickScalar = 5; // approx. 500ms

impl Default for ItemCommentsCommon {
    fn default() -> Self {
        Self {
            ticks_since_last_update: 0,
            inputs_debouncer: Debouncer::new(INPUTS_DEBOUNCER_THROTTLING_TIME),
            loading: true,
            widget_state: ItemCommentsWidgetState::default(),
        }
    }
}

impl ItemCommentsCommon {
    pub(super) fn render<F>(
        &self,
        f: &mut tui::Frame<CrosstermBackend<Stdout>>,
        inside: Rect,
        ctx: &AppContext,
        specific_error_handler: F,
    ) -> Result<()>
    where
        F: FnOnce() -> Option<String>,
    {
        // (Initial) loading case
        if self.loading {
            render_text_message(f, inside, "Loading...");
            return Ok(());
        }

        // Unavailable comments cache case
        let viewed_item_comments =
            if let Some(cached_comments) = ctx.get_state().get_currently_viewed_item_comments() {
                cached_comments
            } else {
                render_text_message(f, inside, "Comments fetching issue. Please retry later.");
                return Ok(());
            };

        // Common error cases
        if ctx
            .get_state()
            .get_currently_viewed_item_comments_chain()
            .is_empty()
        {
            render_text_message(
                f,
                inside,
                "An error has occurred on this thread. Please retry later.",
            );
            return Ok(());
        } else if viewed_item_comments.is_empty() {
            render_text_message(f, inside, "No comments yet.");
            return Ok(());
        }

        // Specific error cases
        if let Some(error_message) = specific_error_handler() {
            render_text_message(f, inside, error_message.as_str());
            return Ok(());
        }

        // Widget rendering
        let widget = ItemCommentsWidget::with_comments(&self.widget_state, viewed_item_comments);
        f.render_widget(widget, inside);

        Ok(())
    }

    /// Try to retrieve a reference to the currently focused comment, if any.
    ///
    /// NB: will panic if some invariants about cached comments do not hold true.
    pub(super) fn get_focused_comment<'a>(
        &self,
        state: &'a AppState,
    ) -> Option<&'a DisplayableHackerNewsItem> {
        let focused_comment_id =
            if let Some(comment_id) = self.widget_state.get_focused_comment_id() {
                comment_id
            } else {
                return None;
            };
        Some(
            state
                .get_currently_viewed_item_comments()
                .expect("comments should be cached in the global state")
                .get(&focused_comment_id)
                .expect("focused comment should be present in the global state"),
        )
    }
}
