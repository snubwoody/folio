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

use crate::settings::Settings;
use crate::{
    Money, Result, State,
    analytics::{self, IncomeAnalytic, SpendingAnalytic},
    service::{self, *},
};
use folio_core::{BugReport, FeatureRequest, SupportResponse};
use iso_currency::{Currency, IntoEnumIterator};
use std::str::FromStr;
use tauri::{Builder, Wry};

pub fn handlers(app: Builder<Wry>) -> Builder<Wry> {
    app.invoke_handler(tauri::generate_handler![
        create_expense,
        create_income,
        fetch_expenses,
        fetch_incomes,
        fetch_income_streams,
        edit_expense,
        create_account,
        delete_category,
        edit_category,
        spending_analytics,
        create_category,
        fetch_accounts,
        edit_account,
        delete_account,
        fetch_budgets,
        create_account,
        delete_expense,
        delete_income,
        create_budget,
        edit_budget,
        currencies,
        set_currency_code,
        settings,
        delete_budget,
        income_analytics,
        edit_income_stream,
        create_income_stream,
        delete_income_stream,
        fetch_categories,
        edit_income,
        feature_request,
        bug_report,
    ])
}

#[tauri::command]
pub async fn settings(state: tauri::State<'_, State>) -> Result<Settings> {
    let settings = state.settings.lock().await.clone();
    Ok(settings)
}

#[tauri::command]
pub async fn set_currency_code(state: tauri::State<'_, State>, currency: Currency) -> Result<()> {
    let mut settings = state.settings.lock().await;
    crate::settings::set_currency_code(currency, &state.pool, &mut settings).await?;
    Ok(())
}

#[tauri::command]
pub fn currencies() -> Vec<Currency> {
    Currency::iter().collect()
}

#[tauri::command]
pub async fn create_expense(state: tauri::State<'_, State>, data: CreateExpense) -> Result<()> {
    let mut data = data;
    data.currency_code = state.settings.lock().await.currency_code().to_string();
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
    let mut data = data;
    data.currency_code = state.settings.lock().await.currency_code().to_string();
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
) -> Result<Budget> {
    Budget::create(Money::from_str(amount)?, category_id, &state.pool).await
}

#[tauri::command]
pub async fn delete_budget(id: String, state: tauri::State<'_, State>) -> Result<()> {
    Budget::delete(&id, &state.pool).await?;
    Ok(())
}

#[tauri::command]
pub async fn edit_budget(
    id: String,
    amount: Money,
    state: tauri::State<'_, State>,
) -> Result<Budget> {
    Budget::edit(&id, amount, &state.pool).await
}

#[tauri::command]
pub async fn create_category(state: tauri::State<'_, State>, title: &str) -> Result<Category> {
    Category::create(title, &state.pool).await
}

#[tauri::command]
pub async fn delete_account(state: tauri::State<'_, State>, id: String) -> Result<()> {
    Account::delete(&id, &state.pool).await
}

#[tauri::command]
pub async fn delete_income(state: tauri::State<'_, State>, id: String) -> Result<()> {
    Income::delete(&id, &state.pool).await
}

#[tauri::command]
pub async fn delete_expense(state: tauri::State<'_, State>, id: String) -> Result<()> {
    Expense::delete(&id, &state.pool).await
}

#[tauri::command]
pub async fn edit_account(
    state: tauri::State<'_, State>,
    id: String,
    opts: EditAccount,
) -> Result<Account> {
    Account::edit(&id, opts, &state.pool).await
}

#[tauri::command]
pub async fn create_income_stream(
    state: tauri::State<'_, State>,
    title: &str,
) -> Result<IncomeStream> {
    IncomeStream::create(title, &state.pool).await
}

#[tauri::command]
pub async fn feature_request(request: FeatureRequest) -> Result<SupportResponse> {
    crate::support::feature_request(request).await
}

#[tauri::command]
pub async fn bug_report(request: BugReport) -> Result<SupportResponse> {
    crate::support::bug_report(request).await
}
