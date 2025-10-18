use std::str::FromStr;

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use tracing::info;

use crate::{service::Category, Money};

#[derive(Debug, Clone, Serialize,Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Budget {
    id: String,
    amount: Money,
    total_spent: Money,
    remaining: Money,
    category: Category,
}

impl Budget {
    pub async fn create(
        amount: Money,
        category_id: &str,
        pool: &SqlitePool,
    ) -> crate::Result<Self> {
        let amount = amount.inner();
        let record = sqlx::query!(
            "INSERT INTO budgets(amount,category_id) VALUES ($1,$2) RETURNING id",
            amount,
            category_id
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
        Ok(Self {
            id: record.id,
            amount: total,
            category,
            total_spent,
            remaining,
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
    use rust_decimal::dec;

    use super::*;
    use crate::service::fetch_categories;

    #[sqlx::test]
    async fn create_budget(pool: SqlitePool) -> crate::Result<()> {
        let category = Category::create("MINE__", &pool).await?;
        let budget = Budget::create(Money::from_unscaled(20), category.id.as_str(), &pool).await?;

        assert_eq!(budget.amount, Money::from_unscaled(20));
        assert_eq!(budget.category.title, "MINE__");
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
