use std::collections::HashMap;

use tui::layout::{Constraint, Direction, Layout, Rect};

use crate::{
    api::HnStoriesSorting,
    errors::Result,
    ui::{
        common::UiComponentId,
        components::{navigation::NAVIGATION_ID, stories::STORIES_PANEL_ID},
    },
};

/// A block is a keyboard-navigable section of the UI.
///
///
/// # Example with two blocks
///
/// ```md
/// ------------------------------------------
/// |         |                              |
/// |         |                              |
/// | stories |       thread                 |
/// |         |                              |
/// |         |                              |
/// ------------------------------------------
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AppBlock {
    /// Welcome splash screen.
    SplashScreen,
    /// Stories list on the home page, sortable by "Top", "Best" or "New".
    HomeStories,
    /// Comments thread on a story.
    StoryThread,
    /// Help screen.
    Help,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Route {
    Home,
    Ask,
    Show,
    Jobs,
    Help,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RouteState {
    pub route: Route,
    pub active_block: AppBlock,
    pub hovered_block: AppBlock,
}

const DEFAULT_ROUTE_STATE: RouteState = RouteState {
    route: Route::Home,
    active_block: AppBlock::SplashScreen,
    hovered_block: AppBlock::HomeStories,
};

/// Global application state.
#[derive(Debug)]
pub struct App {
    /// The current navigation stack.
    ///
    /// Example: home > story #1 details > comment #2 thread.
    navigation_stack: Vec<RouteState>,
    /// The current layout state.
    ///
    /// Each component with a defined target `Rect` will be displayed.
    ///
    /// This is the responsability of `App` since `UserInterface` should not be
    /// aware of any business logic, for instance with regards to navigation.
    layout_components: HashMap<UiComponentId, Rect>,
    /// Main screen(s): current stories sorting.
    main_stories_sorting: HnStoriesSorting,
}

impl Default for App {
    fn default() -> Self {
        Self {
            navigation_stack: vec![DEFAULT_ROUTE_STATE],
            layout_components: HashMap::new(),
            main_stories_sorting: HnStoriesSorting::Top,
        }
    }
}

impl App {
    /// Get the current route state.
    pub fn get_current_route(&self) -> &RouteState {
        self.navigation_stack.last().unwrap_or(&DEFAULT_ROUTE_STATE)
    }

    /// Update the components' layout according to the main one
    /// (with automatic resizing).
    pub fn update_layout(&mut self, layout_chunks: &[Rect]) {
        use Route::*;

        self.layout_components.clear();

        match self.get_current_route().route {
            Home | Ask | Show | Jobs => {
                let main_screen_chunks = Layout::default()
                    .direction(Direction::Horizontal)
                    .horizontal_margin(0)
                    .constraints([Constraint::Percentage(40), Constraint::Percentage(100)].as_ref())
                    .split(layout_chunks[1]);

                self.layout_components
                    .insert(STORIES_PANEL_ID, main_screen_chunks[0]);
            }
            Help => {}
        }

        self.layout_components
            .insert(NAVIGATION_ID, layout_chunks[0]);
    }

    /// Get, if any, the rendering `Rect` target for the given component.
    pub fn get_component_rendering_rect(&self, id: &UiComponentId) -> Option<&Rect> {
        self.layout_components.get(id)
    }

    /// Get the current stories sorting for the main screen (left panel).
    pub fn get_main_stories_sorting(&self) -> &HnStoriesSorting {
        &self.main_stories_sorting
    }
}
