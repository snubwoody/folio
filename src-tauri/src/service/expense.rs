use std::str::FromStr;

use chrono::{Local, NaiveDate};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;

use crate::service::{Account, Category};

pub struct CreateExpense {
    amount: String,
    date: NaiveDate,
    account_id: Option<String>,
    category_id: Option<String>,
    currency_code: String,
}

impl CreateExpense {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn date(mut self, date: NaiveDate) -> Self {
        self.date = date;
        self
    }

    pub fn amount(mut self, amount: &str) -> Self {
        self.amount = amount.to_owned();
        self
    }

    pub fn account_id(mut self, id: &str) -> Self {
        self.account_id = Some(id.to_owned());
        self
    }

    pub fn category_id(mut self, id: &str) -> Self {
        self.category_id = Some(id.to_owned());
        self
    }

    pub fn currency_code(mut self, code: &str) -> Self {
        self.currency_code = code.to_owned();
        self
    }
}

impl Default for CreateExpense {
    fn default() -> Self {
        Self {
            amount: String::from("0"),
            date: Local::now().date_naive(),
            account_id: None,
            category_id: None,
            currency_code: String::from("USD"),
        }
    }
}

// TODO: try deleting account and category deps
#[derive(Debug, Serialize, Deserialize)]
pub struct Expense {
    id: String,
    amount: Decimal,
    date: NaiveDate,
    account: Option<Account>,
    category: Option<Category>,
    currency_code: String,
}

impl Expense {
    pub async fn create(data: CreateExpense, pool: &SqlitePool) -> Result<Self, crate::Error> {
        let amount = data.amount.to_string();
        let date = data.date.to_string();

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

        Self::from_id(&record.id, pool).await
    }

    pub async fn from_id(id: &str, pool: &SqlitePool) -> Result<Self, crate::Error> {
        let record = sqlx::query!("SELECT * FROM expenses WHERE id=$1", id)
            .fetch_one(pool)
            .await?;

        let date = NaiveDate::from_str(&record.transaction_date)?;
        let amount = Decimal::from_str(&record.amount)?;
        let category = match record.category_id {
            Some(id) => Some(Category::from_id(&id, pool).await),
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

#[cfg(test)]
mod test {
    use super::*;

    #[sqlx::test]
    async fn create_expense(pool: SqlitePool) -> Result<(), crate::Error> {
        let account = Account::create("", Decimal::ZERO, &pool).await?;
        let category = Category::create("", &pool).await;
        let data = CreateExpense::new()
            .amount("500.2024242")
            .date(NaiveDate::from_ymd_opt(2015, 2, 1).unwrap())
            .currency_code("XOF")
            .account_id(&account.id)
            .category_id(&category.id);

        let expense = Expense::create(data, &pool).await?;

        let record = sqlx::query!("SELECT * FROM expenses WHERE id=$1", expense.id)
            .fetch_one(&pool)
            .await?;

        assert_eq!(record.account_id.unwrap(), account.id);
        assert_eq!(record.category_id.unwrap(), category.id);
        assert_eq!(record.amount, "500.2024242");
        assert_eq!(record.currency_code, "XOF");
        assert_eq!(record.transaction_date, "2015-02-01");
        Ok(())
    }

    #[sqlx::test]
    async fn fetch_expense(pool: SqlitePool) -> Result<(), crate::Error> {
        let record = sqlx::query!(
            "INSERT INTO expenses(amount,currency_code) VALUES('204.24','ZAR') RETURNING id"
        )
        .fetch_one(&pool)
        .await
        .unwrap();

        let expense = Expense::from_id(&record.id, &pool).await?;
        assert_eq!(expense.amount.to_string(), "204.24");
        assert_eq!(expense.currency_code, "ZAR");
        Ok(())
    }

    #[test]
    fn default_expense_date() {
        assert_eq!(CreateExpense::default().date, Local::now().date_naive());
    }
}
