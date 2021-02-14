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

/// A display-ready Hacker News story.
#[derive(Clone, Debug)]
pub struct DisplayableHackerNewsStory {
    /// Unique ID.
    id: HnItemIdScalar,
    /// Posted at.
    posted_at: DateTime<Utc>,
    /// Username of the poster.
    by_username: String,
    /// Title.
    title: String,
    /// Score.
    score: u32,
    /// Hostname of the URL, if any.
    url_hostname: Option<String>,
}

impl TryFrom<HnStory> for DisplayableHackerNewsStory {
    type Error = HnCliError;

    fn try_from(value: HnStory) -> Result<Self, Self::Error> {
        Ok(Self {
            id: value.id,
            posted_at: datetime_from_hn_time(value.time),
            by_username: value.by,
            title: value.title,
            score: value.score,
            url_hostname: value.url.map(|url| {
                url::Url::parse(&url[..])
                    .map_err(HnCliError::UrlParsingError)
                    .expect("URL parsing error") // TODO: improve this
                    .host_str()
                    .expect("URL must have an hostname")
                    .to_owned()
            }),
        })
    }
}

/// Renders a panel of *selectable* Hacker News stories.
pub fn render_stories_panel(
    f: &mut Frame<CrosstermBackend<Stdout>>,
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
