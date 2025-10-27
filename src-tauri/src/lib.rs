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
pub mod analytics;
pub mod command;
mod settings;

mod error;
mod money;
pub mod service;
use std::path::PathBuf;
use std::sync::{Arc,};
use tokio::sync::Mutex;
use crate::command::*;
pub use error::{Error, Result};
pub use money::Money;
use sqlx::SqlitePool;
use tauri::{WebviewUrl, WebviewWindowBuilder};
use crate::settings::Settings;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub async fn run() {
    tracing_subscriber::fmt::init();
    let state = State::new().await.unwrap();
    tauri::Builder::default()
        .plugin(tauri_plugin_os::init())
        .manage(state)
        .setup(|app| {
            let builder = WebviewWindowBuilder::new(app, "main", WebviewUrl::default())
                .title("Folio")
                .resizable(true)
                .maximized(true);

            #[cfg(windows)]
            let builder = builder.decorations(false);

            builder.build().unwrap();
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
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
            fetch_budgets,
            create_account,
            create_budget,
            edit_budget,
            currencies,
            set_currency_code,
            settings,
            delete_budget,
            income_analytics,
            create_income_stream,
            delete_income_stream,
            fetch_categories,
            edit_income,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[derive(Clone)]
pub struct State {
    pool: SqlitePool,
    settings: Arc<Mutex<Settings>>,
}

impl State {
    pub async fn new() -> Result<Self> {
        let pool = init_database().await?;
        #[cfg(debug_assertions)]
        let mut path = PathBuf::from(".");

        #[cfg(not(debug_assertions))]
        let mut path = get_data_dir().unwrap();
        path = path.join("settings.json");

        let settings = Settings::open(path)?;
        Ok(Self { pool,settings: Arc::new(Mutex::new(settings)) })
    }
}

// TODO: run this after opening the app
pub async fn init_database() -> Result<SqlitePool> {
    #[cfg(debug_assertions)]
    let pool = sqlx::SqlitePool::connect("sqlite://data.db").await?;

    #[cfg(not(debug_assertions))]
    let pool = {
        use sqlx::sqlite::SqliteConnectOptions;
        let data_dir = get_data_dir().unwrap();
        std::fs::create_dir_all(&data_dir)?;
        let data_dir = data_dir.join("data.db");

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
