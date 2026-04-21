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
use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};
use sqlx::{Row, SqlitePool};
use tracing::info;


#[derive(Debug, Serialize, Clone, PartialEq,PartialOrd,Default)]
#[serde(rename_all = "camelCase")]
pub struct Account {
    pub id: String,
    pub name: String,
    pub starting_balance: Money,
    // TODO: fetch balance on demand
    pub balance: Money,
    pub created_at: Option<DateTime<Utc>>,
}

// TODO: add fetch
impl Account {
    pub fn create(
        name: &str,
        starting_balance: Money,
        conn: &Connection,
    ) -> Result<Self, crate::Error> {
        let mut stmt = conn
            .prepare_cached("INSERT INTO accounts(name,starting_balance,created_at) VALUES(?1,?2,?3) RETURNING *")?;
        let now = Utc::now().timestamp();
        let balance = starting_balance.inner();
        let account = stmt.query_row(params![name,balance,now],|row|Self::from_row(row))?;
        info!(id=?account.id,"Created new account");
        Ok(account)
    }

    /// Parses an [`Account`] from a sqlite [`Row`].
    ///
    /// [`Row`]: rusqlite::Row
    fn from_row(row: &rusqlite::Row) -> rusqlite::Result<Self>{
        let created_at = DateTime::from_timestamp(row.get(3)?,0);
        let starting_balance = Money::from_scaled(row.get(2)?);
        // FIXME: calculate balance
        let account = Account{
            id: row.get(0)?,
            name: row.get(1)?,
            balance: Money::ZERO,
            starting_balance,
            created_at,
        };

        Ok(account)
    }

    /// Sets the account name.
    pub fn set_name(id:&str,name:&str,conn: &Connection) -> crate::Result<Self>{
        let mut stmt = conn
            .prepare_cached("UPDATE accounts SET name=?1 WHERE id=?2 RETURNING *")?;
        let account = stmt.query_row([name,id],Self::from_row)?;
        info!(id=?id,"Updated account name");
        Ok(account)
    }

    /// Sets the account starting balance.
    pub fn set_starting_balance(id:&str,balance:Money,conn: &Connection) -> crate::Result<Self>{
        let mut stmt = conn
            .prepare_cached("UPDATE accounts SET starting_balance=?1 WHERE id=?2 RETURNING *")?;
        let account = stmt.query_row(params![balance.inner(),id],Self::from_row)?;
        info!(id=?id,"Updated account starting balance");
        Ok(account)
    }

    // TODO: rename to fetch
    /// Fetch the [`Account`] from the database.
    pub async fn from_id(id: &str, pool: &sqlx::SqlitePool) -> Result<Self, crate::Error> {
        let record = sqlx::query!("SELECT * FROM accounts WHERE id = $1", id)
            .fetch_one(pool)
            .await?;

        let starting_balance = Money::from_scaled(record.starting_balance);
        // FIXME:
        // let balance = Self::calculate_balance(id, pool).await? + starting_balance;
        let balance = Money::ZERO;
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

    /// Calculate the current account balance.
    pub fn calculate_balance(id: &str, conn: &Connection) -> Result<Money, crate::Error> {
        let mut expense_stmt = conn
            .prepare_cached("SELECT COALESCE(SUM(amount),0) FROM transactions WHERE from_account_id = ?1")?;
        let mut income_stmt = conn
            .prepare_cached("SELECT COALESCE(SUM(amount),0) FROM transactions WHERE to_account_id = ?1")?;

        let total_expenses = expense_stmt.query_row([id],|row|{
            let amount: i64 = row.get(0)?;
            Ok(Money::from_scaled(amount))
        })?;
        let total_income = income_stmt.query_row([id],|row|{
            let amount: i64 = row.get(0)?;
            Ok(Money::from_scaled(amount))
        })?;
        let difference = total_income - total_expenses;
        Ok(difference)
    }


    /// Delete an [`Account`].
    pub fn delete(id: &str, conn: &Connection) -> Result<(), crate::Error> {
        conn.execute("DELETE FROM accounts WHERE id=$1",[id])?;
        info!(id=?id,"Deleted account");
        Ok(())
    }

    /// Fetch all the accounts from the database.
    pub async fn fetch_all(pool: &SqlitePool) -> Result<Vec<Account>, crate::Error> {
        // FIXME: don't fetch twice just remove balance field
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

#[cfg(test)]
mod test {
    use super::*;
    use crate::service::Transaction;
    use crate::setup_test_db;

    #[sqlx::test]
    async fn get_accounts(pool: SqlitePool) -> Result<(), crate::Error> {
        let conn = setup_test_db(pool.connect_options().get_filename()).await;
        Account::create("", Money::ZERO, &conn)?;
        Account::create("", Money::ZERO, &conn)?;
        Account::create("", Money::ZERO, &conn)?;

        let accounts = Account::fetch_all(&pool).await?;
        assert_eq!(accounts.len(), 3);
        Ok(())
    }

    #[sqlx::test]
    async fn calculate_account_balance(pool: SqlitePool) -> Result<(), crate::Error> {
        let conn = setup_test_db(pool.connect_options().get_filename()).await;
        let account = Account::create("", Money::ZERO, &conn)?;
        Transaction::expense()
            .account_id(&account.id)
            .amount(Money::from_unscaled(20))
            .create(&pool)
            .await?;
        Transaction::expense()
            .account_id(&account.id)
            .amount(Money::from_unscaled(20))
            .create(&pool)
            .await?;
        Transaction::income()
            .account_id(&account.id)
            .amount(Money::from_unscaled(50))
            .create(&pool)
            .await?;
        let balance = Account::calculate_balance(&account.id, &conn)?;
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
        let conn = setup_test_db(pool.connect_options().get_filename()).await;
        let now = Utc::now().timestamp();
        Account::create("My account", Money::from_unscaled(20), &conn)?;
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
        let conn = setup_test_db(pool.connect_options().get_filename()).await;
        Account::create("My account", Money::ZERO, &conn)?;
        Account::create("My account", Money::ZERO, &conn)?;
        let account = Account::create("My account", Money::ZERO, &conn)?;
        let records = sqlx::query!("SELECT * FROM accounts")
            .fetch_all(&pool)
            .await?;
        assert_eq!(records.len(), 3);

        Account::delete(&account.id,&conn)?;
        let records = sqlx::query!("SELECT * FROM accounts")
            .fetch_all(&pool)
            .await?;
        assert_eq!(records.len(), 2);
        Ok(())
    }

    #[sqlx::test]
    async fn delete_account_with_expense(pool: sqlx::SqlitePool) -> Result<(), crate::Error> {
        let conn = setup_test_db(pool.connect_options().get_filename()).await;
        let account = Account::create("My account", Money::ZERO, &conn)?;
        Transaction::expense()
            .account_id(&account.id)
            .create(&pool)
            .await?;
        let records = sqlx::query!("SELECT * FROM accounts")
            .fetch_all(&pool)
            .await?;
        assert_eq!(records.len(), 1);

        Account::delete(&account.id, &conn)?;
        let records = sqlx::query!("SELECT * FROM accounts")
            .fetch_all(&pool)
            .await?;
        assert_eq!(records.len(), 0);
        Ok(())
    }

    #[sqlx::test]
    async fn delete_account_with_income(pool: sqlx::SqlitePool) -> Result<(), crate::Error> {
        let conn = setup_test_db(pool.connect_options().get_filename()).await;
        let account = Account::create("My account", Money::ZERO, &conn)?;
        Transaction::income()
            .account_id(&account.id)
            .create(&pool)
            .await?;
        let records = sqlx::query!("SELECT * FROM accounts")
            .fetch_all(&pool)
            .await?;
        assert_eq!(records.len(), 1);

        Account::delete(&account.id, &conn)?;
        let records = sqlx::query!("SELECT * FROM accounts")
            .fetch_all(&pool)
            .await?;
        assert_eq!(records.len(), 0);
        Ok(())
    }
}
