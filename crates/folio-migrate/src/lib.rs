//! Folio migration toolkit.
//!
//! Migrations are stored as the version number in a `schema_migrations` table.
mod error;
mod migrator;
pub use error::MigrateError;
pub(crate) use error::Result;
pub use migrator::Migrator;

/// Opens an in memory sqlite database for testing.
#[cfg(test)]
pub(crate) fn test_db() -> rusqlite::Connection {
    use rusqlite::Connection;

    Connection::open_in_memory().expect("Failed to open sqlite connection")
}
