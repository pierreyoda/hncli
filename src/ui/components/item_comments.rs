use std::io::Stdout;

use async_trait::async_trait;
use tui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::Style,
    text::Spans,
    widgets::{Block, BorderType, Borders, Paragraph},
};

use crate::{
    api::HnClient,
    app::AppContext,
    errors::Result,
    ui::{
        common::{UiComponent, UiComponentId, UiTickScalar},
        components::common::COMMON_BLOCK_NORMAL_COLOR,
        displayable_item::{DisplayableHackerNewsItem, DisplayableHackerNewsItemComments},
        utils::html_to_plain_text,
    },
};

use self::comment::render_item_comment;

mod comment;

/// Item comments component.
#[derive(Debug)]
pub struct ItemComments {
    ticks_since_last_update: u64,
    initial_loading: bool,
    viewable_comments: bool,
    comments: DisplayableHackerNewsItemComments,
}

impl Default for ItemComments {
    fn default() -> Self {
        Self {
            initial_loading: true,
            ticks_since_last_update: 0,
            viewable_comments: false,
            comments: DisplayableHackerNewsItemComments::default(),
        }
    }
}

const MEAN_TICKS_BETWEEN_UPDATES: UiTickScalar = 1800; // approx. every 3 minutes

pub const ITEM_COMMENTS_ID: UiComponentId = "item_comments";

#[derive(Debug)]
struct RenderableCommentAssembly {
    corpus: String,
    constraint: Constraint,
}

#[async_trait]
impl UiComponent for ItemComments {
    fn id(&self) -> UiComponentId {
        ITEM_COMMENTS_ID
    }

    fn should_update(&mut self, elapsed_ticks: UiTickScalar, _ctx: &AppContext) -> Result<bool> {
        // TODO: should also update when comments are dirty
        self.ticks_since_last_update += elapsed_ticks;
        Ok(self.initial_loading || self.ticks_since_last_update >= MEAN_TICKS_BETWEEN_UPDATES)
    }

    async fn update(&mut self, client: &mut HnClient, ctx: &mut AppContext) -> Result<()> {
        self.ticks_since_last_update = 0;
        self.viewable_comments = false;

        let viewed_item = match ctx.get_state().get_currently_viewed_item() {
            Some(item) => item,
            None => return Ok(()),
        };
        let viewed_item_kids = match &viewed_item.kids {
            Some(kids) => kids,
            None => return Ok(()),
        };

        let comments_raw = client.get_item_comments(viewed_item_kids).await?;
        self.comments = DisplayableHackerNewsItem::transform_comments(comments_raw)?;
        self.viewable_comments = true;
        self.initial_loading = false;

        Ok(())
    }

    fn handle_inputs(&mut self, _ctx: &mut AppContext) -> Result<bool> {
        Ok(false)
    }

    fn render(
        &mut self,
        f: &mut tui::Frame<CrosstermBackend<Stdout>>,
        inside: Rect,
        _ctx: &AppContext,
    ) -> Result<()> {
        // Initial loading case
        if self.initial_loading {
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

        // No comments case
        if !self.viewable_comments {
            let block = Block::default()
                .style(Style::default().fg(COMMON_BLOCK_NORMAL_COLOR))
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded);

            let text = vec![Spans::from(""), Spans::from("No comments available.")];
            let paragraph = Paragraph::new(text)
                .block(block)
                .alignment(Alignment::Center);
            f.render_widget(paragraph, inside);
            return Ok(());
        }

        // Displayable comments case
        // TODO: this is an MVP rendering function
        let comments_renderables: Vec<RenderableCommentAssembly> = self
            .comments
            .iter()
            .map(|(_, comment)| {
                let text = if let Some(t) = &comment.text {
                    t
                } else {
                    return RenderableCommentAssembly {
                        corpus: "".into(),
                        constraint: Constraint::Min(1),
                    };
                };
                let corpus = html_to_plain_text(text, inside.width as usize);
                let constraint =
                    Constraint::Min(Self::estimate_comment_min_height_from_corpus(&corpus));
                RenderableCommentAssembly { corpus, constraint }
            })
            .collect();
        let comments_constraints: Vec<Constraint> = comments_renderables
            .iter()
            .map(|renderable| renderable.constraint)
            .collect();
        let comments_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(comments_constraints)
            .split(inside);
        for (i, (_, comment)) in self.comments.iter().enumerate() {
            render_item_comment(f, comments_chunks[i], comment);
        }

        Ok(())
    }
}

impl ItemComments {
    fn estimate_comment_min_height_from_corpus(comment_corpus: &String) -> u16 {
        comment_corpus.lines().count() as u16 / 100
    }
}
