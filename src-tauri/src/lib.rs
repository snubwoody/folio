mod error;
mod service;
pub use error::{Error, Result};
use rust_decimal::prelude::*;
use sqlx::SqlitePool;

use crate::service::{Account, Category, CreateExpense, EditExpense, Expense};

#[tauri::command]
async fn create_expense(state: tauri::State<'_, State>, data: CreateExpense) -> Result<()> {
    Expense::create(data, &state.pool).await?;
    Ok(())
}

#[tauri::command]
async fn edit_expense(state: tauri::State<'_, State>, id: String, data: EditExpense) -> Result<()> {
    Expense::update(&id, data, &state.pool).await?;
    Ok(())
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
async fn fetch_categories(state: tauri::State<'_, State>) -> Result<Vec<Category>> {
    service::fetch_categories(&state.pool).await
}

#[tauri::command]
async fn create_category(state: tauri::State<'_, State>, title: &str) -> Result<()> {
    Category::create(title, &state.pool).await?;
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub async fn run() {
    tracing_subscriber::fmt::init();
    let state = State::new().await;
    tauri::Builder::default()
        .manage(state)
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            create_expense,
            fetch_expenses,
            edit_expense,
            create_account,
            fetch_accounts,
            create_category,
            fetch_categories
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[derive(Clone)]
pub struct State {
    pool: SqlitePool,
}

impl State {
    pub async fn new() -> Self {
        let pool = init_database().await;
        Self { pool }
    }
}

pub async fn init_database() -> SqlitePool {
    // FIXME: don't unwrap
    sqlx::SqlitePool::connect("sqlite://data.db").await.unwrap()
}
