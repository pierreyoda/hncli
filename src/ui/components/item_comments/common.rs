use std::io::Stdout;

use tui::{
    backend::CrosstermBackend,
    layout::{Alignment, Rect},
    style::Style,
    text::Spans,
    widgets::{Block, BorderType, Borders, Paragraph},
};

use crate::{
    app::AppContext,
    errors::Result,
    ui::{
        common::UiTickScalar, components::common::COMMON_BLOCK_NORMAL_COLOR,
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
            Self::render_text_message(f, inside, "Loading...");
            return Ok(());
        }

        // Unavailable comments cache case
        let viewed_item_comments = if let Some(cached_comments) =
            ctx.get_state().get_currently_viewed_item_comments()
        {
            cached_comments
        } else {
            Self::render_text_message(f, inside, "Comments fetching issue. Please retry later.");
            return Ok(());
        };

        // Common error cases
        if ctx
            .get_state()
            .get_currently_viewed_item_comments_chain()
            .is_empty()
        {
            Self::render_text_message(
                f,
                inside,
                "An error has occurred on this thread. Please retry later.",
            );
            return Ok(());
        } else if viewed_item_comments.is_empty() {
            Self::render_text_message(f, inside, "No comments yet.");
            return Ok(());
        }

        // Specific error cases
        if let Some(error_message) = specific_error_handler() {
            Self::render_text_message(f, inside, error_message.as_str());
            return Ok(());
        }

        // Widget rendering
        let widget = ItemCommentsWidget::with_comments(&self.widget_state, viewed_item_comments);
        f.render_widget(widget, inside);

        return Ok(());
    }

    fn render_text_message(
        f: &mut tui::Frame<CrosstermBackend<Stdout>>,
        inside: Rect,
        message: &str,
    ) {
        let block = Block::default()
            .style(Style::default().fg(COMMON_BLOCK_NORMAL_COLOR))
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded);

        let text = vec![Spans::from(""), Spans::from(message.to_string())];
        let paragraph = Paragraph::new(text)
            .block(block)
            .alignment(Alignment::Center);
        f.render_widget(paragraph, inside);
    }
}
