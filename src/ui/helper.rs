use std::io::Stdout;

use tui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    text::Spans,
    widgets::Paragraph,
    Frame,
};

use crate::app::AppState;

use super::{
    components::stories::DisplayableHackerNewsItem,
    handlers::{InputsController, Key},
    router::AppRoute,
};

/// Contextual help widget.
enum HelpWidget {
    /// Static text.
    Text(String),
    /// Key reminder. Structure: (icon, text, key).
    KeyReminder(char, String, Key),
}

impl HelpWidget {
    pub fn render(&self, f: &mut Frame<CrosstermBackend<Stdout>>, inside: Rect) {
        use HelpWidget::*;

        let widget_text = match self {
            Text(text) => text.clone(),
            KeyReminder(icon, text, key) => {
                format!("{} - {} to {}", icon, key.get_representation(), text)
            }
        };
        let text = vec![Spans::from(widget_text)];
        let paragraph = Paragraph::new(text).alignment(Alignment::Center);

        f.render_widget(paragraph, inside);
    }
}

/// Global contextual block displaying controls help and other widgets
/// relevant to the current route.
pub struct ContextualHelper {}

impl ContextualHelper {
    pub fn new() -> Self {
        Self {}
    }

    /// Renderer.
    pub fn render(
        &self,
        f: &mut Frame<CrosstermBackend<Stdout>>,
        inside: Rect,
        for_route: &AppRoute,
        app_state: &AppState,
        app_inputs: &InputsController,
    ) {
        match for_route {
            AppRoute::Home(_) => self.render_home_page_help(f, inside, app_state, app_inputs),
            AppRoute::StoryDetails(item) => {
                self.render_item_page_help(f, inside, app_state, app_inputs, item)
            }
            AppRoute::Settings => self.render_settings_page_help(f, inside, app_state, app_inputs),
            AppRoute::Help => self.render_help_page_help(f, inside, app_state, app_inputs),
        }
    }

    // TODO: add centralized key bindings manager
    fn render_home_page_help(
        &self,
        f: &mut Frame<CrosstermBackend<Stdout>>,
        inside: Rect,
        _app_state: &AppState,
        app_inputs: &InputsController,
    ) {
        let widgets = vec![
            HelpWidget::KeyReminder('💡', "toggle help".into(), Key::Char('h')),
            if app_inputs.has_shift_modifier() {
                HelpWidget::Text("🌐 - SHIFT + 'o' to open the item Hacker News page".into())
            } else {
                HelpWidget::KeyReminder('🌐', "open the item link".into(), Key::Char('o'))
            },
            if app_inputs.has_shift_modifier() {
                HelpWidget::Text("❌ - SHIFT + 'c' to quit".into())
            } else {
                HelpWidget::KeyReminder('❌', "quit".into(), Key::Char('q'))
            },
        ];
        Self::render_widgets(f, inside, widgets.as_ref());
    }

    fn render_item_page_help(
        &self,
        f: &mut Frame<CrosstermBackend<Stdout>>,
        inside: Rect,
        app_state: &AppState,
        app_inputs: &InputsController,
        item: &DisplayableHackerNewsItem,
    ) {
        let widget_open_hn_link =
            HelpWidget::Text("🌐 - SHIFT + 'o' to open the item Hacker News page".into());
        let display_comments_panel = app_state.get_item_page_should_display_comments_panel();
        let widget_toggle_comments = HelpWidget::KeyReminder(
            '💬',
            (if display_comments_panel {
                "hide comments"
            } else {
                "show comments"
            })
            .into(),
            Key::Tab,
        );
        let widget_go_back = HelpWidget::KeyReminder('⬅', "go back".into(), Key::Escape);
        let widgets = if let Some(ref hostname) = item.url_hostname {
            vec![
                if app_inputs.has_shift_modifier() {
                    widget_open_hn_link
                } else {
                    HelpWidget::KeyReminder('🌐', format!("open {}", hostname), Key::Char('o'))
                },
                widget_toggle_comments,
                widget_go_back,
            ]
        } else {
            vec![widget_open_hn_link, widget_toggle_comments, widget_go_back]
        };
        Self::render_widgets(f, inside, widgets.as_ref());
    }

    fn render_settings_page_help(
        &self,
        f: &mut Frame<CrosstermBackend<Stdout>>,
        inside: Rect,
        _app_state: &AppState,
        _app_inputs: &InputsController,
    ) {
        let widgets = vec![
            HelpWidget::Text("⬆️  / i or ⬇️  / k to navigate".into()),
            HelpWidget::KeyReminder('✅', "toggle the setting".into(), Key::Tab),
            HelpWidget::KeyReminder('⬅', "go back".into(), Key::Escape),
        ];
        Self::render_widgets(f, inside, widgets.as_ref());
    }

    fn render_help_page_help(
        &self,
        f: &mut Frame<CrosstermBackend<Stdout>>,
        inside: Rect,
        _app_state: &AppState,
        _app_inputs: &InputsController,
    ) {
        let widgets = vec![
            HelpWidget::KeyReminder('💡', "toggle help".into(), Key::Char('h')),
            HelpWidget::KeyReminder('⬅', "go back".into(), Key::Escape),
        ];
        Self::render_widgets(f, inside, widgets.as_ref());
    }

    fn render_widgets(
        f: &mut Frame<CrosstermBackend<Stdout>>,
        inside: Rect,
        widgets: &[HelpWidget],
    ) {
        // automatic layout
        assert!(!widgets.is_empty());
        let width_percentage = 100 / widgets.len() as u16;
        let constraints: Vec<Constraint> = (0..widgets.len())
            .map(|_| Constraint::Percentage(width_percentage))
            .collect();
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(constraints.as_ref())
            .split(inside);

        // widgets rendering
        for (i, widget) in widgets.iter().enumerate() {
            widget.render(f, chunks[i]);
        }
    }
}
