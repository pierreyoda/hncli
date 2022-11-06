use tui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    text::Span,
    widgets::Widget,
};

use crate::ui::{displayable_item::DisplayableHackerNewsItem, utils::html_to_plain_text};

#[derive(Debug)]
pub struct CommentWidget<'a> {
    comment: &'a DisplayableHackerNewsItem,
}

impl<'a> CommentWidget<'a> {
    pub fn with_comment(comment: &'a DisplayableHackerNewsItem) -> Self {
        assert!(comment.is_comment);
        Self { comment }
    }
}

pub const PADDING_TOP: u16 = 1;
pub const HEADER_HEIGHT: u16 = 5;

impl<'a> Widget for CommentWidget<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // Header
        let header_area = Rect::new(
            area.left(),
            area.top() + PADDING_TOP,
            area.width,
            HEADER_HEIGHT,
        );
        // -> username
        buf.set_span(
            header_area.x,
            header_area.y,
            &Span::styled(
                self.comment.by_username.clone(),
                Style::default().fg(Color::LightGreen),
            ),
            header_area.width / 2,
        );
        // -> score
        buf.set_string(
            header_area.right() - 20,
            header_area.y,
            format!("Score: {}", self.comment.score),
            Style::default().fg(Color::LightYellow),
        );
        // -> posted since
        buf.set_span(
            (header_area.right() - header_area.left()) / 2,
            header_area.y,
            &Span::styled(
                self.comment.posted_since.clone(),
                Style::default().fg(Color::Gray),
            ),
            header_area.width / 2,
        );

        // Corpus
        let corpus_str = if let Some(text) = &self.comment.text {
            text.as_str()
        } else {
            ""
        };
        let corpus = html_to_plain_text(corpus_str, area.width as usize);
        let corpus_lines = corpus.lines();

        let corpus_area = Rect::new(
            area.left(),
            header_area.bottom() + HEADER_HEIGHT,
            area.width,
            80,
        );
        for (i, corpus_line) in corpus_lines.enumerate() {
            buf.set_string(
                0,
                corpus_area.top() + i as u16,
                corpus_line,
                Style::default().fg(Color::White),
            );
        }
    }
}
