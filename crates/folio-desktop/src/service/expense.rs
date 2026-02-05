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
use std::str::FromStr;

use chrono::{DateTime, Local, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use tracing::info;

use crate::{
    Money, db,
    service::{Account, Category},
};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, PartialOrd)]
#[serde(rename_all = "camelCase")]
pub struct CreateExpense {
    pub amount: Money,
    pub date: NaiveDate,
    pub account_id: Option<String>,
    pub category_id: Option<String>,
    pub currency_code: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct EditExpense {
    amount: Option<Money>,
    date: Option<NaiveDate>,
    account_id: Option<String>,
    category_id: Option<String>,
}

impl Default for CreateExpense {
    fn default() -> Self {
        Self {
            amount: Money::ZERO,
            date: Local::now().date_naive(),
            account_id: None,
            category_id: None,
            currency_code: String::from("USD"),
        }
    }
}

// TODO: try deleting account deps
#[derive(Debug, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Expense {
    pub id: String,
    pub amount: Money,
    pub date: NaiveDate,
    pub account: Option<Account>,
    pub category: Option<Category>,
    pub currency_code: String,
    pub created_at: DateTime<Utc>,
}

impl Expense {
    pub async fn create(data: CreateExpense, pool: &SqlitePool) -> Result<Self, crate::Error> {
        let now = Utc::now().timestamp();
        let date = data.date.to_string();
        let amount = data.amount.inner();

        let record: db::Expense = sqlx::query_as(
            "INSERT INTO expenses(
				amount,
				transaction_date,
				account_id,
				category_id,
				currency_code,
                 created_at
			)
			VALUES($1,$2,$3,$4,$5,$6)
			RETURNING *",
        )
        .bind(amount)
        .bind(date)
        .bind(data.account_id)
        .bind(data.category_id)
        .bind(data.currency_code)
        .bind(now)
        .fetch_one(pool)
        .await?;

        let expense = Self::from_id(&record.id, pool).await?;
        // info!(expense=?expense,"Created expense");
        info!(id=?expense.id,"Created expense");
        Ok(expense)
    }

    pub async fn update(
        id: &str,
        data: EditExpense,
        pool: &SqlitePool,
    ) -> Result<(), crate::Error> {
        let expense = Self::from_id(id, pool).await?;

        let amount = data.amount.unwrap_or(expense.amount).inner();
        let date = data.date.unwrap_or(expense.date);
        let mut account_id = data.account_id;
        if let Some(account) = expense.account
            && account_id.is_none()
        {
            account_id = Some(account.id)
        }
        let mut category_id = data.category_id;
        if let Some(category) = expense.category
            && category_id.is_none()
        {
            category_id = Some(category.id)
        }

        sqlx::query(
            "
            UPDATE expenses
            SET amount= $1,
             transaction_date= $2,
             category_id=$3,
             account_id=$4
            WHERE id=$5",
        )
        .bind(amount)
        .bind(date)
        .bind(category_id)
        .bind(account_id)
        .bind(id)
        .execute(pool)
        .await?;

        Ok(())
    }

    pub async fn delete(id: &str, pool: &SqlitePool) -> crate::Result<()> {
        sqlx::query("DELETE FROM expenses WHERE id=$1")
            .bind(id)
            .execute(pool)
            .await?;

        Ok(())
    }

    pub async fn from_id(id: &str, pool: &SqlitePool) -> Result<Self, crate::Error> {
        let record: db::Expense = sqlx::query_as("SELECT * FROM expenses WHERE id=$1")
            .bind(id)
            .fetch_one(pool)
            .await?;

        let date = NaiveDate::from_str(&record.transaction_date)?;
        let amount = Money::new(record.amount);
        let created_at = DateTime::from_timestamp(record.created_at, 0).unwrap_or_default();

        let category = match record.category_id {
            Some(id) => Some(Category::from_id(&id, pool).await?),
            None => None,
        };

        let account = match record.account_id {
            Some(id) => Some(Account::from_id(&id, pool).await?),
            None => None,
        };

        Ok(Self {
            id: record.id,
            currency_code: record.currency_code,
            date,
            amount,
            account,
            category,
            created_at,
        })
    }
}

/// Fetch all the expenses from the database.
pub async fn fetch_expenses(pool: &SqlitePool) -> Result<Vec<Expense>, crate::Error> {
    let records: Vec<db::Expense> = sqlx::query_as("SELECT * from expenses")
        .fetch_all(pool)
        .await?;

    let mut expenses = vec![];
    for row in records {
        let expense = Expense::from_id(&row.id, pool).await?;
        expenses.push(expense);
    }

    Ok(expenses)
}

#[cfg(test)]
mod test {
    use super::*;

    #[sqlx::test]
    async fn update_expense(pool: SqlitePool) -> crate::Result<()> {
        let expense = Expense::create(Default::default(), &pool).await?;
        let account = Account::create("", Money::default(), &pool).await?;
        let category = Category::create("", &pool).await?;
        let data = EditExpense {
            date: Some(NaiveDate::from_ymd_opt(1900, 1, 1).unwrap()),
            category_id: Some(category.id.clone()),
            account_id: Some(account.id.clone()),
            amount: Some(Money::from_f64(224.2)),
        };

        Expense::update(&expense.id, data, &pool).await?;

        let expense = Expense::from_id(&expense.id, &pool).await?;
        assert_eq!(expense.account.unwrap().id, account.id);
        assert_eq!(expense.category.unwrap().id, category.id);
        assert_eq!(expense.amount.inner(), 224_200_000);
        assert_eq!(expense.date.to_string(), "1900-01-01");
        Ok(())
    }

    #[sqlx::test]
    async fn create_expense(pool: SqlitePool) -> Result<(), crate::Error> {
        let now = Utc::now().timestamp();
        let account = Account::create("", Money::ZERO, &pool).await?;
        let category = Category::create("", &pool).await?;
        let data = CreateExpense {
            amount: Money::from_f64(500.202),
            date: NaiveDate::from_ymd_opt(2015, 2, 1).unwrap(),
            currency_code: String::from("XOF"),
            account_id: Some(account.id.clone()),
            category_id: Some(category.id.clone()),
        };

        let expense = Expense::create(data, &pool).await?;
        let record = sqlx::query!("SELECT * FROM expenses WHERE id=$1", expense.id)
            .fetch_one(&pool)
            .await?;

        assert_eq!(record.account_id.unwrap(), account.id);
        assert!(record.created_at >= now);
        assert_eq!(record.category_id.unwrap(), category.id);
        assert_eq!(record.amount, 500_202_000);
        assert_eq!(record.currency_code, "XOF");
        assert_eq!(record.transaction_date, "2015-02-01");
        Ok(())
    }

    #[sqlx::test]
    async fn delete_expense(pool: SqlitePool) -> Result<(), crate::Error> {
        let expense1 = Expense::create(Default::default(), &pool).await?;
        let expense2 = Expense::create(Default::default(), &pool).await?;
        let expenses = fetch_expenses(&pool).await?;
        assert_eq!(expenses.len(), 2);

        Expense::delete(&expense1.id, &pool).await?;
        let expenses = fetch_expenses(&pool).await?;
        assert_eq!(expenses.len(), 1);
        assert_eq!(expenses[0].id, expense2.id);
        Ok(())
    }

    #[sqlx::test]
    async fn fetch_expense(pool: SqlitePool) -> Result<(), crate::Error> {
        let amount = Money::from_unscaled(20).inner();
        let record = sqlx::query!(
            "INSERT INTO expenses(amount,currency_code) VALUES($1,'ZAR') RETURNING id",
            amount
        )
        .fetch_one(&pool)
        .await
        .unwrap();

        let expense = Expense::from_id(&record.id, &pool).await?;
        assert_eq!(expense.amount.inner(), 20_000_000);
        assert_eq!(expense.currency_code, "ZAR");
        Ok(())
    }

    #[test]
    fn default_expense_date() {
        assert_eq!(CreateExpense::default().date, Local::now().date_naive());
    }
}
