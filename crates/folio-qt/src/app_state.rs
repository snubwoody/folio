use std::cell::RefCell;
use std::collections::HashMap;
use crate::{db_pool, RUNTIME};
use folio_lib::service::{Expense, fetch_expenses};
use qmetaobject::{QAbstractTableModel, QObject, qt_base_class, qt_method, qt_property};
use qttypes::{QByteArray, QModelIndex, QString, QVariant};
use crate::account_list::AccountListModel;
use crate::transaction_table::TransactionTableModel;

// TODO: create a GlobalQObject
/// Global app state
#[derive(QObject, Default)]
pub struct AppState {
    base: qt_base_class!(trait QObject),
    load_data: qt_method!(fn(&mut self)),
    accounts: qt_property!(RefCell<AccountListModel>),
    transactions: qt_property!(RefCell<TransactionTableModel>)
}

impl AppState{
    pub fn load_data(&mut self){
        self.accounts.borrow_mut()
            .load_accounts();
    }
}