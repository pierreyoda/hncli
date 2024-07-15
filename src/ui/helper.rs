use tui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    text::Spans,
    widgets::Paragraph,
};

use crate::app::state::AppState;

use super::{
    common::RenderFrame,
    displayable_item::DisplayableHackerNewsItem,
    handlers::{InputsController, Key},
    router::AppRoute,
};

/// Contextual help widget.
enum HelpWidget {
    /// Empty widget for padding purposes.
    Empty,
    /// Static text.
    Text(String),
    /// Key reminder. Structure: (icon, text, key).
    KeyReminder(char, String, Key),
}

impl HelpWidget {
    pub fn render(&self, f: &mut RenderFrame, inside: Rect) {
        use HelpWidget::*;

        let widget_text = match self {
            Empty => "".into(),
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
#[derive(Default)]
pub struct ContextualHelper {}

impl ContextualHelper {
    /// Renderer.
    pub fn render(
        &self,
        f: &mut RenderFrame,
        inside: Rect,
        for_route: &AppRoute,
        app_state: &AppState,
        app_inputs: &InputsController,
    ) {
        match for_route {
            AppRoute::Home(_) => self.render_home_page_help(f, inside, app_inputs),
            AppRoute::ItemDetails(item) => {
                self.render_item_page_help(f, inside, app_state, app_inputs, item)
            }
            AppRoute::ItemNestedComments(_) => self.render_comments_page_help(f, inside),
            AppRoute::UserProfile(_) => self.render_user_page_help(f, inside),
            AppRoute::SearchHelp => self.render_search_page_help(f, inside),
            AppRoute::Search => self.render_search_page_help(f, inside),
            AppRoute::Settings => self.render_settings_page_help(f, inside),
            AppRoute::Help => self.render_help_page_help(f, inside),
        }
    }

    fn render_home_page_help(
        &self,
        f: &mut RenderFrame,
        inside: Rect,
        app_inputs: &InputsController,
    ) {
        let widgets = vec![
            HelpWidget::KeyReminder('💡', "toggle help".into(), Key::Char('h')),
            HelpWidget::Text("🌐 'l' or 'o' to open HN/item link".into()),
            if app_inputs.has_ctrl_modifier() {
                HelpWidget::Text("❌ - CTRL + 'c' to quit".into())
            } else {
                HelpWidget::KeyReminder('❌', "quit".into(), Key::Char('q'))
            },
        ];
        Self::render_widgets(f, inside, &widgets);
    }

    fn render_item_page_help(
        &self,
        f: &mut RenderFrame,
        inside: Rect,
        app_state: &AppState,
        app_inputs: &InputsController,
        item: &DisplayableHackerNewsItem,
    ) {
        let widget_open_hn_link =
            HelpWidget::Text("🌐 - SHIFT + 'o' to open the item Hacker News page".into());

        let has_widget_toggle_comments = !app_state
            .get_currently_viewed_item()
            .as_ref()
            .map_or(false, |item| item.is_job);
        let display_comments_panel = app_state.get_item_page_should_display_comments_panel();

        let mut widgets = vec![];

        // open link widget
        if let Some(ref hostname) = item.url_hostname {
            widgets.push(if app_inputs.has_shift_modifier() {
                widget_open_hn_link
            } else {
                HelpWidget::KeyReminder('🌐', format!("open {}", hostname), Key::Char('o'))
            });
        } else {
            widgets.push(widget_open_hn_link);
        }

        // toggle comments widget
        if has_widget_toggle_comments {
            widgets.push(HelpWidget::KeyReminder(
                '💬',
                (if display_comments_panel {
                    "hide comments"
                } else {
                    "show comments"
                })
                .into(),
                Key::Tab,
            ));
        }

        // focus comment widget, if applicable
        if display_comments_panel {
            widgets.push(HelpWidget::KeyReminder(
                '🎯',
                "focus comment".into(),
                Key::Enter,
            ));
            widgets.push(HelpWidget::KeyReminder(
                '👤',
                "user profile".into(),
                Key::Char('p'),
            ));
        }

        // go back widget (if there is room)
        if widgets.len() < 3 {
            widgets.push(HelpWidget::KeyReminder('⬅', "go back".into(), Key::Escape));
        }

        Self::render_widgets(f, inside, &widgets);
    }

    fn render_comments_page_help(&self, f: &mut RenderFrame, inside: Rect) {
        let widget_focus_sub_comments =
            HelpWidget::KeyReminder('💬', "view sub-comment(s)".into(), Key::Enter);
        let widget_go_back = HelpWidget::KeyReminder('⬅', "go back".into(), Key::Escape);

        let widgets = vec![widget_focus_sub_comments, widget_go_back];
        Self::render_widgets(f, inside, &widgets);
    }

    fn render_user_page_help(&self, f: &mut RenderFrame, inside: Rect) {
        let widget_open_profile_page =
            HelpWidget::KeyReminder('🌐', "open the profile page".into(), Key::Char('o'));
        let widget_go_back = HelpWidget::KeyReminder('⬅', "go back".into(), Key::Escape);

        let widgets = vec![widget_open_profile_page, widget_go_back];
        Self::render_widgets(f, inside, &widgets);
    }

    fn render_search_page_help(&self, f: &mut RenderFrame, inside: Rect) {
        let widgets = vec![HelpWidget::Empty];
        Self::render_widgets(f, inside, &widgets);
    }

    fn render_settings_page_help(&self, f: &mut RenderFrame, inside: Rect) {
        let widgets = vec![
            HelpWidget::Text("⬆️  / i or ⬇️  / k to navigate".into()),
            HelpWidget::KeyReminder('✅', "toggle setting".into(), Key::Tab),
            HelpWidget::KeyReminder('⬅', "go back".into(), Key::Escape),
        ];
        Self::render_widgets(f, inside, &widgets);
    }

    fn render_help_page_help(&self, f: &mut RenderFrame, inside: Rect) {
        let widgets = vec![
            HelpWidget::KeyReminder('💡', "toggle help".into(), Key::Char('h')),
            HelpWidget::KeyReminder('⬅', "go back".into(), Key::Escape),
        ];
        Self::render_widgets(f, inside, &widgets);
    }

    fn render_widgets(f: &mut RenderFrame, inside: Rect, widgets: &[HelpWidget]) {
        // automatic layout
        assert!(!widgets.is_empty());
        let width_percentage = 100 / widgets.len() as u16;
        let constraints: Vec<Constraint> = (0..widgets.len())
            .map(|_| Constraint::Percentage(width_percentage))
            .collect();
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(constraints)
            .split(inside);

        // widgets rendering
        for (i, widget) in widgets.iter().enumerate() {
            widget.render(f, chunks[i]);
        }
    }
}
