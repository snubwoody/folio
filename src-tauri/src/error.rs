use std::io;

use serde::{Serialize, ser::SerializeStruct};
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

impl Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut s = serializer.serialize_struct("Error", 1)?;
        let message = self.to_string();
        s.serialize_field("message", &message)?;
        s.end()
    }
}
