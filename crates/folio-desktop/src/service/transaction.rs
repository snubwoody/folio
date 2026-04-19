use crate::{Error, Money};
use chrono::{Local, NaiveDate};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, QueryBuilder, SqlitePool};
use std::marker::PhantomData;
use rusqlite::{params_from_iter, Connection};
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

    /// Finalise the query and insert a transaction.
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
    pub fn expense() -> TransactionBuilder<Expense> {
        Default::default()
    }

    pub fn transfer() -> TransactionBuilder<Transfer> {
        Default::default()
    }

    pub fn income() -> TransactionBuilder<Income> {
        Default::default()
    }

    pub fn edit(id: &str) -> EditBuilder {
        EditBuilder::new(id)
    }

    pub fn transaction_type(&self) -> TransactionType {
        if self.to_account_id.is_none() {
            return TransactionType::Expense;
        }

        if self.from_account_id.is_none() {
            return TransactionType::Income;
        }

        TransactionType::Transfer
    }

    /// Set the transaction account i.e. the `from_account_id` column for expenses and transfers, and the
    /// `to_account_id` for incomes.
    pub async fn set_account(id: &str, account_id: &str, pool: &SqlitePool) -> crate::Result<Self> {
        let transaction = Transaction::fetch(id, pool).await?;
        let mut query_builder = QueryBuilder::new("UPDATE transactions ");

        match transaction.transaction_type() {
            TransactionType::Income => query_builder.push("SET to_account_id = "),
            _ => query_builder.push("SET from_account_id = "),
        };

        query_builder.push_bind(account_id);

        let query = query_builder.push("WHERE id = ").push_bind(id).build();
        query.execute(pool).await?;
        info!(id = id, "Set transaction account");
        Self::fetch(id, pool).await
    }

    /// Set the payee field of a transaction, turning the transaction into a transfer.
    pub async fn set_payee(id: &str, account_id: &str, pool: &SqlitePool) -> crate::Result<Self> {
        let transaction = Transaction::fetch(id, pool).await?;
        let mut query_builder = QueryBuilder::new("UPDATE transactions ");
        query_builder
            .push("SET to_account_id = ")
            .push_bind(account_id);

        if transaction.transaction_type() == TransactionType::Income {
            query_builder
                .push(", from_account_id = ")
                .push_bind(transaction.to_account_id.unwrap_or_default());
        }
        let query = query_builder
            .push(",category_id = NULL ")
            .push("WHERE id = ")
            .push_bind(id)
            .build();
        query.execute(pool).await?;
        info!(id = id, "Set transaction payee");
        Self::fetch(id, pool).await
    }

    /// Deletes all the transactions with the corresponding ids
    pub fn delete<S: AsRef<str>>(ids: &[S], conn: &Connection) -> crate::Result<()> {
        if ids.is_empty() {
            return Ok(());
        }
        let mut query = String::from("DELETE FROM transactions WHERE id IN ");
        query.push_str("(");
        for (index,_) in ids.iter().enumerate() {
            if index == ids.len() - 1 {
                query.push_str("?");
                continue;
            }
            query.push_str("?,");
        }
        query.push_str(")");

        let mut stmt = conn.prepare_cached(&query)?;
        let params = params_from_iter(ids.into_iter().map(|id|id.as_ref()));
        stmt.execute(params)?;

        info!("Deleted {} transactions", ids.len());
        Ok(())
    }

    pub async fn set_outflow(id: &str, amount: Money, pool: &SqlitePool) -> crate::Result<Self> {
        let transaction = Self::fetch(id, pool).await?;
        let mut query = QueryBuilder::new("UPDATE transactions ");
        query.push("SET amount = ").push_bind(amount);

        // Setting outflow on an income changes it to an expense
        if transaction.transaction_type() == TransactionType::Income {
            query
                .push(", to_account_id = NULL, from_account_id = ")
                .push_bind(transaction.to_account_id.unwrap_or_default());
        }
        query
            .push("WHERE id = ")
            .push_bind(id)
            .build()
            .execute(pool)
            .await?;
        info!(id = id, "Updated transaction");
        Self::fetch(id, pool).await
    }

    pub async fn set_inflow(id: &str, amount: Money, pool: &SqlitePool) -> crate::Result<Self> {
        let transaction = Self::fetch(id, pool).await?;

        if transaction.transaction_type() == TransactionType::Transfer {
            return Err(Error::new("Cannot set inflow for a transfer"));
        }

        let mut query = QueryBuilder::new("UPDATE transactions ");
        query.push("SET amount = ").push_bind(amount);

        // Setting inflow on an expense changes it to an income
        if transaction.transaction_type() == TransactionType::Expense {
            query
                .push(", from_account_id = NULL, to_account_id = ")
                .push_bind(transaction.from_account_id.unwrap_or_default());
        }

        query
            .push("WHERE id = ")
            .push_bind(id)
            .build()
            .execute(pool)
            .await?;
        info!(id = id, "Updated transaction");
        Self::fetch(id, pool).await
    }

    /// Fetches the transaction from the database with a matching `id`. If the matching row
    /// is not found an error will be returned.
    pub async fn fetch(id: &str, pool: &SqlitePool) -> crate::Result<Self> {
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
