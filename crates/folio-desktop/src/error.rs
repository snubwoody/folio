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
use std::fmt::{Display, Formatter};
use std::ops::Deref;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

pub trait ErrorExt<T, E>: {
    /// Wrap the error value with additional context.
    fn context<C>(self, context: C) -> std::result::Result<T, AppError>
    where
        C: Display + Send + Sync + 'static;

    /// Wrap the error value with additional context that is evaluated lazily
    /// only once an error does occur.
    fn with_context<C, F>(self, f: F) -> std::result::Result<T, AppError>
    where
        C: Display + Send + Sync + 'static,
        F: FnOnce() -> C;
}


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
    /// Create a new error.
    pub fn new(message: &str) -> Self{
        Self{
            message: message.to_owned(),
            source: None
        }
    }

    // impl or Box?
    /// Create a new error with an underlying error source.
    pub fn with_source<E:std::error::Error + 'static>(message: &str, source: E) -> Self{
        Self{
            message: message.to_owned(),
            source: Some(Box::new(source))
        }
    }
}


// TODO: might not need Send + Sync + 'static
impl<T, E> ErrorExt<T, E> for std::result::Result<T, E>
where
    E: std::error::Error + 'static,
{
    fn context<C>(self, context: C) -> std::result::Result<T, AppError>
    where
        C: Display,
    {
        // Not using map_err to save 2 useless frames off the captured backtrace
        // in ext_context.
        match self {
            Ok(ok) => Ok(ok),
            Err(error) =>
                // TODO: change param to &str?
                Err(AppError::with_source(context.to_string().as_str(),error))
        }
    }

    fn with_context<C, F>(self, context: F) -> std::result::Result<T, AppError>
    where
        C: Display,
        F: FnOnce() -> C,
    {
        match self {
            Ok(ok) => Ok(ok),
            Err(error) => Err(AppError::with_source(context().to_string().as_str(),error)),
        }
    }
}

impl Display for AppError{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,"{}",self.message)
    }
}

impl std::error::Error for AppError{
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_deref()
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
        let result = serde_json::from_str::<Category>("0")
            .context("Cannot parse user")
            .context("Failed to parse JSON");
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
