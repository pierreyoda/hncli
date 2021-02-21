use std::{convert::TryInto, io::Stdout};

use async_trait::async_trait;
use tui::{
    backend::CrosstermBackend,
    layout::Rect,
    style::{Color, Style},
    text::{Span, Spans},
    widgets::{Block, BorderType, Borders, Tabs},
    Frame,
};

use crate::{
    api::{HnClient, HnStoriesSorting},
    app::App,
    errors::{HnCliError, Result},
    ui::{
        common::{UiComponent, UiComponentId, UiTickScalar},
        handlers::Key,
    },
};

#[derive(Debug, PartialEq, Eq)]
enum HomeOptions {
    SortNewest,
    SortTop,
    SortBest,
}

impl HomeOptions {
    fn get_label(&self) -> &str {
        match self {
            HomeOptions::SortNewest => "New",
            HomeOptions::SortTop => "Top",
            HomeOptions::SortBest => "Best",
        }
    }
}

impl TryInto<HnStoriesSorting> for HomeOptions {
    type Error = HnCliError;

    fn try_into(self) -> Result<HnStoriesSorting> {
        Ok(match self {
            HomeOptions::SortNewest => HnStoriesSorting::New,
            HomeOptions::SortTop => HnStoriesSorting::Top,
            HomeOptions::SortBest => HnStoriesSorting::Best,
        })
    }
}

const SORTING_OPTIONS_LIST: [HomeOptions; 3] = [
    HomeOptions::SortNewest,
    HomeOptions::SortTop,
    HomeOptions::SortBest,
];

/// The Options component provides context-dependent options
/// for the current active component.
#[derive(Debug)]
pub struct Options {
    selected_sorting_index: usize,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            // TODO: load from configuration
            selected_sorting_index: 0,
        }
    }
}

pub const OPTIONS_ID: UiComponentId = "options";

#[async_trait]
impl UiComponent for Options {
    fn id(&self) -> UiComponentId {
        OPTIONS_ID
    }

    fn should_update(&mut self, _elapsed_ticks: UiTickScalar, _app: &App) -> Result<bool> {
        Ok(false)
    }

    async fn update(&mut self, _client: &mut HnClient, _app: &mut App) -> Result<()> {
        Ok(())
    }

    fn key_handler(&mut self, key: &Key, app: &mut App) -> Result<bool> {
        Ok(match key {
            Key::Escape | Key::Enter | Key::Char('q') => {
                app.pop_navigation_stack();
                true
            }
            _ => false,
        })
    }

    fn render(
        &self,
        f: &mut Frame<CrosstermBackend<Stdout>>,
        inside: Rect,
        _app: &App,
    ) -> Result<()> {
        let block = Block::default()
            .border_type(BorderType::Thick)
            .borders(Borders::ALL)
            .title("Options (S to toggle sorting)");

        let tabs_titles: Vec<Spans> = SORTING_OPTIONS_LIST
            .iter()
            .enumerate()
            .map(|(i, sorting_option)| {
                Spans::from(Span::styled(
                    sorting_option.get_label(),
                    Style::default().fg(if i == self.selected_sorting_index {
                        Color::Yellow
                    } else {
                        Color::White
                    }),
                ))
            })
            .collect();

        // TODO: this probably needs a custom widget
        let tabs = Tabs::new(tabs_titles)
            .select(self.selected_sorting_index)
            .style(Style::default().fg(Color::White))
            .highlight_style(Style::default().fg(Color::Yellow))
            .divider(Span::raw("|"))
            .block(block);

        f.render_widget(tabs, inside);

        Ok(())
    }
}