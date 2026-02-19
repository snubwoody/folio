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

#[derive(Debug, Serialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Account {
    pub id: String,
    pub name: String,
    pub starting_balance: Money,
    pub balance: Money,
    pub created_at: Option<DateTime<Utc>>,
}

// TODO: add fetch
impl Account {
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

        let id:String = record.get("id");
        Self::from_id(&id, pool).await
    }

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

        sqlx::query(
            "UPDATE accounts SET name=$1,starting_balance=$2 WHERE id=$3",
        )
        .bind(name)
        .bind(starting_balance)
        .bind(id)
        .execute(pool)
        .await?;

        Self::from_id(id, pool).await
    }

    /// Fetch the [`Account`] from the database.
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

    pub async fn calculate_balance(id: &str, pool: &SqlitePool) -> Result<Money, crate::Error> {
        let total_expenses = sqlx::query!(
            "
                SELECT COALESCE(SUM(amount),0) as total FROM expenses WHERE account_id = $1
                ",
            id
        )
        .fetch_one(pool)
        .await?
        .total;

        let total_income = sqlx::query!(
            "
            SELECT COALESCE(SUM(amount),0) as total FROM incomes WHERE account_id = $1",
            id
        )
        .fetch_one(pool)
        .await?
        .total;

        let difference = total_income - total_expenses;
        Ok(Money::from_scaled(difference))
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
    use crate::service::{CreateExpense, CreateIncome, Expense, Income};

    #[sqlx::test]
    async fn get_accounts(pool: SqlitePool) -> Result<(), crate::Error> {
        Account::create("", Money::ZERO, &pool).await?;
        Account::create("", Money::ZERO, &pool).await?;
        Account::create("", Money::ZERO, &pool).await?;

        let accounts = fetch_accounts(&pool).await?;
        assert_eq!(accounts.len(), 3);
        Ok(())
    }

    #[sqlx::test]
    async fn calculate_account_balance(pool: SqlitePool) -> Result<(), crate::Error> {
        let account = Account::create("", Money::ZERO, &pool).await?;
        let data = CreateExpense {
            amount: Money::from_unscaled(20),
            account_id: Some(account.id.clone()),
            ..Default::default()
        };
        let income_data = CreateIncome {
            amount: Money::from_unscaled(50),
            account_id: Some(account.id.clone()),
            ..Default::default()
        };
        Expense::create(data.clone(), &pool).await?;
        Expense::create(data, &pool).await?;
        Income::create(income_data, &pool).await?;
        let balance = Account::calculate_balance(&account.id, &pool).await?;
        assert_eq!(balance, Money::from_unscaled(10));
        Ok(())
    }

    #[sqlx::test]
    async fn fetch_account(pool: sqlx::SqlitePool) -> Result<(), crate::Error> {
        let amount = Money::from_f64(20.0);
        let amount = amount.inner();
        let record = sqlx::query!(
            "INSERT INTO accounts(name,starting_balance) VALUES('C3PO',$1) RETURNING id",
            amount
        )
        .fetch_one(&pool)
        .await?;

        let account = Account::from_id(&record.id, &pool).await?;
        assert_eq!(account.starting_balance.inner(), 20_000_000);
        assert_eq!(account.name, "C3PO");
        Ok(())
    }

    #[sqlx::test]
    async fn create_account(pool: sqlx::SqlitePool) -> Result<(), crate::Error> {
        let now = Utc::now().timestamp();
        Account::create("My account", Money::from_unscaled(20), &pool).await?;
        let account = sqlx::query!("SELECT * FROM accounts")
            .fetch_one(&pool)
            .await?;
        assert!(account.created_at.unwrap() >= now);
        assert_eq!(account.name, "My account");
        assert_eq!(account.starting_balance, Money::from_unscaled(20).inner());
        Ok(())
    }

    #[sqlx::test]
    async fn delete_account(pool: sqlx::SqlitePool) -> Result<(), crate::Error> {
        Account::create("My account", Money::ZERO, &pool).await?;
        Account::create("My account", Money::ZERO, &pool).await?;
        let account = Account::create("My account", Money::ZERO, &pool).await?;
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

    #[sqlx::test]
    async fn delete_account_with_expense(pool: sqlx::SqlitePool) -> Result<(), crate::Error> {
        let account = Account::create("My account", Money::ZERO, &pool).await?;
        let data = CreateExpense {
            account_id: Some(account.id.clone()),
            ..Default::default()
        };
        Expense::create(data, &pool).await?;
        let records = sqlx::query!("SELECT * FROM accounts")
            .fetch_all(&pool)
            .await?;
        assert_eq!(records.len(), 1);

        Account::delete(&account.id, &pool).await?;
        let records = sqlx::query!("SELECT * FROM accounts")
            .fetch_all(&pool)
            .await?;
        assert_eq!(records.len(), 0);
        Ok(())
    }

    #[sqlx::test]
    async fn delete_account_with_income(pool: sqlx::SqlitePool) -> Result<(), crate::Error> {
        let account = Account::create("My account", Money::ZERO, &pool).await?;
        let data = CreateIncome {
            account_id: Some(account.id.clone()),
            ..Default::default()
        };
        Income::create(data, &pool).await?;
        let records = sqlx::query!("SELECT * FROM accounts")
            .fetch_all(&pool)
            .await?;
        assert_eq!(records.len(), 1);

        Account::delete(&account.id, &pool).await?;
        let records = sqlx::query!("SELECT * FROM accounts")
            .fetch_all(&pool)
            .await?;
        assert_eq!(records.len(), 0);
        Ok(())
    }
}
