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
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;

use crate::{Money, service::{Category, IncomeStream}, db};

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
        let records: Vec<db::Expense> = sqlx::query_as("SELECT * FROM expenses WHERE category_id=$1")
            .bind(id)
            .fetch_all(pool)
            .await?;

        // Fail the whole operation instead of skipping errors,
        // to avoid false analytics.
        let totals = records
            .iter()
            .map(|row| Money::new(row.amount).to_decimal())
            .collect::<Vec<_>>();

        let total: Decimal = totals.iter().sum();
        let category = Category::from_id(id, pool).await?;

        Ok(Self { total, category })
    }
}

impl IncomeAnalytic {
    pub async fn from_id(id: &str, pool: &SqlitePool) -> crate::Result<Self> {
        let records: Vec<db::Income> = sqlx::query_as("SELECT * FROM incomes WHERE income_stream=$1")
            .bind(id)
            .fetch_all(pool)
            .await?;

        // Fail the whole operation instead of skipping errors,
        // to avoid false analytics.
        let totals = records
            .iter()
            .map(|row| Money::from_scaled(row.amount).to_decimal())
            .collect::<Vec<_>>();

        let total: Decimal = totals.iter().sum();
        let stream = IncomeStream::from_id(id, pool).await?;

        Ok(Self { total, stream })
    }
}

pub async fn spending_analytics(pool: &SqlitePool) -> crate::Result<Vec<SpendingAnalytic>> {
    let records: Vec<db::Category> = sqlx::query_as("SELECT * FROM categories")
        .fetch_all(pool)
        .await?;

    let mut analytics = vec![];
    for record in records {
        let a = SpendingAnalytic::from_id(&record.id, pool).await?;
        analytics.push(a);
    }
    Ok(analytics)
}

pub async fn income_analytics(pool: &SqlitePool) -> crate::Result<Vec<IncomeAnalytic>> {
    let records: Vec<db::IncomeStream> = sqlx::query_as("SELECT * FROM income_streams")
        .fetch_all(pool)
        .await?;

    let mut analytics = vec![];
    for record in records {
        let a = IncomeAnalytic::from_id(&record.id, pool).await?;
        analytics.push(a);
    }
    Ok(analytics)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{
        Money,
        service::{CreateExpense, CreateIncome, Expense, Income},
    };
    use rust_decimal::dec;

    #[sqlx::test]
    async fn get_spending_analytics(pool: SqlitePool) -> crate::Result<()> {
        let category = Category::create("MONEY", &pool).await?;
        let mut data = CreateExpense {
            amount: Money::from_f64(20.2),
            category_id: Some(category.id.clone()),
            ..Default::default()
        };
        Expense::create(data.clone(), &pool).await?;
        data.amount = Money::from_f64(500.23);
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
            amount: Money::from_f64(20.2),
            income_stream_id: Some(stream.id.clone()),
            ..Default::default()
        };
        Income::create(data.clone(), &pool).await?;
        data.amount = Money::from_f64(500.23);
        Income::create(data.clone(), &pool).await?;

        let analytic = IncomeAnalytic::from_id(&stream.id, &pool).await?;
        assert_eq!(analytic.stream.title, "SALARY__");
        assert_eq!(analytic.total, dec!(520.43));
        Ok(())
    }
}
