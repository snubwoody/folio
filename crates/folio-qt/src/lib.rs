use crate::account_list::AccountListModel;
use crate::transaction_table::TransactionTableModel;
use qmetaobject::{qml_register_type, qrc, QQuickStyle, QmlEngine};
use sqlx::SqlitePool;
use std::sync::{LazyLock, OnceLock};
use qttypes::{QString, QUrl};
use tokio::runtime::Runtime;
use folio_lib::init_database;
use crate::category_list::CategoryListModel;

pub mod account_list;
pub mod category_list;
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
        "ui/Style.qml",
        "ui/components/IconButton.qml",
        "ui/HomePage.qml",
        "ui/AnalyticsPage.qml",
        "ui/SettingsPanel.qml",
        "ui/SettingsTabButton.qml",
        "ui/GeneralSettingsPanel.qml",
        "ui/CategoriesSettingsPanel.qml",
        "ui/components/TextLabel.qml",
        "ui/StreamsSettingsPanel.qml",
        "ui/EditableText.qml",

        "ui/qmldir",

        "icons/plus.svg",
        "icons/bar-chart.svg",
        "icons/home.svg",
        "icons/delete.svg",
        "icons/trash-2.svg",
        "icons/settings.svg",

        "fonts/Satoshi-Variable.ttf",
    });
    qml();

    register_type!{
        AccountListModel,
        TransactionTableModel,
        CategoryListModel,
    }
}

pub fn run() {
    tracing_subscriber::fmt::init();

    // TODO: Don't think this is doing anything
    // TODO: replace everything with qsTr()
    unsafe {
        std::env::set_var("QT_QUICK_CONTROLS_STYLE", "Basic");
    }
    let pool = RUNTIME.block_on(async {
        init_database()
            .await
            .expect("failed to create database pool")
    });
    DB_POOL.set(pool).expect("failed to set pool");

    QQuickStyle::set_style("Basic");
    register_qml();

    let mut engine = QmlEngine::new();
    engine.load_url(QUrl::from(QString::from("qrc:/ui/App.qml")));
    engine.exec();
}

