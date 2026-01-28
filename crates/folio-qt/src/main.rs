use folio_lib::init_database;
use folio_qt::{DB_POOL, RUNTIME, account_list::AccountListModel};
use qmetaobject::{QmlEngine, qml_register_type};
use folio_qt::transaction_table::TransactionTableModel;

fn main() {
    let pool = RUNTIME.block_on(async {
        init_database()
            .await
            .expect("failed to create database pool")
    });
    DB_POOL.set(pool).expect("failed to set pool");

    let mut engine = QmlEngine::new();
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
    engine.load_file("crates/folio-qt/ui/App.qml".into());
    engine.exec();
}
