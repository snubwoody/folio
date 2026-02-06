use crate::{RUNTIME, db_pool};
use folio_lib::service::{Account, Expense, fetch_expenses};
use qmetaobject::{QAbstractTableModel, QObject, USER_ROLE, qt_base_class, qt_method};
use qttypes::{QByteArray, QModelIndex, QString, QVariant};
use sqlx::types::chrono::NaiveDate;
use std::collections::HashMap;

#[derive(QObject, Default)]
pub struct TransactionTableModel {
    base: qt_base_class!(trait QAbstractTableModel),
    load_expenses: qt_method!(fn(&mut self)),
    add_expense: qt_method!(fn(&mut self)),
    expenses: Vec<Expense>,
}

impl TransactionTableModel {
    const COLUMN_COUNT: i32 = 4;

    pub fn load_expenses(&mut self) {
        self.begin_reset_model();
        self.expenses = RUNTIME.block_on(async {
            fetch_expenses(db_pool())
                .await
                .expect("failed to load expenses")
        });
        self.end_reset_model();
    }

    pub fn add_expense(&mut self) {
        // TODO: make the default date the current date
        RUNTIME.block_on(async {
            Expense::create(Default::default(), db_pool())
                .await
                .expect("failed to create expense");
        });
        self.load_expenses();
    }
}

impl QAbstractTableModel for TransactionTableModel {
    fn row_count(&self) -> i32 {
        self.expenses.len() as i32
    }

    fn column_count(&self) -> i32 {
        Self::COLUMN_COUNT
    }

    fn data(&self, index: QModelIndex, role: i32) -> QVariant {
        if index.row() >= self.expenses.len() as i32 || index.column() >= Self::COLUMN_COUNT {
            return QVariant::default();
        }

        let expense = &self.expenses[index.row() as usize];
        const DISPLAY_ROLE: i32 = 0;
        // const EDIT_ROLE: i32 = 2;

        if role != DISPLAY_ROLE {
            return QVariant::default();
        }

        let category = expense.category.clone().unwrap_or_default().title;
        let account = expense.account.clone().unwrap_or_default().name;
        let date = expense.date.to_string();
        let amount = expense.amount.to_string();

        let variant = match index.column() {
            0 => QVariant::from(QString::from(category)),
            1 => QVariant::from(QString::from(account)),
            2 => QVariant::from(QString::from(date)),
            3 => QVariant::from(QString::from(amount)),
            _ => QVariant::default(),
        };

        variant
    }

    fn role_names(&self) -> HashMap<i32, QByteArray> {
        let mut roles = HashMap::new();
        roles.insert(0, "display".into());
        roles
    }
}
