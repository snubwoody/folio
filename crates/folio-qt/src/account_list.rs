use crate::{RUNTIME, db_pool};
use folio_lib::service::{Account, fetch_accounts, EditAccount};
use qmetaobject::{QAbstractListModel, QObject, qt_base_class, qt_method};
use qttypes::{QByteArray, QString, QVariant};
use std::collections::HashMap;
use std::str::FromStr;
use folio_lib::Money;

#[derive(QObject, Default)]
pub struct AccountListModel {
    base: qt_base_class!(trait QAbstractListModel),
    load_accounts: qt_method!(fn(&mut self)),
    add_account: qt_method!(fn(&mut self, name: QString, balance: QString)),
    edit_account: qt_method!(fn(&mut self, id: QString, name: QString, balance: QString)),
    accounts: Vec<Account>,
}

impl AccountListModel {
    const NAME_ROLE: i32 = qmetaobject::USER_ROLE + 1;
    const BALANCE_ROLE: i32 = Self::NAME_ROLE + 1;
    const STARTING_BALANCE_ROLE: i32 = Self::BALANCE_ROLE + 1;
    const ID_ROLE: i32 = Self::STARTING_BALANCE_ROLE + 1;

    pub fn new() -> Self {
        let accounts = RUNTIME.block_on(async { fetch_accounts(db_pool()).await.unwrap() });
        Self {
            accounts,
            ..Default::default()
        }
    }

    pub fn edit(mut self) {
        self.begin_reset_model();

        self.end_reset_model();
    }

    pub fn load_accounts(&mut self) {
        self.begin_reset_model();
        self.accounts = RUNTIME.block_on(async { fetch_accounts(db_pool()).await.unwrap() });
        self.end_reset_model();
    }

    pub fn edit_account(&mut self, id: QString, name: QString, balance: QString) {
        let name = name.to_string();
        let balance = Money::from_str(&balance.to_string()).ok();
        let opts = EditAccount{
            name: Some(name),
            starting_balance: balance
        };
        RUNTIME.block_on(
            async {
                // FIXME: probably shouldn't panic
                let result = Account::edit(&*id.to_string(), opts, db_pool()).await;
                if let Err(err) = result{
                    tracing::error!("Error: {}",err);
                }
            }
        );
        // {
        //     let mut accounts = self.accounts.borrow_mut();
        //     accounts.load_accounts();
        // }
        // self.accounts.borrow_mut().add_account();
        // self.accounts.borrow_mut().load_accounts();

        // self.load_data();
        self.load_accounts();
    }

    pub fn add_account(&mut self, name: QString, balance: QString) {
        let name = name.to_string();
        let balance = Money::from_str(balance.to_string().as_str()).unwrap_or_default();
        RUNTIME.block_on(async { Account::create(&name, balance, db_pool()).await.unwrap() });
        self.load_accounts();
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
            Self::ID_ROLE => QString::from(account.id.clone()).into(),
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
        roles.insert(Self::ID_ROLE, QByteArray::from("id"));
        roles.insert(Self::NAME_ROLE, QByteArray::from("name"));
        roles.insert(Self::BALANCE_ROLE, QByteArray::from("balance"));
        roles.insert(
            Self::STARTING_BALANCE_ROLE,
            QByteArray::from("startingBalance"),
        );
        roles
    }
}
