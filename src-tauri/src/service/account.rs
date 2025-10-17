use rust_decimal::prelude::*;
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;

use crate::{Money, DECIMAL_SCALE};

#[derive(Debug, Serialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Account {
    pub id: String,
    pub name: String,
    pub starting_balance: Money,
}

// TODO: add fetch
impl Account {
    pub async fn create(
        name: &str,
        starting_balance: Decimal,
        pool: &sqlx::SqlitePool,
    ) -> Result<Self, crate::Error> {
        // TODO: add currency code
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

        let starting_balance = Money::from_scaled(record.starting_balance);

        Ok(Self {
            id: record.id,
            name: record.name,
            starting_balance,
        })
    }

    /// Delete an [`Account`].
    #[allow(unused)]
    pub async fn delete(id: &str, pool: &SqlitePool) -> Result<(), crate::Error> {
        sqlx::query!("DELETE FROM accounts WHERE id=$1", id)
            .execute(pool)
            .await?;

        Ok(())
    }
}

/// Fetch all the accounts from the database.
pub async fn fetch_accounts(pool: &SqlitePool) -> Result<Vec<Account>, crate::Error> {
    let records = sqlx::query!("SELECT id FROM accounts")
        .fetch_all(pool)
        .await?;

    let mut accounts = vec![];
    for record in records {
        let account = Account::from_id(&record.id, pool).await?;
        accounts.push(account);
    }

    Ok(accounts)
}

#[cfg(test)]
mod test {
    use super::*;

    #[sqlx::test]
    async fn get_accounts(pool: SqlitePool) -> Result<(), crate::Error> {
        Account::create("", Decimal::ZERO, &pool).await?;
        Account::create("", Decimal::ZERO, &pool).await?;
        Account::create("", Decimal::ZERO, &pool).await?;

        let accounts = fetch_accounts(&pool).await?;
        assert_eq!(accounts.len(), 3);
        Ok(())
    }

    #[sqlx::test]
    async fn fetch_account(pool: sqlx::SqlitePool) -> Result<(), crate::Error> {
        let mut amount = dec!(20.0);
        dbg!(&amount.to_i64());
        let amount = amount.to_i64().unwrap();
        let record = sqlx::query!(
            "INSERT INTO accounts(name,starting_balance) VALUES('C3PO',$1) RETURNING id",
            amount
        )
        .fetch_one(&pool)
        .await?;
        
        let account = Account::from_id(&record.id, &pool).await?;
        dbg!(&account);
        // assert_eq!(account.name, "C3PO");
        // assert_eq!(account.starting_balance, dec!(20.0));

        Ok(())
    }

    #[sqlx::test]
    async fn create_account(pool: sqlx::SqlitePool) -> Result<(), crate::Error> {
        Account::create("My account", dec!(20.00), &pool).await?;
        let account = sqlx::query!("SELECT * FROM accounts")
            .fetch_one(&pool)
            .await?;
        assert_eq!(account.name, "My account");
        assert_eq!(account.starting_balance, dec!(20.00).to_i64().unwrap());
        Ok(())
    }

    #[sqlx::test]
    async fn delete_account(pool: sqlx::SqlitePool) -> Result<(), crate::Error> {
        Account::create("My account", dec!(20.00), &pool).await?;
        Account::create("My account", dec!(20.00), &pool).await?;
        let account = Account::create("My account", dec!(20.00), &pool).await?;
        let records = sqlx::query!("SELECT * FROM accounts")
            .fetch_all(&pool)
            .await?;
        assert_eq!(records.len(), 3);

        Account::delete(&account.id, &pool).await?;
        let records = sqlx::query!("SELECT * FROM accounts")
            .fetch_all(&pool)
            .await?;
        assert_eq!(records.len(), 2);
        Ok(())
    }
}
