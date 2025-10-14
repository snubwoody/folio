use std::io;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("IO Error: {0}")]
    IoError(#[from] io::Error),
    #[error("Failed to parse date: {0}")]
    ParseDateError(#[from] chrono::ParseError),
    #[error("Database error: {0}")]
    SqlxError(#[from] sqlx::Error),
    #[error("Failed to parse decimal: {0}")]
    ParseDecimalError(#[from] rust_decimal::Error),
}
