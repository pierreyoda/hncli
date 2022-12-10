use tui::layout::{Constraint, Direction, Layout, Rect};

use crate::{
    api::types::HnItemIdScalar,
    app::AppState,
    config::AppConfiguration,
    ui::{
        components::{
            item_comments::COMMENT_ITEM_NESTED_COMMENTS_ID, item_summary::ITEM_SUMMARY_ID,
        },
        displayable_item::DisplayableHackerNewsItem,
        handlers::{ApplicationAction, InputsController},
        router::{AppRoute, AppRouter},
    },
};

use super::{Screen, ScreenComponentsRegistry, ScreenEventResponse};

/// Screen displaying the sub-comments of an HackerNews comment.
#[derive(Debug)]
pub struct NestedCommentsScreen {
    parent_comment: DisplayableHackerNewsItem,
}

impl NestedCommentsScreen {
    pub fn new(parent_comment: DisplayableHackerNewsItem) -> Self {
        assert!(parent_comment.is_comment);
        assert!(!Self::get_parent_comment_kids(&parent_comment).is_empty());
        Self { parent_comment }
    }

    fn get_parent_comment_kids(parent_comment: &DisplayableHackerNewsItem) -> &[HnItemIdScalar] {
        parent_comment
            .kids
            .as_ref()
            .map_or(&[], |kids| kids.as_ref())
    }
}

impl Screen for NestedCommentsScreen {
    fn before_mount(&mut self, state: &mut AppState, _config: &AppConfiguration) {
        state.push_currently_viewed_item_comments_chain(self.parent_comment.id);
        state.push_currently_viewed_item_comments_chain(
            *Self::get_parent_comment_kids(&self.parent_comment)
                .first()
                .expect("sub_comments Screen: at least 1 sub-comment should be present"),
        );
    }

    fn handle_inputs(
        &mut self,
        inputs: &InputsController,
        router: &mut AppRouter,
        state: &mut AppState,
    ) -> (ScreenEventResponse, Option<AppRoute>) {
        if inputs.is_active(&ApplicationAction::Back) {
            let restored_comment_id = state.pop_currently_viewed_item_comments_chain();
            state.set_previously_viewed_comment_id(restored_comment_id);
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
        components_registry.insert(COMMENT_ITEM_NESTED_COMMENTS_ID, main_layout_chunks[1]);
    }
}
