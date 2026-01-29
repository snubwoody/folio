use crate::account_list::AccountListModel;
use crate::transaction_table::TransactionTableModel;
use crate::{RUNTIME, db_pool};
use folio_lib::Money;
use folio_lib::service::{Expense, fetch_expenses};
use qmetaobject::{QAbstractTableModel, QObject, qt_base_class, qt_method, qt_property};
use qttypes::{QByteArray, QModelIndex, QString, QVariant};
use std::cell::RefCell;
use std::collections::HashMap;

// TODO: create a GlobalQObject
/// Global app state
#[derive(QObject, Default)]
pub struct AppState {
    base: qt_base_class!(trait QObject),
    accounts: qt_property!(RefCell<AccountListModel>),
    transactions: qt_property!(RefCell<TransactionTableModel>),
    load_data: qt_method!(fn(&mut self)),
    add_account: qt_method!(fn(&mut self)),
}

impl AppState {
    pub fn load_data(&mut self) {
        self.accounts.borrow_mut().load_accounts();
    }

    pub fn add_account(&mut self) {
        RUNTIME.block_on(async {
            folio_lib::service::Account::create("New account", Money::from_unscaled(200), db_pool())
                .await
                .unwrap()
        });
        self.load_data();
    }
}
