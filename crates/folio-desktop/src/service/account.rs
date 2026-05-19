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

use crate::service::Transaction;
use crate::{Money, SqliteConnection};
use chrono::Weekday::Mon;
use chrono::{DateTime, NaiveDate, Utc};
use rusqlite::params;
use serde::{Deserialize, Serialize};
use sqlx::{Row, SqlitePool};
use tracing::info;

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
pub struct AccountService {
    connection: SqliteConnection,
}

impl AccountService {
    /// Creates a new [`AccountService`]
    pub fn new(connection: SqliteConnection) -> Self {
        Self { connection }
    }

    /// Creates a new account.
    pub fn create_account(&self, name: &str, starting_balance: Money) -> crate::Result<Account> {
        let connection = self.connection.get();
        let now = Utc::now().timestamp();
        let balance = starting_balance.inner();
        let mut stmt = connection.prepare_cached(
            "insert into accounts(name,starting_balance,created_at) values(?1,?2,?3) returning *",
        )?;
        let account = stmt.query_row(params![name, balance, now], |row| Account::try_from(row))?;

        info!(id=?account.id,"Created new account");
        Ok(account)
    }

    /// Fetch the [`Account`] from the database.
    ///
    /// # Error
    /// Returns an error if the account doesn't exist.
    pub fn fetch_account(&self, id: &str) -> crate::Result<Account> {
        let connection = self.connection.get();
        let mut stmt = connection.prepare_cached("select * from accounts where id=?")?;
        let account = stmt.query_row([id], |row| Account::try_from(row))?;
        Ok(account)
    }

    pub fn edit_account(&self, id: &str, opts: EditAccount) -> crate::Result<Account> {
        let connection = self.connection.get();
        let mut stmt = connection
            .prepare_cached("update accounts set name=coalesce(?1,name),starting_balance=coalesce(?2,starting_balance) where id=?3 returning *")?;
        let account = stmt.query_row(
            (opts.name, opts.starting_balance.map(|m| m.inner()), id),
            |row| Account::try_from(row),
        )?;

        info!(id=?account.id,"Updated account");
        Ok(account)
    }

    /// Deletes an [`Account`].
    pub fn delete_account(&self, id: &str) -> crate::Result<()> {
        let connection = self.connection.get();
        let mut stmt = connection.prepare_cached("delete from accounts where id=?")?;
        stmt.execute([id])?;

        info!(id=?id,"Deleted account");
        Ok(())
    }

    /// Calculates the balance for an account. The balance is the sum of all expenses minus
    /// the sum of all incomes.
    pub fn calculate_balance(&self, id: &str) -> Result<Money, crate::Error> {
        let connection = self.connection.get();
        let expense_sql =
            "select coalesce(sum(amount),0) as total from transactions where from_account_id=?";
        let income_sql =
            "select coalesce(sum(amount),0) as total from transactions where to_account_id=?";

        let total_expenses = connection.query_row(expense_sql, [id], |row| row.get::<_, i64>(0))?;

        let total_income = connection.query_row(income_sql, [id], |row| row.get::<_, i64>(0))?;

        let difference = total_income - total_expenses;
        Ok(Money::from_scaled(difference))
    }

    /// Fetch all the accounts from the database.
    pub fn fetch_all(&self) -> crate::Result<Vec<Account>> {
        let connection = self.connection.get();
        let mut stmt = connection.prepare_cached("select * from accounts")?;
        let rows = stmt.query_map((), |row| Account::try_from(row))?;

        let mut accounts = vec![];
        for record in rows {
            accounts.push(record?);
        }

        Ok(accounts)
    }
}

impl<'a> TryFrom<&rusqlite::Row<'a>> for Account {
    type Error = rusqlite::Error;

    fn try_from(row: &rusqlite::Row) -> Result<Self, Self::Error> {
        let created_at = row
            .get::<_, i64>("created_at")
            .ok()
            .map(|t| DateTime::from_timestamp(t, 0).unwrap_or_default());

        let account = Self {
            id: row.get("id")?,
            name: row.get("name")?,
            starting_balance: Money::from_scaled(row.get("starting_balance")?),
            created_at,
            balance: Money::default(),
        };
        Ok(account)
    }
}
