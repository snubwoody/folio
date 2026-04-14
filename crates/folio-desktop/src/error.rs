// Copyright (C) 2025 Wakunguma Kalimukwa
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

use serde::{Serialize, ser::SerializeStruct};
use std::{io, num::ParseFloatError};
use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

// TODO: with_context (with_message) like Anyhow
// TODO: add AppError to frontend
// TODO: add source chain
// TODO: stack trace errors like, implement source
#[derive(Debug)]
pub struct AppError{
    message: String,
    source: Option<Box<dyn std::error::Error>>
}

impl AppError{
    pub fn new(message: &str) -> Self{
        Self{
            message: message.to_owned(),
            source: None
        }
    }

    // impl or Box?
    pub fn with_source<E:std::error::Error + 'static>(message: &str, source: E) -> Self{
        Self{
            message: message.to_owned(),
            source: Some(Box::new(source))
        }
    }
}

// TODO: Switch to anyhow?
#[derive(Error, Debug)]
pub enum Error {
    #[error("IO Error: {0}")]
    IoError(#[from] io::Error),
    #[error("Failed to parse date: {0}")]
    ParseDateError(#[from] chrono::ParseError),
    #[error("Database error: {0}")]
    SqlxError(#[from] sqlx::Error),
    #[error("Failed to parse JSON file: {0}")]
    SerdeJsonError(#[from] serde_json::Error),
    #[error("Migrate error: {0}")]
    MigrateError(#[from] sqlx::migrate::MigrateError),
    #[error("Failed to parse decimal: {0}")]
    ParseDecimalError(#[from] rust_decimal::Error),
    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error),
    #[error("Failed to parse float: {0}")]
    ParseFloatError(#[from] ParseFloatError),
    #[error("Invalid operation: {0}")]
    InvalidOperation(String),
}

impl Error {
    pub fn invalid_op(message: &str) -> Self {
        Self::InvalidOperation(message.to_owned())
    }
}

// TODO: test Serialise for AppError

impl Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut s = serializer.serialize_struct("Error", 1)?;
        let message = self.to_string();
        s.serialize_field("message", &message)?;
        s.end()
    }
}

#[cfg(test)]
mod test{
    use std::error::Error;
    use crate::service::Category;
    use super::*;

    #[test]
    fn source(){
        let result = serde_json::from_str::<Category>("0");
        match result {
            Ok(_) => {},
            Err(e) => {
                dbg!(e.source());
                let error = AppError::with_source("Failed to parse category",e);
                dbg!(error);
            }
        }
    }
}
