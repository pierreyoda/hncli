use std::io::Stdout;

use async_trait::async_trait;

use handlers::Key;
use tui::{
    backend::CrosstermBackend,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, BorderType, Borders, Tabs},
    Frame,
};

use crate::{
    api::{HnClient, HnStoriesSections},
    app::AppContext,
    errors::Result,
    ui::{
        common::{UiComponent, UiComponentId, UiTickScalar},
        handlers,
        router::AppRoute,
    },
};

use super::common::COMMON_BLOCK_NORMAL_COLOR;

const TABS_TITLES: [&'static str; 5] = ["Home", "Ask HN", "Show HN", "Jobs", "Help"];

/// The Navigation bar provides a convenient way to switch between screens
/// screens by either pressing the hotkey associated with the title, or by
/// directly switching tabs with the help of the arrow keys.
#[derive(Debug)]
pub struct Navigation {
    titles: Vec<&'static str>,
    selected_index: usize,
}

impl Default for Navigation {
    fn default() -> Self {
        Self {
            titles: TABS_TITLES.to_vec(),
            selected_index: 0,
        }
    }
}

impl Navigation {
    fn next(&mut self) {
        self.selected_index = (self.selected_index + 1) % self.titles.len();
    }

    fn previous(&mut self) {
        if self.selected_index > 0 {
            self.selected_index -= 1;
        } else {
            self.selected_index = self.titles.len() - 1;
        }
    }
}

pub const NAVIGATION_ID: UiComponentId = "navigation";

#[async_trait]
impl UiComponent for Navigation {
    fn id(&self) -> UiComponentId {
        NAVIGATION_ID
    }

    fn should_update(&mut self, _elapsed_ticks: UiTickScalar, _ctx: &AppContext) -> Result<bool> {
        Ok(false)
    }

    async fn update(&mut self, _client: &mut HnClient, _ctx: &mut AppContext) -> Result<()> {
        Ok(())
    }

    fn key_handler(&mut self, key: &Key, ctx: &mut AppContext) -> Result<bool> {
        Ok(match key {
            Key::Left => {
                self.previous();
                true
            }
            Key::Right => {
                self.next();
                true
            }
            Key::Enter => {
                if ctx.get_state().get_latest_interacted_with_component() == Some(&NAVIGATION_ID) {
                    self.navigate_to_current_selection(ctx);
                    true
                } else {
                    false
                }
            }
            _ => false,
        })
    }

    fn render(
        &mut self,
        f: &mut Frame<CrosstermBackend<Stdout>>,
        inside: Rect,
        ctx: &AppContext,
    ) -> Result<()> {
        let current_tab_index = match ctx.get_router().get_current_route() {
            AppRoute::Home(section) => match section {
                HnStoriesSections::Home => 0,
                HnStoriesSections::Ask => 1,
                HnStoriesSections::Show => 2,
                HnStoriesSections::Jobs => 3,
            },
            AppRoute::Help => 4,
            _ => usize::MAX,
        };
        let selected_title = TABS_TITLES[current_tab_index];
        let tabs_titles: Vec<Spans> = self
            .titles
            .iter()
            .map(|title| {
                Spans::from(vec![Span::styled(
                    *title,
                    Style::default()
                        .fg(Color::White)
                        .add_modifier(if *title == selected_title {
                            Modifier::UNDERLINED | Modifier::BOLD
                        } else {
                            Modifier::BOLD
                        }),
                )])
            })
            .collect();

        let tabs = Tabs::new(tabs_titles)
            .select(self.selected_index)
            .block(
                Block::default()
                    .style(Style::default().fg(COMMON_BLOCK_NORMAL_COLOR))
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .title("Menu"),
            )
            .style(Style::default().fg(Color::White))
            .highlight_style(Style::default().fg(Color::Yellow))
            .divider(Span::raw("|"));

        f.render_widget(tabs, inside);

        Ok(())
    }
}

impl Navigation {
    fn navigate_to_current_selection(&self, ctx: &mut AppContext) {
        let route = match self.selected_index {
            0 => AppRoute::Home(HnStoriesSections::Home),
            1 => AppRoute::Home(HnStoriesSections::Ask),
            2 => AppRoute::Home(HnStoriesSections::Show),
            3 => AppRoute::Home(HnStoriesSections::Jobs),
            4 => AppRoute::Help,
            _ => unreachable!(),
        };
        ctx.router_replace_current_in_navigation_stack(route);
    }
}

#[cfg(test)]
mod tests {
    use super::Navigation;

    #[test]
    fn test_navigation_logic() {
        let mut navigation = Navigation::default();
        assert_eq!(navigation.selected_index, 0);

        navigation.next();
        assert_eq!(navigation.selected_index, 1);
        navigation.next();
        navigation.next();
        assert_eq!(navigation.selected_index, 3);
        navigation.next();
        navigation.next();
        assert_eq!(navigation.selected_index, 0);

        navigation.previous();
        assert_eq!(navigation.selected_index, 4);
        navigation.previous();
        assert_eq!(navigation.selected_index, 3);
    }
}
