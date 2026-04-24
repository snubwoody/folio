use chrono::{Datelike, Local};
use std::collections::HashMap;
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
use serde::{Deserialize, Serialize};
use sqlx::{Row, SqlitePool};

use crate::{Money, service::Category};

#[derive(Serialize, Deserialize, Debug, PartialOrd, PartialEq)]
pub struct Analytic {
    category: Category,
    total: Money,
}

impl Analytic {
    pub fn new(category: Category, total: Money) -> Self {
        Self { category, total }
    }
}

/// Fetches the total spend per category and total received per income stream. Only
/// transactions in the current month will be counted.
pub async fn analytics(pool: &SqlitePool) -> crate::Result<Vec<Analytic>> {
    let today = Local::now();
    let month_start = today.date_naive().with_day(1);
    let rows = sqlx::query(
        "
            SELECT
                t.amount,t.transaction_date,
                c.title,c.id,c.is_income_stream,c.created_at
            FROM transactions AS t
            JOIN categories c on c.id = t.category_id
            WHERE c.deleted_at IS NULL AND t.transaction_date >= $1
        ",
    )
    .bind(month_start)
    .fetch_all(pool)
    .await?;

    let mut analytics = HashMap::new();
    for row in rows {
        let amount: Money = row.get("amount");
        let category = Category {
            id: row.get("id"),
            title: row.get("title"),
            created_at: row.get("created_at"),
            is_income_stream: row.get("is_income_stream"),
            deleted_at: None,
        };

        match analytics.get(&category) {
            Some(total) => analytics.insert(category, *total + amount),
            None => analytics.insert(category, amount),
        };
    }

    let analytics = analytics
        .into_iter()
        .map(|a| Analytic::new(a.0, a.1))
        .collect();

    Ok(analytics)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::Money;
    use chrono::NaiveDate;

    use crate::service::{AccountService, CategoryService, Transaction};

    #[sqlx::test]
    async fn fetch_analytics(pool: SqlitePool) -> crate::Result<()> {
        let account_service = AccountService::new(pool.clone());
        let c1 = Category::create("Expense", &pool).await?;
        let a1 = account_service
            .create_account("Expense", Money::ZERO)
            .await?;
        Transaction::expense()
            .account_id(&a1.id)
            .category(&c1.id)
            .amount(Money::from_unscaled(100))
            .create(&pool)
            .await?;

        Transaction::expense()
            .account_id(&a1.id)
            .category(&c1.id)
            .amount(Money::from_unscaled(100))
            .create(&pool)
            .await?;

        let analytics = analytics(&pool).await?;
        assert_eq!(analytics.len(), 1);
        assert_eq!(analytics[0].total, Money::from_unscaled(200));
        Ok(())
    }

    #[sqlx::test]
    async fn fetch_analytics_in_current_month(pool: SqlitePool) -> crate::Result<()> {
        let service = CategoryService::new(pool.clone());
        let account_service = AccountService::new(pool.clone());
        let c1 = service.create_category("Expense").await?;
        let a1 = account_service
            .create_account("Expense", Money::ZERO)
            .await?;
        Transaction::expense()
            .account_id(&a1.id)
            .category(&c1.id)
            .date(NaiveDate::MIN)
            .amount(Money::from_unscaled(100))
            .create(&pool)
            .await?;

        Transaction::expense()
            .account_id(&a1.id)
            .category(&c1.id)
            .amount(Money::from_unscaled(100))
            .create(&pool)
            .await?;

        let analytics = analytics(&pool).await?;
        assert_eq!(analytics[0].total, Money::from_unscaled(100));
        Ok(())
    }
}
