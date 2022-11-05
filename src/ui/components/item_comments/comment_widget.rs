use tui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    text::{Span, Spans},
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
        let corpus_spans: Vec<Span> = corpus_lines
            .map(|corpus_line| {
                let line = format!("{}\n", corpus_line);
                Span::styled(line, Style::default().fg(Color::White))
            })
            .collect();

        let corpus_area = Rect::new(
            area.left(),
            header_area.bottom() + HEADER_HEIGHT,
            area.width,
            Self::compute_corpus_height(corpus_spans.len()),
        );
        buf.set_spans(
            corpus_area.left(),
            corpus_area.top(),
            &Spans(corpus_spans),
            corpus_area.width,
        );
    }
}

impl<'a> CommentWidget<'a> {
    fn compute_corpus_height(corpus_lines_count: usize) -> u16 {
        corpus_lines_count as u16 / 10
    }
}
