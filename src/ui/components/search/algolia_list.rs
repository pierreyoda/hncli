use async_trait::async_trait;
use ratatui::{
    layout::{Alignment, Rect},
    style::{Color, Style},
    text::Line,
    widgets::{Block, BorderType, Borders, Paragraph},
};
use unicode_width::UnicodeWidthStr;

use crate::{
    api::{HnClient, algolia_types::AlgoliaHnSearchTag},
    app::AppContext,
    errors::Result,
    ui::{
        common::{RenderFrame, UiComponent, UiComponentId, UiTickScalar},
        components::{
            common::{COMMON_BLOCK_NORMAL_COLOR, render_text_message},
            widgets::custom_list::{CustomList, CustomListState},
        },
        displayable_algolia_item::{
            DisplayableAlgoliaComment, DisplayableAlgoliaItem, DisplayableAlgoliaStory,
        },
        handlers::ApplicationAction,
        screens::search::SearchScreenPart,
        utils::{debouncer::Debouncer, loader::Loader, open_browser_tab},
    },
};

#[derive(Debug)]
enum AlgoliaListStatus {
    Unselected,
    Selected,
    Focused,
}

/// The Hacker News Algolia results list.
#[derive(Debug)]
pub struct AlgoliaList {
    status: AlgoliaListStatus,
    empty_input: bool,
    loading: bool,
    loader: Loader,
    debouncer: Debouncer,
    list_state: CustomListState<u64, DisplayableAlgoliaItem>,
    /// Cached query state.
    algolia_query: Option<String>,
}

impl Default for AlgoliaList {
    fn default() -> Self {
        Self {
            status: AlgoliaListStatus::Unselected,
            empty_input: false,
            loading: false,
            loader: Loader::default(),
            debouncer: Debouncer::new(5),
            list_state: CustomListState::with_items(vec![]),
            algolia_query: None,
        }
    }
}

pub const ALGOLIA_LIST_ID: UiComponentId = "algolia_list";

#[async_trait]
impl UiComponent for AlgoliaList {
    fn id(&self) -> UiComponentId {
        ALGOLIA_LIST_ID
    }

    fn before_unmount(&mut self) {
        self.loader.stop();
    }

    fn should_update(&mut self, elapsed_ticks: UiTickScalar, ctx: &AppContext) -> Result<bool> {
        self.debouncer.tick(elapsed_ticks);
        let should_update = Some(
            ctx.get_state()
                .get_current_algolia_query_state()
                .get_value(),
        ) != self.algolia_query.as_ref()
            && self.debouncer.is_action_allowed();
        if should_update {
            self.loading = true;
        }
        self.loader.update();

        Ok(should_update)
    }

    async fn update(&mut self, client: &mut HnClient, ctx: &mut AppContext) -> Result<()> {
        self.loading = true;
        self.status = match ctx.get_state().get_currently_used_algolia_part() {
            SearchScreenPart::Results(false) => AlgoliaListStatus::Selected,
            SearchScreenPart::Results(true) => AlgoliaListStatus::Focused,
            _ => AlgoliaListStatus::Unselected,
        };

        let state = ctx.get_state();
        let (algolia_query, algolia_category) = (
            state.get_current_algolia_query_state().get_value(),
            state.get_currently_searched_algolia_category(),
        );

        self.empty_input = algolia_query.is_empty();
        if self.empty_input {
            self.list_state.clear();
            return Ok(());
        }

        if let Some(category) = algolia_category {
            self.algolia_query = Some(algolia_query.clone());

            let (for_stories, for_comments, for_usernames) = (
                matches!(category, AlgoliaHnSearchTag::Story),
                matches!(category, AlgoliaHnSearchTag::Comment),
                matches!(category, AlgoliaHnSearchTag::AuthorUsername(_)),
            );

            // TODO: avoid .clones
            let displayable_algolia_items = if for_stories {
                let results = client
                    .algolia()
                    .search_stories(algolia_query, &[AlgoliaHnSearchTag::Story])
                    .await?;
                results
                    .get_hits()
                    .iter()
                    .map(|i| {
                        DisplayableAlgoliaItem::Story(DisplayableAlgoliaStory::from(i.clone()))
                    })
                    .collect()
            } else if for_comments {
                let results = client.algolia().search_comments(algolia_query).await?;
                results
                    .get_hits()
                    .iter()
                    .map(|i| {
                        DisplayableAlgoliaItem::Comment(DisplayableAlgoliaComment::from(i.clone()))
                    })
                    .collect()
            } else if for_usernames {
                let results = client.algolia().search_user_stories(algolia_query).await?;
                results
                    .get_hits()
                    .iter()
                    .map(|i| {
                        DisplayableAlgoliaItem::Story(DisplayableAlgoliaStory::from(i.clone()))
                    })
                    .collect()
            } else {
                vec![]
            };
            self.list_state.replace_items(displayable_algolia_items);

            if matches!(self.status, AlgoliaListStatus::Focused)
                && !self.list_state.is_empty()
                && self.list_state.selected().is_none()
            {
                self.list_state.select(Some(0));
            }

            self.loading = false;
        }

        Ok(())
    }

    fn handle_inputs(&mut self, ctx: &mut AppContext) -> Result<bool> {
        if self.loading || !matches!(self.status, AlgoliaListStatus::Focused) {
            return Ok(false);
        }

        let (inputs, selected) = (ctx.get_inputs(), self.list_state.selected());

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
            let item_link = selected_item.get_link();
            if let Some(url) = item_link {
                open_browser_tab(&url);
            }
            true
        } else {
            false
        })
    }

    fn render(&mut self, f: &mut RenderFrame, inside: Rect, _ctx: &AppContext) -> Result<()> {
        let block_border_style = Style::default().fg(match self.status {
            AlgoliaListStatus::Unselected => Color::White,
            AlgoliaListStatus::Selected => Color::Yellow,
            AlgoliaListStatus::Focused => Color::Green,
        });

        // Empty input case
        if self.empty_input {
            let block = Block::default()
                .style(Style::default().fg(COMMON_BLOCK_NORMAL_COLOR))
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(block_border_style);

            let text = vec![Line::from("No search input.")];
            let paragraph = Paragraph::new(text)
                .block(block)
                .alignment(Alignment::Center);
            f.render_widget(paragraph, inside);
            return Ok(());
        }

        // Loading case
        if self.loading {
            let block = Block::default()
                .style(Style::default().fg(COMMON_BLOCK_NORMAL_COLOR))
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(block_border_style);

            let text = vec![Line::from(""), Line::from(self.loader.text())];
            let paragraph = Paragraph::new(text)
                .block(block)
                .alignment(Alignment::Center);
            f.render_widget(paragraph, inside);
            return Ok(());
        }

        // Empty case
        if self.list_state.is_empty() {
            render_text_message(f, inside, "No results...");
            return Ok(());
        }

        // Custom List
        let block = Block::default()
            .style(Style::default())
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(block_border_style)
            .title("Search results");
        let custom_list_results = CustomList::new(
            &mut self.list_state,
            |rect, buf, item, is_selected| {
                // selected color
                let style = Style::default().fg(if is_selected {
                    Color::Yellow
                } else {
                    Color::White
                });
                // title
                let title = item.title();
                let (x, _) = buf.set_stringn(rect.x, rect.y, title, rect.width as usize, style);
                // meta information
                if x >= rect.width {
                    return;
                }
                let meta = item.meta();
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

        f.render_widget(custom_list_results, inside);

        Ok(())
    }
}
