//! Contains database types
use sqlx::FromRow;

#[derive(FromRow, Debug, Clone, PartialOrd, PartialEq)]
pub struct Budget {
    pub id: String,
    pub amount: i64,
    pub category_id: String,
    pub created_at: i64,
    pub month: Option<i64>,
    pub year: Option<i64>,
}

#[derive(FromRow, Debug, Clone, PartialOrd, PartialEq)]
pub struct Category {
    pub id: String,
    pub title: String,
    pub created_at: Option<i64>,
}

#[derive(FromRow, Debug, Clone, PartialOrd, PartialEq)]
pub struct IncomeStream {
    pub id: String,
    pub title: String,
    pub created_at: Option<i64>,
}

#[derive(FromRow, Debug, Clone, PartialOrd, PartialEq)]
pub struct Expense {
    pub id: String,
    pub amount: i64,
    pub transaction_date: String,
    pub account_id: Option<String>,
    pub category_id: Option<String>,
    pub currency_code: String,
    pub created_at: i64,
}

#[derive(FromRow, Debug, Clone, PartialOrd, PartialEq)]
pub struct Income {
    pub id: String,
    pub amount: i64,
    pub transaction_date: String,
    pub account_id: Option<String>,
    pub income_stream: Option<String>,
    pub currency_code: String,
    pub created_at: i64,
}
