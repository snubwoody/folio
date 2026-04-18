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

use crate::analytics::Analytic;
use crate::error::ErrorExt;
use crate::settings::Settings;
use crate::{
    Money, Result, State, analytics,
    service::{self, *},
};
use chrono::NaiveDate;
use iso_currency::{Currency, IntoEnumIterator};
use std::str::FromStr;
use tauri::{Builder, Wry};
use tracing::{debug, error, info, warn};

/// Adds tauri commands to the app instance.
pub fn handlers(app: Builder<Wry>) -> Builder<Wry> {
    app.invoke_handler(tauri::generate_handler![
        create_expense,
        set_transaction_payee,
        set_transaction_inflow,
        create_account,
        delete_category,
        set_sidebar_state,
        edit_category,
        parse_date,
        delete_transactions,
        create_category,
        fetch_accounts,
        edit_account,
        delete_account,
        fetch_budgets,
        create_account,
        set_transaction_outflow,
        account_balance,
        create_budget,
        edit_budget,
        currencies,
        set_transaction_account,
        set_currency_code,
        settings,
        get_budget,
        analytics,
        create_income_stream,
        fetch_categories,
        create_missing_budgets,
        log_info,
        log_error,
        edit_transaction,
        log_debug,
        log_warn,
        fetch_transactions,
    ])
}

#[tauri::command]
pub fn log_info(message: String) {
    info!("[frontend] {message}")
}

#[tauri::command]
pub fn log_warn(message: String) {
    warn!("[frontend] {message}")
}

#[tauri::command]
pub fn log_debug(message: String) {
    debug!("[frontend] {message}")
}

#[tauri::command]
pub fn log_error(message: String) {
    error!("[frontend] {message}")
}

#[tauri::command]
pub fn parse_date(value: String) -> NaiveDate {
    super::date::parse_date(&value)
}

#[tauri::command]
pub async fn settings(state: tauri::State<'_, State>) -> Result<Settings> {
    let settings = state.settings.lock().await.clone();
    Ok(settings)
}

#[tauri::command]
pub async fn set_currency_code(state: tauri::State<'_, State>, currency: Currency) -> Result<()> {
    let mut settings = state.settings.lock().await;
    settings
        .set_currency_code(currency)
        .inspect_err(|err| warn!("{}", err.report()))?;
    Ok(())
}

#[tauri::command]
pub async fn set_sidebar_state(state: tauri::State<'_, State>, open: bool) -> Result<()> {
    let mut settings = state.settings.lock().await;
    settings
        .set_sidebar_state(open)
        .inspect_err(|err| warn!("{}", err.report()))?;
    Ok(())
}

#[tauri::command]
pub async fn set_transaction_payee(
    state: tauri::State<'_, State>,
    id: String,
    account_id: String,
) -> Result<Transaction> {
    Transaction::set_payee(&id, &account_id, &state.pool)
        .await
        .context("Failed to set transaction payee")
        .inspect_err(|err| warn!("{}", err.report()))
}

#[tauri::command]
pub fn currencies() -> Vec<Currency> {
    Currency::iter().collect()
}

#[tauri::command]
pub async fn create_expense(
    state: tauri::State<'_, State>,
    amount: Money,
    date: NaiveDate,
    account: String,
) -> Result<Transaction> {
    Transaction::expense()
        .amount(amount)
        .account_id(&account)
        .date(date)
        .create(&state.pool)
        .await
        .inspect_err(|err| warn!("{}", err.report()))
}

#[tauri::command]
pub async fn edit_transaction(
    state: tauri::State<'_, State>,
    data: EditBuilder,
) -> Result<Transaction> {
    data.update(&state.pool)
        .await
        .inspect_err(|err| warn!("{}", err.report()))
}

#[tauri::command]
pub async fn set_transaction_outflow(
    state: tauri::State<'_, State>,
    id: String,
    amount: Money,
) -> Result<Transaction> {
    Transaction::set_outflow(&id, amount, &state.pool)
        .await
        .inspect_err(|err| warn!("{}", err.report()))
}

#[tauri::command]
pub async fn set_transaction_inflow(
    state: tauri::State<'_, State>,
    id: String,
    amount: Money,
) -> Result<Transaction> {
    Transaction::set_inflow(&id, amount, &state.pool)
        .await
        .inspect_err(|err| warn!("{}", err.report()))
}

#[tauri::command]
pub async fn set_transaction_account(
    state: tauri::State<'_, State>,
    id: String,
    account: String,
) -> Result<Transaction> {
    Transaction::set_account(&id, &account, &state.pool)
        .await
        .context("Failed to set transaction account")
        .inspect_err(|err| warn!("{}", err.report()))
}

#[tauri::command]
pub async fn fetch_transactions(state: tauri::State<'_, State>) -> Result<Vec<Transaction>> {
    Transaction::fetch_all(&state.pool)
        .await
        .inspect_err(|err| warn!("{}", err.report()))
}

#[tauri::command]
pub async fn delete_category(state: tauri::State<'_, State>, id: String) -> Result<()> {
    Category::delete(&id, &state.pool)
        .await
        .inspect_err(|err| warn!("{}", err.report()))
}

#[tauri::command]
pub async fn create_missing_budgets(state: tauri::State<'_, State>) -> Result<()> {
    service::create_missing_budgets(&state.pool)
        .await
        .inspect_err(|err| warn!("{}", err.report()))
}

#[tauri::command]
pub async fn edit_category(
    state: tauri::State<'_, State>,
    id: String,
    title: String,
) -> Result<Category> {
    Category::edit(&id, &title, &state.pool)
        .await
        .context("Failed to edit category")
        .inspect_err(|err| warn!("{}", err.report()))
}

#[tauri::command]
pub async fn analytics(state: tauri::State<'_, State>) -> Result<Vec<Analytic>> {
    analytics::analytics(&state.pool)
        .await
        .context("Failed to fetch analytics")
        .inspect_err(|err| warn!("{}", err.report()))
}

#[tauri::command]
pub async fn create_account(
    state: tauri::State<'_, State>,
    name: &str,
    starting_balance: Money,
) -> Result<Account> {
    Account::create(name, starting_balance, &state.pool)
        .await
        .inspect_err(|err| warn!("{}", err.report()))
}

#[tauri::command]
pub async fn account_balance(state: tauri::State<'_, State>, id: String) -> Result<Money> {
    Account::calculate_balance(&id, &state.pool)
        .await
        .inspect_err(|err| warn!("{}", err.report()))
}

#[tauri::command]
pub async fn fetch_accounts(state: tauri::State<'_, State>) -> Result<Vec<Account>> {
    Account::fetch_all(&state.pool)
        .await
        .context("Failed to fetch accounts")
        .inspect_err(|err| warn!("{}", err.report()))
}

#[tauri::command]
pub async fn fetch_categories(state: tauri::State<'_, State>) -> Result<Vec<Category>> {
    Category::fetch_all(&state.pool)
        .await
        .inspect_err(|err| warn!("{}", err.report()))
}

#[tauri::command]
pub async fn fetch_budgets(state: tauri::State<'_, State>) -> Result<Vec<Budget>> {
    service::fetch_budgets(&state.pool)
        .await
        .inspect_err(|err| warn!("{}", err.report()))
}

#[tauri::command]
pub async fn get_budget(category_id: String, state: tauri::State<'_, State>) -> Result<Budget> {
    Budget::from_category(&category_id, &state.pool)
        .await
        .inspect_err(|err| warn!("{}", err.report()))
}

#[tauri::command]
pub async fn create_budget(
    amount: &str,
    category_id: &str,
    state: tauri::State<'_, State>,
) -> Result<Budget> {
    Budget::create(Money::from_str(amount)?, category_id, &state.pool)
        .await
        .context("Failed to create budget")
        .inspect_err(|err| warn!("{}", err.report()))
}

#[tauri::command]
pub async fn edit_budget(
    id: String,
    amount: Money,
    state: tauri::State<'_, State>,
) -> Result<Budget> {
    Budget::edit(&id, amount, &state.pool)
        .await
        .context("Failed to edit budget")
        .inspect_err(|err| warn!("{}", err.report()))
}

#[tauri::command]
pub async fn create_category(state: tauri::State<'_, State>, title: &str) -> Result<Category> {
    Category::create(title, &state.pool)
        .await
        .context("Failed to create category")
        .inspect_err(|err| warn!("{}", err.report()))
}

#[tauri::command]
pub async fn delete_account(state: tauri::State<'_, State>, id: String) -> Result<()> {
    Account::delete(&id, &state.pool)
        .await
        .context("Failed to delete account")
        .inspect_err(|err| warn!("{}", err.report()))
}

#[tauri::command]
pub async fn delete_transactions(state: tauri::State<'_, State>, ids: Vec<String>) -> Result<()> {
    Transaction::delete(ids.as_slice(), &state.pool)
        .await
        .context("Failed to delete transactions")
        .inspect_err(|err| warn!("{}", err.report()))
}

#[tauri::command]
pub async fn edit_account(
    state: tauri::State<'_, State>,
    id: String,
    opts: EditAccount,
) -> Result<Account> {
    Account::edit(&id, opts, &state.pool)
        .await
        .inspect_err(|err| warn!("{}", err.report()))
}

#[tauri::command]
pub async fn create_income_stream(state: tauri::State<'_, State>, title: &str) -> Result<Category> {
    Category::create_income_stream(title, &state.pool)
        .await
        .context("Failed to create income stream")
        .inspect_err(|err| warn!("{}", err.report()))
}
