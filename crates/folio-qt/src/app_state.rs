use crate::account_list::AccountListModel;
use crate::transaction_table::TransactionTableModel;
use crate::{RUNTIME, db_pool};
use folio_lib::Money;
use folio_lib::service::{Account, EditAccount};
use qmetaobject::{QGadget, QObject, qt_base_class, qt_method, qt_property};
use qttypes::QString;
use std::cell::RefCell;
use std::str::FromStr;

/// Global app state
#[derive(QObject, Default)]
pub struct AppState {
    base: qt_base_class!(trait QObject),
    accounts: qt_property!(RefCell<AccountListModel>),
    transactions: qt_property!(RefCell<TransactionTableModel>),
    load_data: qt_method!(fn(&mut self)),
    add_account: qt_method!(fn(&mut self, name: QString, balance: QString)),
    edit_account: qt_method!(fn(&mut self, id: QString, name: QString, balance: QString)),
}

#[derive(PartialOrd, PartialEq, Default, Clone, Debug, QGadget)]
struct EditAccountOpts {
    name: Option<QString>,
    starting_balance: Option<QString>,
}

impl From<EditAccountOpts> for EditAccount {
    fn from(value: EditAccountOpts) -> Self {
        // TODO: doc the money conversion
        let name = value.name.map(|s| s.to_string());
        let balance = value
            .starting_balance
            .and_then(|v| Money::from_str(&v.to_string()).ok());
        Self {
            name,
            starting_balance: balance,
        }
    }
}

impl AppState {
    pub fn load_data(&mut self) {
        self.accounts.borrow_mut().load_accounts();
        self.transactions.borrow_mut().load_expenses();
    }

    pub fn edit_account(&mut self, id: QString, name: QString, balance: QString) {
        let name = name.to_string();
        let balance = Money::from_str(&balance.to_string()).ok();
        let opts = EditAccount {
            name: Some(name),
            starting_balance: balance,
        };
        RUNTIME.block_on(async {
            // FIXME: probably shouldn't panic
            let result = Account::edit(&id.to_string(), opts, db_pool()).await;
            if let Err(err) = result {
                tracing::error!("Error: {}", err);
            }
        });
        // {
        //     let mut accounts = self.accounts.borrow_mut();
        //     accounts.load_accounts();
        // }
        // self.accounts.borrow_mut().add_account();
        // self.accounts.borrow_mut().load_accounts();

        // self.load_data();
    }

    pub fn add_account(&mut self, name: QString, balance: QString) {
        let name = name.to_string();
        let balance = Money::from_str(balance.to_string().as_str()).unwrap_or_default();
        RUNTIME.block_on(async { Account::create(&name, balance, db_pool()).await.unwrap() });
        self.load_data();
    }
}
