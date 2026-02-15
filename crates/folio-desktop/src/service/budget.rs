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
    pub async fn create(
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

        info!(budget=?budget,"Created budget");
        Ok(budget)
    }

    pub async fn edit(id: &str, amount: Money, pool: &SqlitePool) -> crate::Result<Self> {
        let amount = amount.inner();
        sqlx::query("UPDATE budgets SET amount = $1 WHERE id=$2")
            .bind(amount)
            .bind(id)
            .execute(pool)
            .await?;

        info!(id = id, amount = amount, "Updated budget");

        Self::from_id(id, pool).await
    }

    pub async fn delete(id: &str, pool: &SqlitePool) -> crate::Result<()> {
        sqlx::query("DELETE FROM budgets WHERE id=$1")
            .bind(id)
            .execute(pool)
            .await?;

        info!(id = id, "Deleted budget");
        Ok(())
    }

    pub async fn from_id(id: &str, pool: &SqlitePool) -> crate::Result<Self> {
        let record: db::Budget = sqlx::query_as("SELECT * FROM budgets WHERE id=$1")
            .bind(id)
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
    use crate::service::fetch_categories;

    #[sqlx::test]
    async fn create_budget(pool: SqlitePool) -> crate::Result<()> {
        let now = Utc::now().timestamp();
        let category = Category::create("MINE__", &pool).await?;
        let budget = Budget::create(Money::from_unscaled(20), category.id.as_str(), &pool).await?;

        assert_eq!(budget.amount, Money::from_unscaled(20));
        assert_eq!(budget.category.title, "MINE__");
        assert!(budget.created_at.timestamp() >= now);
        Ok(())
    }

    #[sqlx::test]
    async fn fetch_budgets(pool: SqlitePool) -> crate::Result<()> {
        let budgets = super::fetch_budgets(&pool).await?;
        assert!(budgets.len() == 1);
        Ok(())
    }

    #[sqlx::test]
    async fn edit_budget(pool: SqlitePool) -> crate::Result<()> {
        let category = Category::create("MINE__", &pool).await?;
        let budget = Budget::create(Money::from_unscaled(20), category.id.as_str(), &pool).await?;
        let budget = Budget::edit(&budget.id, Money::ZERO, &pool).await?;
        let b = Budget::from_id(&budget.id, &pool).await?;

        assert_eq!(b.amount, Money::ZERO);
        Ok(())
    }

    #[sqlx::test]
    async fn delete_budget(pool: SqlitePool) -> crate::Result<()> {
        let category = Category::create("MINE__", &pool).await?;
        let budget = Budget::create(Money::from_unscaled(20), category.id.as_str(), &pool).await?;
        Budget::delete(&budget.id, &pool).await?;
        let b = Budget::from_id(&budget.id, &pool).await;

        assert!(b.is_err());
        Ok(())
    }

    #[sqlx::test]
    async fn get_budget(pool: SqlitePool) -> crate::Result<()> {
        let category_id = &fetch_categories(&pool).await?[0].id;
        let amount = Money::from_f64(10.2).inner();
        let record = sqlx::query!(
            "INSERT INTO budgets(amount,category_id) VALUES ($1,$2) RETURNING id",
            amount,
            category_id
        )
        .fetch_one(&pool)
        .await?;

        let budget = Budget::from_id(&record.id, &pool).await?;
        assert_eq!(budget.amount, Money::from_f64(10.2));
        assert_eq!(&budget.category.id, category_id);
        Ok(())
    }
}
