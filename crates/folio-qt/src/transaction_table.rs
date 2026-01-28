use std::collections::HashMap;
use crate::{db_pool, RUNTIME};
use folio_lib::service::{Expense, fetch_expenses};
use qmetaobject::{QAbstractTableModel, QObject, qt_base_class, qt_method};
use qttypes::{QByteArray, QModelIndex, QString, QVariant};

const COLUMN_COUNT: i32 = 4;

#[derive(QObject, Default)]
pub struct TransactionTableModel {
    base: qt_base_class!(trait QAbstractTableModel),
    load_expenses: qt_method!(fn(&mut self)),
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
        // TODO: create a macro for this
        const DISPLAY_ROLE: i32 = 0;

        if role != DISPLAY_ROLE{
            return QVariant::default();
        }

        let category = expense.category.clone().unwrap_or_default().title;
        let account = expense.account.clone().unwrap_or_default().name;
        let date = expense.date.to_string();
        let amount = expense.amount.to_string();

        match index.column() {
            0 => QVariant::from(QString::from(category)),
            1 => QVariant::from(QString::from(account)),
            2 => QVariant::from(QString::from(date)),
            3 => QVariant::from(QString::from(amount)),
            _ => QVariant::default(),
        }
    }

}
