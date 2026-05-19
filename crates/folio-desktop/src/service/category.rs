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
use crate::{Money, SqliteConnection, db, service::Budget};
use chrono::{DateTime, Datelike, Local, Utc};
use rusqlite::params;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Row, SqlitePool};
use tracing::{debug, info, warn};

/// Service struct for managing categories and category groups.
#[derive(Clone)]
pub struct CategoryService {
    connection: SqliteConnection,
}

impl CategoryService {
    /// Creates a new category service.
    pub fn new(connection: SqliteConnection) -> Self {
        Self { connection }
    }

    /// Create a new category, a corresponding budget pointing to this category
    /// will be created as well. To only create a category use [`create_raw`].
    ///
    /// [`create_raw`]: CategoryService::create_category_raw
    pub fn create_category(&self, title: &str) -> crate::Result<Category> {
        let category = self.create_category_raw(title)?;
        info!(id=?category.id,"Created new category");
        self.create_budget(Money::ZERO, &category.id)?;

        Ok(category)
    }

    /// Creates a category without creating a budget.
    pub fn create_category_raw(&self, title: &str) -> crate::Result<Category> {
        let conn = self.connection.get();
        let mut stmt = conn
            .prepare_cached("INSERT INTO categories(title,created_at) VALUES(?1,?2) RETURNING *")?;
        let now = Utc::now().timestamp();
        let category = stmt.query_row(params![title, now], |row| Category::try_from(row))?;

        info!(id=?category.id,"Created new category");
        Ok(category)
    }

    /// Creates a new income stream.
    pub fn create_income_stream(&self, title: &str) -> crate::Result<Category> {
        let conn = self.connection.get();
        let mut stmt = conn
            .prepare_cached("INSERT INTO categories(title,created_at,is_income_stream) VALUES(?1,?2,true) RETURNING *")?;

        let now = Utc::now().timestamp();
        let category = stmt.query_row(params![title, now], |row| Category::try_from(row))?;

        info!(id=?category.id,"Created new income stream");
        Ok(category)
    }

    /// Fetch a [`Category`] from the database.
    pub fn fetch_category(&self, id: &str) -> crate::Result<Category> {
        let connection = self.connection.get();
        let mut stmt = connection.prepare_cached("SELECT * FROM categories WHERE id=?")?;
        let category = stmt.query_row([id], |row| Category::try_from(row))?;
        Ok(category)
    }

    pub fn edit_category(&self, id: &str, title: &str) -> crate::Result<Category> {
        let connection = self.connection.get();
        let mut stmt =
            connection.prepare_cached("UPDATE categories SET title=?1 WHERE id=?2 RETURNING *")?;
        let category = stmt.query_row([title, id], |row| Category::try_from(row))?;
        Ok(category)
    }

    /// Delete an account from the database.
    pub fn delete_category(&self, id: &str) -> crate::Result<()> {
        let conn = self.connection.get();
        let mut stmt = conn.prepare_cached("UPDATE categories SET deleted_at=?2 WHERE id=?1")?;
        let now = Utc::now().timestamp();
        stmt.execute(params![id, now])?;

        Ok(())
    }

    /// Get the total amount spent in the month for the [`Category`].
    pub fn total_spent(&self, id: &str) -> crate::Result<Money> {
        let connection = self.connection.get();
        let mut stmt = connection.prepare_cached("select * from transactions where category_id = ?")?;
        let transactions = stmt.query_map([id], |row|Transaction::try_from(row))?;

        let now = Local::now();
        let mut total = Money::ZERO;
        for transaction in transactions {
            let transaction = transaction?;
            let date = transaction.transaction_date;
            if date.year() != now.year() || date.month() != now.month() {
                continue;
            }
            total += transaction.amount;
        }
        Ok(total)
    }

    /// Fetches all the categories from the database
    pub fn fetch_categories(&self) -> Result<Vec<Category>, crate::Error> {
        let connection = self.connection.get();
        let mut stmt = connection.prepare_cached("select * from categories where deleted_at is null")?;

        let rows= stmt
            .query_map((), |row|Category::try_from(row))?;

        let mut categories = vec![];
        for row in rows {
            categories.push(row?);
        }
        Ok(categories)
    }

    pub fn fetch_categories_only(&self) -> Result<Vec<Category>, crate::Error> {
        let connection = self.connection.get();
        let mut stmt = connection
            .prepare_cached("select * from categories where deleted_at is null and is_income_stream is false")?;

        let rows = stmt
            .query_map((), |row|Category::try_from(row))?;
        
        let mut categories = vec![];
        for row in rows {
            categories.push(row?);
        }
        Ok(categories)
    }

    pub fn create_budget(&self, amount: Money, category_id: &str) -> crate::Result<Budget> {
        let connection = self.connection.get();
        let mut stmt = connection.prepare_cached("insert into budgets(amount,category_id) values(?1,?2) returning *")?;
        let row = stmt.query_row(params![amount.inner(),category_id], |row|Budget::try_from(row))?;
        info!(category = ?category_id, id = ?row.id,"Created new budget");
        Ok(row)
    }

    pub fn edit_budget(&self, id: &str, amount: Money) -> crate::Result<Budget> {
        let connection = self.connection.get();
        let mut stmt = connection
            .prepare_cached("update budgets set amount = ?1 where id = ?2 returning *")?;

        let budget = stmt.query_row(params![amount.inner(),id], |row|Budget::try_from(row))?;

        info!(id = id, "Updated budget");
        Ok(budget)
    }

    pub fn fetch_budget(&self, id: &str) -> crate::Result<Budget> {
        let connection = self.connection.get();
        let mut stmt = connection.prepare_cached("select * from budgets where id = ?")?;
        let budget = stmt.query_row([id], |row|Budget::try_from(row))?;
        Ok(budget)
    }

    /// Fetches the budget with the corresponding `category_id`.
    pub fn fetch_budget_from_category(&self, category_id: &str) -> crate::Result<Budget> {
        let connection = self.connection.get();
        let mut stmt = connection.prepare_cached("select * from budgets where category_id = ?")?;
        let budget = stmt.query_row([category_id], |row|Budget::try_from(row))?;
        Ok(budget)
    }

    pub fn create_missing_budgets(&self) -> crate::Result<()> {
        let categories = self.fetch_categories_only()?;
        let budgets = self.fetch_budgets()?;

        let mut filtered = vec![];
        for c in categories {
            let mut contains = false;
            for b in &budgets {
                if b.category_id == c.id {
                    contains = true;
                    break;
                }
            }
            if !contains {
                filtered.push(c);
            }
        }

        let len = filtered.len();
        if len != 0 {
            debug!("Found {len} categories without budgets")
        }

        for c in filtered {
            let result = self.create_budget(Money::ZERO, &c.id);
            if let Err(err) = result {
                warn!("Failed to create budget: {err}")
            }
        }

        Ok(())
    }

    pub fn fetch_budgets(&self) -> crate::Result<Vec<Budget>> {
        let categories = self.fetch_categories_only()?;
        
        let connection = self.connection.get();
        let mut stmt = connection.prepare_cached("select * from budgets")?;
        let rows = stmt.query_map((),|row|Budget::try_from(row))?;

        let mut budgets = vec![];
        for row in rows {
            let budget = row?;

            // Filter budgets with a deleted category
            if !categories.iter().any(|c| c.id == budget.category_id) {
                continue;
            }

            budgets.push(budget);
        }
        Ok(budgets)
    }
}

#[derive(
    Debug, Default, Serialize, Deserialize, Clone, PartialEq, PartialOrd, FromRow, Eq, Ord, Hash,
)]
#[serde(rename_all = "camelCase")]
pub struct Category {
    pub id: String,
    pub title: String,
    pub created_at: Option<DateTime<Utc>>,
    pub deleted_at: Option<DateTime<Utc>>,
    /// `true` if the category is used for incomes, false otherwise
    pub is_income_stream: bool,
}

// TODO: test is_sorted
// TODO: add default "No group" in UI for categories without a group
#[derive(FromRow, Debug, Serialize, Deserialize, PartialOrd, PartialEq, Clone)]
pub struct CategoryGroup {
    pub id: String,
    pub title: String,
    pub sort_order: i64,
    pub created_at: i64,
}

// TODO: ops
// - Add category
// - Remove category
// - Reorder
impl CategoryGroup {
    /// Fetch a category group from the database.
    pub async fn get(id: &str, pool: &SqlitePool) -> crate::Result<Self> {
        let group: CategoryGroup = sqlx::query_as("SELECT * FROM category_groups WHERE id = $1")
            .bind(id)
            .fetch_one(pool)
            .await?;
        Ok(group)
    }

    /// Create a new category group.
    pub async fn create(title: &str, pool: &SqlitePool) -> crate::Result<Self> {
        let row: Self = sqlx::query_as("INSERT INTO category_groups(title) VALUES($1) RETURNING *")
            .bind(title)
            .fetch_one(pool)
            .await?;

        Ok(row)
    }

    /// Update the title of the category group.
    pub async fn set_title(id: &str, title: &str, pool: &SqlitePool) -> crate::Result<Self> {
        let row: Self =
            sqlx::query_as("UPDATE category_groups SET title=$1 WHERE id=$2 RETURNING *")
                .bind(title)
                .bind(id)
                .fetch_one(pool)
                .await?;

        Ok(row)
    }

    pub async fn delete(id: &str, pool: &SqlitePool) -> crate::Result<()> {
        sqlx::query("DELETE FROM category_groups WHERE id=$1")
            .bind(id)
            .execute(pool)
            .await?;

        Ok(())
    }
}

impl<'a> TryFrom<&rusqlite::Row<'a>> for Category {
    type Error = rusqlite::Error;

    fn try_from(row: &rusqlite::Row) -> Result<Self, Self::Error> {
        let created_at = match row.get(2) {
            Ok(timestamp) => DateTime::from_timestamp(timestamp, 0),
            Err(_) => None,
        };

        let deleted_at = match row.get(3) {
            Ok(timestamp) => DateTime::from_timestamp(timestamp, 0),
            Err(_) => None,
        };

        let category = Self {
            id: row.get(0)?,
            title: row.get(1)?,
            created_at,
            deleted_at,
            is_income_stream: row.get(4)?,
        };

        Ok(category)
    }
}

impl<'a> TryFrom<&rusqlite::Row<'a>> for CategoryGroup {
    type Error = rusqlite::Error;

    fn try_from(row: &rusqlite::Row) -> Result<Self, Self::Error> {
        // let created_at = DateTime::from_timestamp(row.get(3)?, 0).unwrap_or_default();
        let group = Self {
            id: row.get(0)?,
            title: row.get(1)?,
            sort_order: row.get(2)?,
            created_at: row.get(3)?,
        };

        Ok(group)
    }
}
