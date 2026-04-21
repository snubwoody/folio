// Copyright (C) 2025 Wakunguma Kalimukwa
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

use crate::Money;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{Row, SqlitePool};
use tracing::info;

// TODO: test the command input
#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EditAccount {
    name: Option<String>,
    starting_balance: Option<Money>,
}

impl EditAccount {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn name(mut self, name: &str) -> Self {
        self.name = Some(name.to_owned());
        self
    }

    pub fn starting_balance(mut self, money: Money) -> Self {
        self.starting_balance = Some(money);
        self
    }
}

/// Service struct for creating, reading and editing accounts.
#[derive(Clone)]
pub struct AccountService{
    pool: SqlitePool
}

impl AccountService{
    /// Creates a new [`AccountService`]
    pub fn new(pool: SqlitePool) -> Self{
        Self { pool }
    }

    /// Creates a new account.
    pub async fn create_account(
        &self,
        name: &str,
        starting_balance: Money,
    ) -> crate::Result<Account> {
        let now = Utc::now().timestamp();
        let balance = starting_balance.inner();
        let record = sqlx::query(
            "INSERT INTO accounts(name,starting_balance,created_at) VALUES($1,$2,$3) RETURNING id",
        )
            .bind(name)
            .bind(balance)
            .bind(now)
            .fetch_one(&self.pool)
            .await?;

        let id: String = record.get("id");
        info!(id=?id,"Created new account");
        self.fetch_account(&id).await
    }


    /// Fetch the [`Account`] from the database.
    ///
    /// # Error
    /// Returns an error if the account doesn't exist.
    pub async fn fetch_account(&self,id: &str) -> crate::Result<Account> {
        let record = sqlx::query!("SELECT * FROM accounts WHERE id = $1", id)
            .fetch_one(&self.pool)
            .await?;

        let starting_balance = Money::from_scaled(record.starting_balance);
        let balance = self.calculate_balance(id).await? + starting_balance;
        let created_at = record
            .created_at
            .and_then(|t| DateTime::from_timestamp(t, 0));
        Ok(Account {
            id: record.id,
            name: record.name,
            starting_balance,
            balance,
            created_at,
        })
    }

    pub async fn edit_account(
        &self,
        id: &str,
        opts: EditAccount,
    ) -> crate::Result<Account> {
        let account = self.fetch_account(id).await?;
        let starting_balance = opts
            .starting_balance
            .unwrap_or(account.starting_balance)
            .inner();
        let name = opts.name.unwrap_or(account.name);

        sqlx::query("UPDATE accounts SET name=$1,starting_balance=$2 WHERE id=$3")
            .bind(name)
            .bind(starting_balance)
            .bind(id)
            .execute(&self.pool)
            .await?;

        info!(id=?id,"Updated account");
        self.fetch_account(id).await
    }

    /// Deletes an [`Account`].
    pub async fn delete_account(&self,id: &str) -> crate::Result<()> {
        sqlx::query("DELETE FROM accounts WHERE id=$1")
            .bind(id)
            .execute(&self.pool)
            .await?;

        info!(id=?id,"Deleted account");
        Ok(())
    }

    /// Calculates the balance for an account. The balance is the sum of all expenses minus
    /// the sum of all incomes.
    pub async fn calculate_balance(&self,id: &str,) -> Result<Money, crate::Error> {
        let total_expenses = sqlx::query!(
            "
                SELECT COALESCE(SUM(amount),0) as total FROM transactions WHERE from_account_id = $1
                ",
            id
        )
            .fetch_one(&self.pool)
            .await?
            .total;

        let total_income = sqlx::query!(
            "
            SELECT COALESCE(SUM(amount),0) as total FROM transactions WHERE to_account_id = $1",
            id
        )
            .fetch_one(&self.pool)
            .await?
            .total;

        let difference = total_income - total_expenses;
        Ok(Money::from_scaled(difference))
    }

    /// Fetch all the accounts from the database.
    pub async fn fetch_all(&self) -> crate::Result<Vec<Account>> {
        let records = sqlx::query!("SELECT id FROM accounts")
            .fetch_all(&self.pool)
            .await?;

        let mut accounts = vec![];
        for record in records {
            let account = self.fetch_account(&record.id).await?;
            accounts.push(account);
        }

        Ok(accounts)
    }
}

// TODO: move to models
#[derive(Debug, Serialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Account {
    pub id: String,
    pub name: String,
    pub starting_balance: Money,
    // TODO: deprecate: fetch balance on demand
    pub balance: Money,
    pub created_at: Option<DateTime<Utc>>,
}

// TODO: add fetch
impl Account {
    #[deprecated]
    pub async fn create(
        name: &str,
        starting_balance: Money,
        pool: &SqlitePool,
    ) -> Result<Self, crate::Error> {
        // TODO: add currency code
        let now = Utc::now().timestamp();
        let balance = starting_balance.inner();
        let record = sqlx::query(
            "INSERT INTO accounts(name,starting_balance,created_at) VALUES($1,$2,$3) RETURNING id",
        )
        .bind(name)
        .bind(balance)
        .bind(now)
        .fetch_one(pool)
        .await?;

        let id: String = record.get("id");
        info!(id=?id,"Created new account");
        Self::from_id(&id, pool).await
    }

    #[deprecated]
    pub async fn edit(
        id: &str,
        opts: EditAccount,
        pool: &SqlitePool,
    ) -> Result<Self, crate::Error> {
        let account = Self::from_id(id, pool).await?;
        let starting_balance = opts
            .starting_balance
            .unwrap_or(account.starting_balance)
            .inner();
        let name = opts.name.unwrap_or(account.name);

        sqlx::query("UPDATE accounts SET name=$1,starting_balance=$2 WHERE id=$3")
            .bind(name)
            .bind(starting_balance)
            .bind(id)
            .execute(pool)
            .await?;

        info!(id=?id,"Updated account");
        Self::from_id(id, pool).await
    }

    /// Fetch the [`Account`] from the database.
    #[deprecated]
    pub async fn from_id(id: &str, pool: &sqlx::SqlitePool) -> Result<Self, crate::Error> {
        let record = sqlx::query!("SELECT * FROM accounts WHERE id = $1", id)
            .fetch_one(pool)
            .await?;

        let starting_balance = Money::from_scaled(record.starting_balance);
        let balance = Self::calculate_balance(id, pool).await? + starting_balance;
        let created_at = record
            .created_at
            .and_then(|t| DateTime::from_timestamp(t, 0));
        Ok(Self {
            id: record.id,
            name: record.name,
            starting_balance,
            balance,
            created_at,
        })
    }

    #[deprecated]
    pub async fn calculate_balance(id: &str, pool: &SqlitePool) -> Result<Money, crate::Error> {
        let total_expenses = sqlx::query!(
            "
                SELECT COALESCE(SUM(amount),0) as total FROM transactions WHERE from_account_id = $1
                ",
            id
        )
        .fetch_one(pool)
        .await?
        .total;

        let total_income = sqlx::query!(
            "
            SELECT COALESCE(SUM(amount),0) as total FROM transactions WHERE to_account_id = $1",
            id
        )
        .fetch_one(pool)
        .await?
        .total;

        let difference = total_income - total_expenses;
        Ok(Money::from_scaled(difference))
    }

    /// Delete an [`Account`].
    #[deprecated]
    pub async fn delete(id: &str, pool: &SqlitePool) -> Result<(), crate::Error> {
        sqlx::query!("DELETE FROM accounts WHERE id=$1", id)
            .execute(pool)
            .await?;

        info!(id=?id,"Deleted account");
        Ok(())
    }

    /// Fetch all the accounts from the database.
    #[deprecated]
    pub async fn fetch_all(pool: &SqlitePool) -> Result<Vec<Account>, crate::Error> {
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
}
