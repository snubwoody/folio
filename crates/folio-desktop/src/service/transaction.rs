use chrono::NaiveDate;
use sqlx::FromRow;
use crate::service;

#[derive(FromRow, Debug, Clone, PartialOrd, PartialEq)]
pub struct Transaction{
    pub id: String,
    pub amount: i64,
    pub from_account_id: Option<String>,
    pub to_account_id: Option<String>,
    pub transaction_date: NaiveDate,
    pub category_id: Option<String>,
    pub created_at: i64,
    pub note: Option<String>
}

impl Transaction{
    /// Fetches the transaction from the database with a matching `id`. If the matching row
    /// is not found an error will be returned.
    pub async fn fetch(id: &str,pool:&sqlx::SqlitePool) -> crate::Result<Self>{
        let transaction: Self = sqlx::query_as("SELECT * FROM transactions WHERE id=$1")
            .bind(id)
            .fetch_one(pool)
            .await?;

        Ok(transaction)
    }
}
