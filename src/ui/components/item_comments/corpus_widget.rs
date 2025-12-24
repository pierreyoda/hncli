use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    widgets::Widget,
};

use crate::ui::{
    displayable_item::DisplayableHackerNewsItem, theme::UiTheme, utils::html_to_plain_text,
};

#[derive(Debug)]
pub struct CommentWidget<'a> {
    theme: &'a UiTheme,
    comment: &'a DisplayableHackerNewsItem,
}

impl<'a> CommentWidget<'a> {
    pub fn with_comment(theme: &'a UiTheme, comment: &'a DisplayableHackerNewsItem) -> Self {
        assert!(comment.is_comment);
        Self { theme, comment }
    }
}

pub const PADDING: u16 = 3;
pub const HEADER_HEIGHT: u16 = 5;

impl<'a> Widget for CommentWidget<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // Header
        let header_area = Rect::new(area.left(), area.top() + PADDING, area.width, HEADER_HEIGHT);
        // -> username
        buf.set_string(
            header_area.x + PADDING,
            header_area.y,
            &self.comment.by_username,
            Style::default().fg(self.theme.get_main_color()),
        );
        // -> posted since
        buf.set_string(
            header_area.right() - self.comment.posted_since.len() as u16 - PADDING,
            header_area.y,
            &self.comment.posted_since,
            Style::default().fg(self.theme.get_block_color()),
        );

        // Corpus
        let corpus_str = if let Some(text) = &self.comment.text {
            &text
        } else {
            ""
        };
        let corpus = html_to_plain_text(corpus_str, area.width as usize).unwrap();
        let corpus_lines = corpus.lines();

        let corpus_area = Rect::new(
            area.left(),
            header_area.bottom() + HEADER_HEIGHT,
            area.width - PADDING * 2,
            80,
        );
        for (i, corpus_line) in corpus_lines.enumerate() {
            buf.set_string(
                PADDING * 2,
                corpus_area.top() + i as u16,
                corpus_line,
                Style::default().fg(Color::White),
            );
        }
    }
}
