use std::{
    collections::HashMap,
    io::Stdout,
    sync::mpsc,
    thread,
    time::{Duration, Instant},
};

use common::{UiComponent, UiComponentId, UiTickScalar};
use components::{navigation::Navigation, stories::StoriesPanel};
use crossterm::{
    event::{self, Event},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use handlers::Key;
use mpsc::Receiver;
use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    Terminal,
};

use crate::{
    api::HnClient,
    app::App,
    errors::{HnCliError, Result},
};

pub mod common;
pub mod components;
pub mod handlers;
mod panels;
mod screens;
mod utils;

type TerminalUi = Terminal<CrosstermBackend<Stdout>>;

#[derive(Clone, Debug)]
pub enum UserInterfaceEvent {
    KeyEvent(Key),
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

impl UserInterface {
    /// Create a new `UserInterface` instance and prepare the terminal for it.
    pub fn new(mut terminal: TerminalUi, client: HnClient) -> Result<Self> {
        enable_raw_mode()?;
        terminal.clear()?;
        terminal.hide_cursor()?;
        Ok(Self {
            terminal,
            client,
            app: App::default(),
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

                if event::poll(timeout).expect("poll works") {
                    if let Event::Key(key_event) = event::read().unwrap() {
                        let key: Key = key_event.into();
                        tx.send(UserInterfaceEvent::KeyEvent(key)).unwrap();
                    }
                }

                if last_tick.elapsed() >= tick_rate && tx.send(UserInterfaceEvent::Tick).is_ok() {
                    last_tick = Instant::now();
                }
            }
        });

        // components
        // TODO: create register_component function (or macro) here
        let navigation = Box::new(Navigation::default());
        self.components.insert(
            navigation.id(),
            ComponentWrapper::from_component(navigation),
        );
        let stories_panel = Box::new(StoriesPanel::default());
        self.components.insert(
            stories_panel.id(),
            ComponentWrapper::from_component(stories_panel),
        );

        Ok(rx)
    }

    /// Launch the main UI loop.
    pub async fn run(&mut self, rx: Receiver<UserInterfaceEvent>) -> Result<()> {
        self.terminal.hide_cursor()?;

        'ui: loop {
            let app = &mut self.app;
            let components = &self.components;
            self.terminal
                .draw(|frame| {
                    // compute main layout chunks
                    let size = frame.size();
                    let chunks_main = Layout::default()
                        .direction(Direction::Vertical)
                        .margin(2)
                        .constraints(
                            [
                                Constraint::Length(3),
                                Constraint::Min(2),
                                Constraint::Length(3),
                            ]
                            .as_ref(),
                        )
                        .split(size);

                    // refresh application chunks
                    app.update_layout(&chunks_main[..]);

                    // render components
                    for (id, wrapper) in components.iter() {
                        match app.get_component_rendering_rect(id) {
                            None => (), // no rendering
                            Some(inside_rect) => wrapper
                                .component
                                .render(frame, *inside_rect, app)
                                .expect("no component rendering error"),
                        }
                    }
                })
                .map_err(HnCliError::IoError)?;

            match rx.recv()? {
                UserInterfaceEvent::KeyEvent(key) => match key {
                    // TODO: properly handle CTRL+C
                    Key::Char('q') => {
                        disable_raw_mode()?;
                        self.terminal.show_cursor()?;
                        break 'ui;
                    }
                    key => {
                        if !self.app.handle_key_event(&key) {
                            self.handle_key_event(&key)?;
                        }
                    }
                },
                UserInterfaceEvent::Tick => {
                    self.update().await?;
                }
            }
        }

        Ok(())
    }

    /// Check all active components for any necessary update.
    async fn update(&mut self) -> Result<()> {
        for wrapper in self.components.values_mut() {
            wrapper.ticks_elapsed += 1;
            // TODO: better error handling (per-component?)
            if wrapper.active
                && wrapper
                    .component
                    .should_update(wrapper.ticks_elapsed, &self.app)?
            {
                wrapper
                    .component
                    .update(&mut self.client, &mut self.app)
                    .await?;
                wrapper.ticks_elapsed = 0;
            }
        }

        Ok(())
    }

    /// Handle an incoming key event through all active components.
    fn handle_key_event(&mut self, key: &Key) -> Result<()> {
        for wrapper in self.components.values_mut() {
            if !wrapper.active {
                continue;
            }
            if wrapper.component.key_handler(key, &mut self.app)? {
                break;
            }
        }

        Ok(())
    }
}
