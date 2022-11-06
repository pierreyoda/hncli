use tui::{buffer::Buffer, layout::Rect, widgets::Widget};

use crate::{api::types::HnItemIdScalar, ui::displayable_item::DisplayableHackerNewsItemComments};

use super::comment_widget::CommentWidget;

/// Persistent state of `ItemCommentsWidget`.
#[derive(Debug, Default)]
pub struct ItemCommentsWidgetState {
    /// ID of the currently focused comment.
    focused_comment_id: Option<HnItemIdScalar>,
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
            self.focused_comment_id = Some(parent_item_kids[next_index]);
        } else {
            self.reset_focused_comment(parent_item_kids);
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
        self.focused_comment_id = parent_item_kids.first().cloned();
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
}

impl<'a> ItemCommentsWidget<'a> {
    pub fn with_comments(
        parent_kids: &'a [HnItemIdScalar],
        comments: &'a DisplayableHackerNewsItemComments,
        state: &'a ItemCommentsWidgetState,
    ) -> Self {
        Self {
            state,
            comments,
            parent_kids,
        }
    }
}

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

        // General case
        let focused_comment = self
            .comments
            .get(focused_comment_id)
            .expect("ItemCommentsWidget can get the expected comment");
        let focused_comment_widget = CommentWidget::with_comment(focused_comment);
        focused_comment_widget.render(area, buf);
    }
}
