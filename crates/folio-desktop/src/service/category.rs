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
pub struct CategoryService {
    pool: SqlitePool,
}

impl CategoryService {
    /// Creates a new category service.
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    /// Create a new category, a corresponding budget pointing to this category
    /// will be created as well. To only create a category use [`create_raw`].
    ///
    /// [`create_raw`]: CategoryService::create_category_raw
    pub async fn create_category(&self, title: &str) -> crate::Result<Category> {
        let category = self.create_category_raw(title).await?;
        tracing::info!(id=?category.id,"Created new category");
        Budget::create(Money::ZERO, &category.id, &self.pool).await?;

        Ok(category)
    }

    /// Creates a category without creating a budget.
    pub async fn create_category_raw(&self, title: &str) -> crate::Result<Category> {
        let now = Utc::now().timestamp();
        let record: db::Category =
            sqlx::query_as("INSERT INTO categories(title,created_at) VALUES($1,$2) RETURNING *")
                .bind(title)
                .bind(now)
                .fetch_one(&self.pool)
                .await?;

        self.fetch_category(&record.id).await
    }

    /// Creates a new income stream.
    pub async fn create_income_stream(&self, title: &str) -> crate::Result<Category> {
        let now = Utc::now().timestamp();
        let record: db::Category =
            sqlx::query_as("INSERT INTO categories(title,created_at,is_income_stream) VALUES($1,$2,true) RETURNING *")
                .bind(title)
                .bind(now)
                .fetch_one(&self.pool)
                .await?;

        tracing::info!(id=?record.id,"Created new income stream");
        self.fetch_category(&record.id).await
    }

    /// Fetch a [`Category`] from the database.
    pub async fn fetch_category(&self, id: &str) -> crate::Result<Category> {
        let record: db::Category = sqlx::query_as("SELECT * FROM categories WHERE id=$1")
            .bind(id)
            .fetch_one(&self.pool)
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

    pub async fn edit_category(&self, id: &str, title: &str) -> crate::Result<Category> {
        sqlx::query("UPDATE categories SET title=$1 WHERE id=$2")
            .bind(title)
            .bind(id)
            .execute(&self.pool)
            .await?;

        self.fetch_category(id).await
    }

    /// Delete an account from the database.
    pub async fn delete_category(&self, id: &str) -> crate::Result<()> {
        let now = Utc::now().timestamp();
        sqlx::query("UPDATE categories SET deleted_at=$2 WHERE id=$1")
            .bind(id)
            .bind(now)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    /// Get the total amount spent in the month for the [`Category`].
    pub async fn total_spent(&self, id: &str) -> crate::Result<Money> {
        let now = Local::now();
        let mut total = Money::ZERO;
        let transactions: Vec<Transaction> =
            sqlx::query_as("SELECT * FROM transactions WHERE category_id = $1")
                .bind(id)
                .fetch_all(&self.pool)
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
    pub async fn fetch_categories(&self) -> Result<Vec<Category>, crate::Error> {
        let records: Vec<db::Category> =
            sqlx::query_as("SELECT * FROM categories WHERE deleted_at IS NULL")
                .fetch_all(&self.pool)
                .await?;

        let mut categories = vec![];
        for record in records {
            let category = self.fetch_category(&record.id).await?;
            categories.push(category);
        }
        Ok(categories)
    }

    pub async fn fetch_categories_only(&self) -> Result<Vec<Category>, crate::Error> {
        let records: Vec<db::Category> = sqlx::query_as(
            "SELECT * FROM categories WHERE deleted_at IS NULL AND is_income_stream IS false",
        )
        .fetch_all(&self.pool)
        .await?;

        let mut categories = vec![];
        for record in records {
            let category = self.fetch_category(&record.id).await?;
            categories.push(category);
        }
        Ok(categories)
    }
}

#[derive(
    Debug, Default, Serialize, Deserialize, Clone, PartialEq, PartialOrd, Eq, Ord, FromRow, Hash,
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
