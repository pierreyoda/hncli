use crate::{
    api::{
        algolia_types::AlgoliaHnSearchTag,
        client::{HnStoriesSections, HnStoriesSorting},
        types::HnItemIdScalar,
    },
    config::AppConfiguration,
    ui::{
        common::UiComponentId,
        components::{
            stories::STORIES_PANEL_ID,
            widgets::text_input::{TextInputState, TextInputStateAction},
        },
        displayable_item::{DisplayableHackerNewsItem, DisplayableHackerNewsItemComments},
        screens::search::SearchScreenPart,
    },
};

/// Global application state.
/// TODO: avoid some cloning if not too inconvenient (current item viewed / current user from Screens)
#[derive(Debug)]
pub struct AppState {
    /// Latest component interacted with, *i.e.* the latest component having
    /// swallowed an UI event.
    pub(super) latest_interacted_with_component: Option<UiComponentId>,
    /// Main screen(s): loading stories?
    main_stories_loading: bool,
    /// Main screen(s): currently viewed section.
    main_stories_section: HnStoriesSections,
    /// Main screen(s): current stories sorting.
    main_stories_sorting: HnStoriesSorting,
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
    /// The currently viewed user ID.
    currently_viewed_user_id: Option<String>,
    /// The current Hacker News Algolia search state.
    current_algolia_query_state: TextInputState,
    /// The currently used Hacker News Algolia Screen part.
    currently_used_algolia_part: SearchScreenPart,
    /// The currently searched Hacker News Algolia category.
    currently_searched_algolia_category: Option<AlgoliaHnSearchTag>,
}

impl AppState {
    pub fn from_config(config: &AppConfiguration) -> Self {
        Self {
            latest_interacted_with_component: Some(STORIES_PANEL_ID),
            main_stories_loading: true,
            main_stories_section: HnStoriesSections::Home,
            main_stories_sorting: HnStoriesSorting::Top,
            currently_viewed_item: None,
            currently_viewed_item_switched: false,
            currently_viewed_item_comments: None,
            currently_viewed_item_comments_chain: vec![],
            previously_viewed_comment_id: None,
            item_page_display_comments_panel: config.get_display_comments_panel_by_default(),
            currently_viewed_user_id: None,
            current_algolia_query_state: TextInputState::default(),
            currently_used_algolia_part: SearchScreenPart::Filters,
            currently_searched_algolia_category: None,
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

    /// Ensure that there are no invalid (*i.e.*, from our point of view, unfetchable) comment kids in our currently viewed item.
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

    /// Get the currently viewed user ID.
    pub fn get_currently_viewed_user_id(&self) -> Option<&String> {
        self.currently_viewed_user_id.as_ref()
    }

    /// Set the currently viewed user ID.
    pub fn set_currently_viewed_user_id(&mut self, viewed_id: Option<String>) {
        self.currently_viewed_user_id = viewed_id;
    }

    /// Get the current Hacker News Algolia query search state.
    pub fn get_current_algolia_query_state(&self) -> &TextInputState {
        &self.current_algolia_query_state
    }

    /// Mutably get the current Hacker News Algolia query search state.
    pub fn get_current_algolia_query_state_mut(&mut self) -> &mut TextInputState {
        &mut self.current_algolia_query_state
    }

    /// Get the currently used Hacker News Algolia Screen part.
    pub fn get_currently_used_algolia_part(&self) -> SearchScreenPart {
        self.currently_used_algolia_part
    }

    /// Set the currently used Hacker News Algolia Screen part.
    pub fn set_currently_used_algolia_part(&mut self, part: SearchScreenPart) {
        self.currently_used_algolia_part = part;
    }

    /// Get the currently searched Hacker News Algolia category.
    pub fn get_currently_searched_algolia_category(&self) -> Option<&AlgoliaHnSearchTag> {
        self.currently_searched_algolia_category.as_ref()
    }

    /// Set the currently searched Hacker News Algolia category.
    pub fn set_currently_searched_algolia_category(
        &mut self,
        category: Option<AlgoliaHnSearchTag>,
    ) {
        self.currently_searched_algolia_category = category;
    }
}
