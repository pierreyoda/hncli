use std::{
    convert::TryFrom,
    io::Stdout,
    sync::mpsc,
    thread,
    time::{Duration, Instant},
};

use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use mpsc::Receiver;
use screens::{render_home_screen, UserInterfaceScreen};
use stories::DisplayableHackerNewsStory;
use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Tabs},
    Terminal,
};

use crate::{
    api::{HnClient, HnStoriesSorting},
    errors::{HnCliError, Result},
};

mod screens;
mod stories;
mod utils;

type TerminalUi = Terminal<CrosstermBackend<Stdout>>;

#[derive(Clone, Debug)]
pub enum UserInterfaceEvent {
    Key(KeyCode),
    Tick,
}

pub struct UserInterface {
    terminal: TerminalUi,
    client: HnClient,
}

impl UserInterface {
    /// Create a new `UserInterface` instance and prepare the terminal for it.
    pub fn new(mut terminal: TerminalUi, client: HnClient) -> Result<Self> {
        terminal.clear().map_err(HnCliError::IoError)?;
        Ok(Self { terminal, client })
    }

    /// Set up the Event Loop channels.
    pub fn setup(&self) -> Result<Receiver<UserInterfaceEvent>> {
        let tick_rate = Duration::from_millis(100);

        let (tx, rx) = mpsc::channel();
        thread::spawn(move || {
            let mut last_tick = Instant::now();
            loop {
                let timeout = tick_rate
                    .checked_sub(last_tick.elapsed())
                    .unwrap_or_else(|| Duration::from_secs(0));

                if event::poll(timeout).expect("poll works") {
                    if let Event::Key(key) = event::read().expect("can read events") {
                        tx.send(UserInterfaceEvent::Key(key.code))
                            .expect("can send events");
                    }
                }

                if last_tick.elapsed() >= tick_rate && tx.send(UserInterfaceEvent::Tick).is_ok() {
                    last_tick = Instant::now();
                }
            }
        });

        Ok(rx)
    }

    /// Launches the main UI loop.
    pub async fn run(&mut self, rx: Receiver<UserInterfaceEvent>) -> Result<()> {
        enable_raw_mode()?;

        let mut current_screen: UserInterfaceScreen = UserInterfaceScreen::Home;
        let screen_titles: Vec<&str> = vec!["Home", "Ask HN", "Show HN", "Jobs"];

        // TODO: fetch and/or store in screen?
        // let stories = self.client.get_home_stories(HnStoriesSorting::Top).await?;
        // dbg!(stories.clone());
        // let displayable_stories: Vec<DisplayableHackerNewsStory> = stories
        //     .iter()
        //     .map(|story| {
        //         DisplayableHackerNewsStory::try_from(story.clone())
        //             .expect("can map DisplayableHackerNewsStory")
        //     })
        //     .collect();
        // dbg!(displayable_stories.clone());

        'ui: loop {
            self.terminal
                .draw(|frame| {
                    let size = frame.size();

                    let chunks = Layout::default()
                        .direction(Direction::Vertical)
                        .margin(2)
                        .constraints(
                            [
                                Constraint::Percentage(100),
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
                UserInterfaceEvent::Key(code) => match code {
                    KeyCode::Char('q') => {
                        disable_raw_mode()?;
                        self.terminal.show_cursor()?;
                        break 'ui;
                    }
                    KeyCode::Char('h') => current_screen = UserInterfaceScreen::Home,
                    KeyCode::Char('a') => current_screen = UserInterfaceScreen::AskHackerNews,
                    KeyCode::Char('s') => current_screen = UserInterfaceScreen::ShowHackerNews,
                    KeyCode::Char('j') => current_screen = UserInterfaceScreen::Jobs,
                    _ => {}
                },
                UserInterfaceEvent::Tick => {}
            }
        }

        Ok(())
    }
}
