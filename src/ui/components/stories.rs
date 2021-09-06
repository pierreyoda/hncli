//! The stories panel lists all the given Hacker News stories.

use std::{convert::TryFrom, io::Stdout};

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use fuzzy_matcher::{skim::SkimMatcherV2, FuzzyMatcher};

use tui::{
    backend::CrosstermBackend,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, BorderType, Borders, List, ListItem},
};

use crate::{
    api::{
        types::{HnItem, HnItemIdScalar},
        HnClient, HnStoriesSections, HnStoriesSorting,
    },
    app::AppContext,
    errors::{HnCliError, Result},
    ui::{
        common::{UiComponent, UiComponentId, UiTickScalar},
        handlers::ApplicationAction,
        router::AppRoute,
        utils::{datetime_from_hn_time, open_browser_tab, ItemWithId, StatefulList},
    },
};

use super::common::COMMON_BLOCK_NORMAL_COLOR;

/// A display-ready Hacker News story or job posting.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DisplayableHackerNewsItem {
    /// Unique ID.
    pub id: HnItemIdScalar,
    /// Posted at.
    pub posted_at: DateTime<Utc>,
    /// Posted since, formatted for display.
    pub posted_since: String,
    /// Username of the poster.
    pub by_username: String,
    /// Title.
    pub title: String,
    /// Text, if any.
    pub text: Option<String>,
    /// Score.
    pub score: u32,
    /// Item URL, if any.
    pub url: Option<String>,
    /// Hostname of the URL, if any.
    pub url_hostname: Option<String>,
}

impl ItemWithId<HnItemIdScalar> for DisplayableHackerNewsItem {
    fn get_id(&self) -> HnItemIdScalar {
        self.id
    }
}

const MINUTES_PER_DAY: i64 = 24 * 60;

impl DisplayableHackerNewsItem {
    pub fn get_hacker_news_link(&self) -> String {
        format!("https://news.ycombinator.com/item?id={}", self.id)
    }

    pub fn formatted_posted_since(posted_at: &DateTime<Utc>) -> String {
        let now = Utc::now();
        let minutes = (now - *posted_at).num_minutes();
        match minutes {
            _ if minutes >= MINUTES_PER_DAY => {
                format!("{} ago", Self::pluralized(minutes / MINUTES_PER_DAY, "day"))
            }
            _ if minutes >= 60 => format!("{} ago", Self::pluralized(minutes / 60, "hour")),
            _ => format!("{} ago", Self::pluralized(minutes, "minute")),
        }
    }

    fn pluralized(value: i64, word: &str) -> String {
        if value > 1 {
            format!("{} {}s", value, word)
        } else {
            format!("{} {}", value, word)
        }
    }
}

impl TryFrom<HnItem> for DisplayableHackerNewsItem {
    type Error = HnCliError;

    fn try_from(value: HnItem) -> Result<Self> {
        match value {
            HnItem::Story(story) => {
                let posted_at = datetime_from_hn_time(story.time);
                Ok(Self {
                    id: story.id,
                    posted_at,
                    posted_since: Self::formatted_posted_since(&posted_at),
                    by_username: story.by,
                    title: story.title,
                    text: story.text,
                    score: story.score,
                    url: story.url.clone(),
                    url_hostname: story.url.map(|url| {
                        url::Url::parse(&url[..])
                            .map_err(HnCliError::UrlParsingError)
                            .expect("story URL parsing error") // TODO: avoid expect here
                            .host_str()
                            .expect("story URL must have an hostname")
                            .to_owned()
                    }),
                })
            }
            HnItem::Job(job) => {
                let posted_at = datetime_from_hn_time(job.time);
                Ok(Self {
                    id: job.id,
                    posted_at,
                    posted_since: Self::formatted_posted_since(&posted_at),
                    by_username: job.by,
                    title: job.title,
                    text: job.text,
                    score: job.score,
                    url: job.url.clone(),
                    url_hostname: job.url.map(|url| {
                        url::Url::parse(&url[..])
                            .map_err(HnCliError::UrlParsingError)
                            .expect("job URL parsing error") // TODO: avoid expect here
                            .host_str()
                            .expect("job URL must have an hostname")
                            .to_owned()
                    }),
                })
            }
            _ => Err(HnCliError::HnItemProcessingError(
                value.get_id().to_string(),
            )),
        }
    }
}

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

impl StoriesPanel {
    fn filtered_items(
        items: impl Iterator<Item = DisplayableHackerNewsItem>,
        filter_query: String,
    ) -> impl Iterator<Item = DisplayableHackerNewsItem> {
        let matcher = SkimMatcherV2::default();
        items.filter(move |i| {
            matcher
                .fuzzy_match(i.title.as_str(), &filter_query)
                .is_some()
        })
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
            Self::filtered_items(displayable_stories, filter_query.clone())
                .take(self.list_cutoff)
                .collect()
        } else {
            displayable_stories.take(self.list_cutoff).collect()
        };

        self.list_state.replace_items(filtered_stories);
        if self.list_state.get_state().selected().is_none() {
            self.list_state.get_state().select(Some(0));
        }

        self.sorting_type_for_last_update = Some(sorting_type);
        self.search_for_last_update = search_query.cloned();

        Ok(())
    }

    fn handle_inputs(&mut self, ctx: &mut AppContext) -> Result<bool> {
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
        } else if selected.is_some()
            && inputs.is_active(&ApplicationAction::SelectItem)
            && ctx.get_state().get_latest_interacted_with_component() == Some(&STORIES_PANEL_ID)
        {
            let items = self.list_state.get_items();
            let selected_item = &items[selected.unwrap()];
            ctx.get_state_mut()
                .set_currently_viewed_item(Some(selected_item.clone()));
            ctx.router_push_navigation_stack(AppRoute::StoryDetails(selected_item.clone()));
            true
        } else {
            false
        })
    }

    fn render(
        &mut self,
        f: &mut tui::Frame<CrosstermBackend<Stdout>>,
        inside: Rect,
        ctx: &AppContext,
    ) -> Result<()> {
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
                    item.title.clone(),
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
