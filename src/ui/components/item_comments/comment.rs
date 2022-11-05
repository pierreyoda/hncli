use std::io::Stdout;

use tui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::Spans,
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};

use crate::ui::{
    components::common::COMMON_BLOCK_NORMAL_COLOR, displayable_item::DisplayableHackerNewsItem,
    utils::html_to_plain_text,
};

/// Renders a HackerNews item comment.
pub fn render_item_comment(
    f: &mut Frame<CrosstermBackend<Stdout>>,
    in_rect: Rect,
    comment: &DisplayableHackerNewsItem,
    comment_corpus: String,
) {
    if !comment.is_comment {
        // sanity check
        return;
    }
    let text = if let Some(t) = &comment.text {
        t
    } else {
        return;
    };

    // Main layout
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Percentage(10), Constraint::Percentage(90)])
        .split(in_rect);

    // Header
    let header_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![Constraint::Percentage(80), Constraint::Percentage(20)])
        .split(chunks[0]);

    let header_by_text = vec![Spans::from(comment.by_username.clone())];
    let header_by_paragraph = Paragraph::new(header_by_text)
        .alignment(Alignment::Left)
        .style(Style::default().fg(Color::LightBlue));
    f.render_widget(header_by_paragraph, header_chunks[0]);

    let header_posted_since_text = vec![Spans::from(comment.posted_since.clone())];
    let header_posted_since_paragraph =
        Paragraph::new(header_posted_since_text).alignment(Alignment::Right);
    f.render_widget(header_posted_since_paragraph, header_chunks[1]);

    // Corpus
    let corpus = html_to_plain_text(&text, chunks[1].width as usize);
    let corpus_text = vec![Spans::from(corpus)];
    let corpus_paragraph = Paragraph::new(corpus_text)
        .alignment(Alignment::Left)
        .block(
            Block::default()
                .style(Style::default().fg(COMMON_BLOCK_NORMAL_COLOR))
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        );
    f.render_widget(corpus_paragraph, chunks[1]);
}
