use rusqlite::Connection;
use std::fs;

// TODO: use enum error, maybe anyhow
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Migration {
    version: u64,
    up: String,
    down: String,
}

impl Migration {
    pub fn new(up: &str, down: &str, version: u64) -> Migration {
        Migration {
            version,
            up: up.to_owned(),
            down: down.to_owned(),
        }
    }

    /// Creates a new up migration.
    pub fn up(query: &str, version: u64) -> Migration {
        Migration::new(query, "", version)
    }

    /// Creates a new down migration.
    pub fn down(query: &str, version: u64) -> Migration {
        Migration::new("", query, version)
    }
}

/// Creates the `schema_migrations` table if it does not exist.
fn create_migrations_table(conn: &Connection) {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS schema_migrations(version INT NOT NULL UNIQUE)",
        (),
    )
    .expect("Failed to create schema_migrations table");
}

/// Migration runner.
pub struct Migrator {
    migrations: Vec<Migration>,
}

impl Migrator {
    pub fn new() -> Migrator {
        Migrator {
            migrations: Vec::new(),
        }
    }

    /// Adds a migration.
    pub fn add_migration(&mut self, migration: Migration) {
        self.migrations.push(migration);
    }

    pub fn load_from_file() {}
    /// Loads migrations from a directory
    pub fn load_from_dir() {}

    /// Run all the migrations.
    pub fn migrate(&self, conn: &Connection) {
        create_migrations_table(&conn);
        for migration in &self.migrations {
            conn.execute(&migration.up, ())
                .expect("Failed to run migration");

            conn.execute(
                "INSERT INTO schema_migrations(version) VALUES(?)",
                [migration.version],
            )
            .expect("Failed to record migration");
        }
    }
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
    use rusqlite::ErrorCode;

    use super::*;
    use crate::test_db;

    #[test]
    fn run_migration() {
        let conn = test_db();
        let mut migrator = Migrator::new();
        let migration = Migration::up("CREATE TABLE users(id TEXT PRIMARY KEY)", 0);
        migrator.add_migration(migration);
        migrator.migrate(&conn);

        conn.query_row(
            "INSERT INTO users(id) VALUES ('Player 1') RETURNING *",
            (),
            |row| {
                assert_eq!(row.get::<_, String>("id").unwrap(), "Player 1");
                Ok(())
            },
        )
        .expect("Failed to run query");
    }

    #[test]
    fn skip_applied_migrations() {
        let conn = test_db();
        create_migrations_table(&conn);
        conn.execute("INSERT INTO schema_migrations(version) VALUES (1010)",(),)
            .expect("Failed to run query");
        let mut migrator = Migrator::new();
        let m1 = Migration::up("CREATE TABLE users(id TEXT PRIMARY KEY)", 1);
        let m2 = Migration::up("CREATE TABLE organisations(id TEXT PRIMARY KEY)", 1010);
        migrator.add_migration(m1);
        migrator.add_migration(m2);
        migrator.migrate(&conn);

        let result = conn.execute("INSERT INTO organisations(id) VALUES ('Player 1')",(),);
        dbg!(result);
    }

    #[test]
    fn update_migrations_table_after_migrating() {
        let conn = test_db();
        let mut migrator = Migrator::new();
        let migration = Migration::up("CREATE TABLE users(id TEXT PRIMARY KEY)", 100);
        migrator.add_migration(migration);
        migrator.migrate(&conn);

        conn.query_row("SELECT * FROM schema_migrations", (), |row| {
            let version: u64 = row.get("version").unwrap();
            assert_eq!(version, 100);
            Ok(())
        })
        .unwrap()
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
    fn schema_migrations_unique_version() {
        let conn = test_db();
        create_migrations_table(&conn);

        conn.execute("INSERT INTO schema_migrations(version) VALUES($1)",[0],)
            .unwrap();

        let result = conn.execute("INSERT INTO schema_migrations(version) VALUES($1)",[0],);
        let err = result.expect_err("Expected duplicate insert to fail");
        match err {
            rusqlite::Error::SqliteFailure(a,_) => assert!(matches!(a.code,ErrorCode::ConstraintViolation)),
            _ => panic!("Invalid error")
        }
    }

    #[test]
    fn schema_migrations_not_null() {
        let conn = test_db();
        create_migrations_table(&conn);
        let result = conn.execute("INSERT INTO schema_migrations(version) VALUES(null)",(),);
        let err = result.expect_err("Expected null insert to fail");
        match err {
            rusqlite::Error::SqliteFailure(a,_) => assert!(matches!(a.code,ErrorCode::ConstraintViolation)),
            _ => panic!("Invalid error")
        }
    }

    #[test]
    fn init_migrations_table_already_exists() {
        let conn = test_db();
        create_migrations_table(&conn);
        create_migrations_table(&conn);
        create_migrations_table(&conn);
    }
}
