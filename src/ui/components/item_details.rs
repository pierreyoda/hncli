use std::{io::Stdout, vec};

use async_trait::async_trait;
use tui::{
    backend::CrosstermBackend,
    layout::{Alignment, Rect},
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
        utils::html_to_plain_text,
    },
};

/// Item details component.
///
/// Does not do any fetching, everything is pre-cached.
///
/// ```md
/// ___________________________________________
/// |                                         |
/// |                <TITLE>                  |
/// |            <URL HOSTNAME?>              |
/// |      <SCORE> POINTS / BY <USERNAME>     |
/// |   <#COMMENTS COUNT>  / POSTED <X> AGO   |
/// |_________________________________________|
/// ```
#[derive(Debug, Default)]
pub struct ItemDetails {
    text: Option<String>,
}

pub const ITEM_DETAILS_ID: UiComponentId = "item_details";

#[async_trait]
impl UiComponent for ItemDetails {
    fn id(&self) -> UiComponentId {
        ITEM_DETAILS_ID
    }

    fn should_update(&mut self, _elapsed_ticks: UiTickScalar, ctx: &AppContext) -> Result<bool> {
        Ok(
            if let Some(item) = ctx.get_state().get_currently_viewed_item() {
                item.text != self.text
            } else {
                false
            },
        )
    }

    async fn update(&mut self, _client: &mut HnClient, ctx: &mut AppContext) -> Result<()> {
        self.text = if let Some(item) = ctx.get_state().get_currently_viewed_item() {
            item.text.clone()
        } else {
            None
        };
        Ok(())
    }

    fn handle_inputs(&mut self, _ctx: &mut AppContext) -> Result<bool> {
        Ok(false)
    }

    fn render(
        &mut self,
        f: &mut tui::Frame<CrosstermBackend<Stdout>>,
        inside: Rect,
        ctx: &AppContext,
    ) -> Result<()> {
        let viewed_item = if let Some(item) = ctx.get_state().get_currently_viewed_item() {
            item
        } else {
            return Ok(());
        };

        let block = Block::default()
            .style(Style::default().fg(COMMON_BLOCK_NORMAL_COLOR))
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded);

        let item_title = viewed_item.title.clone().unwrap_or_else(|| "".into());
        let comments_count = ctx
            .get_state()
            .get_currently_viewed_item_comments()
            .map(|comments| comments.len());
        let text_base = vec![
            Spans::from(item_title),
            Spans::from(viewed_item.url_hostname.clone().unwrap_or_default()),
            Spans::from(format!(
                "{} points by {} {}",
                viewed_item.score, viewed_item.by_username, viewed_item.posted_since
            )),
            Spans::from(if let Some(count) = comments_count {
                format!("{} comments", count)
            } else {
                "".into()
            }),
        ];
        let text_corpus = Self::build_item_text_spans(self, inside, ctx);

        let paragraph = Paragraph::new([text_base, text_corpus].concat())
            .block(block)
            .alignment(Alignment::Center);
        f.render_widget(paragraph, inside);

        Ok(())
    }
}

impl ItemDetails {
    fn build_item_text_spans(&self, inside: Rect, ctx: &AppContext) -> Vec<Spans> {
        if let Some(ref corpus) = self.text {
            if ctx
                .get_state()
                .get_item_page_should_display_comments_panel()
            {
                vec![]
            } else {
                let rendered = html_to_plain_text(corpus.as_str(), inside.width as usize);
                let spans = rendered
                    .split('\n')
                    .map(|line| Spans::from(line.to_string()))
                    .collect();
                [vec![Spans::from(""), Spans::from("")], spans].concat()
            }
        } else {
            vec![]
        }
    }
}
