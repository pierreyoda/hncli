use std::{
    collections::HashMap,
    io::Stdout,
    sync::mpsc::{self, Receiver},
    thread,
    time::{Duration, Instant},
};

use crossterm::{
    event::{self, Event, KeyEvent},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use flash::FlashMessage;
use ratatui::{
    Terminal,
    backend::CrosstermBackend,
    layout::{Direction, Layout},
};

use common::{UiComponent, UiComponentId, UiTickScalar};
use components::{help::Help, navigation::Navigation, options::Options, stories::StoriesPanel};
use utils::breakpoints::Breakpoints;

use crate::{
    api::HnClient,
    app::App,
    config::AppConfiguration,
    errors::{HnCliError, Result},
    ui::flash::{FlashMessageDurationType, FlashMessageType},
};

use self::{
    components::{
        help_search::AlgoliaHelp,
        item_comments::{CommentItemNestedComments, ItemTopLevelComments},
        item_details::ItemDetails,
        item_summary::ItemSummary,
        search::{
            Search, algolia_input::AlgoliaInput, algolia_list::AlgoliaList,
            algolia_tags::AlgoliaTags,
        },
        settings::Settings,
        user_profile::UserProfile,
    },
    handlers::ApplicationAction,
    helper::ContextualHelper,
    screens::search::SearchScreenPart,
};

pub mod common;
pub mod components;
pub mod displayable_algolia_item;
pub mod displayable_item;
mod flash;
pub mod handlers;
mod helper;
mod panels;
pub mod router;
pub mod screens;
pub mod utils;

const FLASH_MESSAGE_DURATION_MS: usize = 2000;

type TerminalUi = Terminal<CrosstermBackend<Stdout>>;

#[derive(Clone, Debug)]
pub enum UserInterfaceEvent {
    KeyEvent(KeyEvent),
    Tick,
}

pub struct ComponentWrapper {
    component: Box<dyn UiComponent>,
    ticks_elapsed: UiTickScalar,
    /// An active component will update itself.
    active: bool,
}

impl ComponentWrapper {
    pub fn from_component(component: Box<dyn UiComponent>) -> Self {
        Self {
            component,
            ticks_elapsed: 0,
            active: true,
        }
    }
}

pub struct UserInterface {
    terminal: TerminalUi,
    client: HnClient,
    app: App,
    /// Components registry.
    components: HashMap<UiComponentId, ComponentWrapper>,
}

pub const UI_TICK_RATE_MS: u64 = 100;
pub const FLASH_MESSAGE_DEFAULT_DURATION_MS: FlashMessageDurationType = 3000;

impl UserInterface {
    /// Create a new `UserInterface` instance and prepare the terminal for it.
    pub fn new(mut terminal: TerminalUi, client: HnClient) -> Result<Self> {
        enable_raw_mode()
            .map_err(|_| HnCliError::CrosstermError("enable_raw_mode error".into()))?;
        terminal
            .clear()
            .map_err(|_| HnCliError::CrosstermError("clear error".into()))?;
        terminal
            .hide_cursor()
            .map_err(|_| HnCliError::CrosstermError("hide_cursor error".into()))?;

        let config = AppConfiguration::from_file_or_defaults();
        Ok(Self {
            terminal,
            client,
            app: App::new(config),
            components: HashMap::new(),
        })
    }

    /// Set up the Event Loop channels and the various UI components.
    pub fn setup(&mut self) -> Result<Receiver<UserInterfaceEvent>> {
        // event loop
        let tick_rate = Duration::from_millis(UI_TICK_RATE_MS);
        let (tx, rx) = mpsc::channel();
        thread::spawn(move || {
            let mut last_tick = Instant::now();
            loop {
                let timeout = tick_rate
                    .checked_sub(last_tick.elapsed())
                    .unwrap_or_else(|| Duration::from_secs(0));

                if event::poll(timeout).expect("event polling works") {
                    if let Event::Key(key_event) = event::read().unwrap() {
                        tx.send(UserInterfaceEvent::KeyEvent(key_event)).unwrap();
                    }
                }

                if last_tick.elapsed() >= tick_rate && tx.send(UserInterfaceEvent::Tick).is_ok() {
                    last_tick = Instant::now();
                }
            }
        });

        // Components registration
        self.register_component(Help::default());
        self.register_component(Settings::default());
        self.register_component(Navigation::default());
        self.register_component(Search::default());
        self.register_component(StoriesPanel::default());
        self.register_component(ItemDetails::default());
        self.register_component(ItemSummary::default());
        self.register_component(ItemTopLevelComments::default());
        self.register_component(CommentItemNestedComments::default());
        self.register_component(AlgoliaTags::default());
        self.register_component(AlgoliaInput::default());
        self.register_component(AlgoliaList::default());
        self.register_component(AlgoliaHelp::default());
        self.register_component(UserProfile::default());
        self.register_component(Options::default());

        for component_wrapper in self.components.values_mut() {
            component_wrapper
                .component
                .before_mount(&mut self.app.get_context());
        }

        Ok(rx)
    }

    /// Launch the main UI loop.
    pub async fn run(&mut self, rx: Receiver<UserInterfaceEvent>) -> Result<()> {
        self.terminal
            .hide_cursor()
            .map_err(|_| HnCliError::CrosstermError("hide_cursor error".into()))?;

        // Flash message setup
        // TODO: adapt to other colors when needed
        let flash_message =
            FlashMessage::new(FlashMessageType::Warning, FLASH_MESSAGE_DEFAULT_DURATION_MS);
        let mut had_flash_message = false;
        let mut flash_message_elapsed_ticks: UiTickScalar = 0;

        // Contextual help setup
        let contextual_helper = ContextualHelper::default();
        let breakpoints_default = Breakpoints::new("ui_default", &[100, 0]);
        let breakpoints_helper = Breakpoints::new("ui_helper", &[90, 10]).breakpoint(30, &[97, 3]);
        let breakpoints_flash = Breakpoints::new("ui_flash", &[80, 20]).breakpoint(40, &[80, 20]);

        'ui: loop {
            let app = &mut self.app;
            let components = &mut self.components;
            self.terminal
                .draw(|frame| {
                    let has_flash_message = app.get_context().get_state().has_flash_message();
                    let show_contextual_help =
                        app.get_context().get_config().get_show_contextual_help();

                    // global layout
                    let breakpoints = match (show_contextual_help, has_flash_message) {
                        (true, _) => &breakpoints_helper,
                        (false, true) => &breakpoints_flash,
                        _ => &breakpoints_default,
                    };
                    let frame_size = frame.size();
                    let global_layout_chunks = Layout::default()
                        .direction(Direction::Vertical)
                        .constraints(breakpoints.to_constraints(frame_size.height))
                        .split(frame_size);

                    // refresh application chunks
                    let (previous_components_ids, current_components_ids) =
                        app.update_layout(global_layout_chunks[0]);
                    for previous_component_id in previous_components_ids {
                        if !current_components_ids.contains(&previous_component_id) {
                            components
                                .get_mut(previous_component_id)
                                .expect(&format!(
                                    "main UI loop: no component found for: {}",
                                    previous_component_id
                                ))
                                .component
                                .as_mut()
                                .before_unmount();
                        }
                    }

                    // render components
                    for (id, wrapper) in components.iter_mut() {
                        let component_rendering_rect =
                            app.get_component_rendering_rect(id).cloned();
                        wrapper.active = component_rendering_rect.is_some();
                        let app_context = app.get_context();
                        match component_rendering_rect {
                            None => (), // no rendering
                            Some(inside_rect) => wrapper
                                .component
                                .render(frame, inside_rect, &app_context)
                                .expect("main UI loop: no component rendering error"),
                        }
                    }

                    // render flash message
                    if has_flash_message {
                        if !had_flash_message {
                            flash_message_elapsed_ticks = 0;
                        }
                        let mut app_context = app.get_context();
                        if let Some(state) = app_context.get_state_mut().get_flash_message_mut() {
                            state.update(flash_message_elapsed_ticks);
                            flash_message_elapsed_ticks = 0;
                            if let Some(message) = state.get_message() {
                                flash_message.render(frame, global_layout_chunks[1], message);
                            }
                            had_flash_message = true;
                        } else {
                            had_flash_message = false;
                        }
                    }
                    // render contextual helper
                    else if show_contextual_help {
                        let app_context = app.get_context();
                        let current_route = app_context.get_router().get_current_route();
                        contextual_helper.render(
                            frame,
                            global_layout_chunks[1],
                            current_route,
                            app_context.get_state(),
                            app_context.get_inputs(),
                        );
                    }
                })
                .map_err(HnCliError::IoError)?;

            match rx.recv()? {
                UserInterfaceEvent::KeyEvent(event) => {
                    app.pump_event(event);
                    let app_context = app.get_context();
                    let inputs = app_context.get_inputs();
                    // TODO: errors on quit should be logged but not panic
                    if inputs.is_active(&ApplicationAction::Quit) {
                        disable_raw_mode().map_err(|_| {
                            HnCliError::CrosstermError("disable_raw_mode error".into())
                        })?;
                        self.terminal
                            .show_cursor()
                            .map_err(|_| HnCliError::CrosstermError("show_cursor error".into()))?;
                        break 'ui;
                    }
                    if inputs.is_active(&ApplicationAction::QuitShortcut)
                        && self.can_quit_via_shortcut()
                    {
                        disable_raw_mode().map_err(|_| {
                            HnCliError::CrosstermError("disable_raw_mode error".into())
                        })?;
                        let _ = self
                            .terminal
                            .show_cursor()
                            .map_err(|_| HnCliError::CrosstermError("show_curor error".into()));
                        break 'ui;
                    }
                    if self.app.handle_inputs() && !self.handle_inputs()? {
                        self.app.update_latest_interacted_with_component(None);
                    }
                }
                _ => {
                    flash_message_elapsed_ticks += 1;
                    self.update().await?;
                }
            }
        }

        Ok(())
    }

    /// Check all active components for any necessary update.
    async fn update(&mut self) -> Result<()> {
        let mut app_context = self.app.get_context();
        for wrapper in self.components.values_mut() {
            wrapper.ticks_elapsed += 1;
            // TODO: better error handling (per-component?)
            if wrapper.active
                && wrapper
                    .component
                    .should_update(wrapper.ticks_elapsed, &app_context)?
            {
                wrapper
                    .component
                    .update(&mut self.client, &mut app_context)
                    .await?;
                wrapper.ticks_elapsed = 0;
            }
        }

        Ok(())
    }

    /// Handle an incoming key event through all active components.
    fn handle_inputs(&mut self) -> Result<bool> {
        let mut swallowed = false;
        let mut latest_interacted_with_component = None;
        let mut app_context = self.app.get_context();
        for wrapper in self.components.values_mut() {
            if !wrapper.active {
                continue;
            }
            if wrapper.component.handle_inputs(&mut app_context)? {
                latest_interacted_with_component = Some(wrapper.component.id());
                swallowed = true;
                break;
            }
        }
        if latest_interacted_with_component.is_some() {
            self.app
                .update_latest_interacted_with_component(latest_interacted_with_component);
        }

        Ok(swallowed)
    }

    fn can_quit_via_shortcut(&mut self) -> bool {
        let app_context = self.app.get_context();
        // In Algolia input mode?
        if app_context.get_state().get_currently_used_algolia_part() == SearchScreenPart::Input {
            return false;
        }
        // Check configuration first
        if app_context
            .get_config()
            .get_enable_global_sub_screen_quit_shortcut()
        {
            return true;
        }
        // If the shortcut is disabled, check if we are on the home-screen
        app_context.get_router().is_on_root_screen()
    }

    fn register_component<C: UiComponent + 'static>(&mut self, component: C) {
        let boxed_component = Box::new(component);
        self.components.insert(
            boxed_component.id(),
            ComponentWrapper::from_component(boxed_component),
        );
    }
}
