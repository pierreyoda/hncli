use std::collections::HashMap;

use crossterm::event::KeyEvent;
use tui::layout::Rect;

use crate::{
    api::{types::HnItemIdScalar, HnStoriesSections, HnStoriesSorting},
    config::AppConfiguration,
    ui::{
        common::UiComponentId,
        components::stories::STORIES_PANEL_ID,
        displayable_item::{DisplayableHackerNewsItem, DisplayableHackerNewsItemComments},
        handlers::{ApplicationAction, InputsController},
        router::{AppRoute, AppRouter},
        screens::{Screen, ScreenComponentsRegistry, ScreenEventResponse},
    },
};

use self::history::AppHistory;

pub mod history;

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
        if route.is_settings() || route.is_help() {
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

/// Global application state.
#[derive(Debug)]
pub struct AppState {
    /// Latest component interacted with, *i.e.* the latest component having
    /// swallowed an UI event.
    latest_interacted_with_component: Option<UiComponentId>,
    /// Main screen(s): loading stories?
    main_stories_loading: bool,
    /// Main screen(s): currently viewed section.
    main_stories_section: HnStoriesSections,
    /// Main screen(s): current stories sorting.
    main_stories_sorting: HnStoriesSorting,
    /// Main screen(s): search query if in search mode.
    main_search_mode_query: Option<String>,
    /// The currently viewed item (not a comment).
    currently_viewed_item: Option<DisplayableHackerNewsItem>,
    /// Has the currently viewed item (not a comment) changed recently?
    currently_viewed_item_switched: bool,
    /// The comments of the currently viewed item, if applicable.
    currently_viewed_item_comments: Option<DisplayableHackerNewsItemComments>,
    /// The successive IDs of the viewed comment, starting at the root parent comment.
    currently_viewed_item_comments_chain: Vec<HnItemIdScalar>,
    /// The ID of the comment to restore when coming back from a sub-comment.
    previously_viewed_comment_id: Option<HnItemIdScalar>,
    /// Item details screen: is the comments panel visible or not.
    item_page_display_comments_panel: bool,
}

impl AppState {
    fn from_config(config: &AppConfiguration) -> Self {
        Self {
            latest_interacted_with_component: Some(STORIES_PANEL_ID),
            main_stories_loading: true,
            main_stories_section: HnStoriesSections::Home,
            main_stories_sorting: HnStoriesSorting::Top,
            main_search_mode_query: None,
            currently_viewed_item: None,
            currently_viewed_item_switched: false,
            currently_viewed_item_comments: None,
            currently_viewed_item_comments_chain: vec![],
            previously_viewed_comment_id: None,
            item_page_display_comments_panel: config.get_display_comments_panel_by_default(),
        }
    }
}

impl AppState {
    /// Get the latest component interacted with.
    pub fn get_latest_interacted_with_component(&self) -> Option<&UiComponentId> {
        self.latest_interacted_with_component.as_ref()
    }

    /// Get the are the main stories loading boolean.
    pub fn get_main_stories_loading(&self) -> bool {
        self.main_stories_loading
    }

    /// Set the are the main stories loading boolean.
    pub fn set_main_stories_loading(&mut self, loading: bool) {
        self.main_stories_loading = loading;
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

    /// Get the main screens search mode query, if any.
    pub fn get_main_search_mode_query(&self) -> Option<&String> {
        self.main_search_mode_query.as_ref()
    }

    /// Set the main screens search mode query.
    pub fn set_main_search_mode_query(&mut self, query: Option<String>) {
        self.main_search_mode_query = query;
    }

    /// Get the currently viewed item.
    pub fn get_currently_viewed_item(&self) -> Option<&DisplayableHackerNewsItem> {
        self.currently_viewed_item.as_ref()
    }

    /// Set the currently viewed item.
    pub fn set_currently_viewed_item(&mut self, viewed: Option<DisplayableHackerNewsItem>) {
        self.currently_viewed_item = viewed;
        self.currently_viewed_item_switched = true;
    }

    /// Get has the currently viewed item (not a comment) changed recently?
    pub fn get_currently_viewed_item_switched(&self) -> bool {
        self.currently_viewed_item_switched
    }

    /// Get the comments of the currently viewed item.
    pub fn get_currently_viewed_item_comments(&self) -> Option<&DisplayableHackerNewsItemComments> {
        self.currently_viewed_item_comments.as_ref()
    }

    /// Set the comments of the currently viewed item.
    pub fn set_currently_viewed_item_comments(
        &mut self,
        comments: Option<DisplayableHackerNewsItemComments>,
    ) {
        self.update_currently_viewed_item_kids_from_fetched_comments(&comments);

        // Different item: replace the comments
        if self.currently_viewed_item_switched {
            self.currently_viewed_item_comments = comments;
            self.currently_viewed_item_switched = false;
            return;
        }
        // Same item: merge the comments (since some children comments may be new)
        if let Some(current_comments_cache) = &mut self.currently_viewed_item_comments {
            if let Some(incoming_comments_cache) = comments {
                for (incoming_comment_id, incoming_comment) in incoming_comments_cache {
                    // we prefer the freshly updated comments over potentially outdated ones
                    current_comments_cache.insert(incoming_comment_id, incoming_comment);
                }
            }
            // else: when no further children comments are found, we preserve our current comments cache for this item
        } else {
            self.currently_viewed_item_comments = comments;
        }
    }

    /// Ensure that there are now invalid (*i.e.*, from our point of view, unfetchable) comment kids in our currently viewed item.
    fn update_currently_viewed_item_kids_from_fetched_comments(
        &mut self,
        comments: &Option<DisplayableHackerNewsItemComments>,
    ) {
        if let Some(viewed_item) = &mut self.currently_viewed_item {
            if let Some(cached_comments) = comments {
                viewed_item.kids = viewed_item.kids.as_ref().map(|kids| {
                    kids.iter()
                        .filter(|kid_id| cached_comments.contains_key(*kid_id))
                        .cloned()
                        .collect()
                });
            }
        }
    }

    /// Reset the successively viewed comments for the currently viewed item.
    pub fn reset_currently_viewed_item_comments_chain(&mut self) {
        self.currently_viewed_item_comments_chain.clear();
    }

    /// Get the successively viewed comments for the currently viewed item.
    pub fn get_currently_viewed_item_comments_chain(&self) -> &[HnItemIdScalar] {
        &self.currently_viewed_item_comments_chain
    }

    /// Push a new comment ID to the successively viewed comments for the currently viewed item.
    pub fn push_currently_viewed_item_comments_chain(&mut self, comment_id: HnItemIdScalar) {
        match self.currently_viewed_item_comments_chain.last() {
            Some(latest_comment_id) if latest_comment_id != &comment_id => {
                self.currently_viewed_item_comments_chain.push(comment_id)
            }
            None if self.currently_viewed_item_comments_chain.is_empty() => {
                self.currently_viewed_item_comments_chain.push(comment_id)
            }
            _ => (),
        };
    }

    /// Replace the latest comment ID in the successively viewed comments for the currently viewed item.
    pub fn replace_latest_in_currently_viewed_item_comments_chain(
        &mut self,
        comment_id_option: Option<HnItemIdScalar>,
    ) {
        if let Some(comment_id) = comment_id_option {
            self.currently_viewed_item_comments_chain.pop();
            self.currently_viewed_item_comments_chain.push(comment_id);
        }
    }

    /// Pop the latest comment ID from the successively viewed comments for the currently viewed item.
    ///
    /// Also returns the newly last comment, *i.e.* the now currently viewed comment, if any.
    pub fn pop_currently_viewed_item_comments_chain(&mut self) -> Option<HnItemIdScalar> {
        self.currently_viewed_item_comments_chain.pop();
        self.currently_viewed_item_comments_chain.last().cloned()
    }

    /// Get the ID of the comment to restore when coming back from a sub-comment.
    pub fn get_previously_viewed_comment_id(&self) -> Option<HnItemIdScalar> {
        self.previously_viewed_comment_id
    }

    /// Set the ID of the comment to restore when coming back from a sub-comment.
    pub fn set_previously_viewed_comment_id(&mut self, comment_id: Option<HnItemIdScalar>) {
        self.previously_viewed_comment_id = comment_id;
    }

    /// Get the is comments panel visible on item details screen boolean.
    pub fn get_item_page_should_display_comments_panel(&self) -> bool {
        self.item_page_display_comments_panel
    }

    /// Set the is comments panel visible on item details screen boolean.
    pub fn set_item_page_should_display_comments_panel(&mut self, value: bool) {
        self.item_page_display_comments_panel = value;
    }
}

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
        // global help page toggle
        if self.inputs.is_active(&ApplicationAction::ToggleHelp) {
            if self.router.get_current_route().is_help() {
                self.get_context().router_pop_navigation_stack();
            } else {
                self.get_context()
                    .router_push_navigation_stack(AppRoute::Help);
            }
            return true;
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
