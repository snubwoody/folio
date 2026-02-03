use crate::account_list::AccountListModel;
use crate::app_state::AppState;
use crate::transaction_table::TransactionTableModel;
use qmetaobject::{qml_register_type, qrc};
use sqlx::SqlitePool;
use std::sync::{LazyLock, OnceLock};
use tokio::runtime::Runtime;
use crate::category_list::CategoryListModel;

pub mod account_list;
pub mod category_list;
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

macro_rules! register_type {
    ($($t:ty),* $(,)?) => {
        $(
            qml_register_type::<$t>(
                cstr::cstr!("App"),
                1,
                0,
                cstr::cstr!($t),
            );
        )*
    };
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
        "ui/SettingsPanel.qml",
        "ui/SettingsTabButton.qml",
        "ui/GeneralSettingsPanel.qml",
        "ui/CategoriesSettingsPanel.qml",
        "ui/StreamsSettingsPanel.qml",
        "ui/EditableText.qml",

        "ui/qmldir",

        "icons/plus.svg",
        "icons/bar-chart.svg",
        "icons/home.svg",
        "icons/delete.svg",
        "icons/trash-2.svg",
        "icons/settings.svg",
    });
    qml();

    register_type!{
        AccountListModel,
        TransactionTableModel,
        CategoryListModel,
        AppState
    }
}

