//! Folio migration toolkit.
//!
//! Migrations are stored as the version number in a `schema_migrations` table.
use rusqlite::Connection;
use std::fs;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Migration {
    version: u64,
    query: String,
    kind: MigrationKind,
}

impl Migration {
    /// Creates a new up migration.
    pub fn up(query: &str, version: u64) -> Migration {
        Migration {
            version,
            query: query.to_owned(),
            kind: MigrationKind::Up,
        }
    }

    /// Creates a new down migration.
    pub fn down(query: &str, version: u64) -> Migration {
        Migration {
            version,
            query: query.to_owned(),
            kind: MigrationKind::Down,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum MigrationKind {
    Up,
    Down,
}

/// Creates the `schema_migrations` table if it does not exist.
fn create_migrations_table(conn: &Connection) {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS schema_migrations(version INT)",
        (),
    )
    .expect("Failed to create schema_migrations table");
}

#[cfg(test)]
pub fn test_db() -> Connection {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_migration() {
        let conn = test_db();
        migrate(&conn);
    }

    #[test]
    fn init_migrations_table() {
        let conn = test_db();
        create_migrations_table(&conn);

        conn.execute(
            "INSERT INTO schema_migrations(version) VALUES($1)",
            [29492424],
        )
        .unwrap();
    }

    #[test]
    fn init_migrations_table_already_exists() {
        let conn = test_db();
        create_migrations_table(&conn);
        create_migrations_table(&conn);
        create_migrations_table(&conn);
    }
}
