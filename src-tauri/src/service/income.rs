use std::str::FromStr;

use chrono::{Local, NaiveDate};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use tracing::info;

use crate::service::{Account, IncomeStream};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, PartialOrd)]
#[serde(rename_all = "camelCase")]
pub struct CreateIncome {
    pub amount: String,
    pub date: NaiveDate,
    pub account_id: Option<String>,
    pub income_stream_id: Option<String>,
    pub currency_code: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct EditIncome {
    pub amount: Option<String>,
    pub date: Option<NaiveDate>,
    pub account_id: Option<String>,
    pub income_stream_id: Option<String>,
}

impl Default for CreateIncome {
    fn default() -> Self {
        Self {
            amount: String::from("0"),
            date: Local::now().date_naive(),
            account_id: None,
            income_stream_id: None,
            currency_code: String::from("USD"),
        }
    }
}

// TODO: try deleting account and category deps
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Income {
    id: String,
    amount: Decimal,
    date: NaiveDate,
    account: Option<Account>,
    income_stream: Option<IncomeStream>,
    currency_code: String,
}

impl Income {
    pub async fn create(data: CreateIncome, pool: &SqlitePool) -> Result<Self, crate::Error> {
        let amount = data.amount.to_string();
        let date = data.date.to_string();

        let record = sqlx::query!(
            "INSERT INTO incomes(
				amount,
				transaction_date,
				account_id,
				income_stream,
				currency_code
			)
			VALUES($1,$2,$3,$4,$5)
			RETURNING id",
            amount,
            date,
            data.account_id,
            data.income_stream_id,
            data.currency_code
        )
        .fetch_one(pool)
        .await?;

        let income = Self::from_id(&record.id, pool).await?;
        info!(income=?income,"Created income");
        Ok(income)
    }

    pub async fn update(id: &str, data: EditIncome, pool: &SqlitePool) -> Result<(), crate::Error> {
        let expense = Self::from_id(id, pool).await?;

        let amount = data.amount.unwrap_or(expense.amount.to_string());
        let date = data.date.unwrap_or(expense.date);
        let mut account_id = data.account_id;
        if let Some(account) = expense.account
            && account_id.is_none()
        {
            account_id = Some(account.id)
        }
        let mut income_stream_id = data.income_stream_id;
        if let Some(income_stream) = expense.income_stream
            && income_stream_id.is_none()
        {
            income_stream_id = Some(income_stream.id)
        }

        sqlx::query!(
            "
            UPDATE incomes 
            SET amount= $1,
             transaction_date= $2,
             income_stream=$3, 
             account_id=$4
            WHERE id=$5",
            amount,
            date,
            income_stream_id,
            account_id,
            id
        )
        .execute(pool)
        .await?;
        Ok(())
    }

    pub async fn from_id(id: &str, pool: &SqlitePool) -> Result<Self, crate::Error> {
        let record = sqlx::query!("SELECT * FROM incomes WHERE id=$1", id)
            .fetch_one(pool)
            .await?;

        let date = NaiveDate::from_str(&record.transaction_date)?;
        let amount = Decimal::from_str(&record.amount)?;
        let income_stream = match record.income_stream {
            Some(id) => Some(IncomeStream::from_id(&id, pool).await?),
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
            income_stream,
        })
    }
}

/// Fetch all the expenses from the database.
pub async fn fetch_incomes(pool: &SqlitePool) -> Result<Vec<Income>, crate::Error> {
    let records = sqlx::query!("SELECT id from incomes")
        .fetch_all(pool)
        .await?;

    let mut incomes = vec![];
    for row in records {
        let income = Income::from_id(&row.id, pool).await?;
        incomes.push(income);
    }

    Ok(incomes)
}

#[cfg(test)]
mod test {
    use super::*;

    #[sqlx::test]
    async fn update_expense(pool: SqlitePool) -> crate::Result<()> {
        let income = Income::create(Default::default(), &pool).await?;
        let account = Account::create("", Decimal::default(), &pool).await?;
        let income_stream = IncomeStream::create("", &pool).await?;
        let data = EditIncome {
            date: Some(NaiveDate::from_ymd_opt(1900, 1, 1).unwrap()),
            income_stream_id: Some(income_stream.id.clone()),
            account_id: Some(account.id.clone()),
            amount: Some("224.2".to_owned()),
        };

        Income::update(&income.id, data, &pool).await?;

        let income = Income::from_id(&income.id, &pool).await?;
        assert_eq!(income.account.unwrap().id, account.id);
        assert_eq!(income.income_stream.unwrap().id, income_stream.id);
        assert_eq!(income.amount.to_string(), "224.2");
        assert_eq!(income.date.to_string(), "1900-01-01");
        Ok(())
    }

    #[sqlx::test]
    async fn create_expense(pool: SqlitePool) -> Result<(), crate::Error> {
        let account = Account::create("", Decimal::ZERO, &pool).await?;
        let stream = IncomeStream::create("", &pool).await?;
        let data = CreateIncome {
            amount: String::from("500.2024242"),
            date: NaiveDate::from_ymd_opt(2015, 2, 1).unwrap(),
            currency_code: String::from("XOF"),
            account_id: Some(account.id.clone()),
            income_stream_id: Some(stream.id.clone()),
        };

        let income = Income::create(data, &pool).await?;
        let record = sqlx::query!("SELECT * FROM incomes WHERE id=$1", income.id)
            .fetch_one(&pool)
            .await?;

        assert_eq!(record.account_id.unwrap(), account.id);
        assert_eq!(record.income_stream.unwrap(), stream.id);
        assert_eq!(record.amount, "500.2024242");
        assert_eq!(record.currency_code, "XOF");
        assert_eq!(record.transaction_date, "2015-02-01");
        Ok(())
    }

    #[sqlx::test]
    async fn fetch_expense(pool: SqlitePool) -> Result<(), crate::Error> {
        let record = sqlx::query!(
            "INSERT INTO incomes(amount,currency_code) VALUES('204.24','ZAR') RETURNING id"
        )
        .fetch_one(&pool)
        .await
        .unwrap();

        let expense = Income::from_id(&record.id, &pool).await?;
        assert_eq!(expense.amount.to_string(), "204.24");
        assert_eq!(expense.currency_code, "ZAR");
        Ok(())
    }

    #[test]
    fn default_expense_date() {
        assert_eq!(CreateIncome::default().date, Local::now().date_naive());
    }
}
