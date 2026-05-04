use rusqlite::Connection;
use std::{fs, io::Lines, iter::Peekable, str};

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

    /// Parses a sql migration file.
    fn parse(sql: &str){
        
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
        "CREATE TABLE IF NOT EXISTS schema_migrations(version INT PRIMARY KEY NOT NULL)",
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

    /// Returns a list of the applied migration versions.
    pub fn applied_migrations(&self,conn: &Connection) -> Vec<u64>{
        let mut stmt = conn.prepare_cached("SELECT version FROM schema_migrations")
            .expect("Failed prepare query");
        let mut versions = vec![];
        let rows = stmt.query_map((), |row|{
            row.get::<_,i64>(0)
        }).expect("");

        for row in rows{
            versions.push(row.unwrap() as u64);
        }
        versions
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
        let applied_migrations = self.applied_migrations(conn);
        for migration in &self.migrations {
            if applied_migrations.contains(&migration.version){
                continue;
            }
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

/// Parses a sql migration and returns the up and down migrations.
pub fn parse_migration(sql: &str) -> (Option<String>,Option<String>) {
    let mut lines = sql.lines().peekable();
    let mut up = None;
    let mut down = None;
    
    while let Some(line) = lines.next(){
        let stripped = line.replace(" ", "");
        if stripped == "--migrate:up"{
            up = Some(seek_block(&mut lines));
        }
        if stripped == "--migrate:down"{
            down = Some(seek_block(&mut lines));
        }
    }
    
    (up,down)
}

fn seek_block(iter: &mut Peekable<str::Lines>) -> String{
    let mut block = String::new();
    while let Some(line) = iter.peek(){
        let stripped = line.replace(" ", "");
        if stripped == "--migrate:up" || stripped == "--migrate:down"{
            break;
        }

        block.push_str(iter.next().unwrap());
    }

    block
}

#[cfg(test)]
mod tests {
    use rusqlite::ErrorCode;

    use super::*;
    use crate::test_db;

    #[test]
    fn parse_up_migration(){
        let sql = "--migrate:up\nCREATE TABLE schemas(name TEXT PRIMARY KEY);\n--migrate:down\nDROP TABLE schemas;";
        let (up,down) = parse_migration(sql);
        assert_eq!(up.unwrap(),"CREATE TABLE schemas(name TEXT PRIMARY KEY);");
        assert_eq!(down.unwrap(),"DROP TABLE schemas;");
    }

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

        conn.execute("INSERT INTO users(id) VALUES ('Player 1')",(),).unwrap();
        let result = conn.execute("INSERT INTO organisations(id) VALUES ('Player 1')",(),);
        assert!(result.is_err());
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
    fn get_applied_migrations() {
        let conn = test_db();
        create_migrations_table(&conn);

        conn.execute("INSERT INTO schema_migrations(version) VALUES (5),(10),(15),(20)",(),)
            .unwrap();
        let migrator = Migrator::new();
        let migrations = migrator.applied_migrations(&conn);
        assert_eq!(migrations,[5,10,15,20]);
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
