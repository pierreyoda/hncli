use tui::{
    buffer::Buffer,
    layout::{Margin, Rect},
    style::{Color, Style},
    widgets::Widget,
};

use crate::{api::types::HnItemIdScalar, ui::displayable_item::DisplayableHackerNewsItemComments};

use super::comment_widget::CommentWidget;

/// Persistent state of `ItemCommentsWidget`.
#[derive(Debug, Default)]
pub struct ItemCommentsWidgetState {
    /// ID of the currently focused comment.
    focused_comment_id: Option<HnItemIdScalar>,
    /// Index of the currently focused comment, among the parent item's kids (starts at 0).
    focused_comment_index: Option<usize>,
}

impl ItemCommentsWidgetState {
    pub fn update(
        &mut self,
        comments: &DisplayableHackerNewsItemComments,
        parent_item_id: HnItemIdScalar,
        previous_parent_item_id: HnItemIdScalar,
        parent_item_kids: &[HnItemIdScalar],
    ) {
        if parent_item_id == previous_parent_item_id {
            self.reconciliate_focused_comment(comments, parent_item_kids);
        } else {
            self.reset_focused_comment(parent_item_kids);
        }
    }

    pub fn previous_main_comment(&mut self, parent_item_kids: &[HnItemIdScalar]) {
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
        } else {
            self.reset_focused_comment(parent_item_kids);
        }
    }

    pub fn next_main_comment(&mut self, parent_item_kids: &[HnItemIdScalar]) {
        if let Some(focused_id) = self.focused_comment_id {
            let focused_index = parent_item_kids
                .iter()
                .position(|id| id == &focused_id)
                .unwrap_or(0);
            let next_index = (focused_index + 1) % parent_item_kids.len();
            self.focused_comment_index = Some(next_index);
            self.focused_comment_id = Some(parent_item_kids[next_index]);
        } else {
            self.reset_focused_comment(parent_item_kids);
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
        let comment_index = parent_item_kids.iter().position(|id| id == &comment_id);
        if let Some(index) = comment_index {
            self.focused_comment_id = Some(comment_id);
            self.focused_comment_index = Some(index);
        }
    }

    /// Reconciliate the currently focused main-level comment when replacing
    /// the comments of an already viewed HackerNews item.
    fn reconciliate_focused_comment(
        &mut self,
        comments: &DisplayableHackerNewsItemComments,
        parent_item_kids: &[HnItemIdScalar],
    ) {
        match self.focused_comment_id {
            Some(comment_id) if comments.contains_key(&comment_id) => (),
            _ => self.reset_focused_comment(parent_item_kids),
        }
    }

    /// Reset the currently focused main-level comment to the first possible
    /// main-level comment, if any.
    fn reset_focused_comment(&mut self, parent_item_kids: &[HnItemIdScalar]) {
        let first_comment = parent_item_kids.first();
        self.focused_comment_index = if first_comment.is_some() {
            Some(0)
        } else {
            None
        };
        self.focused_comment_id = first_comment.cloned();
    }
}

/// Custom `tui-rs` widget in charge of displaying a HackerNews Item comments.
#[derive(Debug)]
pub struct ItemCommentsWidget<'a> {
    /// Persistent state.
    state: &'a ItemCommentsWidgetState,
    /// HackerNews main descendants of the parent item.
    parent_kids: &'a [HnItemIdScalar],
    /// Comments of the top-level parent item.
    comments: &'a DisplayableHackerNewsItemComments,
    /// Number of main-level comments.
    main_comments_count: usize,
    /// Number of sub-level comments.
    sub_comments_count: usize,
}

impl<'a> ItemCommentsWidget<'a> {
    pub fn with_comments(
        parent_id: HnItemIdScalar,
        parent_kids: &'a [HnItemIdScalar],
        comments: &'a DisplayableHackerNewsItemComments,
        sub_comments_count: usize,
        state: &'a ItemCommentsWidgetState,
    ) -> Self {
        Self {
            state,
            comments,
            parent_kids,
            sub_comments_count,
            main_comments_count: comments
                .values()
                .filter(|comment| comment.parent == Some(parent_id))
                .count(),
        }
    }
}

pub const PADDING: u16 = 2;
pub const FOOTER_HEIGHT: u16 = 5;

impl<'a> Widget for ItemCommentsWidget<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // No comments case
        if self.parent_kids.is_empty() {
            return;
        }

        // No focused comment case
        let focused_comment_id = if let Some(comment_id) = &self.state.focused_comment_id {
            comment_id
        } else {
            return;
        };

        // Retrieve comment
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
            area.inner(&Margin {
                vertical: PADDING,
                horizontal: PADDING,
            }),
            buf,
        );

        // Footer
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
        let footer_text = if self.sub_comments_count > 0 {
            format!(
                "Comment {} / {} | {} sub-comments",
                focused_comment_index + 1,
                self.main_comments_count,
                self.sub_comments_count
            )
        } else {
            format!(
                "Comment {} / {}",
                focused_comment_index + 1,
                self.main_comments_count
            )
        };
        buf.set_string(
            (footer_area.right() - footer_area.left()) / 2 - footer_text.len() as u16 / 2,
            footer_area.y,
            footer_text,
            Style::default().fg(Color::LightBlue),
        )
    }
}
