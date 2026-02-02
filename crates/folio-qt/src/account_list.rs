use crate::{RUNTIME, db_pool};
use folio_lib::service::{Account, fetch_accounts};
use qmetaobject::{QAbstractListModel, QObject, qt_base_class, qt_method};
use qttypes::{QByteArray, QString, QVariant};
use std::collections::HashMap;

#[derive(QObject, Default)]
pub struct AccountListModel {
    base: qt_base_class!(trait QAbstractListModel),
    load_accounts: qt_method!(fn(&mut self)),
    accounts: Vec<Account>,
}

impl AccountListModel {
    const NAME_ROLE: i32 = qmetaobject::USER_ROLE + 1;
    const BALANCE_ROLE: i32 = Self::NAME_ROLE + 1;
    const STARTING_BALANCE_ROLE: i32 = Self::BALANCE_ROLE + 1;

    pub fn new() -> Self {
        let accounts = RUNTIME.block_on(async { fetch_accounts(db_pool()).await.unwrap() });
        Self {
            accounts,
            ..Default::default()
        }
    }

    pub fn load_accounts(&mut self) {
        self.begin_reset_model();
        self.accounts = RUNTIME.block_on(async { fetch_accounts(db_pool()).await.unwrap() });
        self.end_reset_model();
    }

    pub fn add_account(&mut self) {
        self.begin_reset_model();
        self.accounts.push(Account {
            name: "New account".into(),
            ..Default::default()
        });
        self.end_reset_model();
    }
}

impl QAbstractListModel for AccountListModel {
    fn row_count(&self) -> i32 {
        self.accounts.len() as i32
    }

    fn data(&self, index: qttypes::QModelIndex, role: i32) -> qttypes::QVariant {
        let index = index.row() as usize;
        if index >= self.accounts.len() {
            return QVariant::default();
        }

        let account = &self.accounts[index];

        match role {
            Self::NAME_ROLE => QString::from(account.name.clone()).into(),
            Self::BALANCE_ROLE => QString::from(account.balance.to_string()).into(),
            Self::STARTING_BALANCE_ROLE => {
                QString::from(account.starting_balance.to_string()).into()
            }
            _ => QVariant::default(),
        }
    }

    fn role_names(&self) -> HashMap<i32, QByteArray> {
        let mut roles = HashMap::new();
        roles.insert(Self::NAME_ROLE, QByteArray::from("name"));
        roles.insert(Self::BALANCE_ROLE, QByteArray::from("balance"));
        roles.insert(
            Self::STARTING_BALANCE_ROLE,
            QByteArray::from("startingBalance"),
        );
        roles
    }
}
