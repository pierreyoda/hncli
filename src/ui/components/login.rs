use async_trait::async_trait;
use tui::{
    layout::{Constraint, Direction, Layout, Rect},
    text::Spans,
    widgets::{Paragraph, Widget},
};

use crate::{
    api::HnClient,
    app::AppContext,
    errors::Result,
    ui::{
        common::{RenderFrame, UiComponent, UiComponentId, UiTickScalar},
        handlers::ApplicationAction,
    },
};

use super::widgets::text_input::{
    TextInputState, TextInputStateActionBridge, TextInputWidget, TEXT_INPUT_AVAILABLE_ACTIONS,
};

#[derive(Debug)]
pub enum LoginFocusedInput {
    Username,
    Password,
}

impl Default for LoginFocusedInput {
    fn default() -> Self {
        Self::Username
    }
}

impl LoginFocusedInput {
    pub fn toggle(&self) -> Self {
        match self {
            Self::Username => Self::Password,
            Self::Password => Self::Username,
        }
    }
}

pub const LOGIN_ID: UiComponentId = "login";

#[derive(Debug, Default)]
pub struct Login {
    focused_input: LoginFocusedInput,
    input_state_username: TextInputState,
    input_state_password: TextInputState,
}

#[async_trait]
impl UiComponent for Login {
    fn id(&self) -> UiComponentId {
        LOGIN_ID
    }

    fn should_update(&mut self, _elapsed_ticks: UiTickScalar, _ctx: &AppContext) -> Result<bool> {
        Ok(false)
    }

    async fn update(&mut self, _client: &mut HnClient, _ctx: &mut AppContext) -> Result<()> {
        Ok(())
    }

    fn handle_inputs(&mut self, ctx: &mut AppContext) -> Result<bool> {
        let inputs = ctx.get_inputs();
        if inputs.is_active(&ApplicationAction::NavigateDown)
            || inputs.is_active(&ApplicationAction::NavigateUp)
        {
            self.focused_input.toggle();
            return Ok(true);
        }

        let input_state = match self.focused_input {
            LoginFocusedInput::Username => &mut self.input_state_username,
            LoginFocusedInput::Password => &mut self.input_state_password,
        };
        for input_available_action in TEXT_INPUT_AVAILABLE_ACTIONS {
            if inputs.is_active(&input_available_action) {}
        }

        // if let Some((_, char)) = inputs.get_active_input_key() {
        //     if state.get_current_algolia_query_state().get_value().len()
        //         < MAX_ALGOLIA_INPUT_LENGTH
        //     {
        //         state
        //             .get_current_algolia_query_state_mut()
        //             .handle_action(&TextInputStateAction::InsertCharacter(char));
        //         return (ScreenEventResponse::Caught, None);
        //     }
        // }
        for available_action in TEXT_INPUT_AVAILABLE_ACTIONS {
            if inputs.is_active(&available_action) {
                input_state.handle_event(inputs, &available_action);
                return Ok(true);
            }
        }

        Ok(false)
    }

    fn render(&mut self, f: &mut RenderFrame, inside: Rect, _ctx: &AppContext) -> Result<()> {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(4)
            .constraints(&[
                Constraint::Percentage(10),
                Constraint::Percentage(45),
                Constraint::Percentage(45),
            ])
            .split(inside);

        let header_text = vec![Spans::from("Sign In to HackerNews")];
        let header_paragraph = Paragraph::new(header_text);
        f.render_widget(header_paragraph, chunks[0]);

        let input_username = TextInputWidget::with_state(&self.input_state_username);
        f.render_widget(input_username, chunks[1]);

        let input_password = TextInputWidget::with_state(&self.input_state_password);
        f.render_widget(input_password, inside);

        Ok(())
    }
}
