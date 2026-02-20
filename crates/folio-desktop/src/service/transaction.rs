use crate::Money;
use chrono::{Local, NaiveDate};
use serde::Serialize;
use sqlx::{FromRow, SqlitePool};
use std::marker::PhantomData;

pub struct Expense;
pub struct Income;
pub struct Transfer;

pub struct TransactionBuilder<T> {
    amount: Money,
    from_account_id: Option<String>,
    to_account_id: Option<String>,
    transaction_date: NaiveDate,
    category_id: Option<String>,
    note: Option<String>,
    marker: PhantomData<T>,
}

impl<T> Default for TransactionBuilder<T> {
    fn default() -> Self {
        Self {
            amount: Money::ZERO,
            from_account_id: None,
            to_account_id: None,
            category_id: None,
            note: None,
            transaction_date: Local::now().date_naive(),
            marker: PhantomData,
        }
    }
}

impl<T> TransactionBuilder<T> {
    pub fn amount(mut self, amount: Money) -> TransactionBuilder<T> {
        self.amount = amount;
        self
    }

    pub fn note(mut self, note: &str) -> TransactionBuilder<T> {
        self.note = Some(note.to_owned());
        self
    }

    /// Sets the transaction date
    pub fn date(mut self, date: NaiveDate) -> TransactionBuilder<T> {
        self.transaction_date = date;
        self
    }

    pub fn category(mut self, id: &str) -> TransactionBuilder<T> {
        self.category_id = Some(id.to_owned());
        self
    }
}

impl TransactionBuilder<Transfer> {
    pub fn accounts(mut self, from: &str, to: &str) -> Self {
        self.from_account_id = Some(from.to_owned());
        self.to_account_id = Some(to.to_owned());
        self
    }

    pub async fn create(self, pool: &sqlx::SqlitePool) -> crate::Result<Transaction> {
        let row: Transaction = sqlx::query_as(
            "INSERT INTO transactions(transaction_date,from_account_id,to_account_id,amount,note,category_id)
                VALUES ($1,$2,$3,$4,$5,$6)
                RETURNING *"
        )
            .bind(self.transaction_date)
            .bind(self.from_account_id.unwrap_or_default())
            .bind(self.to_account_id.unwrap_or_default())
            .bind(self.amount.inner())
            .bind(self.note)
            .bind(self.category_id)
            .fetch_one(pool)
            .await?;
        Ok(row)
    }
}

impl TransactionBuilder<Income> {
    pub fn account_id(mut self, id: &str) -> Self {
        self.to_account_id = Some(id.to_owned());
        self
    }

    pub async fn create(self, pool: &sqlx::SqlitePool) -> crate::Result<Transaction> {
        let row: Transaction = sqlx::query_as(
            "INSERT INTO transactions(transaction_date,to_account_id,amount,note,category_id)
                VALUES ($1,$2,$3,$4,$5)
                RETURNING *",
        )
        .bind(self.transaction_date)
        .bind(self.to_account_id.unwrap_or_default())
        .bind(self.amount.inner())
        .bind(self.note)
        .bind(self.category_id)
        .fetch_one(pool)
        .await?;
        Ok(row)
    }
}

impl TransactionBuilder<Expense> {
    pub fn account_id(mut self, id: &str) -> Self {
        self.from_account_id = Some(id.to_owned());
        self
    }

    pub async fn create(self, pool: &sqlx::SqlitePool) -> crate::Result<Transaction> {
        let row: Transaction = sqlx::query_as(
            "INSERT INTO transactions(transaction_date,from_account_id,amount,note,category_id)
                VALUES ($1,$2,$3,$4,$5)
                RETURNING *",
        )
        .bind(self.transaction_date)
        .bind(self.from_account_id.unwrap_or_default())
        .bind(self.amount.inner())
        .bind(self.note)
        .bind(self.category_id)
        .fetch_one(pool)
        .await?;
        Ok(row)
    }
}

pub struct UpdateBuilder{
    
}

// TODO: replace amount with Money
#[derive(FromRow, Debug, Clone, PartialOrd, PartialEq, Serialize)]
#[serde(rename_all="camelCase")]
pub struct Transaction {
    pub id: String,
    pub amount: i64,
    pub from_account_id: Option<String>,
    pub to_account_id: Option<String>,
    pub transaction_date: NaiveDate,
    pub category_id: Option<String>,
    pub created_at: i64,
    pub note: Option<String>,
}

impl Transaction {
    pub fn expense() -> TransactionBuilder<Expense> {
        Default::default()
    }

    pub fn transfer() -> TransactionBuilder<Transfer> {
        Default::default()
    }

    pub fn income() -> TransactionBuilder<Income> {
        Default::default()
    }

    /// Fetches the transaction from the database with a matching `id`. If the matching row
    /// is not found an error will be returned.
    pub async fn fetch(id: &str, pool: &sqlx::SqlitePool) -> crate::Result<Self> {
        let transaction: Self = sqlx::query_as("SELECT * FROM transactions WHERE id=$1")
            .bind(id)
            .fetch_one(pool)
            .await?;

        Ok(transaction)
    }

    /// Fetch all the transactions from the database.
    pub async fn fetch_all(pool: &SqlitePool) -> Result<Vec<Self>, crate::Error> {
        let rows: Vec<Self> = sqlx::query_as("SELECT * from transactions")
            .fetch_all(pool)
            .await?;

        Ok(rows)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn transaction_builder_fields() {
        let date = NaiveDate::parse_from_str("2024-12-12", "%Y-%m-%d").unwrap();
        let builder = TransactionBuilder::<Expense>::default()
            .note("NOTE__")
            .date(date)
            .category("C_")
            .amount(Money::MAX);

        assert_eq!(builder.note.unwrap(), "NOTE__");
        assert_eq!(builder.category_id.unwrap(), "C_");
        assert_eq!(builder.transaction_date, date);
        assert_eq!(builder.amount, Money::MAX);
    }

    #[test]
    fn transaction_builder_expense() {
        let builder = TransactionBuilder::<Expense>::default().account_id("A1");
        assert_eq!(builder.from_account_id.unwrap(), "A1");
    }

    #[test]
    fn transaction_builder_income() {
        let builder = TransactionBuilder::<Income>::default().account_id("A1");
        assert_eq!(builder.to_account_id.unwrap(), "A1");
    }

    #[test]
    fn transaction_builder_transaction() {
        let builder = TransactionBuilder::<Transfer>::default().accounts("A1", "A2");
        assert_eq!(builder.from_account_id.unwrap(), "A1");
        assert_eq!(builder.to_account_id.unwrap(), "A2");
    }
}
