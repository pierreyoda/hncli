use std::{
    collections::HashMap,
    io::Stdout,
    sync::mpsc,
    thread,
    time::{Duration, Instant},
};

use common::{UiComponent, UiTickScalar};
use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use event::KeyEvent;
use handlers::Key;
use mpsc::Receiver;
use screens::UserInterfaceScreen;
use stories::StoriesPanel;
use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Tabs},
    Terminal,
};

use crate::{
    api::HnClient,
    app::App,
    errors::{HnCliError, Result},
};

mod common;
mod handlers;
mod panels;
mod screens;
mod stories;
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
    components: HashMap<&'static str, ComponentWrapper>,
}

impl UserInterface {
    /// Create a new `UserInterface` instance and prepare the terminal for it.
    pub fn new(mut terminal: TerminalUi, client: HnClient) -> Result<Self> {
        terminal.clear().map_err(HnCliError::IoError)?;
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
        let tick_rate = Duration::from_millis(100);
        let (tx, rx) = mpsc::channel();
        thread::spawn(move || {
            let mut last_tick = Instant::now();
            loop {
                let timeout = tick_rate
                    .checked_sub(last_tick.elapsed())
                    .unwrap_or_else(|| Duration::from_secs(0));

                if event::poll(timeout).expect("poll works") {
                    if let Event::Key(key_event) = event::read().expect("can read events") {
                        let key: Key = key_event.into();
                        tx.send(UserInterfaceEvent::KeyEvent(key))
                            .expect("can send events");
                    }
                }

                if last_tick.elapsed() >= tick_rate && tx.send(UserInterfaceEvent::Tick).is_ok() {
                    last_tick = Instant::now();
                }
            }
        });

        // components
        let stories_panel = Box::new(StoriesPanel::default());
        self.components.insert(
            stories_panel.id(),
            ComponentWrapper::from_component(stories_panel),
        );

        Ok(rx)
    }

    /// Launches the main UI loop.
    pub async fn run(&mut self, rx: Receiver<UserInterfaceEvent>) -> Result<()> {
        enable_raw_mode()?;
        self.terminal.hide_cursor()?;

        let mut current_screen: UserInterfaceScreen = UserInterfaceScreen::Home;
        let screen_titles: Vec<&str> = vec!["Home", "Ask HN", "Show HN", "Jobs"];

        'ui: loop {
            self.terminal
                .draw(|frame| {
                    let size = frame.size();

                    let chunks = Layout::default()
                        .direction(Direction::Vertical)
                        .margin(2)
                        .constraints(
                            [
                                Constraint::Length(3),
                                Constraint::Min(2),
                                Constraint::Length(2),
                            ]
                            .as_ref(),
                        )
                        .split(size);

                    let screens = screen_titles
                        .iter()
                        .map(|title| {
                            // underline the first (shortcut) character
                            let (first, rest) = title.split_at(1);
                            Spans::from(vec![
                                Span::styled(
                                    first,
                                    Style::default()
                                        .fg(Color::Yellow)
                                        .add_modifier(Modifier::BOLD)
                                        .add_modifier(Modifier::UNDERLINED),
                                ),
                                Span::styled(rest, Style::default().fg(Color::White)),
                            ])
                        })
                        .collect();

                    let screens_tabs = Tabs::new(screens)
                        .select(current_screen.clone().into())
                        .block(Block::default().title("Menu").borders(Borders::ALL))
                        .style(Style::default().fg(Color::White))
                        .highlight_style(Style::default().fg(Color::Yellow))
                        .divider(Span::raw("|"));

                    frame.render_widget(screens_tabs, chunks[0]);
                    // render_home_screen(frame, chunks[1], &displayable_stories[..]);
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
                    key => self.handle_key_event(&key)?,
                },
                //     KeyCode::Char('h') => current_screen = UserInterfaceScreen::Home,
                //     KeyCode::Char('a') => current_screen = UserInterfaceScreen::AskHackerNews,
                //     KeyCode::Char('s') => current_screen = UserInterfaceScreen::ShowHackerNews,
                //     KeyCode::Char('j') => current_screen = UserInterfaceScreen::Jobs,
                //     _ => {}
                // },
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
            if wrapper.active && wrapper.component.should_update(wrapper.ticks_elapsed)? {
                wrapper
                    .component
                    .update(&mut self.client, &mut self.app)
                    .await?;
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
