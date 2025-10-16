
// TODO:

// struct BudgetOverview{
//     amount_spend: f32,
//     amount_left: f32,
//     total_income: f32,
//     total_expense: f32,
//     percentage_of_income: f32
// }

use std::str::FromStr;

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::{SqlitePool};
use tracing::info;

use crate::service::Category;

#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct Budget{
    id: String,
    amount: Decimal,
    category: Category
}

impl Budget{
    pub async fn create(amount: Decimal,category_id: &str,pool: &SqlitePool) -> crate::Result<Self>{
        let amount = amount.to_string();
        let record = sqlx::query!(
            "INSERT INTO budgets(amount,category_id) VALUES ($1,$2) RETURNING id",
            amount,
            category_id
        ).fetch_one(pool).await?;

        let budget = Self::from_id(&record.id, pool).await?;

        info!(budget=?budget,"Created budget");
        Ok(budget)
    }

    pub async fn from_id(id: &str, pool: &SqlitePool) -> crate::Result<Self>{
        let record = sqlx::query!("SELECT * FROM budgets WHERE id=$1",id)
            .fetch_one(pool)
            .await?;

        let category = Category::from_id(&record.category_id, pool).await?;

        Ok(Self { id: record.id, amount: Decimal::from_str(&record.amount)?, category })
    }
}

pub async fn fetch_budgets(pool: &SqlitePool) -> crate::Result<Vec<Budget>>{
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
mod test{
    use rust_decimal::dec;

    use crate::service::fetch_categories;
    use super::*;

    #[sqlx::test]
    async fn create_budget(pool: SqlitePool) -> crate::Result<()>{
        let category = Category::create("MINE__", &pool).await?;
        let budget = Budget::create(dec!(20.24), category.id.as_str(), &pool).await?;

        assert_eq!(budget.amount,dec!(20.24));
        assert_eq!(budget.category.title,"MINE__");
        Ok(())
    }

    #[sqlx::test]
    async fn get_category(pool: SqlitePool) -> crate::Result<()>{
        let category_id = &fetch_categories(&pool).await?[0].id;
        let record = sqlx::query!("INSERT INTO budgets(amount,category_id) VALUES ($1,$2) RETURNING id","0.2",category_id)
            .fetch_one(&pool)
            .await?;

        let budget = Budget::from_id(&record.id, &pool).await?;
        assert_eq!(budget.amount.to_string(),"0.2");
        assert_eq!(&budget.category.id,category_id);
        Ok(())
    }
}