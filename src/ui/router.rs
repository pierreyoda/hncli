use std::fmt;

use crate::ui::screens::{help::HelpScreen, home::HomeScreen};

use super::screens::Screen;

#[derive(Clone, Debug)]
pub enum AppRoute {
    Home,
    Help,
}

/// Stack-based global application router.
pub struct AppRouter {
    /// The current navigation stack.
    ///
    /// Example: home > story #1 details > comment #2 thread.
    navigation_stack: Vec<AppRoute>,
}

impl AppRouter {
    pub fn new(initial_route: AppRoute) -> (Self, Box<dyn Screen>) {
        let initial_screen = Self::build_screen_from_route(initial_route.clone());
        (
            Self {
                navigation_stack: vec![initial_route],
            },
            initial_screen,
        )
    }

    /// Get the current route state.
    pub fn get_current_route(&self) -> &AppRoute {
        self.navigation_stack.last().unwrap()
    }

    /// Push a new navigation route state.
    pub fn push_navigation_stack(&mut self, route: AppRoute) {
        self.navigation_stack.push(route);
    }

    /// Go to the previous navigation route state.
    pub fn pop_navigation_stack(&mut self) -> Option<AppRoute> {
        self.navigation_stack.pop()
    }

    pub fn build_screen_from_route(route: AppRoute) -> Box<dyn Screen> {
        use AppRoute::*;
        match route {
            Home => Box::new(HomeScreen::new()),
            Help => Box::new(HelpScreen::new()),
        }
    }
}

impl fmt::Debug for AppRouter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AppRouter")
            .field("navigation_stack", &self.navigation_stack)
            .finish()
    }
}
