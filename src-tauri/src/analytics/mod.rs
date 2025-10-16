use std::str::FromStr;

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;

use crate::service::{Category, IncomeStream};

#[derive(Debug, Serialize, Deserialize)]
pub struct SpendingAnalytic {
    category: Category,
    total: Decimal,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IncomeAnalytic {
    stream: IncomeStream,
    total: Decimal,
}

impl SpendingAnalytic {
    pub async fn from_id(id: &str, pool: &SqlitePool) -> crate::Result<Self> {
        let records = sqlx::query!("SELECT amount FROM expenses WHERE category_id=$1", id)
            .fetch_all(pool)
            .await?;

        // Fail the whole operation instead of skipping errors,
        // to avoid false analytics.
        let totals = records
            .iter()
            .map(|row| Decimal::from_str(&row.amount))
            .collect::<Result<Vec<_>, _>>()?;

        let total: Decimal = totals.iter().sum();
        let category = Category::from_id(id, pool).await?;

        Ok(Self { total, category })
    }
}

impl IncomeAnalytic {
    pub async fn from_id(id: &str, pool: &SqlitePool) -> crate::Result<Self> {
        let records = sqlx::query!("SELECT amount FROM incomes WHERE income_stream=$1", id)
            .fetch_all(pool)
            .await?;

        // Fail the whole operation instead of skipping errors,
        // to avoid false analytics.
        let totals = records
            .iter()
            .map(|row| Decimal::from_str(&row.amount))
            .collect::<Result<Vec<_>, _>>()?;

        let total: Decimal = totals.iter().sum();
        let stream = IncomeStream::from_id(id, pool).await?;

        Ok(Self { total, stream })
    }
}

pub async fn spending_analytics(pool: &SqlitePool) -> crate::Result<Vec<SpendingAnalytic>> {
    let record = sqlx::query!("SELECT id FROM categories")
        .fetch_all(pool)
        .await?;

    let mut analytics = vec![];
    for record in record {
        let a = SpendingAnalytic::from_id(&record.id, pool).await?;
        analytics.push(a);
    }
    Ok(analytics)
}

pub async fn income_analytics(pool: &SqlitePool) -> crate::Result<Vec<IncomeAnalytic>> {
    let record = sqlx::query!("SELECT id FROM income_streams")
        .fetch_all(pool)
        .await?;

    let mut analytics = vec![];
    for record in record {
        let a = IncomeAnalytic::from_id(&record.id, pool).await?;
        analytics.push(a);
    }
    Ok(analytics)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::service::{CreateExpense, CreateIncome, Expense, Income};
    use rust_decimal::dec;

    #[sqlx::test]
    async fn get_spending_analytics(pool: SqlitePool) -> crate::Result<()> {
        let category = Category::create("MONEY", &pool).await?;
        let mut data = CreateExpense {
            amount: 20.2.to_string(),
            category_id: Some(category.id.clone()),
            ..Default::default()
        };
        Expense::create(data.clone(), &pool).await?;
        data.amount = 500.23.to_string();
        Expense::create(data.clone(), &pool).await?;

        let analytic = SpendingAnalytic::from_id(&category.id, &pool).await?;
        assert_eq!(analytic.category.title, "MONEY");
        assert_eq!(analytic.total, dec!(520.43));
        Ok(())
    }

    #[sqlx::test]
    async fn get_income_analytics(pool: SqlitePool) -> crate::Result<()> {
        let stream = IncomeStream::create("SALARY__", &pool).await?;
        let mut data = CreateIncome {
            amount: 20.2.to_string(),
            income_stream_id: Some(stream.id.clone()),
            ..Default::default()
        };
        Income::create(data.clone(), &pool).await?;
        data.amount = 500.23.to_string();
        Income::create(data.clone(), &pool).await?;

        let analytic = IncomeAnalytic::from_id(&stream.id, &pool).await?;
        assert_eq!(analytic.stream.title, "SALARY__");
        assert_eq!(analytic.total, dec!(520.43));
        Ok(())
    }
}
