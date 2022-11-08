//! The stories panel lists all the given Hacker News stories.

use std::{convert::TryFrom, io::Stdout};

use async_trait::async_trait;
use fuzzy_matcher::{skim::SkimMatcherV2, FuzzyMatcher};

use tui::{
    backend::CrosstermBackend,
    layout::{Alignment, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, BorderType, Borders, List, ListItem, Paragraph},
};

use crate::{
    api::{types::HnItemIdScalar, HnClient, HnStoriesSections, HnStoriesSorting},
    app::AppContext,
    errors::Result,
    ui::{
        common::{UiComponent, UiComponentId, UiTickScalar},
        displayable_item::DisplayableHackerNewsItem,
        handlers::ApplicationAction,
        router::AppRoute,
        utils::{open_browser_tab, StatefulList},
    },
};

use super::common::COMMON_BLOCK_NORMAL_COLOR;

#[derive(Debug)]
pub struct StoriesPanel {
    ticks_since_last_update: u64,
    sorting_type_for_last_update: Option<HnStoriesSorting>,
    search_for_last_update: Option<String>,
    list_cutoff: usize,
    list_state: StatefulList<HnItemIdScalar, DisplayableHackerNewsItem>,
}

// TODO: load from configuration
const HOME_MAX_DISPLAYED_STORIES: usize = 50;
const MEAN_TICKS_BETWEEN_UPDATES: UiTickScalar = 600; // approx. every minute

impl Default for StoriesPanel {
    fn default() -> Self {
        Self {
            ticks_since_last_update: 0,
            sorting_type_for_last_update: None,
            search_for_last_update: None,
            list_cutoff: HOME_MAX_DISPLAYED_STORIES,
            list_state: StatefulList::with_items(vec![]),
        }
    }
}

const FUZZY_MATCHING_SCORE_CUTOFF: i64 = 90;

impl StoriesPanel {
    fn filtered_items(
        items: impl Iterator<Item = DisplayableHackerNewsItem>,
        filter_query: String,
        max_count: usize,
    ) -> Vec<DisplayableHackerNewsItem> {
        if filter_query.trim().is_empty() {
            return items.take(max_count).collect();
        }
        let matcher = SkimMatcherV2::default();
        let processed_filter_query = filter_query.to_lowercase();
        items
            .filter(move |i| {
                if let Some(fuzzy_score) = matcher.fuzzy_match(
                    i.title
                        .clone()
                        .unwrap_or_else(|| "".into())
                        .to_lowercase()
                        .as_str(),
                    &processed_filter_query,
                ) {
                    fuzzy_score >= FUZZY_MATCHING_SCORE_CUTOFF
                } else {
                    false
                }
            })
            .take(max_count)
            .collect()
    }
}

pub const STORIES_PANEL_ID: UiComponentId = "panel_stories";

// TODO: add loading screen when fetching data
#[async_trait]
impl UiComponent for StoriesPanel {
    fn id(&self) -> UiComponentId {
        STORIES_PANEL_ID
    }

    fn should_update(&mut self, elapsed_ticks: UiTickScalar, ctx: &AppContext) -> Result<bool> {
        self.ticks_since_last_update += elapsed_ticks;

        Ok(self.ticks_since_last_update >= MEAN_TICKS_BETWEEN_UPDATES
            || match &self.sorting_type_for_last_update {
                Some(last_sorting_type) => {
                    last_sorting_type != ctx.get_state().get_main_stories_sorting()
                }
                None => true, // first fetch
            }
            || self.search_for_last_update.as_ref() != ctx.get_state().get_main_search_mode_query())
    }

    async fn update(&mut self, client: &mut HnClient, ctx: &mut AppContext) -> Result<()> {
        self.ticks_since_last_update = 0;

        ctx.get_state_mut().set_main_stories_loading(true);

        let sorting_type = *ctx.get_state().get_main_stories_sorting();
        let search_query = ctx.get_state().get_main_search_mode_query();

        // Data fetching
        let router = ctx.get_router();
        let stories = if let Some(current_section) = router.get_current_route().get_home_section() {
            if current_section == &HnStoriesSections::Home {
                client.get_home_items(&sorting_type).await?
            } else {
                client.get_home_section_items(current_section).await?
            }
        } else {
            client.get_home_items(&sorting_type).await?
        };
        let cut_stories_iter = stories.iter();
        let displayable_stories = cut_stories_iter.cloned().map(|item| {
            DisplayableHackerNewsItem::try_from(item).expect("can map DisplayableHackerNewsItem")
        });

        let filtered_stories = if let Some(filter_query) = search_query {
            Self::filtered_items(displayable_stories, filter_query.clone(), self.list_cutoff)
        } else {
            displayable_stories.take(self.list_cutoff).collect()
        };

        self.list_state.replace_items(filtered_stories);
        if self.list_state.get_state().selected().is_none() {
            self.list_state.get_state().select(Some(0));
        }

        self.sorting_type_for_last_update = Some(sorting_type);
        self.search_for_last_update = search_query.cloned();

        ctx.get_state_mut().set_main_stories_loading(false);

        Ok(())
    }

    fn handle_inputs(&mut self, ctx: &mut AppContext) -> Result<bool> {
        if ctx.get_state().get_main_stories_loading() {
            return Ok(false);
        }

        let inputs = ctx.get_inputs();
        let selected = self.list_state.get_state().selected();
        Ok(if inputs.is_active(&ApplicationAction::NavigateUp) {
            self.list_state.previous();
            true
        } else if inputs.is_active(&ApplicationAction::NavigateDown) {
            self.list_state.next();
            true
        } else if inputs.is_active(&ApplicationAction::OpenHackerNewsLink) {
            let items = self.list_state.get_items();
            let selected_item = &items[selected.unwrap()];
            let item_hn_link = selected_item.get_hacker_news_link();
            open_browser_tab(item_hn_link.as_str());
            true
        } else if inputs.is_active(&ApplicationAction::OpenExternalOrHackerNewsLink) {
            let items = self.list_state.get_items();
            let selected_item = &items[selected.unwrap()];
            let item_link = selected_item
                .url
                .clone()
                .unwrap_or_else(|| selected_item.get_hacker_news_link());
            open_browser_tab(item_link.as_str());
            true
        } else {
            match selected {
                Some(selected_index)
                    if inputs.is_active(&ApplicationAction::SelectItem)
                        && ctx.get_state().get_latest_interacted_with_component()
                            == Some(&STORIES_PANEL_ID) =>
                {
                    // TODO: fix bug where first entry on initial screen cannot be selected
                    let items = self.list_state.get_items();
                    let selected_item = &items[selected_index];
                    ctx.get_state_mut()
                        .set_currently_viewed_item(Some(selected_item.clone()));
                    ctx.router_push_navigation_stack(AppRoute::ItemDetails(selected_item.clone()));
                    true
                }
                _ => false,
            }
        })
    }

    fn render(
        &mut self,
        f: &mut tui::Frame<CrosstermBackend<Stdout>>,
        inside: Rect,
        ctx: &AppContext,
    ) -> Result<()> {
        // Loading case
        if ctx.get_state().get_main_stories_loading() {
            let block = Block::default()
                .style(Style::default().fg(COMMON_BLOCK_NORMAL_COLOR))
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded);

            let text = vec![Spans::from(""), Spans::from("Loading...")];
            let paragraph = Paragraph::new(text)
                .block(block)
                .alignment(Alignment::Center);
            f.render_widget(paragraph, inside);
            return Ok(());
        }

        // General case
        let block_title = match ctx.get_state().get_main_stories_section() {
            HnStoriesSections::Home => "Top stories",
            HnStoriesSections::Ask => "Ask Hacker News",
            HnStoriesSections::Show => "Show Hacker News",
            HnStoriesSections::Jobs => "Jobs",
        };
        let block = Block::default()
            .style(Style::default().fg(COMMON_BLOCK_NORMAL_COLOR))
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .title(block_title);

        // List Items
        let stories = self.list_state.get_items();
        let list_stories_items: Vec<ListItem> = stories
            .iter()
            .map(|item| {
                ListItem::new(Spans::from(vec![Span::styled(
                    item.title.clone().unwrap_or_else(|| "".into()),
                    Style::default().fg(Color::White),
                )]))
            })
            .collect();

        // List
        let list_stories = List::new(list_stories_items)
            .block(block)
            .style(Style::default().fg(Color::White))
            .highlight_style(
                Style::default()
                    .bg(Color::Yellow)
                    .fg(Color::Black)
                    .add_modifier(Modifier::BOLD),
            )
            .highlight_symbol(">> ")
            .highlight_style(Style::default().fg(Color::Yellow));

        f.render_stateful_widget(list_stories, inside, self.list_state.get_state());

        Ok(())
    }
}
