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
    pub deleted_at: Option<i64>,
    pub is_income_stream: bool,
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
