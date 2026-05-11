use crate::{Error, Money, SqliteConnection};
use chrono::{Local, NaiveDate};
use rusqlite::{Row, params};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, QueryBuilder, SqlitePool};
use tracing_subscriber::fmt::format;
use std::{fmt::Display, marker::PhantomData};
use tracing::info;

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
        info!(id=?row.id,"Created new income");
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
        info!(id=?row.id,"Created new expense");
        Ok(row)
    }
}

#[derive(Debug, Clone, PartialOrd, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct EditBuilder {
    pub id: String,
    pub amount: Option<Money>,
    pub category_id: Option<String>,
    pub from_account_id: Option<String>,
    pub to_account_id: Option<String>,
    pub note: Option<String>,
    pub transaction_date: Option<NaiveDate>,
}

impl EditBuilder {
    pub fn new(id: &str) -> Self {
        Self {
            id: id.to_owned(),
            ..Default::default()
        }
    }
    /// Set the new transaction amount
    pub fn amount(mut self, value: Money) -> Self {
        self.amount = Some(value);
        self
    }

    /// Set the new transaction category
    pub fn category(mut self, id: &str) -> Self {
        self.category_id = Some(id.to_owned());
        self
    }

    /// Set the new transaction note
    pub fn note(mut self, value: &str) -> Self {
        self.note = Some(value.to_owned());
        self
    }

    pub fn date(mut self, date: NaiveDate) -> Self {
        self.transaction_date = Some(date);
        self
    }

    pub fn from_account(mut self, id: &str) -> Self {
        self.from_account_id = Some(id.to_owned());
        self
    }

    pub fn to_account(mut self, id: &str) -> Self {
        self.to_account_id = Some(id.to_owned());
        self
    }

    pub async fn update(self, pool: &sqlx::SqlitePool) -> crate::Result<Transaction> {
        let row: Transaction = sqlx::query_as(
            "UPDATE transactions
            SET amount = COALESCE($1,amount),
                note = COALESCE($2,note),
                transaction_date = COALESCE($3,transaction_date),
                from_account_id = COALESCE($4,from_account_id),
                to_account_id = COALESCE($5,to_account_id),
                category_id = COALESCE($6,category_id)
            WHERE id=$7
            RETURNING *
            ",
        )
        .bind(self.amount)
        .bind(self.note)
        .bind(self.transaction_date)
        .bind(self.from_account_id)
        .bind(self.to_account_id)
        .bind(self.category_id)
        .bind(&self.id)
        .fetch_one(pool)
        .await?;

        info!(id = self.id, "Updated transaction");
        Ok(row)
    }
}

#[derive(Debug, Clone, Copy, PartialOrd, PartialEq, Serialize)]
pub enum TransactionType {
    /// A transaction is an expense when `from_account_id` is not `None`
    /// and `to_account_id` is `None`.
    Expense,
    /// A transaction is an income when `from_account_id` is `None`
    /// and `to_account_id` is not `None`.
    Income,
    /// A transaction is a transfer when `from_account_id` and `to_account_id` are both not `None`.
    Transfer,
}

#[derive(Clone)]
pub struct TransactionService {
    pool: SqlitePool,
    connection: SqliteConnection,
}

impl TransactionService {
    pub fn new(pool: SqlitePool, connection: SqliteConnection) -> Self {
        Self { pool, connection }
    }

    pub fn expense(&self) -> TransactionBuilder<Expense> {
        Default::default()
    }

    pub fn transfer(&self) -> TransactionBuilder<Transfer> {
        Default::default()
    }

    pub fn income(&self) -> TransactionBuilder<Income> {
        Default::default()
    }

    pub fn edit(&self, id: &str) -> EditBuilder {
        EditBuilder::new(id)
    }

    /// Fetches the transaction from the database with a matching `id`. If the matching row
    /// is not found an error will be returned.
    pub fn fetch(&self, id: &str) -> crate::Result<Transaction> {
        let connection = self.connection.get();
        let mut stmt = connection.prepare_cached("select * from transactions where id = ?")?;
        let row = stmt.query_row([id], |row| Transaction::try_from(row))?;
        Ok(row)
    }

    /// Fetch all the transactions from the database.
    pub fn fetch_all(&self) -> Result<Vec<Transaction>, crate::Error> {
        let connection = self.connection.get();
        let mut stmt = connection.prepare_cached("select * from transactions")?;
        let rows = stmt.query_map((), |row| Transaction::try_from(row))?;
        let mut transactions = Vec::new();
        for row in rows {
            transactions.push(row?)
        }
        Ok(transactions)
    }

    /// Deletes all the provided transactions.
    pub fn delete_all<S: AsRef<str> + Display>(&self, ids: &[S]) -> crate::Result<()> {
        if ids.is_empty() {
            return Ok(());
        }
        let connection = self.connection.get();
        let mut query = String::from("delete from transactions where id in (");

        for id in ids {
            query.push_str(&format!("'{id}',"));
        }

        // Remove the last comma
        query.pop();
        query.push(')');
        connection.execute(&query, ())?;

        info!("Deleted {} transactions", ids.len());
        Ok(())
    }

    pub fn set_outflow(&self, id: &str, amount: Money) -> crate::Result<Transaction> {
        let transaction = self.fetch(id)?;
        let connection = self.connection.get();

        let mut query = String::from("update transactions set amount = ?1");
        if transaction.transaction_type() == TransactionType::Income {
            let sql = format!(
                ",to_account_id = NULL, from_account_id = '{}'",
                transaction.to_account_id.unwrap_or_default()
            );
            query.push_str(&sql);
        }
        query.push_str(" where id = ?2 returning *");

        let mut stmt = connection.prepare_cached(&query)?;
        let row = stmt.query_row(params![amount.inner(), id], |row| {
            Transaction::try_from(row)
        })?;

        info!(id = id, "Set transaction outflow");
        Ok(row)
    }

    pub fn set_inflow(&self, id: &str, amount: Money) -> crate::Result<Transaction> {
        let transaction = self.fetch(id)?;

        if transaction.transaction_type() == TransactionType::Transfer {
            return Err(Error::new("Cannot set inflow for a transfer"));
        }

        let connection = self.connection.get();

        let mut query = String::from("update transactions set amount = ?1");

        // Setting inflow on an expense changes it to an income
        if transaction.transaction_type() == TransactionType::Expense {
            let sql = format!(
                ",from_account_id = NULL, to_account_id = '{}'",
                transaction.from_account_id.unwrap_or_default()
            );
            query.push_str(&sql);
        }
        query.push_str(" where id = ?2 returning *");

        let mut stmt = connection.prepare_cached(&query)?;
        let row = stmt.query_row(params![amount.inner(), id], |row| {
            Transaction::try_from(row)
        })?;

        info!(id = id, "Set transaction inflow");
        Ok(row)
    }

    /// Set the transaction account i.e. the `from_account_id` column for expenses and transfers, and the
    /// `to_account_id` for incomes.
    pub fn set_account(&self, id: &str, account_id: &str) -> crate::Result<Transaction> {
        let transaction = self.fetch(id)?;
        let connection = self.connection.get();
        let mut query = String::from("update transactions ");

        match transaction.transaction_type() {
            TransactionType::Income => query.push_str("set to_account_id = "),
            _ => query.push_str("set from_account_id = "),
        };

        query.push_str(&format!("'{account_id}'"));
        query.push_str(" where id = ? returning *");

        let mut stmt = connection.prepare_cached(&query)?;
        let row = stmt.query_row([id], |row| Transaction::try_from(row))?;

        info!(id = id, account = account_id, "Set transaction account");
        Ok(row)
    }

    /// Set the payee field of a transaction, turning the transaction into a transfer.
    pub fn set_payee(&self, id: &str, account_id: &str) -> crate::Result<Transaction> {
        let transaction = self.fetch(id)?;

        let connection = self.connection.get();
        let mut query = String::from("update transactions set to_account_id = ?1");

        if transaction.transaction_type() == TransactionType::Income {
            let sql = format!(", from_account_id = '{}'",transaction.to_account_id.unwrap_or_default());
            query
                .push_str(&sql);
        }
        query.push_str(",category_id = null where id = ?2 returning *");

        let mut stmt = connection.prepare_cached(&query)?;
        let row = stmt.query_row([account_id,id], |row| Transaction::try_from(row))?;
        
        info!(id = id, payee = account_id, "Set transaction payee");

        Ok(row)
    }
}

#[derive(FromRow, Debug, Clone, PartialOrd, PartialEq, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Transaction {
    pub id: String,
    pub amount: Money,
    pub from_account_id: Option<String>,
    pub to_account_id: Option<String>,
    pub transaction_date: NaiveDate,
    pub category_id: Option<String>,
    pub created_at: i64,
    pub note: Option<String>,
}

impl Transaction {
    pub fn transaction_type(&self) -> TransactionType {
        if self.to_account_id.is_none() {
            return TransactionType::Expense;
        }

        if self.from_account_id.is_none() {
            return TransactionType::Income;
        }

        TransactionType::Transfer
    }
}

impl<'a> TryFrom<&rusqlite::Row<'a>> for Transaction {
    type Error = rusqlite::Error;

    fn try_from(row: &Row) -> std::result::Result<Self, Self::Error> {
        let date: String = row.get(4)?;

        let transaction = Self {
            id: row.get(0)?,
            amount: Money::from_scaled(row.get(1)?),
            from_account_id: row.get(2).ok(),
            to_account_id: row.get(3).ok(),
            transaction_date: NaiveDate::parse_from_str(&date, "%Y-%m-%d").unwrap_or_default(),
            category_id: row.get(5).ok(),
            created_at: row.get(6)?,
            note: row.get(7).ok(),
        };
        Ok(transaction)
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
    fn edit_builder_fields() {
        let date = NaiveDate::parse_from_str("2024-12-12", "%Y-%m-%d").unwrap();
        let builder = EditBuilder::default()
            .date(date)
            .note("Note__")
            .amount(Money::MAX)
            .category("C")
            .to_account("A1")
            .from_account("A2");

        assert_eq!(builder.note.unwrap(), "Note__");
        assert_eq!(builder.category_id.unwrap(), "C");
        assert_eq!(builder.transaction_date.unwrap(), date);
        assert_eq!(builder.to_account_id.unwrap(), "A1");
        assert_eq!(builder.from_account_id.unwrap(), "A2");
        assert_eq!(builder.amount.unwrap(), Money::MAX);
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
