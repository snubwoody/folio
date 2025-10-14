use crate::init_database;
use rust_decimal::prelude::*;
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;

/// TODO: add transaction
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Account {
    pub id: String,
    pub name: String,
    pub starting_balance: Decimal,
}

// TODO: add fetch
impl Account {
    pub async fn create(
        name: &str,
        starting_balance: Decimal,
        pool: &sqlx::SqlitePool,
    ) -> Result<Self, crate::Error> {
        let balance = starting_balance.to_string();
        let record = sqlx::query!(
            "INSERT INTO accounts(name,starting_balance) VALUES($1,$2) RETURNING id",
            name,
            balance
        )
        .fetch_one(pool)
        .await?;

        Self::from_id(&record.id, pool).await
    }

    /// Fetch the [`Account`] from the database.
    pub async fn from_id(id: &str, pool: &sqlx::SqlitePool) -> Result<Self, crate::Error> {
        let record = sqlx::query!("SELECT * FROM accounts WHERE id = $1", id)
            .fetch_one(pool)
            .await?;

        let starting_balance = Decimal::from_str(&record.starting_balance)?;

        Ok(Self {
            id: record.id,
            name: record.name,
            starting_balance,
        })
    }
}

/// Query used for creating accounts.
struct CreateAccount {
    name: String,
    starting_balance: Decimal,
}

#[cfg(test)]
mod test {
    use super::*;

    #[sqlx::test]
    async fn fetch_account(pool: sqlx::SqlitePool) -> Result<(), crate::Error> {
        let record = sqlx::query!(
            "INSERT INTO accounts(name,starting_balance) VALUES('C3PO','20.000') RETURNING id"
        )
        .fetch_one(&pool)
        .await?;

        let account = Account::from_id(&record.id, &pool).await?;
        assert_eq!(account.name, "C3PO");
        assert_eq!(account.starting_balance.to_string(), "20.000");

        Ok(())
    }

    #[sqlx::test]
    async fn create_account(pool: sqlx::SqlitePool) {
        Account::create("My account", dec!(20.00), &pool).await;
        let account = sqlx::query!("SELECT * FROM accounts")
            .fetch_one(&pool)
            .await
            .unwrap();
        assert_eq!(account.name, "My account");
        assert_eq!(account.starting_balance, "20.00");
    }
}
