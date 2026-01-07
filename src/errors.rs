use std::{io, sync::mpsc::RecvError};

use thiserror::Error;
use url::ParseError;

use crate::api::types::{HnItemDateScalar, HnItemIdScalar};

#[derive(Debug, Error)]
pub enum HnCliError {
    #[error("Chrono datetime error from timestamp: {0}")]
    ChronoError(HnItemDateScalar),
    #[error("IO error")]
    IoError(#[source] io::Error),
    #[error("HTTP client error")]
    HttpError(#[from] reqwest::Error),
    #[error("Threading error")]
    ThreadingError(#[from] RecvError),
    #[error("Crossterm error: {0}")]
    CrosstermError(String),
    #[error("html2text error: {0}")]
    Html2TextError(#[source] html2text::Error),
    #[error("UI error: {0}")]
    UiError(String),
    #[error("Config synchronization error: {0}")]
    ConfigSynchronizationError(String),
    #[error("History synchronization error: {0}")]
    HistorySynchronizationError(String),
    #[error("URL parsing error")]
    UrlParsingError(#[from] ParseError),
    #[error("The HN item with ID {0} was not found")]
    ItemNotFound(HnItemIdScalar),
    #[error("The HN user with ID {0} was not found")]
    UserNotFound(String),
    #[error("The HN item with ID {0} could not be processed")]
    HnItemProcessingError(String),
}

/// A `Result` alias where the `Err` case is `HnCliError`.
pub type Result<T> = std::result::Result<T, HnCliError>;
