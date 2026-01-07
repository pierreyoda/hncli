use std::convert::TryInto;

use async_trait::async_trait;
use ratatui::{
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Tabs},
};

use crate::{
    api::{HnClient, client::HnStoriesSorting},
    app::AppContext,
    errors::{HnCliError, Result},
    ui::{
        common::{RenderFrame, UiComponent, UiComponentId, UiTickScalar},
        handlers::ApplicationAction,
        utils::debouncer::Debouncer,
    },
};

#[derive(Clone, Debug, PartialEq, Eq)]
enum HomeSortingOptions {
    Newest,
    Top,
    Best,
}

impl HomeSortingOptions {
    fn get_label(&self) -> &str {
        match self {
            HomeSortingOptions::Newest => "New",
            HomeSortingOptions::Top => "Top",
            HomeSortingOptions::Best => "Best",
        }
    }
}

impl TryInto<HnStoriesSorting> for HomeSortingOptions {
    type Error = HnCliError;

    fn try_into(self) -> Result<HnStoriesSorting> {
        Ok(match self {
            HomeSortingOptions::Newest => HnStoriesSorting::New,
            HomeSortingOptions::Top => HnStoriesSorting::Top,
            HomeSortingOptions::Best => HnStoriesSorting::Best,
        })
    }
}

const SORTING_OPTIONS_LIST: [HomeSortingOptions; 3] = [
    HomeSortingOptions::Newest,
    HomeSortingOptions::Top,
    HomeSortingOptions::Best,
];

/// The Options component provides context-dependent options
/// for the current active component.
#[derive(Debug)]
pub struct Options {
    /// Reset when pressing another key.
    keyboard_debouncer: Debouncer,
    /// Index of the currently selected sorting option for
    /// items sorting.
    selected_sorting_index: usize,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            keyboard_debouncer: Debouncer::new(10), // approx. 1000ms
            // TODO: load from configuration
            selected_sorting_index: 1,
        }
    }
}

pub const OPTIONS_ID: UiComponentId = "options";

#[async_trait]
impl UiComponent for Options {
    fn id(&self) -> UiComponentId {
        OPTIONS_ID
    }

    async fn should_update(
        &mut self,
        elapsed_ticks: UiTickScalar,
        _ctx: &AppContext,
    ) -> Result<bool> {
        self.keyboard_debouncer.tick(elapsed_ticks);

        Ok(false)
    }

    async fn update(&mut self, _client: &mut HnClient, _ctx: &mut AppContext) -> Result<()> {
        Ok(())
    }

    async fn handle_inputs(&mut self, ctx: &mut AppContext) -> Result<bool> {
        if ctx
            .get_inputs()
            .is_active(&ApplicationAction::HomeToggleSortingOption)
        {
            if !self.keyboard_debouncer.is_action_allowed() {
                return Ok(false);
            }
            self.selected_sorting_index =
                (self.selected_sorting_index + 1) % SORTING_OPTIONS_LIST.len();
            let sorting_type = SORTING_OPTIONS_LIST[self.selected_sorting_index]
                .clone()
                .try_into()?;
            ctx.get_state_mut().set_main_stories_sorting(sorting_type);
            Ok(true)
        } else {
            self.keyboard_debouncer.reset();
            Ok(false)
        }
    }

    fn render(&mut self, f: &mut RenderFrame, inside: Rect, ctx: &AppContext) -> Result<()> {
        let theme = ctx.get_theme();

        let block = Block::default()
            .style(Style::default().fg(theme.get_block_color()))
            .border_type(BorderType::Thick)
            .borders(Borders::ALL)
            .title("Options (S to toggle sorting)");

        let tabs_titles: Vec<Line> = SORTING_OPTIONS_LIST
            .iter()
            .enumerate()
            .map(|(i, sorting_option)| {
                Line::from(Span::styled(
                    sorting_option.get_label(),
                    Style::default().fg(if i == self.selected_sorting_index {
                        theme.get_main_color()
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
            .highlight_style(Style::default().fg(theme.get_accent_color()))
            .divider(Span::raw("|"))
            .block(block);

        f.render_widget(tabs, inside);

        Ok(())
    }
}
