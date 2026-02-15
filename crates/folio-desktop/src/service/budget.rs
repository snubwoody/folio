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

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{Row, SqlitePool};
use tracing::info;

use crate::{Money, db, service::Category};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Budget {
    pub id: String,
    pub amount: Money,
    pub total_spent: Money,
    pub remaining: Money,
    pub category: Category,
    pub created_at: DateTime<Utc>,
}

impl Budget {
    /// Inserts a new row into the `budgets` table.
    pub(crate) async fn create(
        amount: Money,
        category_id: &str,
        pool: &SqlitePool,
    ) -> crate::Result<Self> {
        let amount = amount.inner();
        let record: db::Budget =
            sqlx::query_as("INSERT INTO budgets(amount,category_id) VALUES ($1,$2) RETURNING *")
                .bind(amount)
                .bind(category_id)
                .fetch_one(pool)
                .await?;

        let budget = Self::from_id(&record.id, pool).await?;

        info!(category_id=?category_id,id=?budget.id,"Created new budget");
        Ok(budget)
    }

    pub async fn edit(id: &str, amount: Money, pool: &SqlitePool) -> crate::Result<Self> {
        let amount = amount.inner();
        sqlx::query("UPDATE budgets SET amount = $1 WHERE id=$2")
            .bind(amount)
            .bind(id)
            .execute(pool)
            .await?;

        info!(id = id, "Updated budget");

        Self::from_id(id, pool).await
    }

    pub async fn from_id(id: &str, pool: &SqlitePool) -> crate::Result<Self> {
        let record: db::Budget = sqlx::query_as("SELECT * FROM budgets WHERE id=$1")
            .bind(id)
            .fetch_one(pool)
            .await?;

        let total = Money::new(record.amount);
        let category = Category::from_id(&record.category_id, pool).await?;
        let total_spent = Category::total_spent(&category.id, pool).await?;
        let remaining = (total - total_spent).max(Money::ZERO);
        let created_at = DateTime::from_timestamp(record.created_at, 0).unwrap_or_default();

        Ok(Self {
            id: record.id,
            amount: total,
            category,
            total_spent,
            remaining,
            created_at,
        })
    }

    /// Fetches the budget with the corresponding `category_id`.
    pub async fn from_category(category_id: &str, pool: &SqlitePool) -> crate::Result<Self> {
        let record: db::Budget = sqlx::query_as("SELECT * FROM budgets WHERE category_id=$1")
            .bind(category_id)
            .fetch_one(pool)
            .await?;

        let total = Money::new(record.amount);
        let category = Category::from_id(&record.category_id, pool).await?;
        let total_spent = Category::total_spent(&category.id, pool).await?;
        let remaining = total - total_spent;
        let created_at = DateTime::from_timestamp(record.created_at, 0).unwrap_or_default();

        Ok(Self {
            id: record.id,
            amount: total,
            category,
            total_spent,
            remaining,
            created_at,
        })
    }
}

pub async fn fetch_budgets(pool: &SqlitePool) -> crate::Result<Vec<Budget>> {
    let records = sqlx::query("SELECT id FROM budgets")
        .fetch_all(pool)
        .await?;

    let mut budgets = vec![];
    for record in records {
        let id: String = record.get("id");
        let budget = Budget::from_id(&id, pool).await?;
        budgets.push(budget);
    }
    Ok(budgets)
}

#[cfg(test)]
mod test {
    use super::*;

    #[sqlx::test]
    async fn fetch_budgets(pool: SqlitePool) -> crate::Result<()> {
        let len = super::fetch_budgets(&pool).await?.len();
        Category::create("", &pool).await?;
        let budgets = super::fetch_budgets(&pool).await?;
        assert!(budgets.len() == len + 1);
        Ok(())
    }

    #[sqlx::test]
    async fn edit_budget(pool: SqlitePool) -> crate::Result<()> {
        let category = Category::create("MINE__", &pool).await?;
        let budget = Budget::from_category(&category.id, &pool).await?;
        Budget::edit(&budget.id, Money::from_f64(244.00), &pool).await?;
        let b = Budget::from_category(&category.id, &pool).await?;

        assert_eq!(b.amount, Money::from_f64(244.00));
        Ok(())
    }

    #[sqlx::test]
    async fn get_budget(pool: SqlitePool) -> crate::Result<()> {
        let category = Category::create("__", &pool).await?;
        let budget = Budget::from_category(&category.id, &pool).await?;
        assert_eq!(budget.amount, Money::ZERO);
        Ok(())
    }

    #[sqlx::test]
    async fn remaining_caps_at_zero(pool: SqlitePool) -> crate::Result<()> {
        let category = Category::create("__", &pool).await?;
        let budget = Budget::from_category(&category.id, &pool).await?;
        assert_eq!(budget.amount, Money::ZERO);
        Ok(())
    }
}
