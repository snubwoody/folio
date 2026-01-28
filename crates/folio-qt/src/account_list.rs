use folio_lib::{
    State,
    service::{Account, fetch_accounts},
};
use folio_qt::{DB_POOL, RUNTIME, db_pool};
use qmetaobject::{QAbstractListModel, QObject, qt_base_class, qt_method, qt_property, qt_signal};
use qttypes::{QByteArray, QString, QVariant};
use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
};

#[derive(QObject)]
pub struct AccountListModel {
    base: qt_base_class!(trait QAbstractListModel),
    load_accounts: qt_method!(fn(&mut self)),
    accounts: Vec<Account>,
}

impl AccountListModel {
    pub async fn new(pool: &sqlx::SqlitePool) -> Self {
        let accounts = RUNTIME.block_on(async { fetch_accounts(&pool).await.unwrap() });
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
}

impl Default for AccountListModel {
    fn default() -> Self {
        Self {
            base: Default::default(),
            load_accounts: Default::default(),
            accounts: Default::default(),
        }
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

        const NAME_ROLE: i32 = 257;
        const BALANCE_ROLE: i32 = 258;

        match role {
            NAME_ROLE => QString::from(account.name.clone()).into(),
            BALANCE_ROLE => QString::from(account.balance.to_string()).into(),
            _ => QVariant::default(),
        }
    }

    fn role_names(&self) -> std::collections::HashMap<i32, qttypes::QByteArray> {
        let mut roles = HashMap::new();
        roles.insert(257, QByteArray::from("name"));
        roles.insert(258, QByteArray::from("balance"));
        roles
    }
}
