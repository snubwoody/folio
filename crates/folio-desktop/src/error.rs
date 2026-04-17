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
use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

/// Error extension trait that provides extra context for errors.
pub trait ErrorExt<T, E>: {
    /// Wrap the error value with additional context.
    fn context<C>(self, context: C) -> std::result::Result<T, Error>
    where
        C: Display + Send + Sync + 'static;

    /// Wrap the error value with additional context that is evaluated lazily
    /// only once an error does occur.
    fn with_context<C, F>(self, f: F) -> std::result::Result<T, Error>
    where
        C: Display + Send + Sync + 'static,
        F: FnOnce() -> C;
}


// TODO: add Error to frontend
#[derive(Debug)]
pub struct Error{
    message: String,
    source: Option<Box<dyn std::error::Error + Send>>
}

impl Error{
    /// Create a new error.
    pub fn new(message: &str) -> Self{
        Self{
            message: message.to_owned(),
            source: None
        }
    }

    /// Create a new error with an underlying error source.
    pub fn with_source<E:std::error::Error + Send + 'static>(message: &str, source: E) -> Self{
        Self{
            message: message.to_owned(),
            source: Some(Box::new(source))
        }
    }

    /// Returns a multiline string containing the error message and sources.
    pub fn report(&self) -> String{
        let mut message = String::from(format!("Error: {}",self.to_string()));
        if self.source.is_some(){
            message.push_str("\n\tCaused by:")
        }
        let mut source = self.source.as_deref().map(|e| e as &(dyn std::error::Error + 'static));
        let index = 1;

        while let Some(s) = source{
            message.push_str(&format!("\n\t\t{index}: {}",s.to_string()));
            source = s.source()
        }
        message
    }
}


impl<T, E> ErrorExt<T, E> for std::result::Result<T, E>
where
    E: std::error::Error + Send + 'static,
{
    fn context<C>(self, context: C) -> std::result::Result<T, Error>
    where
        C: Display,
    {
        // Not using map_err to save 2 useless frames off the captured backtrace
        // in ext_context.
        match self {
            Ok(ok) => Ok(ok),
            Err(error) =>
                // TODO: change param to &str?
                Err(Error::with_source(context.to_string().as_str(),error))
        }
    }

    fn with_context<C, F>(self, context: F) -> std::result::Result<T, Error>
    where
        C: Display,
        F: FnOnce() -> C,
    {
        match self {
            Ok(ok) => Ok(ok),
            Err(error) => Err(Error::with_source(context().to_string().as_str(),error)),
        }
    }
}

impl Display for Error{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,"{}",self.message)
    }
}

impl std::error::Error for Error{
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        // Type masturbation indeed
        self.source.as_deref().map(|e| e as &(dyn std::error::Error + 'static))
    }
}


macro_rules! from_error {
    ($($t:ty),+) => {
        $(
            impl From<$t> for Error{
                fn from(value: $t) -> Self {
                    Error::with_source(value.to_string().as_str(),value)
                }
            }
        )+
    };
}

from_error!{
    chrono::ParseError,
    std::io::Error,
    sqlx::Error,
    serde_json::Error,
    sqlx::migrate::MigrateError,
    rust_decimal::Error,
    reqwest::Error,
    std::num::ParseFloatError
}


impl Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut s = serializer.serialize_struct("Error", 1)?;
        let message = self.to_string();
        s.serialize_field("message", &message)?;
        if let Some(source) = &self.source{
            s.serialize_field("source", &source.to_string())?;
        }
        s.end()
    }
}

