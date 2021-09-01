use std::collections::HashMap;

use tui::layout::Rect;

use crate::{
    api::{HnStoriesSections, HnStoriesSorting},
    ui::{
        common::UiComponentId,
        components::stories::DisplayableHackerNewsItem,
        handlers::Key,
        router::{AppRoute, AppRouter},
        screens::{Screen, ScreenComponentsRegistry, ScreenEventResponse},
    },
};

/// Interact with application state from the components.
pub struct AppContext<'a> {
    state: &'a mut AppState,
    router: &'a mut AppRouter,
    /// Stored to change screen on route change.
    screen: &'a mut Box<dyn Screen>,
}

impl<'a> AppContext<'a> {
    pub fn get_state(&self) -> &AppState {
        self.state
    }

    pub fn get_state_mut(&mut self) -> &mut AppState {
        self.state
    }

    pub fn get_router(&self) -> &AppRouter {
        self.router
    }

    /// Push a new navigation route state.
    pub fn router_push_navigation_stack(&mut self, route: AppRoute) {
        self.router.push_navigation_stack(route.clone());
        self.update_screen();
    }

    /// Go to the previous navigation route state.
    pub fn router_pop_navigation_stack(&mut self) -> Option<AppRoute> {
        let previous = self.router.pop_navigation_stack();
        self.update_screen();
        previous
    }

    /// Replace the current route state.
    ///
    /// Used by the navigation component.
    pub fn router_replace_current_in_navigation_stack(
        &mut self,
        route: AppRoute,
    ) -> Option<AppRoute> {
        if route.is_help() {
            self.router.push_navigation_stack(route);
            self.update_screen();
            None
        } else {
            let previous = self.router.pop_navigation_stack();
            self.router.push_navigation_stack(route);
            self.update_screen();
            previous
        }
    }

    fn update_screen(&mut self) {
        *self.screen = AppRouter::build_screen_from_route(self.router.get_current_route().clone());
        self.screen.before_mount(&mut self.state);
    }
}

unsafe impl<'a> Send for AppContext<'a> {}

/// Global application state.
#[derive(Debug)]
pub struct AppState {
    /// Latest component interacted with, *i.e.* the latest component having
    /// swallowed an UI event.
    latest_interacted_with_component: Option<UiComponentId>,
    /// Main screen(s): currently viewed section.
    main_stories_section: HnStoriesSections,
    /// Main screen(s): current stories sorting.
    main_stories_sorting: HnStoriesSorting,
    /// The currently viewed item (Story or Job posting).
    currently_viewed_item: Option<DisplayableHackerNewsItem>,
    /// Item details screen: is the comments panel visible or not.
    item_page_display_comments_panel: bool,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            latest_interacted_with_component: None,
            main_stories_sorting: HnStoriesSorting::Top,
            main_stories_section: HnStoriesSections::Home,
            currently_viewed_item: None,
            // TODO: add user-configurable option for initial value
            item_page_display_comments_panel: false,
        }
    }
}

impl AppState {
    /// Get the latest component interacted with.
    pub fn get_latest_interacted_with_component(&self) -> Option<&UiComponentId> {
        self.latest_interacted_with_component.as_ref()
    }

    /// Get the current stories sorting for the main screen.
    pub fn get_main_stories_sorting(&self) -> &HnStoriesSorting {
        &self.main_stories_sorting
    }

    /// Set the current stories sorting for the main screen.
    pub fn set_main_stories_sorting(&mut self, sorting: HnStoriesSorting) {
        self.main_stories_sorting = sorting;
    }

    /// Get the current stories section for the main screen.
    pub fn get_main_stories_section(&self) -> &HnStoriesSections {
        &self.main_stories_section
    }

    /// Set the current stories section for the main screen.
    pub fn set_main_stories_section(&mut self, section: HnStoriesSections) {
        self.main_stories_section = section;
    }

    /// Get the currently viewed story/job item.
    pub fn get_currently_viewed_item(&self) -> &Option<DisplayableHackerNewsItem> {
        &self.currently_viewed_item
    }

    /// Set the currently viewed story/job item.
    pub fn set_currently_viewed_item(&mut self, viewed: Option<DisplayableHackerNewsItem>) {
        self.currently_viewed_item = viewed;
    }

    /// Get the is comments panel visible on item details screen boolean.
    pub fn get_item_page_should_display_comments_panel(&self) -> bool {
        self.item_page_display_comments_panel
    }

    /// Toggle the is comments panel visible on item details screen boolean.
    pub fn toggle_item_page_should_display_comments_panel(&mut self) {
        self.item_page_display_comments_panel = !self.item_page_display_comments_panel;
    }
}

/// Global application.
#[derive(Debug)]
pub struct App {
    /// Application state.
    state: AppState,
    /// Application router.
    router: AppRouter,
    /// Cached current Screen.
    current_screen: Box<dyn Screen>,
    /// The current layout state.
    ///
    /// Each component with a defined target `Rect` will be displayed.
    ///
    /// This is the responsibility of `App` since `UserInterface` should not be
    /// aware of any business logic, for instance with regards to navigation.
    layout_components: ScreenComponentsRegistry,
}

impl App {
    pub fn new() -> Self {
        let mut state: AppState = Default::default();
        let initial_route = AppRoute::Home(HnStoriesSections::Home);
        let (router, current_screen) = AppRouter::new(initial_route, &mut state);
        Self {
            state,
            router,
            current_screen,
            layout_components: HashMap::new(),
        }
    }

    /// Get the context handle allowing components to interact with the application.
    pub fn get_context(&mut self) -> AppContext {
        AppContext {
            state: &mut self.state,
            router: &mut self.router,
            screen: &mut self.current_screen,
        }
    }

    /// Handle an incoming key event, at the application level. Returns true if
    /// the event is to be captured (swallowed) and not passed down to components.
    pub fn handle_key_event(&mut self, key: &Key) -> bool {
        // global help page toggle
        if matches!(key, Key::Char('h')) {
            if self.router.get_current_route().is_help() {
                self.get_context().router_pop_navigation_stack();
            } else {
                self.get_context()
                    .router_push_navigation_stack(AppRoute::Help);
            }
            return true;
        }

        // screen event handling
        let (response, new_route) =
            self.current_screen
                .handle_key_event(key, &mut self.router, &mut self.state);
        if let Some(route) = new_route {
            // update the current screen if the route changed
            self.current_screen = AppRouter::build_screen_from_route(route);
            self.current_screen.before_mount(&mut self.state);
        }
        match response {
            ScreenEventResponse::Caught => true,
            ScreenEventResponse::PassThrough => false,
        }
    }

    /// Update the components' layout according to current terminal
    /// frame size (with automatic resizing).
    ///
    /// Also organically takes care of routing, since components not found in the
    /// `layout_components` hash are not rendered. This is done for simplicity purposes.
    pub fn update_layout(&mut self, frame_size: Rect) {
        self.layout_components.clear();
        self.current_screen
            .compute_layout(frame_size, &mut self.layout_components, &self.state);
    }

    /// Update the last component interacted with from the UI loop.
    pub fn update_latest_interacted_with_component(&mut self, id: Option<UiComponentId>) {
        self.state.latest_interacted_with_component = id;
    }

    /// Get, if any, the rendering `Rect` target for the given component.
    pub fn get_component_rendering_rect(&self, id: &UiComponentId) -> Option<&Rect> {
        self.layout_components.get(id)
    }
}

unsafe impl Send for App {}
