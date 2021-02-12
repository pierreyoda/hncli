use thiserror::Error;

use crate::api::types::HnItemIdScalar;

#[derive(Debug, Error)]
pub enum HnCliError {
    #[error("HTTP client error")]
    HttpError(#[from] reqwest::Error),
    #[error("The HN item with ID {0} was not found")]
    ItemNotFound(HnItemIdScalar),
    #[error("The HN user with ID {0} was not found")]
    UserNotFound(String),
}

/// A `Result` alias where the `Err` case is `HnCliError`.
pub type Result<T> = std::result::Result<T, HnCliError>;
