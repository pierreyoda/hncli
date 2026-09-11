//! The "resume Item reading" list.

use async_trait::async_trait;
use chrono::Utc;
use log::info;
use ratatui::{
    layout::{HorizontalAlignment, Rect},
    style::{Color, Style},
    text::Line,
    widgets::{Block, BorderType, Borders, Paragraph},
};
use unicode_width::UnicodeWidthStr;

use crate::{
    api::{HnClient, types::HnItemIdScalar},
    app::{
        AppContext,
        history::{HistoryPersistCommand, ResumeItemHistoryData, SynchronizedHistoryItem},
    },
    errors::Result,
    ui::{
        common::{RenderFrame, UiComponent, UiComponentId, UiTickScalar},
        components::widgets::custom_list::{CustomList, CustomListState},
        displayable_item::DisplayableHackerNewsItem,
        flash::{FLASH_MESSAGE_DEFAULT_DURATION_MS, FlashMessage, FlashMessageType},
        handlers::ApplicationAction,
        router::AppRoute,
        utils::{ItemWithId, open_browser_tab},
    },
};

const RESUME_SCREEN_MAX_DISPLAYED_ITEMS: usize = 50;

impl ItemWithId<HnItemIdScalar> for ResumeItemHistoryData {
    fn get_id(&self) -> HnItemIdScalar {
        self.get_value()
    }
}

#[derive(Debug)]
pub struct ResumeList {
    loaded: bool,
    list_cutoff: usize,
    /// The Item selected in the list, waiting to be fetched.
    ///
    /// Inputs handling has no access to the API client: the fetching, and the
    /// ensuing navigation, are deferred to the next `update` call.
    pending_selected_item_id: Option<HnItemIdScalar>,
    list_state: CustomListState<HnItemIdScalar, ResumeItemHistoryData>,
}

impl Default for ResumeList {
    fn default() -> Self {
        Self {
            loaded: false,
            list_cutoff: RESUME_SCREEN_MAX_DISPLAYED_ITEMS,
            pending_selected_item_id: None,
            list_state: CustomListState::with_items(vec![]),
        }
    }
}

pub const RESUME_LIST_ID: UiComponentId = "resume_list";

#[async_trait]
impl UiComponent for ResumeList {
    fn id(&self) -> UiComponentId {
        RESUME_LIST_ID
    }

    fn before_unmount(&mut self) {
        self.loaded = false;
        self.pending_selected_item_id = None;
    }

    async fn should_update(
        &mut self,
        _elapsed_ticks: UiTickScalar,
        _ctx: &AppContext,
    ) -> Result<bool> {
        Ok(!self.loaded || self.pending_selected_item_id.is_some())
    }

    async fn update(&mut self, client: &mut HnClient, ctx: &mut AppContext) -> Result<()> {
        if !self.loaded {
            let resume_items: Vec<_> = ctx
                .get_history()
                .restored_resume_items()
                .into_iter()
                .take(self.list_cutoff)
                .cloned()
                .collect();
            self.list_state.replace_items(resume_items);
            if !self.list_state.is_empty() && self.list_state.selected().is_none() {
                self.list_state.select(Some(0));
            }
            self.loaded = true;
        }

        if let Some(item_id) = self.pending_selected_item_id.take() {
            let raw_item = match client.classic().await.get_item(item_id).await {
                Ok(raw) => raw,
                Err(_) => {
                    ctx.get_state_mut().set_flash_message(FlashMessage::new(
                        "Could not open this Item, it may no longer be available.",
                        FlashMessageType::Error,
                        FLASH_MESSAGE_DEFAULT_DURATION_MS,
                    ));
                    return Ok(());
                }
            };
            let item_result: Result<DisplayableHackerNewsItem> = raw_item.try_into();
            match item_result {
                Ok(item) => {
                    ctx.router_push_navigation_stack(AppRoute::ItemDetails(item));
                }
                Err(_) => {
                    ctx.get_state_mut().set_flash_message(FlashMessage::new(
                        "Could not open this Item, it may no longer be available.",
                        FlashMessageType::Error,
                        FLASH_MESSAGE_DEFAULT_DURATION_MS,
                    ));
                    return Ok(());
                }
            }
        }

        Ok(())
    }

    async fn handle_inputs(&mut self, ctx: &mut AppContext) -> Result<bool> {
        if self.list_state.is_empty() {
            return Ok(false);
        }

        let inputs = ctx.get_inputs();
        let selected = *self.list_state.selected();
        Ok(if inputs.is_active(&ApplicationAction::NavigateUp) {
            self.list_state.previous();
            ctx.get_state_mut()
                .set_resume_screen_confirming_deletion(false);
            true
        } else if inputs.is_active(&ApplicationAction::NavigateDown) {
            self.list_state.next();
            ctx.get_state_mut()
                .set_resume_screen_confirming_deletion(false);
            true
        } else if let Some(selected_index) = selected {
            let selected_item_id = self.list_state.get_items()[selected_index].get_id();
            if inputs.is_active(&ApplicationAction::ResumeClearEntry) {
                if ctx.get_state().get_resume_screen_confirming_deletion() {
                    ctx.get_history_mut()
                        .persist(&[HistoryPersistCommand::ResumeRemove {
                            item_id: selected_item_id,
                        }]);
                    ctx.get_state_mut()
                        .set_resume_screen_confirming_deletion(false);
                    self.loaded = false;
                } else {
                    ctx.get_state_mut()
                        .set_resume_screen_confirming_deletion(true);
                }
                true
            } else if inputs.is_active(&ApplicationAction::OpenHackerNewsLink)
                // the external URL of a stored Item, if any, is unknown until it is
                // fetched: both link actions can only open its Hacker News page here
                || inputs.is_active(&ApplicationAction::OpenExternalOrHackerNewsLink)
            {
                open_browser_tab(&DisplayableHackerNewsItem::hacker_news_link_for(
                    selected_item_id,
                ));
                true
            } else if inputs.is_active(&ApplicationAction::SelectItem)
                && ctx.get_state().get_latest_interacted_with_component() == Some(&RESUME_LIST_ID)
            {
                self.pending_selected_item_id = Some(selected_item_id);
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

        let block = Block::default()
            .style(Style::default().fg(theme.get_block_color()))
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded);

        // Empty case
        if self.list_state.is_empty() {
            let text = vec![Line::from(""), Line::from("No items in history.")];
            let paragraph = Paragraph::new(text)
                .block(block)
                .alignment(HorizontalAlignment::Center);
            f.render_widget(paragraph, inside);
            return Ok(());
        }

        // Custom List
        let now = Utc::now();
        let custom_list_resume_items = CustomList::new(
            &mut self.list_state,
            |rect, buf, item, is_selected| {
                // selected color
                let style = Style::default().fg(if is_selected {
                    theme.get_accent_color()
                } else {
                    Color::White
                });
                // title, as stored when the Item was last read
                let (x, _) =
                    buf.set_stringn(rect.x, rect.y, item.get_label(), rect.width as usize, style);
                let last_read_minutes = (now - *item.get_timestamp()).num_minutes();
                if x >= rect.width || last_read_minutes < 1 {
                    return;
                }
                let meta = format!(
                    "last read {}",
                    DisplayableHackerNewsItem::formatted_posted_since(item.get_timestamp())
                );
                let meta_width = meta.width();
                buf.set_stringn(
                    rect.x + rect.width.saturating_sub(meta_width as u16 + 5),
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
        .highlight_style(Style::default().fg(theme.get_accent_color()));

        f.render_widget(custom_list_resume_items, inside);

        Ok(())
    }
}
