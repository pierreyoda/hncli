use std::collections::HashMap;

use crossterm::event::KeyEvent;
use tui::layout::Rect;

use crate::{
    api::client::HnStoriesSections,
    config::AppConfiguration,
    ui::{
        common::UiComponentId,
        handlers::{ApplicationAction, InputsController},
        router::{AppRoute, AppRouter},
        screens::{Screen, ScreenComponentsRegistry, ScreenEventResponse},
    },
};

use self::{history::AppHistory, state::AppState};

pub mod history;
pub mod state;

/// Interact with application state from the components.
pub struct AppContext<'a> {
    state: &'a mut AppState,
    router: &'a mut AppRouter,
    config: &'a mut AppConfiguration,
    inputs: &'a InputsController,
    history: &'a mut AppHistory,
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

    pub fn get_config(&self) -> &AppConfiguration {
        self.config
    }

    pub fn get_config_mut(&mut self) -> &mut AppConfiguration {
        self.config
    }

    pub fn get_history(&self) -> &AppHistory {
        self.history
    }

    pub fn get_inputs(&self) -> &InputsController {
        self.inputs
    }

    pub fn get_router(&self) -> &AppRouter {
        self.router
    }

    /// Push a new navigation route state.
    pub fn router_push_navigation_stack(&mut self, route: AppRoute) {
        self.router.push_navigation_stack(route);
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
        if route.is_settings() || route.is_help() || route.is_search_help() {
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
        self.screen.before_mount(self.state, self.config);
    }
}

unsafe impl<'a> Send for AppContext<'a> {}

/// Global application.
#[derive(Debug)]
pub struct App {
    /// Application state.
    state: AppState,
    /// Application router.
    router: AppRouter,
    /// Application configuration.
    config: AppConfiguration,
    /// Application inputs controller.
    inputs: InputsController,
    /// Application usage history.
    history: AppHistory,
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
    pub fn new(config: AppConfiguration) -> Self {
        let mut state = AppState::from_config(&config);
        let initial_route = AppRoute::Home(HnStoriesSections::Home);
        let (router, current_screen) = AppRouter::new(initial_route, &mut state, &config);
        let history = AppHistory::restored();

        Self {
            state,
            router,
            config,
            history,
            current_screen,
            inputs: InputsController::new(),
            layout_components: HashMap::new(),
        }
    }

    /// Get the context handle allowing components to interact with the application.
    pub fn get_context(&mut self) -> AppContext {
        AppContext {
            inputs: &self.inputs,
            state: &mut self.state,
            router: &mut self.router,
            config: &mut self.config,
            history: &mut self.history,
            screen: &mut self.current_screen,
        }
    }

    /// Inject an event to be processed into `InputsController`.
    pub fn pump_event(&mut self, event: KeyEvent) {
        self.inputs.pump_event(event, &self.state);
    }

    /// Handle inputs, at the application level. Returns true if
    /// the active event is to be captured (swallowed) and not passed down to screens.
    pub fn handle_inputs(&mut self) -> bool {
        // global help page toggle (not in search)
        if !self.router.get_current_route().is_in_search_mode() {
            if self.inputs.is_active(&ApplicationAction::ToggleHelp) {
                if self.router.get_current_route().is_help() {
                    self.get_context().router_pop_navigation_stack();
                } else {
                    self.get_context()
                        .router_push_navigation_stack(AppRoute::Help);
                }
                return true;
            }
        }

        // screen event handling
        let (response, new_route) = self.current_screen.handle_inputs(
            &self.inputs,
            &mut self.router,
            &mut self.state,
            &mut self.history,
        );
        if let Some(route) = new_route {
            // update the current screen if the route changed
            self.current_screen = AppRouter::build_screen_from_route(route);
            self.current_screen
                .before_mount(&mut self.state, &self.config);
        }
        match response {
            ScreenEventResponse::Caught => false,
            ScreenEventResponse::PassThrough => true,
        }
    }

    /// Update the components' layout according to current terminal
    /// frame size (with automatic resizing).
    ///
    /// Also organically takes care of routing, since components not found in the
    /// `layout_components` hash are not rendered. This is done for simplicity purposes.
    ///
    /// Returns the previously mounted components, and the newly mounted ones.
    pub fn update_layout(&mut self, frame_size: Rect) -> (Vec<&str>, Vec<&str>) {
        let old_layout_components_ids = self.layout_components.keys().copied().collect();
        self.layout_components.clear();
        self.current_screen
            .compute_layout(frame_size, &mut self.layout_components, &self.state);
        (
            old_layout_components_ids,
            self.layout_components.keys().copied().collect(),
        )
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
