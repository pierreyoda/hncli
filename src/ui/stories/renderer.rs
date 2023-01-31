use std::{convert::TryFrom, io::Stdout};

use chrono::{DateTime, Utc};
use tui::{
    backend::CrosstermBackend,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, BorderType, Borders, List, ListItem},
    Frame,
};

use crate::{
    api::types::{HnItemIdScalar, HnStory},
    errors::HnCliError,
};

use super::utils::datetime_from_hn_time;

const HOME_MAX_DISPLAYED_STORIES: usize = 20;

/// Renders a panel of *selectable* Hacker News stories.
pub fn render_stories_panel(
    f: &mut RenderFrame,
    in_rect: Rect,
    ranked_stories: &[DisplayableHackerNewsStory],
    selected_story_id: Option<HnItemIdScalar>,
) {
    // Data
    let stories: Vec<&DisplayableHackerNewsStory> = ranked_stories
        .iter()
        .take(HOME_MAX_DISPLAYED_STORIES)
        .collect();

    // Layout
    let layout_stories_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::White))
        .title("Stories")
        .border_type(BorderType::Plain);

    // Stories list
    let list_stories_items: Vec<ListItem> = stories
        .iter()
        .map(|story| {
            ListItem::new(Spans::from(vec![Span::styled(
                story.title.clone(),
                Style::default(),
            )]))
        })
        .collect();

    let list_stories = List::new(list_stories_items)
        .block(layout_stories_block)
        .highlight_style(
            Style::default()
                .bg(Color::Yellow)
                .fg(Color::Black)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(">> ");

    f.render_widget(list_stories, in_rect)
}
