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
use chrono::{DateTime, Datelike, Local, Utc};
use rusqlite::{params, Connection, Row};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, SqlitePool};

use crate::{Money, db, service::Budget};

#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[serde(rename_all = "camelCase")]
pub struct Category {
    pub id: String,
    pub title: String,
    pub created_at: Option<DateTime<Utc>>,
    pub deleted_at: Option<DateTime<Utc>>,
    /// `true` if the category is used for incomes, false otherwise
    pub is_income_stream: bool,
}

// TODO: add change_group
impl Category {
    /// Create a new category, a corresponding budget pointing to this category
    /// will be created as well. To only create a category use [`create_raw`].
    ///
    /// [`create_raw`]: Category::create_raw
    pub fn create(title: &str, conn: &Connection) -> crate::Result<Self> {
        let category = Category::create_raw(title,conn)?;
        Budget::create(Money::ZERO, &category.id, conn)?;
        Ok(category)
    }

    pub fn create_income_stream(title: &str, conn: &Connection) -> crate::Result<Self> {
        let mut stmt = conn
            .prepare_cached("INSERT INTO categories(title,created_at,is_income_stream) VALUES(?1,?2,true) RETURNING *")?;

        let now = Utc::now().timestamp();
        let category = stmt.query_row(params![title,now],|row|Self::try_from(row))?;

        tracing::info!(id=?category.id,"Created new income stream");

        Ok(category)
    }

    /// Creates a category without creating a budget.
    pub fn create_raw(title: &str, conn: &Connection) -> crate::Result<Self> {
        let mut stmt = conn
            .prepare_cached("INSERT INTO categories(title,created_at) VALUES(?1,?2) RETURNING *")?;
        let now = Utc::now().timestamp();
        let category = stmt.query_row(params![title,now],|row|Self::try_from(row))?;

        tracing::info!(id=?category.id,"Created new category");

        Ok(category)
    }

    /// Fetch the category from the database.
    pub fn fetch(id: &str, conn: &Connection) -> crate::Result<Self> {
        let mut stmt = conn.prepare_cached("SELECT * FROM categories WHERE id=?")?;
        let category = stmt.query_row([id],|row|Self::try_from(row))?;
        Ok(category)
    }

    /// Edit a category.
    pub fn edit(id: &str, title: &str, conn: &Connection) -> crate::Result<Self> {
        let mut stmt = conn
            .prepare_cached("UPDATE categories SET title=?1 WHERE id=?2 RETURNING *")?;
        let category = stmt.query_row([title,id],|row|Self::try_from(row))?;
        Ok(category)
    }

    /// Delete a category.
    pub fn delete(id: &str, conn: &Connection) -> crate::Result<()> {
        let mut stmt = conn.prepare_cached("UPDATE categories SET deleted_at=?2 WHERE id=?1")?;
        let now = Utc::now().timestamp();
        stmt.execute(params![id,now])?;

        Ok(())
    }

    /// Get the total amount spent in the month for the [`Category`].
    pub async fn total_spent(id: &str, pool: &SqlitePool) -> crate::Result<Money> {
        let now = Local::now();
        let mut total = Money::ZERO;

        let transactions: Vec<Transaction> =
            sqlx::query_as("SELECT * FROM transactions WHERE category_id = $1")
                .bind(id)
                .fetch_all(pool)
                .await?;

        for transaction in transactions {
            let date = transaction.transaction_date;
            if date.year() != now.year() || date.month() != now.month() {
                continue;
            }
            total += transaction.amount;
        }
        Ok(total)
    }

    /// Fetches all the categories from the database
    pub fn fetch_all(conn: &Connection) -> Result<Vec<Self>, crate::Error> {
        let mut stmt = conn
            .prepare_cached("SELECT * FROM categories WHERE deleted_at IS NULL")?;
        let rows = stmt.query_map([],|row|Self::try_from(row))?;
        let mut categories = vec![];
        for row in rows{
            categories.push(row?)
        }
        Ok(categories)
    }
    
}

// TODO: test is_sorted
// TODO: add default "No group" in UI for categories without a group
#[derive(FromRow, Debug, Serialize, Deserialize, PartialOrd, PartialEq, Clone,Default)]
#[serde(rename_all = "camelCase")]
pub struct CategoryGroup {
    pub id: String,
    pub title: String,
    pub sort_order: i64,
    pub created_at: DateTime<Utc>
}

// TODO: ops
// - Add category
// - Remove category
// - Reorder
impl CategoryGroup {
    /// Fetch a category group from the database.
    pub fn get(id: &str, conn: &Connection) -> crate::Result<Self> {
        let mut stmt = conn
            .prepare_cached("SELECT * FROM category_groups WHERE id=?")?;
        let group = stmt.query_row([id],|row|Self::try_from(row))?;
        Ok(group)
    }

    /// Create a new category group.
    pub fn create(title: &str, conn: &Connection) -> crate::Result<Self> {
        let mut stmt = conn
            .prepare_cached("INSERT INTO category_groups(title) VALUES (?) RETURNING *")?;
        let group = stmt.query_row([title],|row|Self::try_from(row))?;
        Ok(group)
    }

    /// Update the title of the category group.
    pub fn set_title(id: &str, title: &str, conn: &Connection) -> crate::Result<Self> {
        let mut stmt = conn
            .prepare_cached("UPDATE category_groups SET title=?1 WHERE id=?2 RETURNING *")?;

        let category_group = stmt.query_row([title,id],|row|Self::try_from(row))?;
        Ok(category_group)
    }

    pub fn delete(id: &str, conn: &Connection) -> crate::Result<()> {
        // TODO: on delete cascade set null
        let mut stmt = conn.prepare_cached("DELETE FROM category_groups WHERE id=$1")?;
        stmt.execute([id])?;

        Ok(())
    }
}

impl<'a> TryFrom<&Row<'a>> for Category{
    type Error = rusqlite::Error;

    fn try_from(row: &Row) -> Result<Self, Self::Error> {
        let created_at = match row.get(2) {
            Ok(timestamp) => DateTime::from_timestamp(timestamp,0),
            Err(_) => None
        };

        let deleted_at = match row.get(3) {
            Ok(timestamp) => DateTime::from_timestamp(timestamp,0),
            Err(_) => None
        };

        let category = Self{
            id: row.get(0)?,
            title: row.get(1)?,
            created_at,
            deleted_at,
            is_income_stream: row.get(4)?
        };

        Ok(category)
    }
}

impl<'a> TryFrom<&Row<'a>> for CategoryGroup{
    type Error = rusqlite::Error;

    fn try_from(row: &Row) -> Result<Self, Self::Error> {
        let created_at = DateTime::from_timestamp(row.get(3)?, 0)
            .unwrap_or_default();
        let group = Self{
            id: row.get(0)?,
            title: row.get(1)?,
            sort_order: row.get(2)?,
            created_at
        };

        Ok(group)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::service::{Account, Transaction};
    use crate::setup_test_db;

    #[sqlx::test]
    async fn total_spent(pool: SqlitePool) -> crate::Result<()> {
        let conn = setup_test_db(pool.connect_options().get_filename()).await;
        let category = Category::create("", &conn)?;
        let account = Account::create("", Money::ZERO, &conn)?;
        Transaction::expense()
            .account_id(&account.id)
            .amount(Money::from_unscaled(100))
            .category(&category.id)
            .create(&pool)
            .await?;

        Transaction::expense()
            .account_id(&account.id)
            .amount(Money::from_unscaled(20))
            .category(&category.id)
            .create(&pool)
            .await?;

        let total = Category::total_spent(&category.id, &pool).await?;
        assert_eq!(total, Money::from_unscaled(120));
        Ok(())
    }

    #[sqlx::test]
    async fn get_categories(pool: SqlitePool) -> Result<(), crate::Error> {
        let conn = setup_test_db(pool.connect_options().get_filename()).await;
        let rows = sqlx::query!("SELECT id FROM categories")
            .fetch_all(&pool)
            .await?;
        Category::create("", &conn)?;
        Category::create("", &conn)?;
        Category::create("", &conn)?;
        let categories = Category::fetch_all(&conn)?;
        assert_eq!(categories.len(), rows.len() + 3);
        Ok(())
    }

    #[sqlx::test]
    async fn fetch_category(pool: SqlitePool) -> crate::Result<()> {
        let conn = setup_test_db(pool.connect_options().get_filename()).await;
        let record = sqlx::query!("INSERT INTO categories(title) VALUES('Rent') RETURNING id")
            .fetch_one(&pool)
            .await?;

        let category = Category::fetch(&record.id, &conn)?;
        assert_eq!(category.title, "Rent");
        Ok(())
    }

    #[sqlx::test]
    async fn create_category(pool: SqlitePool) -> crate::Result<()> {
        let conn = setup_test_db(pool.connect_options().get_filename()).await;
        let now = Utc::now().timestamp();
        let category = Category::create("Ent", &conn)?;
        let record = sqlx::query!("SELECT * FROM categories WHERE id=$1", category.id)
            .fetch_one(&pool)
            .await?;

        assert!(record.created_at.unwrap() >= now);
        assert_eq!(record.title, "Ent");
        assert!(!record.is_income_stream.unwrap());
        Ok(())
    }

    #[sqlx::test]
    async fn create_income_stream(pool: SqlitePool) -> crate::Result<()> {
        let conn = setup_test_db(pool.connect_options().get_filename()).await;
        let now = Utc::now().timestamp();
        let category = Category::create_income_stream("Ent", &conn)?;
        let record = sqlx::query!("SELECT * FROM categories WHERE id=$1", category.id)
            .fetch_one(&pool)
            .await?;

        assert!(record.created_at.unwrap() >= now);
        assert_eq!(record.title, "Ent");
        assert!(record.is_income_stream.unwrap());
        Ok(())
    }
}
