use ratatui::layout::Rect;

use crate::{
    app::{AppContext, state::AppState},
    ui::{
        components::{navigation::NAVIGATION_ID, settings::SETTINGS_ID},
        flash::{FLASH_MESSAGE_DEFAULT_DURATION_MS, FlashMessage, FlashMessageType},
        handlers::{ApplicationAction, InputsController},
        router::{AppRoute, AppRouter},
        utils::breakpoints::{Breakpoints, BreakpointsDirection},
    },
};

use super::{Screen, ScreenComponentsRegistry, ScreenEventResponse};

/// The settings screen of hncli.
#[derive(Debug)]
pub struct SettingsScreen {
    breakpoints: Breakpoints,
}

impl SettingsScreen {
    pub fn new() -> Self {
        Self {
            breakpoints: Breakpoints::new("settings", &[20, 80])
                .breakpoint(25, &[15, 85])
                .breakpoint(35, &[10, 90])
                .breakpoint(50, &[7, 93]),
        }
    }
}

impl Screen for SettingsScreen {
    fn before_unmount(&mut self, ctx: &mut AppContext) {
        let flash_message = ctx
            .svp()
            .v(crate::app::strings::StringKey::SettingsSavedFlash);
        ctx.get_state_mut().set_flash_message(FlashMessage::new(
            flash_message,
            FlashMessageType::Info,
            FLASH_MESSAGE_DEFAULT_DURATION_MS,
        ));
    }

    fn handle_inputs(
        &mut self,
        inputs: &InputsController,
        router: &mut AppRouter,
        _state: &mut AppState,
    ) -> (ScreenEventResponse, Option<AppRoute>) {
        if inputs.is_active(&ApplicationAction::Back) {
            router.pop_navigation_stack();
            (
                ScreenEventResponse::Caught,
                Some(router.get_current_route().clone()),
            )
        } else {
            (ScreenEventResponse::PassThrough, None)
        }
    }

    fn compute_layout(
        &self,
        frame_size: Rect,
        components_registry: &mut ScreenComponentsRegistry,
        _state: &AppState,
    ) {
        self.breakpoints.apply(
            components_registry,
            &[NAVIGATION_ID, SETTINGS_ID],
            frame_size,
            BreakpointsDirection::Vertical,
        );
    }
}
