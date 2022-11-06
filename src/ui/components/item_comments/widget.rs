use tui::{buffer::Buffer, layout::Rect, widgets::Widget};

use crate::{
    api::types::HnItemIdScalar,
    ui::displayable_item::{DisplayableHackerNewsItem, DisplayableHackerNewsItemComments},
};

use super::comment_widget::CommentWidget;

/// Persistent state of `ItemCommentsWidget`.
#[derive(Debug)]
pub struct ItemCommentsWidgetState {
    /// Number of top-level comments.
    main_comments_count: usize,
    /// Item-level comment offset.
    main_comments_offset: usize,
    /// ID of the currently focused comment.
    focused_comment_id: Option<HnItemIdScalar>,
}

impl Default for ItemCommentsWidgetState {
    fn default() -> Self {
        Self {
            main_comments_count: 0,
            main_comments_offset: 0,
            focused_comment_id: None,
        }
    }
}

impl ItemCommentsWidgetState {
    pub fn update(
        &mut self,
        comments: &DisplayableHackerNewsItemComments,
        parent_item_id: HnItemIdScalar,
        previous_parent_item_id: HnItemIdScalar,
    ) {
        self.main_comments_count = Self::count_main_comments(parent_item_id, comments);
        if parent_item_id == previous_parent_item_id {
            self.reconciliate_focused_comment();
        } else {
            self.main_comments_offset = 0;
            self.focused_comment_id = None;
        }
    }

    pub fn previous_main_comment(&mut self) {
        self.main_comments_offset = if self.main_comments_offset == 0 {
            self.main_comments_count - 1
        } else {
            self.main_comments_offset - 1
        };
    }

    pub fn next_main_comment(&mut self) {
        self.main_comments_offset = (self.main_comments_offset + 1) % self.main_comments_count;
    }

    /// Reconciliate the currently focused main-level comment when replacing
    /// the comments of an already viewed HackerNews item.
    fn reconciliate_focused_comment(&mut self) {
        // TODO:
        self.main_comments_offset = 0;
        self.focused_comment_id = None;
    }

    fn count_main_comments(
        parent_item_id: HnItemIdScalar,
        comments: &DisplayableHackerNewsItemComments,
    ) -> usize {
        comments
            .values()
            .filter(|comment| comment.parent == Some(parent_item_id))
            .count()
    }
}

/// Custom `tui-rs` widget in charge of displaying a HackerNews Item comments.
#[derive(Debug)]
pub struct ItemCommentsWidget<'a> {
    /// Persistent state.
    state: &'a ItemCommentsWidgetState,
    /// HackerNews main descendants of the parent item.
    parent_kids: &'a [HnItemIdScalar],
    /// Top-level comments of the parent item.
    main_comments: Vec<&'a DisplayableHackerNewsItem>,
}

impl<'a> ItemCommentsWidget<'a> {
    pub fn with_comments(
        parent_item_id: HnItemIdScalar,
        parent_kids: &'a [HnItemIdScalar],
        comments: &'a DisplayableHackerNewsItemComments,
        state: &'a ItemCommentsWidgetState,
    ) -> Self {
        let main_comments = comments
            .values()
            .filter(|comment| comment.parent == Some(parent_item_id))
            .collect();
        Self {
            state,
            parent_kids,
            main_comments,
        }
    }
}

impl<'a> Widget for ItemCommentsWidget<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // No comments case
        if self.parent_kids.is_empty() {
            return;
        }

        // General case
        let focused_comment = self
            .main_comments
            .get(self.state.main_comments_offset)
            .expect("ItemCommentsWidget can get the expected comment");
        let focused_comment_widget = CommentWidget::with_comment(focused_comment);
        focused_comment_widget.render(area, buf);
    }
}
