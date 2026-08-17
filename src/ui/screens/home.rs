use ratatui::layout::Rect;

use crate::{
    api::client::HnStoriesSections,
    app::{AppContext, state::AppState},
    ui::{
        components::{navigation::NAVIGATION_ID, options::OPTIONS_ID, stories::STORIES_PANEL_ID},
        handlers::InputsController,
        router::{AppRoute, AppRouter},
        utils::breakpoints::{Breakpoints, BreakpointsDirection},
    },
};

use super::{Screen, ScreenComponentsRegistry, ScreenEventResponse};

/// The Home screen of hncli.
///
/// The current layout is as following:
///
/// ```md
/// ------------------------------------------
/// |              navigation                |
/// ------------------------------------------
/// |                                        |
/// |                                        |
/// |               stories                  |
/// |                                        |
/// |                                        |
/// ------------------------------------------
/// |          options (eg. sorting)         |
/// ------------------------------------------
/// ```
#[derive(Debug)]
pub struct HomeScreen {
    section: HnStoriesSections,
    breakpoints: Breakpoints,
}

impl HomeScreen {
    pub fn new(section: HnStoriesSections) -> Self {
        Self {
            section,
            breakpoints: Breakpoints::new("home_screen", &[20, 65, 15])
                .breakpoint(25, &[10, 80, 10])
                .breakpoint(45, &[8, 85, 7]),
        }
    }
}

impl Screen for HomeScreen {
    fn before_mount(&mut self, ctx: &mut AppContext) {
        let state = ctx.get_state_mut();
        state.set_main_stories_section(self.section);

        // Restore the default focus to the stories list, matching the app's initial state.
        // Otherwise, `latest_interacted_with_component` may still point to a component from
        // a previously viewed and now unmounted screen (the item comments panel),
        // which would prevent `StoriesPanel` from reacting to `SelectItem` until the user
        // first navigates (up/down) within the list.
        state.set_latest_interacted_with_component(Some(STORIES_PANEL_ID));
    }

    fn handle_inputs(
        &mut self,
        _inputs: &InputsController,
        _router: &mut AppRouter,
        _state: &mut AppState,
    ) -> (ScreenEventResponse, Option<AppRoute>) {
        (ScreenEventResponse::PassThrough, None)
    }

    fn compute_layout(
        &self,
        frame_size: Rect,
        components_registry: &mut ScreenComponentsRegistry,
        _state: &AppState,
    ) {
        self.breakpoints.apply(
            components_registry,
            &[NAVIGATION_ID, STORIES_PANEL_ID, OPTIONS_ID],
            frame_size,
            BreakpointsDirection::Vertical,
        );
    }
}
