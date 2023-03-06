use async_trait::async_trait;
use tui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, BorderType, Borders, Tabs},
};

use crate::{
    api::{algolia_types::AlgoliaHnSearchTag, HnClient},
    app::AppContext,
    errors::Result,
    ui::{
        common::{RenderFrame, UiComponent, UiComponentId, UiTickScalar},
        components::common::COMMON_BLOCK_NORMAL_COLOR,
        handlers::ApplicationAction,
        screens::search::SearchScreenPart,
    },
};

const TABS_TITLES: [&str; 5] = ["Story", "Comment", "Show HN", "Ask HN", "Username"];

/// Component allowing switching between the various Hacker News Algolia tags.
#[derive(Debug)]
pub struct AlgoliaTags {
    titles: Vec<&'static str>,
    hovered_index: usize,
    selected_indices: Vec<bool>,
}

impl Default for AlgoliaTags {
    fn default() -> Self {
        Self {
            titles: TABS_TITLES.to_vec(),
            hovered_index: 0,
            selected_indices: TABS_TITLES.iter().map(|_| false).collect(),
        }
    }
}

impl AlgoliaTags {
    fn next(&mut self) {
        self.hovered_index = (self.hovered_index + 1) % self.titles.len();
    }

    fn previous(&mut self) {
        if self.hovered_index > 0 {
            self.hovered_index -= 1;
        } else {
            self.hovered_index = self.titles.len() - 1;
        }
    }

    fn toggle_search_selection(&mut self, index: usize) {
        assert!(index < self.titles.len());
        self.selected_indices[index] = !self.selected_indices[index];
        if self.selected_indices.iter().all(|activated| !activated) {
            // TODO: find better UX?
            self.selected_indices[0] = true;
        }
        self.enforce_tags_coherency();
    }

    fn apply_search_selections(&self, ctx: &mut AppContext) {
        let mut categories = Vec::with_capacity(self.selected_indices.len());
        for (index, activated) in self.selected_indices.iter().enumerate() {
            if *activated {
                categories.push(search_tag_index_to_algolia_filter(index));
            }
        }
        ctx.get_state_mut()
            .set_currently_searched_algolia_category(categories);
    }

    fn enforce_tags_coherency(&mut self) {
        if self.selected_indices[1] {
            // comments only
            self.selected_indices = TABS_TITLES.iter().map(|_| false).collect();
            self.selected_indices[1] = true;
        } else if self.selected_indices[4] {
            // users only
            self.selected_indices = TABS_TITLES.iter().map(|_| false).collect();
            self.selected_indices[4] = true;
        }
    }
}

fn search_tag_index_to_algolia_filter(index: usize) -> AlgoliaHnSearchTag {
    match index {
        0 => AlgoliaHnSearchTag::Story,
        1 => AlgoliaHnSearchTag::Comment,
        2 => AlgoliaHnSearchTag::ShowHackerNews,
        3 => AlgoliaHnSearchTag::AskHackerNews,
        4 => AlgoliaHnSearchTag::AuthorUsername("".into()),
        _ => unreachable!(),
    }
}

pub const ALGOLIA_TAGS_ID: UiComponentId = "algolia_tags";

#[async_trait]
impl UiComponent for AlgoliaTags {
    fn id(&self) -> UiComponentId {
        ALGOLIA_TAGS_ID
    }

    fn should_update(&mut self, _elapsed_ticks: UiTickScalar, _ctx: &AppContext) -> Result<bool> {
        Ok(false)
    }

    async fn update(&mut self, _client: &mut HnClient, _ctx: &mut AppContext) -> Result<()> {
        Ok(())
    }

    fn handle_inputs(&mut self, ctx: &mut AppContext) -> Result<bool> {
        let inputs = ctx.get_inputs();
        Ok(if inputs.is_active(&ApplicationAction::NavigateLeft) {
            self.previous();
            true
        } else if inputs.is_active(&ApplicationAction::NavigateRight) {
            self.next();
            true
        } else if inputs.is_active(&ApplicationAction::SelectItem) {
            self.toggle_search_selection(self.hovered_index);
            true
        } else {
            false
        })
    }

    fn render(&mut self, f: &mut RenderFrame, inside: Rect, ctx: &AppContext) -> Result<()> {
        let tabs_titles = self
            .titles
            .iter()
            .zip(&self.selected_indices)
            .map(|(title, activated)| {
                Spans::from(vec![Span::styled(
                    *title,
                    Style::default()
                        .fg(Color::White)
                        .add_modifier(if *activated {
                            Modifier::UNDERLINED | Modifier::BOLD
                        } else {
                            Modifier::BOLD
                        }),
                )])
            })
            .collect();

        let tabs_border_style = if matches!(
            ctx.get_state().get_currently_used_algolia_part(),
            SearchScreenPart::Filters
        ) {
            Style::default().fg(Color::Yellow)
        } else {
            Style::default()
        };
        let tabs = Tabs::new(tabs_titles)
            .select(self.hovered_index)
            .block(
                Block::default()
                    .style(Style::default().fg(COMMON_BLOCK_NORMAL_COLOR))
                    .borders(Borders::ALL)
                    .border_type(BorderType::Plain)
                    .border_style(tabs_border_style)
                    .title("Search Filters"),
            )
            .style(Style::default().fg(Color::White))
            .highlight_style(Style::default().fg(Color::LightYellow))
            .divider(Span::raw("/"));

        f.render_widget(tabs, inside);

        Ok(())
    }
}
