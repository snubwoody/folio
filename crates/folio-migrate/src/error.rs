use std::io;

use thiserror::Error;

pub(crate) type Result<T> = std::result::Result<T,MigrateError>;

#[derive(Debug,Error)]
pub enum MigrateError{
    #[error("Sqlite error: {0}")]
    RusqliteError(#[from] rusqlite::Error),
    #[error("IO error: {0}")]
    IoError(#[from] io::Error),
    #[error("Invalid migration: {0}")]
    InvalidMigration(String)
}

impl MigrateError{
    pub fn invalid_migration(message: &str) -> Self{
        Self::InvalidMigration(message.to_owned())
    }
}
