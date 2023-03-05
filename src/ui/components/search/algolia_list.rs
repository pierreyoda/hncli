use async_trait::async_trait;
use tui::layout::Rect;

use crate::{
    api::{types::HnItemIdScalar, HnClient},
    app::AppContext,
    errors::Result,
    ui::{
        common::{RenderFrame, UiComponent, UiComponentId, UiTickScalar},
        components::widgets::custom_list::CustomListState,
        displayable_algolia_item::DisplayableAlgoliaItem,
        utils::{debouncer::Debouncer, loader::Loader},
    },
};

/// The Hacker News Algolia results list.
#[derive(Debug)]
pub struct AlgoliaList {
    loading: bool,
    loader: Loader,
    debouncer: Debouncer,
    list_state: CustomListState<u64, DisplayableAlgoliaItem>,
    /// Cached state.
    algolia_query: Option<String>,
}

impl Default for AlgoliaList {
    fn default() -> Self {
        Self {
            loading: true,
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
        return Ok(true);
        self.debouncer.tick(elapsed_ticks);
        Ok(
            ctx.get_state().get_current_algolia_query() != self.algolia_query.as_ref()
                && self.debouncer.is_action_allowed(),
        )
    }

    async fn update(&mut self, client: &mut HnClient, ctx: &mut AppContext) -> Result<()> {
        let state = ctx.get_state();

        let (algolia_query, algolia_filters) = (
            state.get_current_algolia_query(),
            state.get_currently_searched_algolia_categories(),
        );

        // let result = client
        //     .algolia()
        //     .search_stories(algolia_query, &algolia_filters)
        //     .await?;

        let result = client.algolia().search_comments("teqt");

        Ok(())
    }

    fn handle_inputs(&mut self, ctx: &mut AppContext) -> Result<bool> {
        todo!()
    }

    fn render(&mut self, f: &mut RenderFrame, inside: Rect, ctx: &AppContext) -> Result<()> {
        todo!()
    }
}
