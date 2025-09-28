use std::{fs::File, io};

extern crate log;
extern crate simplelog;

use simplelog::{Config, WriteLogger};

use api::HnClient;
use errors::HnCliError;
use ratatui::{Terminal, backend::CrosstermBackend};
use ui::UserInterface;

mod api;
mod app;
mod config;
mod errors;
mod ui;

// TODO: set terminal title (dynamically if possible)
#[tokio::main]
async fn main() -> Result<(), HnCliError> {
    // File logger setup (mainly used for development purposes)
    WriteLogger::init(
        log::LevelFilter::Info,
        Config::default(),
        File::create("hncli_log.txt").map_err(HnCliError::IoError)?,
    )
    .expect("logging to file should be properly initialized");

    // HackerNews client setup
    let client = HnClient::new()?;

    // TUI setup
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let terminal = Terminal::new(backend).map_err(HnCliError::IoError)?;

    // UI setup & run
    let mut ui = UserInterface::new(terminal, client)?;
    let events_receiver = ui.setup()?;
    ui.run(events_receiver).await
}
