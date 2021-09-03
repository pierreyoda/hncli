use std::{io::Stdout, vec};

use async_trait::async_trait;
use tui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Span, Spans},
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};

use crate::{
    api::HnClient,
    app::AppContext,
    errors::Result,
    ui::{
        common::{UiComponent, UiComponentId, UiTickScalar},
        handlers::ApplicationAction,
    },
};

#[derive(Debug)]
enum SettingsOption {
    /// On the item details page, should we display the comments panel by default or not?
    DisplayCommentsPanelByDefault(bool),
    /// Show the global contextual help?
    ShowContextualHelp(bool),
}

impl SettingsOption {
    pub fn get_representation(&self) -> Span {
        match self {
            Self::DisplayCommentsPanelByDefault(value) => Self::get_boolean_representation(*value),
            Self::ShowContextualHelp(value) => Self::get_boolean_representation(*value),
        }
    }

    fn get_boolean_representation(value: bool) -> Span<'static> {
        if value {
            Span::styled("Enabled", Style::default().fg(Color::Green))
        } else {
            Span::styled("Disabled", Style::default().fg(Color::Red))
        }
    }
}

#[derive(Debug)]
struct SettingsControl {
    label: String,
    option: SettingsOption,
}

impl SettingsControl {
    pub fn render(&self, f: &mut Frame<CrosstermBackend<Stdout>>, inside: Rect, is_active: bool) {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Percentage(75), Constraint::Percentage(25)].as_ref())
            .split(inside);

        let label_text = vec![
            Spans::from(""),
            Spans::from(""),
            Spans::from(""),
            Spans::from(Span::styled(
                self.label.as_str(),
                Style::default().fg(if is_active {
                    Color::Yellow
                } else {
                    Color::White
                }),
            )),
        ];
        let label_paragraph = Paragraph::new(label_text).alignment(Alignment::Left);
        f.render_widget(label_paragraph, chunks[0]);

        let value_text = vec![
            Spans::from(""),
            Spans::from(""),
            Spans::from(""),
            Spans::from(self.option.get_representation()),
        ];
        let value_paragraph = Paragraph::new(value_text).alignment(Alignment::Right);
        f.render_widget(value_paragraph, chunks[1]);
    }
}

#[derive(Debug)]
pub struct Settings {
    controls: Vec<SettingsControl>,
    selected_control_index: usize,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            controls: vec![],
            selected_control_index: 0,
        }
    }
}

pub const SETTINGS_ID: UiComponentId = "settings";

#[async_trait]
impl UiComponent for Settings {
    fn before_mount(&mut self, ctx: &mut AppContext) {
        self.refresh_controls(ctx);
    }

    fn id(&self) -> UiComponentId {
        SETTINGS_ID
    }

    fn should_update(&mut self, _elapsed_ticks: UiTickScalar, _ctx: &AppContext) -> Result<bool> {
        Ok(false)
    }

    async fn update(&mut self, _client: &mut HnClient, _ctx: &mut AppContext) -> Result<()> {
        Ok(())
    }

    fn handle_inputs(&mut self, ctx: &mut AppContext) -> Result<bool> {
        let inputs = ctx.get_inputs();
        Ok(if inputs.is_active(&ApplicationAction::NavigateUp) {
            self.previous_control();
            true
        } else if inputs.is_active(&ApplicationAction::NavigateDown) {
            self.next_control();
            true
        } else if inputs.is_active(&ApplicationAction::SettingsToggleControl) {
            self.toggle_current_control(ctx);
            true
        } else {
            false
        })
    }

    fn render(
        &mut self,
        f: &mut Frame<CrosstermBackend<Stdout>>,
        inside: Rect,
        _ctx: &AppContext,
    ) -> Result<()> {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(7), Constraint::Percentage(93)].as_ref())
            .split(inside);

        // header block
        let header_text = vec![Spans::from("Settings")];
        let header_paragraph = Paragraph::new(header_text)
            .block(Self::get_common_block())
            .alignment(Alignment::Center);
        f.render_widget(header_paragraph, chunks[0]);

        // controls block
        assert!(!self.controls.is_empty());
        let controls_width_percentage = 100 / self.controls.len() as u16;
        let controls_constraints: Vec<Constraint> = (0..self.controls.len())
            .map(|_| Constraint::Percentage(controls_width_percentage))
            .collect();
        let controls_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(controls_constraints.as_ref())
            .split(chunks[1]);
        for (i, control) in self.controls.iter().enumerate() {
            control.render(f, controls_chunks[i], i == self.selected_control_index);
        }

        Ok(())
    }
}

impl Settings {
    fn next_control(&mut self) {
        self.selected_control_index = (self.selected_control_index + 1) % self.controls.len();
    }

    fn previous_control(&mut self) {
        if self.selected_control_index > 0 {
            self.selected_control_index -= 1;
        } else {
            self.selected_control_index = self.controls.len() - 1;
        }
    }

    fn toggle_current_control(&mut self, ctx: &mut AppContext) {
        match self.selected_control_index {
            0 => ctx
                .get_config_mut()
                .toggle_display_comments_panel_by_default(),
            1 => ctx.get_config_mut().toggle_show_contextual_help(),
            _ => (),
        }
        self.refresh_controls(ctx);
    }

    fn refresh_controls(&mut self, ctx: &AppContext) {
        self.controls = vec![
            SettingsControl {
                label: "Display the comments panel by default:".into(),
                option: SettingsOption::DisplayCommentsPanelByDefault(
                    ctx.get_config().get_display_comments_panel_by_default(),
                ),
            },
            SettingsControl {
                label: "Show the global contextual help".into(),
                option: SettingsOption::ShowContextualHelp(
                    ctx.get_config().get_show_contextual_help(),
                ),
            },
        ];
    }

    fn get_common_block() -> Block<'static> {
        Block::default()
            .border_type(BorderType::Thick)
            .borders(Borders::ALL)
    }
}
