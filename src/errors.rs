use std::{io, sync::mpsc::RecvError};

use crossterm::ErrorKind;
use thiserror::Error;
use url::ParseError;

use crate::api::types::HnItemIdScalar;

#[derive(Debug, Error)]
pub enum HnCliError {
    #[error("IO error")]
    IoError(#[from] io::Error),
    #[error("HTTP client error")]
    HttpError(#[from] reqwest::Error),
    #[error("Threading error")]
    ThreadingError(#[from] RecvError),
    #[error("Crossterm error")]
    CrosstermError(#[from] ErrorKind),
    #[error("URL parsing error")]
    UrlParsingError(#[from] ParseError),
    #[error("The HN item with ID {0} was not found")]
    ItemNotFound(HnItemIdScalar),
    #[error("The HN user with ID {0} was not found")]
    UserNotFound(String),
    #[error("The HN item with ID {0} could not be processed")]
    HnItemProcessingError(String),
    #[error("hncli UI error: {0}")]
    UiError(String),
}

/// A `Result` alias where the `Err` case is `HnCliError`.
pub type Result<T> = std::result::Result<T, HnCliError>;
