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
use sqlx::SqlitePool;
use tracing::info;

use crate::{Money, service::Category};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Budget {
    id: String,
    amount: Money,
    total_spent: Money,
    remaining: Money,
    category: Category,
    created_at: Option<DateTime<Utc>>,
}

impl Budget {
    pub async fn create(
        amount: Money,
        category_id: &str,
        pool: &SqlitePool,
    ) -> crate::Result<Self> {
        let amount = amount.inner();
        let now = Utc::now().timestamp();
        let record = sqlx::query!(
            "INSERT INTO budgets(amount,category_id,created_at) VALUES ($1,$2,$3) RETURNING id",
            amount,
            category_id,
            now
        )
        .fetch_one(pool)
        .await?;

        let budget = Self::from_id(&record.id, pool).await?;

        info!(budget=?budget,"Created budget");
        Ok(budget)
    }

    pub async fn from_id(id: &str, pool: &SqlitePool) -> crate::Result<Self> {
        let record = sqlx::query!("SELECT * FROM budgets WHERE id=$1", id)
            .fetch_one(pool)
            .await?;

        let total = Money::new(record.amount);
        let category = Category::from_id(&record.category_id, pool).await?;
        let total_spent = Category::total_spent(&category.id, pool).await?;
        let remaining = total - total_spent;
        let created_at = record
            .created_at
            .and_then(|t| DateTime::from_timestamp(t, 0));
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
    let records = sqlx::query!("SELECT id FROM budgets")
        .fetch_all(pool)
        .await?;

    let mut budgets = vec![];
    for record in records {
        let budget = Budget::from_id(&record.id, pool).await?;
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
        assert!(budget.created_at.unwrap().timestamp() >= now);
        Ok(())
    }

    #[sqlx::test]
    async fn get_category(pool: SqlitePool) -> crate::Result<()> {
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
