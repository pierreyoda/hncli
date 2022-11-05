use tui::{buffer::Buffer, layout::Rect, widgets::Widget};

use crate::{api::types::HnItemIdScalar, ui::displayable_item::DisplayableHackerNewsItemComments};

use super::comment_widget::CommentWidget;

/// Custom `tui-rs` widget in charge of displaying a HackerNews Item comments.
#[derive(Debug)]
pub struct ItemCommentsWidget<'a> {
    /// HackerNews ID of the parent item for the displayed comments.
    parent_id: HnItemIdScalar,
    /// Item-level comments count.
    main_comments_count: usize,
    /// Item-level comment offset.
    main_comments_offset: usize,
    /// ID of the currently focused comment.
    focused_comment_id: Option<HnItemIdScalar>,
    /// Comments to be displayed.
    comments: &'a DisplayableHackerNewsItemComments,
}

// TODO: foldable sub-comments
impl<'a> ItemCommentsWidget<'a> {
    pub fn with_comments(
        parent_item_id: HnItemIdScalar,
        previous_parent_item_id: HnItemIdScalar,
        comments: &'a DisplayableHackerNewsItemComments,
    ) -> Self {
        // Comments handling
        let main_comments_count = Self::count_main_comments(parent_item_id, comments);

        // Reconciliation, if necessary
        // if parent_item_id == previous_parent_item_id {
        //     self.reconciliate_focused_comment();
        // } else {
        //     self.main_comments_offset = 0;
        // }

        // TODO: reconciliation of focused comment if necessary
        Self {
            parent_id: parent_item_id,
            main_comments_count,
            main_comments_offset: 0,
            focused_comment_id: None,
            comments,
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

impl<'a> Widget for ItemCommentsWidget<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // Comments
        let test_comment = self.comments.get(&33484668).unwrap();
        let test_comment_widget = CommentWidget::with_comment(test_comment);
        test_comment_widget.render(area, buf);
    }
}
