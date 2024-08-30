use log::warn;
use ratatui::{
    buffer::Buffer,
    layout::{Margin, Rect},
    style::{Color, Style},
    widgets::Widget,
};
use unicode_width::UnicodeWidthStr;

use crate::{api::types::HnItemIdScalar, ui::displayable_item::DisplayableHackerNewsItemComments};

use super::corpus_widget::CommentWidget;

/// Persistent state of `ItemCommentsWidget`.
#[derive(Debug, Default)]
pub struct ItemCommentsWidgetState {
    /// ID of the currently focused comment.
    focused_comment_id: Option<HnItemIdScalar>,
    /// Index of the currently focused comment, among the parent item's kids (starts at 0).
    focused_comment_index: Option<usize>,
    /// Number of same-level comments currently being offered for display.
    focused_same_level_comments_count: usize,
    /// If Some, prepare to restore from history navigation state the focused comment.
    history_should_focus_comment_id: Option<HnItemIdScalar>,
}

// TODO: unit tests harness (create new state.rs sub-module)
impl ItemCommentsWidgetState {
    pub fn update(
        &mut self,
        comments: &DisplayableHackerNewsItemComments,
        parent_item_kids: &[HnItemIdScalar],
    ) {
        self.reconciliate_focused_comment(comments, parent_item_kids);
    }

    pub fn previous_main_comment(
        &mut self,
        parent_item_kids: &[HnItemIdScalar],
    ) -> Option<HnItemIdScalar> {
        self.focused_same_level_comments_count = parent_item_kids.len();
        if let Some(focused_id) = self.focused_comment_id {
            let focused_index = parent_item_kids
                .iter()
                .position(|id| id == &focused_id)
                .unwrap_or(0);
            let previous_index = if focused_index == 0 {
                parent_item_kids.len() - 1
            } else {
                focused_index - 1
            };
            self.focused_comment_index = Some(previous_index);
            self.focused_comment_id = Some(parent_item_kids[previous_index]);
            self.focused_comment_id
        } else {
            self.reset_focused_comment(parent_item_kids)
        }
    }

    pub fn next_main_comment(
        &mut self,
        parent_item_kids: &[HnItemIdScalar],
    ) -> Option<HnItemIdScalar> {
        self.focused_same_level_comments_count = parent_item_kids.len();
        if let Some(focused_id) = self.focused_comment_id {
            let focused_index = parent_item_kids
                .iter()
                .position(|id| id == &focused_id)
                .unwrap_or(0);
            let next_index = (focused_index + 1) % parent_item_kids.len();
            self.focused_comment_index = Some(next_index);
            self.focused_comment_id = Some(parent_item_kids[next_index]);
            self.focused_comment_id
        } else {
            self.reset_focused_comment(parent_item_kids)
        }
    }

    pub fn get_focused_comment_id(&self) -> Option<HnItemIdScalar> {
        self.focused_comment_id
    }

    pub fn restore_focused_comment_id(
        &mut self,
        comment_id: HnItemIdScalar,
        parent_item_kids: &[HnItemIdScalar],
    ) {
        self.focused_same_level_comments_count = parent_item_kids.len();
        let comment_index = parent_item_kids.iter().position(|id| id == &comment_id);
        if let Some(index) = comment_index {
            self.focused_comment_id = Some(comment_id);
            self.focused_comment_index = Some(index);
        }
    }

    pub fn history_prepare_focus_on_comment_id(
        &mut self,
        history_focused_comment_id: HnItemIdScalar,
    ) {
        self.history_should_focus_comment_id = Some(history_focused_comment_id);
    }

    pub fn get_focused_same_level_comments_count(&self) -> usize {
        self.focused_same_level_comments_count
    }

    /// Reconciliate the currently focused main-level comment when replacing
    /// the comments of a currently viewed HackerNews item.
    ///
    /// Takes into account the past navigation history, if any pending.
    fn reconciliate_focused_comment(
        &mut self,
        comments: &DisplayableHackerNewsItemComments,
        parent_item_kids: &[HnItemIdScalar],
    ) {
        // navigation history handling
        if let Some(history_focused_comment_id) = self.history_should_focus_comment_id {
            if comments.contains_key(&history_focused_comment_id) {
                self.focused_comment_id = Some(history_focused_comment_id);
            } else {
                warn!("ItemCommentsWidgetState.reconciliate_focused_comment: could not find comment ID: {}", history_focused_comment_id);
            }
            self.history_should_focus_comment_id = None;
        }

        match self.focused_comment_id {
            Some(comment_id) if comments.contains_key(&comment_id) => {
                self.focused_same_level_comments_count = parent_item_kids.len();
            }
            _ => {
                self.reset_focused_comment(parent_item_kids);
            }
        }
    }

    /// Reset the currently focused main-level comment to the first possible
    /// main-level comment, if any.
    fn reset_focused_comment(
        &mut self,
        parent_item_kids: &[HnItemIdScalar],
    ) -> Option<HnItemIdScalar> {
        self.focused_same_level_comments_count = parent_item_kids.len();
        let first_comment_id = parent_item_kids.first();
        self.focused_comment_index = if first_comment_id.is_some() {
            Some(0)
        } else {
            None
        };
        self.focused_comment_id = first_comment_id.cloned();
        self.focused_comment_id
    }
}

/// Custom `tui-rs` widget in charge of displaying a HackerNews Item comments.
#[derive(Debug)]
pub struct ItemCommentsWidget<'a> {
    /// Persistent state.
    state: &'a ItemCommentsWidgetState,
    /// Comments of the top-level parent item.
    comments: &'a DisplayableHackerNewsItemComments,
}

impl<'a> ItemCommentsWidget<'a> {
    pub fn with_comments(
        state: &'a ItemCommentsWidgetState,
        comments: &'a DisplayableHackerNewsItemComments,
    ) -> Self {
        Self { state, comments }
    }
}

pub const PADDING: u16 = 2;
pub const FOOTER_HEIGHT: u16 = 1;

impl<'a> Widget for ItemCommentsWidget<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // No focused comment case
        let focused_comment_id = if let Some(comment_id) = &self.state.focused_comment_id {
            comment_id
        } else {
            return;
        };

        // Current comment
        let focused_comment = if let Some(comment) = self.comments.get(focused_comment_id) {
            comment
        } else {
            let prompt = "Error while displaying the comment.";
            buf.set_string(
                (area.right() - area.left()) / 2 - prompt.len() as u16 / 2,
                (area.bottom() - area.top()) / 2,
                prompt,
                Style::default().fg(Color::LightRed),
            );
            return;
        };

        // Comment rendering
        let focused_comment_widget = CommentWidget::with_comment(focused_comment);
        focused_comment_widget.render(
            area.inner(Margin {
                vertical: PADDING,
                horizontal: PADDING,
            }),
            buf,
        );

        // Footer
        let focused_comment_kids_count = focused_comment.kids.as_ref().map_or(0, |kids| kids.len());
        let focused_comment_index = if let Some(index) = self.state.focused_comment_index {
            index
        } else {
            return;
        };
        let footer_area = Rect::new(
            area.left(),
            area.bottom() - FOOTER_HEIGHT - PADDING,
            area.width,
            FOOTER_HEIGHT,
        );
        let footer_text = if focused_comment_kids_count > 0 {
            format!(
                "Comment {} / {} | {} sub-comment{}",
                focused_comment_index + 1,
                self.state.focused_same_level_comments_count,
                focused_comment_kids_count,
                if focused_comment_kids_count > 1 {
                    "s"
                } else {
                    ""
                },
            )
        } else {
            format!(
                "Comment {} / {}",
                focused_comment_index + 1,
                self.state.focused_same_level_comments_count
            )
        };
        buf.set_string(
            (footer_area.right() - footer_area.left()) / 2 - footer_text.width() as u16 / 2,
            footer_area.y,
            footer_text,
            Style::default().fg(Color::LightBlue),
        );
    }
}

// TODO: unit tests for the widget state would be so nice, but would require some manually created Hacker News comments of a whole thread
