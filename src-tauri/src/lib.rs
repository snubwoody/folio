mod error;
mod service;
pub use error::Error;
use rust_decimal::prelude::*;
use sqlx::SqlitePool;

use crate::service::{CreateExpense, Expense};

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
async fn create_expense(state:tauri::State<'_,State>) -> Result<(), crate::Error> {
	let data = CreateExpense::new();
	Expense::create(data, &state.pool).await;
	Ok(())
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
        .invoke_handler(tauri::generate_handler![greet,create_expense,fetch_expenses])
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
