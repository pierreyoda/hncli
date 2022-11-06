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
        displayable_item::{DisplayableHackerNewsItem, DisplayableHackerNewsItemComments},
        handlers::ApplicationAction,
    },
};

use self::widget::{ItemCommentsWidget, ItemCommentsWidgetState};

mod comment_widget;
mod widget;

/// Item comments component.
#[derive(Debug)]
pub struct ItemComments {
    ticks_since_last_update: u64,
    initial_loading: bool,
    loading: bool,
    viewable_comments: bool,
    viewed_item_id: HnItemIdScalar,
    viewed_item_kids: Vec<HnItemIdScalar>,
    previous_viewed_item_id: HnItemIdScalar,
    comments: DisplayableHackerNewsItemComments,
    widget_state: ItemCommentsWidgetState,
}

impl Default for ItemComments {
    fn default() -> Self {
        Self {
            ticks_since_last_update: 0,
            initial_loading: true,
            loading: true,
            viewable_comments: false,
            viewed_item_id: 0,
            viewed_item_kids: vec![],
            previous_viewed_item_id: 0,
            comments: DisplayableHackerNewsItemComments::new(),
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

    fn should_update(&mut self, elapsed_ticks: UiTickScalar, _ctx: &AppContext) -> Result<bool> {
        // TODO: should also update when comments are dirty
        self.ticks_since_last_update += elapsed_ticks;
        Ok(self.initial_loading || self.ticks_since_last_update >= MEAN_TICKS_BETWEEN_UPDATES)
    }

    async fn update(&mut self, client: &mut HnClient, ctx: &mut AppContext) -> Result<()> {
        self.loading = true;
        self.ticks_since_last_update = 0;
        self.viewable_comments = false;
        self.previous_viewed_item_id = self.viewed_item_id;

        let viewed_item = match ctx.get_state().get_currently_viewed_item() {
            Some(item) => item,
            None => return Ok(()),
        };
        self.viewed_item_id = viewed_item.id;
        let viewed_item_kids = match &viewed_item.kids {
            Some(kids) => kids,
            None => return Ok(()),
        };
        self.viewed_item_kids = viewed_item_kids.clone();

        let comments_raw = client.get_item_comments(viewed_item_kids).await?;
        self.comments = DisplayableHackerNewsItem::transform_comments(comments_raw)?;
        self.viewable_comments = true;
        self.initial_loading = false;
        self.loading = false;

        // Widget state
        self.widget_state.update(
            &self.comments,
            self.viewed_item_id,
            self.previous_viewed_item_id,
            &self.viewed_item_kids,
        );

        Ok(())
    }

    fn handle_inputs(&mut self, ctx: &mut AppContext) -> Result<bool> {
        if self.initial_loading || self.loading {
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

        // No comments available case
        if !self.viewable_comments {
            return Self::render_text_message(f, inside, "No comments available.");
        }

        // Invalid viewed item case
        let viewed_item = if let Some(item) = ctx.get_state().get_currently_viewed_item() {
            item
        } else {
            return Self::render_text_message(f, inside, "Cannot display comments for this item.");
        };

        // No comments case
        let viewed_item_kids = if let Some(kids) = &viewed_item.kids {
            kids.as_slice()
        } else {
            return Self::render_text_message(f, inside, "No comments yet.");
        };

        // General case
        let widget = ItemCommentsWidget::with_comments(
            &self.viewed_item_kids,
            &self.comments,
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
