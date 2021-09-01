use std::fmt;

use crate::{
    api::HnStoriesSections,
    app::AppState,
    ui::screens::{help::HelpScreen, home::HomeScreen, story::StoryDetailsScreen},
};

use super::{components::stories::DisplayableHackerNewsItem, screens::Screen};

/// All the possible routes in the application.
#[derive(Clone, Debug)]
pub enum AppRoute {
    /// Home screen.
    Home(HnStoriesSections),
    /// Story details screen.
    StoryDetails(DisplayableHackerNewsItem),
    /// Help screen.
    Help,
}

impl AppRoute {
    pub fn is_help(&self) -> bool {
        matches!(self, AppRoute::Help)
    }
}

/// Stack-based global application router.
pub struct AppRouter {
    /// The current navigation stack.
    ///
    /// Example: home > story #1 details > comment #2 thread.
    navigation_stack: Vec<AppRoute>,
}

impl AppRouter {
    pub fn new(initial_route: AppRoute, state: &mut AppState) -> (Self, Box<dyn Screen>) {
        let mut initial_screen = Self::build_screen_from_route(initial_route.clone());
        initial_screen.before_mount(state);
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
            Help => Box::new(HelpScreen::new()),
            Home(section) => Box::new(HomeScreen::new(section)),
            StoryDetails(item) => Box::new(StoryDetailsScreen::new(item)),
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
