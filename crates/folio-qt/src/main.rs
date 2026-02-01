use std::env::current_exe;
// TODO: change windows subsystem on release
use folio_lib::init_database;
use folio_qt::app_state::AppState;
use folio_qt::register_qml;
use folio_qt::transaction_table::TransactionTableModel;
use folio_qt::{DB_POOL, RUNTIME, account_list::AccountListModel};
use qmetaobject::{QQuickStyle, QmlEngine, qml_register_type, qrc};
use qttypes::{QString, QUrl};

fn main() {
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
