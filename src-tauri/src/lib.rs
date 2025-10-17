mod analytics;
mod error;
mod service;
mod money;
use std::path::PathBuf;

pub use money::DECIMAL_SCALE;
pub use error::{Error, Result};
use rust_decimal::prelude::*;
use sqlx::{SqlitePool, sqlite::SqliteConnectOptions};

use crate::{
    analytics::{IncomeAnalytic, SpendingAnalytic},
    service::{
        Account, Budget, Category, CreateExpense, CreateIncome, EditExpense, EditIncome, Expense,
        Income, IncomeStream,
    },
};

#[tauri::command]
async fn create_expense(state: tauri::State<'_, State>, data: CreateExpense) -> Result<()> {
    Expense::create(data, &state.pool).await?;
    Ok(())
}

#[tauri::command]
async fn create_income(state: tauri::State<'_, State>, data: CreateIncome) -> Result<()> {
    Income::create(data, &state.pool).await?;
    Ok(())
}

#[tauri::command]
async fn edit_expense(state: tauri::State<'_, State>, id: String, data: EditExpense) -> Result<()> {
    Expense::update(&id, data, &state.pool).await?;
    Ok(())
}

#[tauri::command]
async fn edit_income(state: tauri::State<'_, State>, id: String, data: EditIncome) -> Result<()> {
    Income::update(&id, data, &state.pool).await?;
    Ok(())
}

#[tauri::command]
async fn spending_analytics(state: tauri::State<'_, State>) -> Result<Vec<SpendingAnalytic>> {
    analytics::spending_analytics(&state.pool).await
}

#[tauri::command]
async fn income_analytics(state: tauri::State<'_, State>) -> Result<Vec<IncomeAnalytic>> {
    analytics::income_analytics(&state.pool).await
}

#[tauri::command]
async fn create_account(
    state: tauri::State<'_, State>,
    name: &str,
    starting_balance: &str,
) -> Result<()> {
    let amount = Decimal::from_str(starting_balance)?;
    Account::create(name, amount, &state.pool).await?;
    Ok(())
}

#[tauri::command]
async fn fetch_accounts(state: tauri::State<'_, State>) -> Result<Vec<Account>> {
    service::fetch_accounts(&state.pool).await
}

#[tauri::command]
async fn fetch_expenses(state: tauri::State<'_, State>) -> Result<Vec<Expense>> {
    let expenses = service::fetch_expenses(&state.pool).await?;
    Ok(expenses)
}

#[tauri::command]
async fn fetch_incomes(state: tauri::State<'_, State>) -> Result<Vec<Income>> {
    let expenses = service::fetch_incomes(&state.pool).await?;
    Ok(expenses)
}

#[tauri::command]
async fn fetch_categories(state: tauri::State<'_, State>) -> Result<Vec<Category>> {
    service::fetch_categories(&state.pool).await
}

#[tauri::command]
async fn fetch_income_streams(state: tauri::State<'_, State>) -> Result<Vec<IncomeStream>> {
    service::fetch_income_streams(&state.pool).await
}

#[tauri::command]
async fn fetch_budgets(state: tauri::State<'_, State>) -> Result<Vec<Budget>> {
    service::fetch_budgets(&state.pool).await
}

#[tauri::command]
async fn create_budget(
    amount: &str,
    category_id: &str,
    state: tauri::State<'_, State>,
) -> Result<()> {
    // TODO: maybe just pass a decimal
    Budget::create(Decimal::from_str(amount)?, category_id, &state.pool).await?;
    Ok(())
}

#[tauri::command]
async fn create_category(state: tauri::State<'_, State>, title: &str) -> Result<()> {
    Category::create(title, &state.pool).await?;
    Ok(())
}

#[tauri::command]
async fn create_income_stream(state: tauri::State<'_, State>, title: &str) -> Result<()> {
    IncomeStream::create(title, &state.pool).await?;
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub async fn run() {
    tracing_subscriber::fmt::init();
    let state = State::new().await.unwrap();
    tauri::Builder::default()
        .manage(state)
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            create_expense,
            create_income,
            fetch_expenses,
            fetch_incomes,
            fetch_income_streams,
            edit_expense,
            create_account,
            spending_analytics,
            create_category,
            fetch_accounts,
            fetch_budgets,
            create_account,
            create_budget,
            income_analytics,
            create_income_stream,
            fetch_categories,
            edit_income,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[derive(Clone)]
pub struct State {
    pool: SqlitePool,
}

impl State {
    pub async fn new() -> Result<Self> {
        let pool = init_database().await?;
        Ok(Self { pool })
    }
}

pub async fn init_database() -> Result<SqlitePool> {
    #[cfg(debug_assertions)]
    let pool = sqlx::SqlitePool::connect("sqlite::memory")
        .await?;

    #[cfg(not(debug_assertions))]
    let pool = {
        let data_dir = get_data_dir().unwrap();
        std::fs::create_dir_all(&data_dir)?;
        data_dir.join("data.db");

        let opts = SqliteConnectOptions::new()
            .filename(data_dir)
            .create_if_missing(true);
        sqlx::SqlitePool::connect_with(opts).await?
    };


    sqlx::migrate!().run(&pool).await?;

    Ok(pool)
}

/// Get the platform specific data directory.
pub fn get_data_dir() -> Option<PathBuf> {
    // TODO: add message at startup on fail
    let base_dirs = directories::BaseDirs::new()?;
    let data_dir = base_dirs.data_dir();

    #[cfg(any(windows, target_os = "macos"))]
    let app_name = "Folio";

    #[cfg(target_os = "linux")]
    let app_name = "folio";

    Some(data_dir.join(app_name))
}

#[cfg(test)]
mod test {
    use crate::get_data_dir;

    #[test]
    fn data_dir() -> crate::Result<()> {
        let data_dir = get_data_dir();

        dbg!(data_dir);
        Ok(())
    }
}
