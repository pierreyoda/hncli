use std::io;

use api::HnClient;
use errors::HnCliError;
use tui::{backend::CrosstermBackend, Terminal};
use ui::UserInterface;

mod api;
mod app;
mod errors;
mod ui;

#[tokio::main]
async fn main() -> Result<(), HnCliError> {
    // HackerNews client setup
    let client = HnClient::new()?;

    // TUI setup
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let terminal = Terminal::new(backend).map_err(HnCliError::IoError)?;
    let mut ui = UserInterface::new(terminal, client)?;

    // UI setup & run
    let events_receiver = ui.setup()?;
    ui.run(events_receiver).await
}
