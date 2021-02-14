use std::io::Stdout;

use layout::{Constraint, Direction, Layout};
use tui::{
    backend::CrosstermBackend,
    layout::{self, Rect},
    Frame,
};

use super::components::stories::DisplayableHackerNewsStory;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MainScreenPanels {
    StoriesPanel,
    DetailsPanel,
}

pub fn render_home_screen(
    f: &mut Frame<CrosstermBackend<Stdout>>,
    in_rect: Rect,
    ranked_stories: &[DisplayableHackerNewsStory],
) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(25), Constraint::Percentage(75)].as_ref())
        .split(in_rect);

    // render_stories_panel(f, chunks[0], ranked_stories, None)
}

#[derive(Clone, Debug)]
pub enum UserInterfaceScreen {
    Home = 0,
    AskHackerNews = 1,
    ShowHackerNews = 2,
    Jobs = 3,
}

impl From<UserInterfaceScreen> for usize {
    fn from(value: UserInterfaceScreen) -> Self {
        value as usize
    }
}
