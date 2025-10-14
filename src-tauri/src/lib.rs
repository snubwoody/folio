mod error;
mod service;
pub use error::Error;
use rust_decimal::prelude::*;
use sqlx::SqlitePool;

use crate::service::{Account, CreateExpense, Expense};

#[tauri::command]
async fn create_expense(state:tauri::State<'_,State>) -> Result<(), crate::Error> {
	let data = CreateExpense::new();
	Expense::create(data, &state.pool).await;
	Ok(())
}

#[tauri::command]
async fn create_account(state:tauri::State<'_,State>,name: &str, starting_balance: &str) -> Result<(), crate::Error> {
	let amount = Decimal::from_str(starting_balance)?;
	Account::create(name,amount,&state.pool).await?;
	Ok(())
}

#[tauri::command]
async fn fetch_accounts(state:tauri::State<'_,State>,) -> Result<Vec<Account>, crate::Error> {
	service::fetch_accounts(&state.pool).await
}


#[tauri::command]
async fn fetch_expenses(state:tauri::State<'_,State>) -> Result<Vec<Expense>, crate::Error> {
	let expenses = service::fetch_expenses(&state.pool).await?;
	Ok(expenses)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub async fn run() {
	let state = State::new().await;
    tauri::Builder::default()
		.manage(state)
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
			create_expense,
			fetch_expenses,
			create_account,
			fetch_accounts
		])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[derive(Clone)]
pub struct State{
	pool: SqlitePool
}

impl State{
	pub async fn new() -> Self{
		let pool = init_database().await;
		Self{
			pool
		}
	}
}

pub async fn init_database() -> SqlitePool {
	/// FIXME: don't unwrap
    let pool = sqlx::SqlitePool::connect("sqlite://data.db").await.unwrap();

    pool
}
