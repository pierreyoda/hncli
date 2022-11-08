use tui::layout::{Constraint, Direction, Layout, Rect};

use crate::{
    app::AppState,
    config::AppConfiguration,
    ui::{
        components::{item_comments::ITEM_COMMENTS_ID, item_summary::ITEM_SUMMARY_ID},
        displayable_item::DisplayableHackerNewsItem,
        handlers::{ApplicationAction, InputsController},
        router::{AppRoute, AppRouter},
    },
};

use super::{Screen, ScreenComponentsRegistry, ScreenEventResponse};

/// Screen displaying the sub-comments of an HackerNews comment.
#[derive(Debug)]
pub struct SubCommentsScreen {
    comment: DisplayableHackerNewsItem,
}

impl SubCommentsScreen {
    pub fn new(comment: DisplayableHackerNewsItem) -> Self {
        assert!(comment.is_comment);
        Self { comment }
    }
}

impl Screen for SubCommentsScreen {
    fn before_mount(&mut self, state: &mut AppState, _config: &AppConfiguration) {
        state.set_currently_viewed_item(Some(self.comment.clone()));
    }

    fn handle_inputs(
        &mut self,
        inputs: &InputsController,
        router: &mut AppRouter,
        state: &mut AppState,
    ) -> (ScreenEventResponse, Option<AppRoute>) {
        if inputs.is_active(&ApplicationAction::Back) {
            state.pop_currently_viewed_item_comments_chain();
            router.pop_navigation_stack();
            (
                ScreenEventResponse::Caught,
                Some(router.get_current_route().clone()),
            )
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
            .constraints([Constraint::Percentage(10), Constraint::Percentage(90)])
            .split(frame_size);

        components_registry.insert(ITEM_SUMMARY_ID, main_layout_chunks[0]);
        components_registry.insert(ITEM_COMMENTS_ID, main_layout_chunks[1]);
    }
}
