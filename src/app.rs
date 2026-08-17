use std::collections::HashMap;

use crossterm::event::KeyEvent;
use ratatui::layout::Rect;

use crate::{
    api::client::HnStoriesSections,
    app::strings::{StringValuesProvider, values::StringValuesProviderEnglish},
    config::AppConfiguration,
    ui::{
        common::UiComponentId,
        handlers::{ApplicationAction, InputsController},
        router::{AppRoute, AppRouter},
        screens::{PlaceholderScreen, Screen, ScreenComponentsRegistry, ScreenEventResponse},
        theme::UiTheme,
    },
};

use self::{history::AppHistory, state::AppState};

pub mod history;
pub mod state;
pub mod strings;

/// Interact with application state from the components.
pub struct AppContext<'a> {
    strings_values_provider: &'a dyn StringValuesProvider,
    state: &'a mut AppState,
    router: &'a mut AppRouter,
    config: &'a mut AppConfiguration,
    inputs: &'a InputsController,
    history: &'a mut AppHistory,
    /// Stored to change screen on route change.
    screen: &'a mut Box<dyn Screen>,
}

impl<'a> AppContext<'a> {
    /// Get the rendered string values provider.
    pub fn svp(&self) -> &dyn StringValuesProvider {
        self.strings_values_provider
    }

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

    /// Shorthand for `get_config().get_theme()`.
    pub fn get_theme(&self) -> &UiTheme {
        self.config.get_theme()
    }

    pub fn get_history(&self) -> &AppHistory {
        self.history
    }

    pub fn get_history_mut(&mut self) -> &mut AppHistory {
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
        let mut screen =
            AppRouter::build_screen_from_route(self.router.get_current_route().clone());
        // the hook runs before the screen is stored, since the context borrows the current one
        screen.before_mount(self);
        *self.screen = screen;
    }
}

/// Global application.
#[derive(Debug)]
pub struct App {
    // Application-wide rendered string values provider.
    strings_provider: StringValuesProviderEnglish,
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
        let state = AppState::from_config(&config);
        let initial_route = AppRoute::Home(HnStoriesSections::Home);
        let (router, initial_screen) = AppRouter::new(initial_route);
        let history = AppHistory::restored();

        let mut app = Self {
            strings_provider: StringValuesProviderEnglish,
            state,
            router,
            config,
            history,
            current_screen: Box::new(PlaceholderScreen),
            inputs: InputsController::new(),
            layout_components: HashMap::new(),
        };
        app.mount_screen(initial_screen);
        app
    }

    /// Get the context handle allowing components to interact with the application.
    pub fn get_context(&mut self) -> AppContext<'_> {
        AppContext {
            strings_values_provider: &self.strings_provider,
            inputs: &self.inputs,
            state: &mut self.state,
            router: &mut self.router,
            config: &mut self.config,
            history: &mut self.history,
            screen: &mut self.current_screen,
        }
    }

    /// Mount the given screen as the current one, running its `before_mount` hook.
    ///
    /// The hook runs before the screen is stored, since `AppContext` mutably
    /// borrows the currently mounted screen.
    fn mount_screen(&mut self, mut screen: Box<dyn Screen>) {
        screen.before_mount(&mut self.get_context());
        self.current_screen = screen;
    }

    /// Unmount the current screen, running its `before_unmount` hook.
    ///
    /// The screen is moved out of `self` beforehand, since `AppContext` mutably
    /// borrows the currently mounted screen.
    fn unmount_current_screen(&mut self) {
        let mut screen = std::mem::replace(&mut self.current_screen, Box::new(PlaceholderScreen));
        screen.before_unmount(&mut self.get_context());
    }

    /// Inject an event to be processed into `InputsController`.
    pub fn pump_event(&mut self, event: KeyEvent) {
        self.inputs.pump_event(event, &self.state);
    }

    /// Handle inputs, at the application level. Returns true if
    /// the active event is to be captured (swallowed) and not passed down to screens.
    pub fn handle_inputs(&mut self) -> bool {
        // global help page toggle (not in search)
        if !self.router.get_current_route().is_in_search_mode()
            && self.inputs.is_active(&ApplicationAction::ToggleHelp)
        {
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
                .handle_inputs(&self.inputs, &mut self.router, &mut self.state);
        if let Some(route) = new_route {
            // screen unmount hook
            self.unmount_current_screen();
            // update the current screen if the route changed
            self.mount_screen(AppRouter::build_screen_from_route(route));
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
    pub fn update_layout(&mut self, frame_size: Rect) -> (Vec<UiComponentId>, Vec<UiComponentId>) {
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

    /// Unmount every screen of the navigation stack, from the current one down to the root.
    pub fn before_quit(&mut self) {
        self.unmount_current_screen();
        // parent screens are not instantiated: rebuild them from their route,
        // which fully determines their state
        let parent_routes: Vec<AppRoute> = self
            .router
            .navigation_stack_unmount_order()
            .skip(1) // the current screen was just unmounted
            .cloned()
            .collect();
        for route in parent_routes {
            AppRouter::build_screen_from_route(route).before_unmount(&mut self.get_context());
        }
        self.history.persist(&[]);
    }
}
