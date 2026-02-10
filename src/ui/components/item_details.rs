use async_trait::async_trait;
use ratatui::{
    layout::{HorizontalAlignment, Rect},
    style::Style,
    text::Line,
    widgets::{Block, BorderType, Borders, Paragraph},
};

use crate::{
    api::HnClient,
    app::AppContext,
    errors::Result,
    ui::{
        common::{RenderFrame, UiComponent, UiComponentId, UiTickScalar},
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
    comments_count: Option<usize>,
}

pub const ITEM_DETAILS_ID: UiComponentId = "item_details";

#[async_trait]
impl UiComponent for ItemDetails {
    fn id(&self) -> UiComponentId {
        ITEM_DETAILS_ID
    }

    async fn should_update(
        &mut self,
        _elapsed_ticks: UiTickScalar,
        ctx: &AppContext,
    ) -> Result<bool> {
        let currently_viewed_item = ctx.get_state().get_currently_viewed_item();
        Ok(if let Some(item) = currently_viewed_item {
            item.text != self.text
        } else {
            false
        })
    }

    async fn update(&mut self, _client: &mut HnClient, ctx: &mut AppContext) -> Result<()> {
        self.text = if let Some(item) = ctx.get_state().get_currently_viewed_item() {
            item.text.clone()
        } else {
            None
        };
        self.comments_count = ctx
            .get_state()
            .use_currently_viewed_item_comments(|comments| {
                comments.map(|item_comments| item_comments.len())
            })
            .await;
        Ok(())
    }

    async fn handle_inputs(&mut self, _ctx: &mut AppContext) -> Result<bool> {
        Ok(false)
    }

    fn render(&mut self, f: &mut RenderFrame, inside: Rect, ctx: &AppContext) -> Result<()> {
        let viewed_item = if let Some(item) = ctx.get_state().get_currently_viewed_item() {
            item
        } else {
            return Ok(());
        };

        let theme = ctx.get_theme();

        let block = Block::default()
            .style(Style::default().fg(theme.get_block_color()))
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded);

        let item_title = viewed_item.title.clone().unwrap_or_else(|| "".into());
        let text_base = vec![
            Line::from(item_title),
            Line::from(viewed_item.url_hostname.clone().unwrap_or_default()),
            Line::from(format!(
                "{} points by {} {}",
                viewed_item.score, viewed_item.by_username, viewed_item.posted_since
            )),
            Line::from(if let Some(count) = self.comments_count {
                format!("{count} comments")
            } else {
                "".into()
            }),
        ];
        let text_corpus = Self::build_item_text_line(self, inside, ctx)?;

        let paragraph = Paragraph::new([text_base, text_corpus].concat())
            .block(block)
            .alignment(HorizontalAlignment::Center);
        f.render_widget(paragraph, inside);

        Ok(())
    }
}

impl ItemDetails {
    fn build_item_text_line(&self, inside: Rect, ctx: &AppContext) -> Result<Vec<Line<'_>>> {
        Ok(if let Some(ref corpus) = self.text {
            if ctx
                .get_state()
                .get_item_page_should_display_comments_panel()
            {
                vec![]
            } else {
                let rendered = html_to_plain_text(corpus, inside.width as usize)?;
                let line = rendered
                    .lines()
                    .map(|line| Line::from(line.to_string()))
                    .collect();
                [vec![Line::from(""), Line::from("")], line].concat()
            }
        } else {
            vec![]
        })
    }
}
