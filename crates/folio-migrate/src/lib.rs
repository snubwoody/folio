//! Folio migration toolkit.
//!
//! Migrations are stored as the version number in a `schema_migrations` table.
mod migrator;
pub use migrator::Migrator;

/// Opens an in memory sqlite database for testing.
#[cfg(test)]
pub(crate) fn test_db() -> rusqlite::Connection {
    use rusqlite::Connection;
    let conn = Connection::open_in_memory().expect("Failed to open sqlite connection");
    conn
}
