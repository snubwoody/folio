use std::str::FromStr;

use chrono::{Local, NaiveDate};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use tracing::info;

use crate::{service::{Account, Category}, Money, DECIMAL_SCALE};

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

// TODO: try deleting account and category deps
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Expense {
    pub id: String,
    pub amount: Money,
    pub date: NaiveDate,
    pub account: Option<Account>,
    pub category: Option<Category>,
    pub currency_code: String,
}

impl Expense {
    pub async fn create(data: CreateExpense, pool: &SqlitePool) -> Result<Self, crate::Error> {
        let date = data.date.to_string();
        let amount = data.amount.inner();

        let record = sqlx::query!(
            "INSERT INTO expenses(
				amount,
				transaction_date,
				account_id,
				category_id,
				currency_code
			)
			VALUES($1,$2,$3,$4,$5)
			RETURNING id",
            amount,
            date,
            data.account_id,
            data.category_id,
            data.currency_code
        )
        .fetch_one(pool)
        .await?;

        let expense = Self::from_id(&record.id, pool).await?;
        info!(expense=?expense,"Created expense");
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

        sqlx::query!(
            "
            UPDATE expenses 
            SET amount= $1,
             transaction_date= $2,
             category_id=$3, 
             account_id=$4
            WHERE id=$5",
            amount,
            date,
            category_id,
            account_id,
            id
        )
        .execute(pool)
        .await?;
        Ok(())
    }

    pub async fn from_id(id: &str, pool: &SqlitePool) -> Result<Self, crate::Error> {
        let record = sqlx::query!("SELECT * FROM expenses WHERE id=$1", id)
            .fetch_one(pool)
            .await?;

        let date = NaiveDate::from_str(&record.transaction_date)?;
        let amount = Money::new(record.amount);
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
        })
    }
}

/// Fetch all the expenses from the database.
pub async fn fetch_expenses(pool: &SqlitePool) -> Result<Vec<Expense>, crate::Error> {
    let records = sqlx::query!("SELECT id from expenses")
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
        let account = Account::create("", Decimal::default(), &pool).await?;
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
        assert_eq!(expense.amount.to_string(), "224.2");
        assert_eq!(expense.date.to_string(), "1900-01-01");
        Ok(())
    }

    #[sqlx::test]
    async fn create_expense(pool: SqlitePool) -> Result<(), crate::Error> {
        let account = Account::create("", Decimal::ZERO, &pool).await?;
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
        assert_eq!(record.category_id.unwrap(), category.id);
        assert_eq!(record.amount, 500_202_000);
        assert_eq!(record.currency_code, "XOF");
        assert_eq!(record.transaction_date, "2015-02-01");
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
