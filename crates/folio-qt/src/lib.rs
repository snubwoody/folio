use sqlx::SqlitePool;
use std::sync::{LazyLock, OnceLock};
use qmetaobject::{qml_register_type, qrc};
use tokio::runtime::Runtime;
use crate::account_list::AccountListModel;
use crate::app_state::AppState;
use crate::transaction_table::TransactionTableModel;

pub mod account_list;
pub mod app_state;
pub mod transaction_table;
// TODO: add compile step for qml

/// Global async runtime
pub static RUNTIME: LazyLock<Runtime> =
    LazyLock::new(|| Runtime::new().expect("failed to create runtime"));

/// Global database pool
pub static DB_POOL: OnceLock<SqlitePool> = OnceLock::new();

/// Returns a reference to the database pool
pub fn db_pool() -> &'static SqlitePool {
    DB_POOL.get().expect("database pool not initialised")
}

pub fn register_qml() {
    // TODO: find someway to include all the icons
    // TODO: log to file, don't log personal info
    // Embed the qml
    qrc!(pub qml, "/" {
        "ui/App.qml",
        "ui/AccountList.qml",
        "ui/TransactionTable.qml",
        "ui/AccountPanel.qml",
        "ui/TransactionPanel.qml",
        "ui/Colors.qml",
        "ui/IconButton.qml",
        "ui/HomePage.qml",
        "ui/AnalyticsPage.qml",

        "ui/qmldir",

        "icons/plus.svg",
        "icons/bar-chart.svg",
        "icons/home.svg",
        "icons/settings.svg",
    });
    qml();

    qml_register_type::<AccountListModel>(
        cstr::cstr!("App"),
        1,
        0,
        cstr::cstr!("AccountListModel"),
    );
    qml_register_type::<TransactionTableModel>(
        cstr::cstr!("App"),
        1,
        0,
        cstr::cstr!("TransactionTableModel"),
    );
    qml_register_type::<AppState>(cstr::cstr!("App"), 1, 0, cstr::cstr!("AppState"));
}

