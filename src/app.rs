use crate::api::HnStoriesSorting;

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
    HomeStories,
    Thread,
    Help,
}

#[derive(Debug)]
pub enum Route {
    SplashScreen,
    Home,
    Help,
}

#[derive(Debug)]
pub struct RouteState {
    pub route: Route,
    pub active_block: AppBlock,
    pub hovered_block: AppBlock,
}

/// Global application state.
#[derive(Debug)]
pub struct App {
    /// The current navigation stack.
    ///
    /// Example: home > story #1 details > comment #2 thread
    /// Home screen: current stories sorting.
    home_stories_sorting: HnStoriesSorting,
}

impl Default for App {
    fn default() -> Self {
        Self {
            home_stories_sorting: HnStoriesSorting::Top,
        }
    }
}

impl App {}
