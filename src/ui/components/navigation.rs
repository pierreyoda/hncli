use std::io::Stdout;

use async_trait::async_trait;

use app::{App, AppBlock};
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
    api::HnClient,
    app,
    errors::Result,
    ui::{
        common::{UiComponent, UiComponentId, UiTickScalar},
        handlers,
    },
};

use super::common::get_layout_block_style;

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
            // TODO: more flexible data representation (shortkey, index)
            titles: vec!["Home", "Ask HN", "Show HN", "Jobs", "Help"],
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

    fn should_update(&mut self, _elapsed_ticks: UiTickScalar, _app: &App) -> Result<bool> {
        Ok(false)
    }

    async fn update(&mut self, _client: &mut HnClient, _app: &mut App) -> Result<()> {
        Ok(())
    }

    fn key_handler(&mut self, key: &Key, app: &mut App) -> Result<bool> {
        if !app.in_global_focus() && app.get_current_route().active_block != AppBlock::Navigation {
            return Ok(false);
        }

        Ok(match key {
            Key::Left => {
                self.previous();
                true
            }
            Key::Right => {
                self.next();
                true
            }
            Key::Char(c) => match c {
                // 'h' => {
                //     self.selected_index = 0;
                //     true
                // }
                // 'a' => {
                //     self.selected_index = 1;
                //     true
                // }
                // 's' => {
                //     self.selected_index = 2;
                //     true
                // }
                // 'j' => {
                //     self.selected_index = 3;
                //     true
                // }
                _ => false,
            },
            _ => false,
        })
    }

    fn render(
        &self,
        f: &mut Frame<CrosstermBackend<Stdout>>,
        inside: Rect,
        app: &App,
    ) -> Result<()> {
        let tabs_titles: Vec<Spans> = self
            .titles
            .iter()
            .map(|title| {
                // underline the first character to show the shortcut
                // TODO: do this work once, see above
                let (first, rest) = title.split_at(1);

                // TODO: cache stylings, also easier configuration later on
                Spans::from(vec![
                    Span::styled(
                        first,
                        Style::default()
                            .fg(Color::Yellow)
                            .add_modifier(Modifier::BOLD)
                            .add_modifier(Modifier::UNDERLINED),
                    ),
                    Span::styled(rest, Style::default().fg(Color::White)),
                ])
            })
            .collect();

        let tabs = Tabs::new(tabs_titles)
            .select(self.selected_index)
            .block(
                Block::default()
                    .style(get_layout_block_style(app, AppBlock::Navigation))
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
