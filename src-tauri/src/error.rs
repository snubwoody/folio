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
use std::{io, num::ParseFloatError};
use serde::{Serialize, ser::SerializeStruct};
use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("IO Error: {0}")]
    IoError(#[from] io::Error),
    #[error("Failed to parse date: {0}")]
    ParseDateError(#[from] chrono::ParseError),
    #[error("Database error: {0}")]
    SqlxError(#[from] sqlx::Error),
    #[error("Migrate error: {0}")]
    MigrateError(#[from] sqlx::migrate::MigrateError),
    #[error("Failed to parse decimal: {0}")]
    ParseDecimalError(#[from] rust_decimal::Error),
    #[error("Failed to parse float: {0}")]
    ParseFloatError(#[from] ParseFloatError),
}

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
