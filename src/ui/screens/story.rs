use tui::layout::{Constraint, Direction, Layout, Rect};

use crate::{
    app::AppState,
    config::AppConfiguration,
    ui::{
        components::{item_comments::ITEM_COMMENTS_ID, item_details::ITEM_DETAILS_ID},
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
    fn before_mount(&mut self, state: &mut AppState, config: &AppConfiguration) {
        state.set_currently_viewed_item(Some(self.item.clone()));
        if state
            .get_currently_viewed_item()
            .as_ref()
            .map_or(false, |item| item.is_job)
        {
            state.set_item_page_should_display_comments_panel(false);
        } else {
            state.set_item_page_should_display_comments_panel(
                config.get_display_comments_panel_by_default(),
            );
        }
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
        } else if inputs.is_active(&ApplicationAction::ItemToggleComments)
            && !state
                .get_currently_viewed_item()
                .as_ref()
                .map_or(false, |item| item.is_job)
        {
            state.set_item_page_should_display_comments_panel(
                !state.get_item_page_should_display_comments_panel(),
            );
            (ScreenEventResponse::Caught, None)
        } else if inputs.is_active(&ApplicationAction::OpenHackerNewsLink) {
            let item_hn_link = self.item.get_hacker_news_link();
            open_browser_tab(item_hn_link.as_str());
            (ScreenEventResponse::Caught, None)
        } else if inputs.is_active(&ApplicationAction::OpenExternalOrHackerNewsLink) {
            let item_link = self
                .item
                .url
                .clone()
                .unwrap_or_else(|| self.item.get_hacker_news_link());
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
            components_registry.insert(ITEM_COMMENTS_ID, main_layout_chunks[1]);
        }
    }
}

unsafe impl Send for StoryDetailsScreen {}
