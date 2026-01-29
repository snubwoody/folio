use std::env::current_exe;
// TODO: change windows subsystem on release
use folio_lib::init_database;
use folio_qt::app_state::AppState;
use folio_qt::transaction_table::TransactionTableModel;
use folio_qt::{DB_POOL, RUNTIME, account_list::AccountListModel};
use qmetaobject::{QmlEngine, qml_register_type, qrc};
use qttypes::{QString, QUrl};

fn register_qml() {
    // Embed the qml
    qrc!(pub qml, "/" {
        "ui/App.qml",
        "ui/AccountList.qml",
        "ui/TransactionTable.qml",
        "ui/AccountPanel.qml",
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
fn main() {
    let pool = RUNTIME.block_on(async {
        init_database()
            .await
            .expect("failed to create database pool")
    });
    DB_POOL.set(pool).expect("failed to set pool");

    register_qml();

    let mut engine = QmlEngine::new();
    engine.load_url(QUrl::from(QString::from("qrc:/ui/App.qml")));
    engine.exec();
}
