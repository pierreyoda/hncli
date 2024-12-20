use std::fmt;

use crate::{
    api::client::HnStoriesSections,
    app::state::AppState,
    config::AppConfiguration,
    ui::screens::{
        help::HelpScreen, home::HomeScreen, nested_comments::NestedCommentsScreen,
        search_help::SearchHelpScreen, settings::SettingsScreen, story::StoryDetailsScreen,
        user::UserDetailsScreen,
    },
};

use super::{displayable_item::DisplayableHackerNewsItem, screens::Screen};

/// All the possible routes in the application.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AppRoute {
    /// Home screen.
    Home(HnStoriesSections),
    /// Item details screen.
    ItemDetails(DisplayableHackerNewsItem),
    /// Item nested comments screen.
    ItemNestedComments(DisplayableHackerNewsItem),
    /// User profile screen. Only stores the user ID.
    UserProfile(String),
    /// Algolia-based search screen.
    // Search, // NB: disabled due to unofficial Algolia HN API becoming very limited
    /// Algolia-based search screen help.
    SearchHelp,
    /// Settings screen.
    Settings,
    /// Help screen.
    Help,
}

impl AppRoute {
    pub fn get_home_section(&self) -> Option<&HnStoriesSections> {
        match self {
            Self::Home(section) => Some(section),
            _ => None,
        }
    }

    pub fn is_home(&self) -> bool {
        matches!(self, AppRoute::Home(_))
    }

    pub fn is_search_help(&self) -> bool {
        matches!(self, AppRoute::SearchHelp)
    }

    pub fn is_in_search_mode(&self) -> bool {
        false
        // matches!(self, AppRoute::Search | AppRoute::SearchHelp)
    }

    pub fn is_settings(&self) -> bool {
        matches!(self, AppRoute::Settings)
    }

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
    pub fn new(
        initial_route: AppRoute,
        state: &mut AppState,
        config: &AppConfiguration,
    ) -> (Self, Box<dyn Screen>) {
        let mut initial_screen = Self::build_screen_from_route(initial_route.clone());
        initial_screen.before_mount(state, config);
        (
            Self {
                navigation_stack: vec![initial_route],
            },
            initial_screen,
        )
    }

    /// Are we on the root screen, *i.e.* the initial screen showed on application launch?
    ///
    /// NB: section tabs like "Ask HN" or similar **do** count as being on the initial screen.
    pub fn is_on_root_screen(&self) -> bool {
        self.navigation_stack
            .last()
            .expect("router: there must be at least 1 screen present in the navigation stack")
            .is_home()
    }

    /// Get the current route state.
    pub fn get_current_route(&self) -> &AppRoute {
        self.navigation_stack
            .last()
            .expect("AppRouter has a current route")
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
            Settings => Box::new(SettingsScreen::new()),
            SearchHelp => Box::new(SearchHelpScreen::new()),
            Home(section) => Box::new(HomeScreen::new(section)),
            ItemDetails(item) => Box::new(StoryDetailsScreen::new(item)),
            ItemNestedComments(parent_comment) => {
                Box::new(NestedCommentsScreen::new(parent_comment))
            }
            UserProfile(user_id) => Box::new(UserDetailsScreen::new(user_id)),
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
