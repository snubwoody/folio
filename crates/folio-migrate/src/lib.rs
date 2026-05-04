//! Folio migration toolkit.
//!
//! Migrations are stored as the version number in a `schema_migrations` table.
use rusqlite::Connection;
use std::fs;
mod migrator;
pub use migrator::{Migrator};

/// Opens an in memory sqlite database for testing.
#[cfg(test)]
pub(crate) fn test_db() -> Connection {
    let conn = Connection::open_in_memory().expect("Failed to open sqlite connection");
    conn
}

pub fn migrate(conn: &Connection) {
    let migrations = load_migration();
    let m = &migrations[0];
    conn.execute(m, ()).expect("Failed to run migration");
}

pub fn load_migration() -> Vec<String> {
    let path = "../folio-desktop/migrations/category_group_column.sql";
    let data = fs::read_to_string(path).unwrap();
    let mut in_block = false;
    let mut blocks = vec![];
    let mut block = String::new();
    // TODO: seek until next block or EOF
    for line in data.lines() {
        let stripped_line = line.replace(" ", "");
        if stripped_line == "--migrate:up" || stripped_line == "--migrate:down" {
            if !in_block {
                in_block = true;
                continue;
            }

            blocks.push(block.clone());
            in_block = false
        }

        if in_block {
            block.push_str(line)
        }
    }
    blocks
}
