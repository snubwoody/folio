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
use crate::{
    Money, Result, State,
    analytics::{self, IncomeAnalytic, SpendingAnalytic},
    service::{self, *},
};
use std::str::FromStr;

#[tauri::command]
pub async fn create_expense(state: tauri::State<'_, State>, data: CreateExpense) -> Result<()> {
    Expense::create(data, &state.pool).await?;
    Ok(())
}

#[tauri::command]
pub async fn delete_category(state: tauri::State<'_, State>, id: String) -> Result<()> {
    Category::delete(&id, &state.pool).await
}

#[tauri::command]
pub async fn edit_category(
    state: tauri::State<'_, State>,
    id: String,
    title: String,
) -> Result<Category> {
    Category::edit(&id, &title, &state.pool).await
}

#[tauri::command]
pub async fn edit_income_stream(
    state: tauri::State<'_, State>,
    id: String,
    title: String,
) -> Result<IncomeStream> {
    IncomeStream::edit(&id, &title, &state.pool).await
}

#[tauri::command]
pub async fn delete_income_stream(state: tauri::State<'_, State>, id: String) -> Result<()> {
    IncomeStream::delete(&id, &state.pool).await
}

#[tauri::command]
pub async fn create_income(state: tauri::State<'_, State>, data: CreateIncome) -> Result<()> {
    Income::create(data, &state.pool).await?;
    Ok(())
}

#[tauri::command]
pub async fn edit_expense(
    state: tauri::State<'_, State>,
    id: String,
    data: EditExpense,
) -> Result<()> {
    Expense::update(&id, data, &state.pool).await?;
    Ok(())
}

#[tauri::command]
pub async fn edit_income(
    state: tauri::State<'_, State>,
    id: String,
    data: EditIncome,
) -> Result<()> {
    Income::update(&id, data, &state.pool).await?;
    Ok(())
}

#[tauri::command]
pub async fn spending_analytics(state: tauri::State<'_, State>) -> Result<Vec<SpendingAnalytic>> {
    analytics::spending_analytics(&state.pool).await
}

#[tauri::command]
pub async fn income_analytics(state: tauri::State<'_, State>) -> Result<Vec<IncomeAnalytic>> {
    analytics::income_analytics(&state.pool).await
}

#[tauri::command]
pub async fn create_account(
    state: tauri::State<'_, State>,
    name: &str,
    starting_balance: Money,
) -> Result<()> {
    Account::create(name, starting_balance, &state.pool).await?;
    Ok(())
}

#[tauri::command]
pub async fn fetch_accounts(state: tauri::State<'_, State>) -> Result<Vec<Account>> {
    service::fetch_accounts(&state.pool).await
}

#[tauri::command]
pub async fn fetch_expenses(state: tauri::State<'_, State>) -> Result<Vec<Expense>> {
    let expenses = service::fetch_expenses(&state.pool).await?;
    Ok(expenses)
}

#[tauri::command]
pub async fn fetch_incomes(state: tauri::State<'_, State>) -> Result<Vec<Income>> {
    let expenses = service::fetch_incomes(&state.pool).await?;
    Ok(expenses)
}

#[tauri::command]
pub async fn fetch_categories(state: tauri::State<'_, State>) -> Result<Vec<Category>> {
    service::fetch_categories(&state.pool).await
}

#[tauri::command]
pub async fn fetch_income_streams(state: tauri::State<'_, State>) -> Result<Vec<IncomeStream>> {
    service::fetch_income_streams(&state.pool).await
}

#[tauri::command]
pub async fn fetch_budgets(state: tauri::State<'_, State>) -> Result<Vec<Budget>> {
    service::fetch_budgets(&state.pool).await
}

#[tauri::command]
pub async fn create_budget(
    amount: &str,
    category_id: &str,
    state: tauri::State<'_, State>,
) -> Result<()> {
    Budget::create(Money::from_str(amount)?, category_id, &state.pool).await?;
    Ok(())
}

#[tauri::command]
pub async fn create_category(state: tauri::State<'_, State>, title: &str) -> Result<Category> {
    Category::create(title, &state.pool).await
}

#[tauri::command]
pub async fn create_income_stream(state: tauri::State<'_, State>, title: &str) -> Result<IncomeStream> {
    IncomeStream::create(title, &state.pool).await
}
