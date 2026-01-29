use sqlx::SqlitePool;
use std::sync::{LazyLock, OnceLock};
use tokio::runtime::Runtime;
pub mod account_list;
pub mod app_state;
pub mod transaction_table;

/// Global async runtime
pub static RUNTIME: LazyLock<Runtime> =
    LazyLock::new(|| Runtime::new().expect("failed to create runtime"));

/// Global database pool
pub static DB_POOL: OnceLock<SqlitePool> = OnceLock::new();

/// Returns a reference to the database pool
pub fn db_pool() -> &'static SqlitePool {
    DB_POOL.get().expect("database pool not initialised")
}
