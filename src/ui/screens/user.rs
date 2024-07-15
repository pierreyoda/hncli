use tui::layout::{Constraint, Direction, Layout, Rect};

use crate::{
    app::{history::AppHistory, state::AppState},
    config::AppConfiguration,
    ui::{
        components::user_profile::USER_PROFILE_ID,
        displayable_item::user::DisplayableHackerNewsUser,
        handlers::{ApplicationAction, InputsController},
        router::{AppRoute, AppRouter},
        utils::open_browser_tab,
    },
};

use super::{Screen, ScreenComponentsRegistry, ScreenEventResponse};

/// User details screen.
///
/// See `UserProfile` component for layout.
/// Does not display much, offering quick access to the official online profile with a short-key.
#[derive(Debug)]
pub struct UserDetailsScreen {
    user_id: String,
}

impl UserDetailsScreen {
    pub fn new(user_id: String) -> Self {
        Self { user_id }
    }
}

impl Screen for UserDetailsScreen {
    fn before_mount(&mut self, state: &mut AppState, _config: &AppConfiguration) {
        state.set_currently_viewed_user_id(Some(self.user_id.clone()));
    }

    fn handle_inputs(
        &mut self,
        inputs: &InputsController,
        router: &mut AppRouter,
        _state: &mut AppState,
        _history: &mut AppHistory,
    ) -> (ScreenEventResponse, Option<AppRoute>) {
        if inputs.is_active(&ApplicationAction::Back) {
            router.pop_navigation_stack();
            (
                ScreenEventResponse::Caught,
                Some(router.get_current_route().clone()),
            )
        } else if inputs.is_active(&ApplicationAction::OpenHackerNewsProfile) {
            let item_link = DisplayableHackerNewsUser::build_hacker_news_link(&self.user_id);
            open_browser_tab(item_link.as_str());
            (ScreenEventResponse::Caught, None)
        } else {
            (ScreenEventResponse::PassThrough, None)
        }
    }

    fn compute_layout(
        &self,
        frame_size: Rect,
        components_registry: &mut ScreenComponentsRegistry,
        _state: &AppState,
    ) {
        let main_layout_chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(2)
            .constraints([Constraint::Percentage(100)].as_ref())
            .split(frame_size);

        components_registry.insert(USER_PROFILE_ID, main_layout_chunks[0]);
    }
}
