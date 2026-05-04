//! Folio migration toolkit.
//!
//! Migrations are stored as the version number in a `schema_migrations` table.
mod migrator;
mod error;
pub use migrator::Migrator;
pub use error::MigrateError;
pub(crate) use error::Result;

// TODO: add cli

/// Opens an in memory sqlite database for testing.
#[cfg(test)]
pub(crate) fn test_db() -> rusqlite::Connection {
    use rusqlite::Connection;
    let conn = Connection::open_in_memory().expect("Failed to open sqlite connection");
    conn
}
