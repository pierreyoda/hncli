use std::collections::HashMap;

use tui::layout::{Constraint, Direction, Layout, Rect};

use crate::{
    api::HnStoriesSorting,
    ui::{
        common::UiComponentId,
        components::{
            help::HELP_ID, navigation::NAVIGATION_ID, options::OPTIONS_ID,
            stories::STORIES_PANEL_ID,
        },
        handlers::Key,
    },
};

/// A block is a keyboard-navigable section of the UI.
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
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum AppBlock {
    /// Welcome splash screen.
    SplashScreen,
    /// Navigation.
    Navigation,
    /// Stories list on the home page, sortable by "Top", "Best" or "New".
    HomeStories,
    /// Comments thread on a story.
    StoryThread,
    /// Options.
    Options,
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
    /// Currently focus `AppBlock` in the application.
    ///
    /// If no application has focus (gained with the 'Escape' key),
    /// then the global application has focus which allows for moving between blocks.
    current_focus: Option<AppBlock>,
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
            current_focus: None,
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

    fn get_current_route_mut(&mut self) -> &mut RouteState {
        self.navigation_stack.last_mut().unwrap()
    }

    /// Set the current route state.
    pub fn set_current_route(&mut self, active: Option<AppBlock>, hovered: Option<AppBlock>) {
        let current_route = self.get_current_route_mut();
        if let Some(active_block) = active {
            current_route.active_block = active_block;
        }
        if let Some(hovered_block) = hovered {
            current_route.hovered_block = hovered_block;
        }
    }

    /// Push a new navigation route state.
    pub fn push_navigation_stack(&mut self, route: Route, block: AppBlock) {
        self.navigation_stack.push(RouteState {
            route,
            active_block: block,
            hovered_block: block,
        });
    }

    /// Go to the previous navigation route state.
    pub fn pop_navigation_stack(&mut self) -> Option<RouteState> {
        if self.navigation_stack.is_empty() {
            None
        } else {
            self.navigation_stack.pop()
        }
    }

    /// Is the application in global focus?
    ///
    /// If true, this means that no particular block is currently actively selected.
    pub fn in_global_focus(&self) -> bool {
        self.current_focus.is_none()
    }

    /// Has the given block the current focus?
    pub fn has_current_focus(&self, block: AppBlock) -> bool {
        block == self.get_current_route().hovered_block
    }

    /// Handle an incoming key event, at the application level. Returns true if
    /// the event is to be captured (swallowed) and not passed down to components.
    ///
    /// For keyboard navigation between blocks, here is the current application layout:
    ///
    /// ```md
    /// ------------------------------------------
    /// |              navigation                |
    /// ------------------------------------------
    /// |         |                              |
    /// |         |                              |
    /// | stories |       thread                 |
    /// |         |                              |
    /// |         |                              |
    /// ------------------------------------------
    /// |          options (eg. sorting)         |
    /// ------------------------------------------
    /// ```
    pub fn handle_key_event(&mut self, key: &Key) -> bool {
        let current_route = self.get_current_route_mut();
        let can_horizontally_navigate = matches!(
            current_route.active_block,
            AppBlock::HomeStories | AppBlock::StoryThread
        );

        let in_help = current_route.route == Route::Help;
        match key {
            Key::Escape if !in_help => self.current_focus = None,
            Key::Enter if !in_help => current_route.active_block = current_route.hovered_block,
            Key::Char('h') if !in_help => self.push_navigation_stack(Route::Help, AppBlock::Help),
            Key::Up => match current_route.hovered_block {
                AppBlock::Navigation => current_route.hovered_block = AppBlock::Options,
                AppBlock::HomeStories | AppBlock::StoryThread => {
                    current_route.hovered_block = AppBlock::Navigation
                }
                AppBlock::Options => current_route.hovered_block = AppBlock::HomeStories,
                _ => (),
            },
            Key::Down => match current_route.hovered_block {
                AppBlock::Navigation => current_route.hovered_block = AppBlock::HomeStories,
                AppBlock::HomeStories | AppBlock::StoryThread => {
                    current_route.hovered_block = AppBlock::Options
                }
                AppBlock::Options => current_route.hovered_block = AppBlock::Navigation,
                _ => (),
            },
            Key::Left | Key::Right if can_horizontally_navigate => {
                match current_route.hovered_block {
                    AppBlock::HomeStories => current_route.hovered_block = AppBlock::StoryThread,
                    AppBlock::StoryThread => current_route.hovered_block = AppBlock::HomeStories,
                    _ => (),
                }
            }
            _ => return false,
        }

        true
    }

    /// Update the components' layout according to current terminal
    /// frame size (with automatic resizing).
    ///
    /// Also organically takes care of routing, since components not found in the
    /// `layout_components` hash are not rendered. This is done for simplicity purposes.
    pub fn update_layout(&mut self, frame_size: Rect) {
        use Route::*;

        self.layout_components.clear();

        // main layout chunks
        let main_layout_chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(2)
            .constraints(
                [
                    Constraint::Length(3),
                    Constraint::Min(2),
                    Constraint::Length(3),
                ]
                .as_ref(),
            )
            .split(frame_size);

        match self.get_current_route().route {
            Home | Ask | Show | Jobs => {
                let main_screen_chunks = Layout::default()
                    .direction(Direction::Horizontal)
                    .horizontal_margin(0)
                    .constraints([Constraint::Percentage(40), Constraint::Percentage(100)].as_ref())
                    .split(main_layout_chunks[1]);

                self.layout_components
                    .insert(NAVIGATION_ID, main_layout_chunks[0]);
                self.layout_components
                    .insert(STORIES_PANEL_ID, main_screen_chunks[0]);
                self.layout_components
                    .insert(OPTIONS_ID, main_layout_chunks[2]);
            }
            Help => {
                let full_screen_chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .horizontal_margin(0)
                    .constraints([Constraint::Percentage(100)].as_ref())
                    .split(main_layout_chunks[0]);

                self.layout_components
                    .insert(HELP_ID, full_screen_chunks[0]);
            }
        }
    }

    /// Get, if any, the rendering `Rect` target for the given component.
    pub fn get_component_rendering_rect(&self, id: &UiComponentId) -> Option<&Rect> {
        self.layout_components.get(id)
    }

    /// Get the current stories sorting for the main screen (left panel).
    pub fn get_main_stories_sorting(&self) -> &HnStoriesSorting {
        &self.main_stories_sorting
    }

    /// Set the current stories sorting for the main screen (left panel).
    pub fn set_main_stories_sorting(&mut self, sorting: HnStoriesSorting) {
        self.main_stories_sorting = sorting;
    }
}
