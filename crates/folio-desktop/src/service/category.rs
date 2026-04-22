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
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, SqlitePool};

use crate::{Money, db, service::Budget};

/// Service struct for managing categories and category groups.
#[derive(Clone)]
struct CategoryService {
    pool: SqlitePool
}

impl CategoryService{
    /// Creates a new category service.
    pub fn new(pool: SqlitePool) -> Self{
        Self{pool}
    }
}

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
    pub async fn create(title: &str, pool: &SqlitePool) -> crate::Result<Self> {
        let now = Utc::now().timestamp();
        let record: db::Category =
            sqlx::query_as("INSERT INTO categories(title,created_at) VALUES($1,$2) RETURNING *")
                .bind(title)
                .bind(now)
                .fetch_one(pool)
                .await?;

        tracing::info!(id=?record.id,"Created new category");

        Budget::create(Money::ZERO, &record.id, pool).await?;

        Category::from_id(&record.id, pool).await
    }

    pub async fn create_income_stream(title: &str, pool: &SqlitePool) -> crate::Result<Self> {
        let now = Utc::now().timestamp();
        let record: db::Category =
            sqlx::query_as("INSERT INTO categories(title,created_at,is_income_stream) VALUES($1,$2,true) RETURNING *")
                .bind(title)
                .bind(now)
                .fetch_one(pool)
                .await?;

        tracing::info!(id=?record.id,"Created new income stream");

        Category::from_id(&record.id, pool).await
    }

    /// Creates a category without creating a budget.
    pub async fn create_raw(title: &str, pool: &SqlitePool) -> crate::Result<Self> {
        let now = Utc::now().timestamp();
        let record: db::Category =
            sqlx::query_as("INSERT INTO categories(title,created_at) VALUES($1,$2) RETURNING *")
                .bind(title)
                .bind(now)
                .fetch_one(pool)
                .await?;

        tracing::info!(id=?record.id,"Created new category");

        Category::from_id(&record.id, pool).await
    }

    pub async fn from_id(id: &str, pool: &SqlitePool) -> crate::Result<Self> {
        let record: db::Category = sqlx::query_as("SELECT * FROM categories WHERE id=$1")
            .bind(id)
            .fetch_one(pool)
            .await?;

        let created_at = record
            .created_at
            .and_then(|t| DateTime::from_timestamp(t, 0));
        let deleted_at = record
            .deleted_at
            .and_then(|t| DateTime::from_timestamp(t, 0));

        let category = Category {
            id: record.id,
            title: record.title,
            created_at,
            deleted_at,
            is_income_stream: record.is_income_stream,
        };

        Ok(category)
    }

    pub async fn edit(id: &str, title: &str, pool: &SqlitePool) -> crate::Result<Self> {
        sqlx::query("UPDATE categories SET title=$1 WHERE id=$2")
            .bind(title)
            .bind(id)
            .execute(pool)
            .await?;

        Self::from_id(id, pool).await
    }

    pub async fn delete(id: &str, pool: &SqlitePool) -> crate::Result<()> {
        let now = Utc::now().timestamp();
        sqlx::query("UPDATE categories SET deleted_at=$2 WHERE id=$1")
            .bind(id)
            .bind(now)
            .execute(pool)
            .await?;

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
    pub async fn fetch_all(pool: &SqlitePool) -> Result<Vec<Self>, crate::Error> {
        let records: Vec<db::Category> =
            sqlx::query_as("SELECT * FROM categories WHERE deleted_at IS NULL")
                .fetch_all(pool)
                .await?;

        let mut categories = vec![];
        for record in records {
            let category = Self::from_id(&record.id, pool).await?;
            categories.push(category);
        }
        Ok(categories)
    }

    pub async fn fetch_categories(pool: &SqlitePool) -> Result<Vec<Self>, crate::Error> {
        let records: Vec<db::Category> = sqlx::query_as(
            "SELECT * FROM categories WHERE deleted_at IS NULL AND is_income_stream IS false",
        )
        .fetch_all(pool)
        .await?;

        let mut categories = vec![];
        for record in records {
            let category = Self::from_id(&record.id, pool).await?;
            categories.push(category);
        }
        Ok(categories)
    }
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

#[cfg(test)]
mod test {
    use super::*;
    use crate::service::{AccountService, Transaction};

    #[sqlx::test]
    async fn total_spent(pool: SqlitePool) -> crate::Result<()> {
        let account_service = AccountService::new(pool.clone());
        let category = Category::create("", &pool).await?;
        let account = account_service.create_account("", Money::ZERO).await?;
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
        let rows = sqlx::query!("SELECT id FROM categories")
            .fetch_all(&pool)
            .await?;
        Category::create("", &pool).await?;
        Category::create("", &pool).await?;
        Category::create("", &pool).await?;
        let categories = Category::fetch_all(&pool).await?;
        assert_eq!(categories.len(), rows.len() + 3);
        Ok(())
    }

    #[sqlx::test]
    async fn fetch_category(pool: SqlitePool) -> crate::Result<()> {
        let record = sqlx::query!("INSERT INTO categories(title) VALUES('Rent') RETURNING id")
            .fetch_one(&pool)
            .await?;

        let category = Category::from_id(&record.id, &pool).await?;
        assert_eq!(category.title, "Rent");
        Ok(())
    }

    #[sqlx::test]
    async fn create_category(pool: SqlitePool) -> crate::Result<()> {
        let now = Utc::now().timestamp();
        let category = Category::create("Ent", &pool).await?;
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
        let now = Utc::now().timestamp();
        let category = Category::create_income_stream("Ent", &pool).await?;
        let record = sqlx::query!("SELECT * FROM categories WHERE id=$1", category.id)
            .fetch_one(&pool)
            .await?;

        assert!(record.created_at.unwrap() >= now);
        assert_eq!(record.title, "Ent");
        assert!(record.is_income_stream.unwrap());
        Ok(())
    }
}
