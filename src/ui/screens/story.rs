use tui::layout::{Constraint, Direction, Layout, Rect};

use crate::{
    app::AppState,
    ui::{
        components::{
            item_comments::ITEM_COMMENTS_ID, item_details::ITEM_DETAILS_ID,
            stories::DisplayableHackerNewsItem,
        },
        handlers::Key,
        router::{AppRoute, AppRouter},
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
/// |_________________________________________|
/// ```
#[derive(Debug)]
pub struct StoryDetailsScreen {
    item: DisplayableHackerNewsItem,
    display_comments_panel: bool,
}

impl StoryDetailsScreen {
    pub fn new(item: DisplayableHackerNewsItem) -> Self {
        Self {
            item,
            // TODO: add user-configurable option for initial value
            display_comments_panel: false,
        }
    }
}

impl Screen for StoryDetailsScreen {
    fn before_mount(&mut self, state: &mut AppState) {
        state.set_currently_viewed_item(Some(self.item.clone()));
    }

    fn handle_key_event(
        &mut self,
        key: &Key,
        router: &mut AppRouter,
        _state: &mut AppState,
    ) -> (ScreenEventResponse, Option<AppRoute>) {
        match key {
            Key::Escape => {
                router.pop_navigation_stack();
                (
                    ScreenEventResponse::Caught,
                    Some(router.get_current_route().clone()),
                )
            }
            Key::Tab => {
                self.display_comments_panel = !self.display_comments_panel;
                (ScreenEventResponse::Caught, None)
            }
            Key::Char('o') => {
                // TODO: open link or hacker news discussion page in new browser tab
                (ScreenEventResponse::Caught, None)
            }
            _ => (ScreenEventResponse::PassThrough, None),
        }
    }

    fn compute_layout(
        &self,
        frame_size: Rect,
        components_registry: &mut ScreenComponentsRegistry,
        _state: &AppState,
    ) {
        let (header_size, comments_size) = if self.display_comments_panel {
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
        if self.display_comments_panel {
            components_registry.insert(ITEM_COMMENTS_ID, main_layout_chunks[1]);
        }
    }
}

unsafe impl Send for StoryDetailsScreen {}
