use std::io::Stdout;

use tui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    text::Spans,
    widgets::Paragraph,
    Frame,
};

use super::{handlers::Key, router::AppRoute};

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
    ) {
        match for_route {
            AppRoute::Home => self.render_home_page_help(f, inside),
            AppRoute::Help => self.render_help_page_help(f, inside),
        }
    }

    fn render_home_page_help(&self, f: &mut Frame<CrosstermBackend<Stdout>>, inside: Rect) {
        let widgets = vec![
            HelpWidget::KeyReminder('💡', "toggle help".into(), Key::Char('h')),
            HelpWidget::KeyReminder('✖', "quit".into(), Key::Char('q')),
        ];
        Self::render_widgets(f, inside, widgets.as_ref());
    }

    fn render_help_page_help(&self, f: &mut Frame<CrosstermBackend<Stdout>>, inside: Rect) {
        let widgets = vec![HelpWidget::KeyReminder(
            '💡',
            "toggle help".into(),
            Key::Char('h'),
        )];
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
