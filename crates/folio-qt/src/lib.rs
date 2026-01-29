use sqlx::SqlitePool;
use std::sync::{LazyLock, OnceLock};
use tokio::runtime::Runtime;
pub mod account_list;
pub mod transaction_table;
pub mod app_state;

/// Global async runtime
pub static RUNTIME: LazyLock<Runtime> =
    LazyLock::new(|| Runtime::new().expect("failed to create runtime"));

pub static DB_POOL: OnceLock<SqlitePool> = OnceLock::new();

pub fn db_pool() -> &'static SqlitePool {
    DB_POOL.get().expect("database pool not initialised")
}
