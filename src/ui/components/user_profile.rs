use async_trait::async_trait;
use log::warn;
use tui::{
    layout::{Alignment, Rect},
    style::Style,
    text::Spans,
    widgets::{Block, BorderType, Borders, Paragraph},
};

use crate::{
    api::HnClient,
    app::AppContext,
    errors::Result,
    ui::{
        common::{RenderFrame, UiComponent, UiComponentId, UiTickScalar},
        displayable_item::user::DisplayableHackerNewsUser,
        utils::{html_to_plain_text, loader::Loader},
    },
};

use super::common::{render_text_message, COMMON_BLOCK_NORMAL_COLOR};

/// User profile component.
///
/// ```md
/// ___________________________________________
/// |                                         |
/// |                <USERNAME>               |
/// |           <REGISTRATION DATE>           |
/// |              <TOTAL KARMA>              |
/// |        <ABOUT CORPUS IF DEFINED>        |
/// |_________________________________________|
/// ```
#[derive(Debug, Default)]
pub struct UserProfile {
    loading: bool,
    loader: Loader,
    /// User not found or no public activity (if no state sync issue).
    error: bool,
    /// Cached fetched user data. Does not need to be in application state (yet).
    current_user: Option<DisplayableHackerNewsUser>,
}

pub const USER_PROFILE_ID: UiComponentId = "user_profile";

#[async_trait]
impl UiComponent for UserProfile {
    fn id(&self) -> UiComponentId {
        USER_PROFILE_ID
    }

    fn before_unmount(&mut self) {
        self.loader.stop();
    }

    fn should_update(&mut self, _elapsed_ticks: UiTickScalar, ctx: &AppContext) -> Result<bool> {
        // we don't do automatic refresh every X minutes since the information displayed has little chance to change
        // and some user profiles' JSON are very heavy due to many comments posted
        let should_update = !self.loading
            && ctx.get_state().get_currently_viewed_user_id()
                != self.current_user.as_ref().map(|user| &user.id);
        self.loader.update();
        if should_update {
            self.loading = true;
        }
        Ok(should_update)
    }

    async fn update(&mut self, client: &mut HnClient, ctx: &mut AppContext) -> Result<()> {
        self.loading = true;
        self.error = false;

        let currently_viewed_user_id = ctx.get_state().get_currently_viewed_user_id();
        if let Some(user_id) = currently_viewed_user_id {
            match client.classic().get_user_data(user_id).await {
                Ok(user_raw) => {
                    self.current_user = Some(
                        user_raw
                            .try_into()
                            .expect("UserProfile component: can map DisplayableHackerNewsUser"),
                    );
                }
                Err(why) => {
                    warn!("{}", why);
                    self.error = true;
                    self.current_user = None;
                    ctx.router_pop_navigation_stack();
                    ctx.get_state_mut().set_currently_viewed_user_id(None);
                }
            }
            self.loading = false;
        } else {
            self.current_user = None;
            self.error = true; // use the same error state for convenience
            self.loading = false;
            return Ok(());
        };

        Ok(())
    }

    fn handle_inputs(&mut self, _ctx: &mut AppContext) -> Result<bool> {
        Ok(false)
    }

    fn render(&mut self, f: &mut RenderFrame, inside: Rect, ctx: &AppContext) -> Result<()> {
        // Loading case
        if self.loading {
            render_text_message(f, inside, &self.loader.text());
            return Ok(());
        }

        // Inconsistent state: no currently viewed user ID
        let viewed_user_id = if let Some(user_id) = ctx.get_state().get_currently_viewed_user_id() {
            user_id
        } else {
            render_text_message(
                f,
                inside,
                "Sorry, this user cannot be displayed due to an error.",
            );
            return Ok(());
        };

        // Error case
        let viewed_user = if let Some(user) = &self.current_user {
            user
        } else {
            // TODO: on first ever component appearance, this message very quickly flashes somehow
            render_text_message(
                f,
                inside,
                &format!(
                    "The user data of '{}' cannot be loaded, please retry later.",
                    viewed_user_id
                ),
            );
            return Ok(());
        };

        let block = Block::default()
            .style(Style::default().fg(COMMON_BLOCK_NORMAL_COLOR))
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded);

        let text_base = vec![
            Spans::from(viewed_user.id.to_string()),
            Spans::from(format!("Created: {}", viewed_user.created_at_formatted)),
            Spans::from(format!("Karma: {}", viewed_user.karma)),
        ];
        let about_corpus = self.build_user_about_spans(inside, &viewed_user.about);

        let paragraph = Paragraph::new([text_base, about_corpus].concat())
            .block(block)
            .alignment(Alignment::Center);
        f.render_widget(paragraph, inside);

        Ok(())
    }
}

impl UserProfile {
    fn build_user_about_spans(&self, inside: Rect, about: &Option<String>) -> Vec<Spans> {
        if let Some(corpus) = about {
            let rendered = html_to_plain_text(corpus.as_str(), inside.width as usize);
            let spans = rendered
                .lines()
                .map(|line| Spans::from(line.to_string()))
                .collect();
            [
                vec![Spans::from(""), Spans::from(""), Spans::from("About:")],
                spans,
            ]
            .concat()
        } else {
            vec![]
        }
    }
}
