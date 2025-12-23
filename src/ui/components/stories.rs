//! The stories panel lists all the given Hacker News stories.

use std::convert::TryFrom;

use async_trait::async_trait;

use ratatui::{
    layout::{Alignment, Rect},
    style::{Color, Style},
    text::Line,
    widgets::{Block, BorderType, Borders, Paragraph},
};
use unicode_width::UnicodeWidthStr;

use crate::{
    api::{
        HnClient,
        client::{HnStoriesSections, HnStoriesSorting},
        types::HnItemIdScalar,
    },
    app::AppContext,
    errors::Result,
    ui::{
        common::{RenderFrame, UiComponent, UiComponentId, UiTickScalar},
        displayable_item::DisplayableHackerNewsItem,
        flash::{FLASH_MESSAGE_DEFAULT_DURATION_MS, FlashMessage, FlashMessageType},
        handlers::ApplicationAction,
        router::AppRoute,
        utils::{loader::Loader, open_browser_tab},
    },
};

use super::widgets::custom_list::{CustomList, CustomListState};

#[derive(Debug)]
pub struct StoriesPanel {
    ticks_since_last_update: UiTickScalar,
    loading: bool,
    loader: Loader,
    sorting_type_for_last_update: Option<HnStoriesSorting>,
    list_cutoff: usize,
    list_state: CustomListState<HnItemIdScalar, DisplayableHackerNewsItem>,
}

// TODO: load from configuration
const HOME_MAX_DISPLAYED_STORIES: usize = 50;
const MEAN_TICKS_BETWEEN_UPDATES: UiTickScalar = 1800; // approx. every 3 minute

impl Default for StoriesPanel {
    fn default() -> Self {
        Self {
            ticks_since_last_update: 0,
            loading: true,
            loader: Loader::default(),
            sorting_type_for_last_update: None,
            list_cutoff: HOME_MAX_DISPLAYED_STORIES,
            list_state: CustomListState::with_items(vec![]),
        }
    }
}

pub const STORIES_PANEL_ID: UiComponentId = "panel_stories";

#[async_trait]
impl UiComponent for StoriesPanel {
    fn id(&self) -> UiComponentId {
        STORIES_PANEL_ID
    }

    fn before_unmount(&mut self) {
        self.loader.stop();
    }

    fn should_update(&mut self, elapsed_ticks: UiTickScalar, ctx: &AppContext) -> Result<bool> {
        self.ticks_since_last_update += elapsed_ticks;

        self.loading = self.ticks_since_last_update >= MEAN_TICKS_BETWEEN_UPDATES
            || match &self.sorting_type_for_last_update {
                Some(last_sorting_type) => {
                    last_sorting_type != ctx.get_state().get_main_stories_sorting()
                }
                None => true, // first fetch
            };

        self.loader.update();

        Ok(self.loading)
    }

    async fn update(&mut self, client: &mut HnClient, ctx: &mut AppContext) -> Result<()> {
        self.ticks_since_last_update = 0;
        self.loading = true;

        ctx.get_state_mut().set_main_stories_loading(true);

        let sorting_type = *ctx.get_state().get_main_stories_sorting();

        // Data fetching
        let api = client.classic();
        let router = ctx.get_router();
        let displayable_stories = {
            let fetched_stories =
                if let Some(current_section) = router.get_current_route().get_home_section() {
                    if current_section == &HnStoriesSections::Home {
                        api.get_home_items(&sorting_type).await
                    } else {
                        api.get_home_section_items(current_section).await
                    }
                } else {
                    api.get_home_items(&sorting_type).await
                };
            match fetched_stories {
                Ok(stories) => stories
                    .iter()
                    .take(self.list_cutoff)
                    .cloned()
                    .map(|raw_item| {
                        DisplayableHackerNewsItem::try_from(raw_item)
                            .expect("StoriesPanel.update: can map DisplayableHackerNewsItem")
                    })
                    .collect(),
                _ => {
                    ctx.get_state_mut().set_flash_message(FlashMessage::new(
                        "Could not fetch HackerNews stories.",
                        FlashMessageType::Error,
                        FLASH_MESSAGE_DEFAULT_DURATION_MS,
                    ));
                    vec![]
                }
            }
        };

        self.list_state.replace_items(displayable_stories);
        if self.list_state.selected().is_none() {
            self.list_state.select(Some(0));
        }

        self.sorting_type_for_last_update = Some(sorting_type);

        ctx.get_state_mut().set_main_stories_loading(false);

        self.loading = false;

        Ok(())
    }

    // TODO: when entering then leaving item details, cannot re-enter without moving in the items list
    fn handle_inputs(&mut self, ctx: &mut AppContext) -> Result<bool> {
        if ctx.get_state().get_main_stories_loading() {
            return Ok(false);
        }

        let inputs = ctx.get_inputs();
        let selected = self.list_state.selected();
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
            open_browser_tab(&item_hn_link);
            true
        } else if inputs.is_active(&ApplicationAction::OpenExternalOrHackerNewsLink) {
            let items = self.list_state.get_items();
            let selected_item = &items[selected.unwrap()];
            let item_link = selected_item
                .url
                .clone()
                .unwrap_or_else(|| selected_item.get_hacker_news_link());
            open_browser_tab(&item_link);
            true
        } else if let Some(selected_index) = selected {
            if inputs.is_active(&ApplicationAction::SelectItem)
                && ctx.get_state().get_latest_interacted_with_component() == Some(&STORIES_PANEL_ID)
            {
                let items = self.list_state.get_items();
                let selected_item = &items[*selected_index];
                ctx.get_state_mut()
                    .set_currently_viewed_item(Some(selected_item.clone()));
                ctx.router_push_navigation_stack(AppRoute::ItemDetails(selected_item.clone()));
                true
            } else {
                false
            }
        } else {
            false
        })
    }

    fn render(&mut self, f: &mut RenderFrame, inside: Rect, ctx: &AppContext) -> Result<()> {
        let theme = ctx.get_theme();

        // Loading case
        if ctx.get_state().get_main_stories_loading() || self.loading {
            let block = Block::default()
                .style(Style::default().fg(theme.get_block_color()))
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded);

            let text = vec![Line::from(""), Line::from(self.loader.text())];
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
            .style(Style::default().fg(theme.get_block_color()))
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .title(block_title);

        // Custom List
        let display_story_meta = ctx.get_config().get_display_main_items_list_item_meta();
        let custom_list_stories = CustomList::new(
            &mut self.list_state,
            |rect, buf, item, is_selected| {
                // selected color
                let style = Style::default().fg(if is_selected {
                    Color::Yellow
                } else {
                    Color::White
                });
                // title
                let title = item.title.clone().unwrap_or_default();
                let (x, _) = buf.set_stringn(rect.x, rect.y, title, rect.width as usize, style);
                // (optional) points & comments count
                if !display_story_meta || x >= rect.width {
                    return;
                }
                let meta = format!(
                    "{}, {} score, {} comments",
                    item.posted_since,
                    item.score,
                    item.kids.as_ref().map_or(0, |kids| kids.len()),
                );
                let meta_width = meta.width();
                buf.set_stringn(
                    rect.x + rect.width - (meta_width as u16) - 5,
                    rect.y,
                    meta,
                    meta_width,
                    style,
                );
            },
            |_| 1,
        )
        .block(block)
        .style(Style::default().fg(Color::White))
        .highlight_symbol(">> ")
        .highlight_style(Style::default().fg(Color::Yellow));

        f.render_widget(custom_list_stories, inside);

        Ok(())
    }
}
