use std::{
    cell::{Ref, RefCell, RefMut},
    collections::HashMap,
    sync::Arc,
};

use tui::layout::Rect;

use crate::{
    api::HnStoriesSorting,
    ui::{
        common::UiComponentId,
        components::stories::DisplayableHackerNewsItem,
        handlers::Key,
        router::{AppRoute, AppRouter},
        screens::{Screen, ScreenComponentsRegistry, ScreenEventResponse},
    },
};

/// Manipulate application state from the screens and components.
///
/// NB: Using mutable references (`&'a mut AppState`) would cause lifetime issues in the `ui` module.
#[derive(Clone)]
pub struct AppHandle {
    state: Arc<RefCell<AppState>>,
    router: Arc<RefCell<AppRouter>>,
    screen: Arc<RefCell<Box<dyn Screen>>>,
}

impl AppHandle {
    pub fn get_state(&self) -> Ref<AppState> {
        self.state.borrow()
    }

    pub fn get_state_mut(&mut self) -> RefMut<AppState> {
        self.state.try_borrow_mut().unwrap()
    }

    /// Push a new navigation route state.
    pub fn router_push_navigation_stack(&mut self, route: AppRoute) {
        let mut screen = self.screen.try_borrow_mut().unwrap();
        let mut router = self.router.try_borrow_mut().unwrap();
        router.push_navigation_stack(route.clone());
        *screen = AppRouter::build_screen_from_route(&route);
    }

    /// Go to the previous navigation route state.
    pub fn router_pop_navigation_stack(&mut self) -> Option<AppRoute> {
        let mut screen = self.screen.try_borrow_mut().unwrap();
        let mut router = self.router.try_borrow_mut().unwrap();
        let previous = router.pop_navigation_stack();
        *screen = AppRouter::build_screen_from_route(router.get_current_route());
        previous
    }
}

unsafe impl Send for AppHandle {}

/// Global application state.
#[derive(Debug)]
pub struct AppState {
    /// Main screen(s): current stories sorting.
    main_stories_sorting: HnStoriesSorting,
    /// The currently viewed item (Story or Job posting).
    currently_viewed_item: Option<DisplayableHackerNewsItem>,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            main_stories_sorting: HnStoriesSorting::Top,
            currently_viewed_item: None,
        }
    }
}

impl AppState {
    /// Get the current stories sorting for the main screen (left panel).
    pub fn get_main_stories_sorting(&self) -> &HnStoriesSorting {
        &self.main_stories_sorting
    }

    /// Set the current stories sorting for the main screen (left panel).
    pub fn set_main_stories_sorting(&mut self, sorting: HnStoriesSorting) {
        self.main_stories_sorting = sorting;
    }

    /// Get the currently viewed story/job item.
    pub fn get_currently_viewed_item(&self) -> &Option<DisplayableHackerNewsItem> {
        &self.currently_viewed_item
    }

    /// Set the currently viewed story/job item.
    pub fn set_currently_viewed_item(&mut self, viewed: Option<DisplayableHackerNewsItem>) {
        self.currently_viewed_item = viewed;
    }
}

/// Global application.
#[derive(Debug)]
pub struct App {
    /// Application state.
    state: Arc<RefCell<AppState>>,
    /// Application router.
    router: Arc<RefCell<AppRouter>>,
    /// Cached current Screen.
    current_screen: Arc<RefCell<Box<dyn Screen>>>,
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
        let initial_route = AppRoute::Home;
        let (router, current_screen) = AppRouter::new(initial_route);
        Self {
            state: Arc::new(RefCell::new(Default::default())),
            router: Arc::new(RefCell::new(router)),
            current_screen: Arc::new(RefCell::new(current_screen)),
            layout_components: HashMap::new(),
        }
    }

    pub fn get_handle(&mut self) -> AppHandle {
        AppHandle {
            state: self.state.clone(),
            router: self.router.clone(),
            screen: self.current_screen.clone(),
        }
    }

    /// Handle an incoming key event, at the application level. Returns true if
    /// the event is to be captured (swallowed) and not passed down to components.
    pub fn handle_key_event(&mut self, key: &Key) -> bool {
        let mut handle = self.get_handle();
        match self
            .current_screen
            .try_borrow_mut()
            .unwrap()
            .handle_key_event(key, &mut handle)
        {
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
        let handle = self.get_handle();
        self.current_screen
            .try_borrow_mut()
            .unwrap()
            .compute_layout(frame_size, &mut self.layout_components, &handle);
    }

    /// Get, if any, the rendering `Rect` target for the given component.
    pub fn get_component_rendering_rect(&self, id: &UiComponentId) -> Option<&Rect> {
        self.layout_components.get(id)
    }
}

unsafe impl Send for App {}
