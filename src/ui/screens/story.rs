use ratatui::layout::{Constraint, Direction, Layout, Rect};

use crate::{
    app::{AppContext, history::HistoryPersistCommand, state::AppState},
    ui::{
        components::{item_comments::ITEM_TOP_LEVEL_COMMENTS_ID, item_details::ITEM_DETAILS_ID},
        displayable_item::DisplayableHackerNewsItem,
        handlers::{ApplicationAction, InputsController},
        router::{AppRoute, AppRouter},
        utils::open_browser_tab,
    },
};

use super::{Screen, ScreenComponentsRegistry, ScreenEventResponse};

/// Story details screen.
///
/// ```md
/// ___________________________________________
/// |                                         |
/// |                <TITLE>                  |
/// |            <URL HOSTNAME?>              |
/// |      <SCORE> POINTS / BY <USERNAME>     |
/// |   <#COMMENTS COUNT>  / POSTED <X> AGO   |
/// |_________________________________________|
/// |                                         |
/// |               COMMENTS                  |
/// |_________________________________________|
/// ```
#[derive(Debug)]
pub struct StoryDetailsScreen {
    item: DisplayableHackerNewsItem,
}

impl StoryDetailsScreen {
    pub fn new(item: DisplayableHackerNewsItem) -> Self {
        Self { item }
    }
}

impl Screen for StoryDetailsScreen {
    fn before_mount(&mut self, ctx: &mut AppContext) {
        let display_comments_panel_by_default =
            ctx.get_config().get_display_comments_panel_by_default();

        let state = ctx.get_state_mut();
        state.set_currently_viewed_item(Some(self.item.clone()));
        state.set_currently_viewed_item_has_switched(true);

        state.reset_currently_viewed_item_comments_chain();
        if let Some(item_kids) = self.item.kids.as_ref()
            && let Some(first_comment_id) = item_kids.first()
        {
            state.push_currently_viewed_item_comments_chain(*first_comment_id);
        }

        if let Some(item) = state.get_currently_viewed_item() {
            if item.is_job {
                state.set_item_page_should_display_comments_panel(false);
            } else if item.text.is_none() {
                state.set_item_page_should_display_comments_panel(true);
            } else {
                state
                    .set_item_page_should_display_comments_panel(display_comments_panel_by_default);
            }
        }
    }

    fn before_unmount(&mut self, ctx: &mut AppContext) {
        let mut history_commands: Vec<HistoryPersistCommand> = vec![];
        // navigation history handling, before any state reset
        if let Some(focused_top_level_comment_id) = ctx
            .get_state()
            .get_currently_viewed_item_comments_chain()
            .first()
        {
            history_commands.push(HistoryPersistCommand::TopLevelCommentAdd {
                story_id: self.item.id,
                top_level_comment_id: *focused_top_level_comment_id,
            });
        }
        // restore item history handling
        if self.item.can_resume()
            && let Some(title) = &self.item.title
        {
            history_commands.push(HistoryPersistCommand::ResumeAdd {
                item_id: self.item.id,
                label: title.clone(),
            });
        }
        // history persist to file
        ctx.get_history_mut().persist(&history_commands);

        let state = ctx.get_state_mut();
        state.reset_currently_viewed_item_comments_chain();
        state.set_currently_viewed_item_has_switched(true);
    }

    fn handle_inputs(
        &mut self,
        inputs: &InputsController,
        router: &mut AppRouter,
        state: &mut AppState,
    ) -> (ScreenEventResponse, Option<AppRoute>) {
        if inputs.is_active(&ApplicationAction::Back) {
            router.pop_navigation_stack();
            (
                ScreenEventResponse::Caught,
                Some(router.get_current_route().clone()),
            )
        } else if inputs.is_active(&ApplicationAction::ItemToggleComments) {
            match state.get_currently_viewed_item() {
                Some(item) if item.is_job => {
                    state.set_item_page_should_display_comments_panel(false);
                }
                Some(item) if item.text.is_none() => {
                    state.set_item_page_should_display_comments_panel(true);
                }
                _ => state.set_item_page_should_display_comments_panel(
                    !state.get_item_page_should_display_comments_panel(),
                ),
            }
            (ScreenEventResponse::Caught, None)
        } else if inputs.is_active(&ApplicationAction::OpenHackerNewsLink) {
            let item_hn_link = self.item.get_hacker_news_link();
            open_browser_tab(&item_hn_link);
            (ScreenEventResponse::Caught, None)
        } else if inputs.is_active(&ApplicationAction::OpenExternalOrHackerNewsLink) {
            let item_link = self
                .item
                .url
                .clone()
                .unwrap_or_else(|| self.item.get_hacker_news_link());
            open_browser_tab(&item_link);
            (ScreenEventResponse::Caught, None)
        } else {
            (ScreenEventResponse::PassThrough, None)
        }
    }

    fn compute_layout(
        &self,
        frame_size: Rect,
        components_registry: &mut ScreenComponentsRegistry,
        state: &AppState,
    ) {
        let display_comments_panel = state.get_item_page_should_display_comments_panel();

        let (header_size, comments_size) = if display_comments_panel {
            (15, 85)
        } else {
            (100, 0)
        };
        let main_layout_chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(2)
            .constraints(
                [
                    Constraint::Percentage(header_size),
                    Constraint::Percentage(comments_size),
                ]
                .as_ref(),
            )
            .split(frame_size);

        components_registry.insert(ITEM_DETAILS_ID, main_layout_chunks[0]);
        if display_comments_panel {
            components_registry.insert(ITEM_TOP_LEVEL_COMMENTS_ID, main_layout_chunks[1]);
        }
    }
}
