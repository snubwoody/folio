use crate::{Error, Money};
use chrono::{Local, NaiveDate};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, QueryBuilder, SqlitePool};
use std::marker::PhantomData;
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
enum TransactionType {
    /// A transaction is an expense when `from_account_id` is not `None`
    /// and `to_account_id` is `None`
    Expense,
    Income,
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

    fn transaction_type(&self) -> TransactionType {
        if self.to_account_id.is_none() {
            return TransactionType::Expense;
        }

        if self.from_account_id.is_none() {
            return TransactionType::Income;
        }

        TransactionType::Transfer
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
        // TODO:
        // - income
        // - expense
        // - transfer
        let transaction = Self::fetch(id, pool).await?;

        if transaction.transaction_type() == TransactionType::Transfer {
            return Err(Error::invalid_op("Cannot set inflow for a transfer"));
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
    use crate::service::Account;

    #[test]
    fn transaction_type_expense() {
        let expense = Transaction {
            from_account_id: Some("".to_owned()),
            ..Default::default()
        };
        let income = Transaction {
            to_account_id: Some("".to_owned()),
            ..Default::default()
        };
        let transfer = Transaction {
            from_account_id: Some("".to_owned()),
            to_account_id: Some("".to_owned()),
            ..Default::default()
        };

        assert_eq!(expense.transaction_type(), TransactionType::Expense);
        assert_eq!(income.transaction_type(), TransactionType::Income);
        assert_eq!(transfer.transaction_type(), TransactionType::Transfer);
    }

    #[sqlx::test]
    async fn set_outflow_for_expense(pool: SqlitePool) -> crate::Result<()> {
        let account = Account::create("__", Money::ZERO, &pool).await?;
        let transaction = Transaction::expense()
            .amount(Money::MAX)
            .account_id(&account.id)
            .create(&pool)
            .await?;

        Transaction::set_outflow(&transaction.id, Money::from_f64(10.0), &pool).await?;
        let t = Transaction::fetch(&transaction.id, &pool).await?;
        assert_eq!(t.amount, Money::from_f64(10.0));
        assert_eq!(
            t.from_account_id.unwrap(),
            transaction.from_account_id.unwrap()
        );
        assert!(t.to_account_id.is_none());
        Ok(())
    }

    #[sqlx::test]
    async fn set_inflow_for_income(pool: SqlitePool) -> crate::Result<()> {
        let account = Account::create("__", Money::ZERO, &pool).await?;
        let transaction = Transaction::income()
            .amount(Money::MAX)
            .account_id(&account.id)
            .create(&pool)
            .await?;

        Transaction::set_inflow(&transaction.id, Money::from_f64(10.0), &pool).await?;
        let t = Transaction::fetch(&transaction.id, &pool).await?;
        assert_eq!(t.amount, Money::from_f64(10.0));
        assert_eq!(t.to_account_id.unwrap(), transaction.to_account_id.unwrap());
        assert!(t.from_account_id.is_none());
        Ok(())
    }

    #[sqlx::test]
    async fn set_inflow_for_transfer(pool: SqlitePool) -> crate::Result<()> {
        let account = Account::create("__", Money::ZERO, &pool).await?;
        let account2 = Account::create("__", Money::ZERO, &pool).await?;
        let transaction = Transaction::transfer()
            .amount(Money::MAX)
            .accounts(&account.id, &account2.id)
            .create(&pool)
            .await?;

        let result = Transaction::set_inflow(&transaction.id, Money::from_f64(10.0), &pool).await;
        assert!(result.is_err());
        Ok(())
    }

    #[sqlx::test]
    async fn set_inflow_for_expense(pool: SqlitePool) -> crate::Result<()> {
        let account = Account::create("__", Money::ZERO, &pool).await?;
        let transaction = Transaction::expense()
            .amount(Money::MAX)
            .account_id(&account.id)
            .create(&pool)
            .await?;

        Transaction::set_inflow(&transaction.id, Money::from_f64(10.0), &pool).await?;
        let t = Transaction::fetch(&transaction.id, &pool).await?;
        assert_eq!(t.amount, Money::from_f64(10.0));
        assert_eq!(
            t.to_account_id.unwrap(),
            transaction.from_account_id.unwrap()
        );
        assert!(t.from_account_id.is_none());
        Ok(())
    }

    #[sqlx::test]
    async fn set_outflow_for_income(pool: SqlitePool) -> crate::Result<()> {
        // Setting outflow on an income should turn it into an expense
        let account = Account::create("__", Money::ZERO, &pool).await?;
        let transaction = Transaction::income()
            .amount(Money::MAX)
            .account_id(&account.id)
            .create(&pool)
            .await?;

        Transaction::set_outflow(&transaction.id, Money::from_f64(10.0), &pool).await?;
        let t = Transaction::fetch(&transaction.id, &pool).await?;
        assert_eq!(t.amount, Money::from_f64(10.0));
        assert_eq!(
            t.from_account_id.unwrap(),
            transaction.to_account_id.unwrap()
        );
        assert!(t.to_account_id.is_none());
        Ok(())
    }

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
