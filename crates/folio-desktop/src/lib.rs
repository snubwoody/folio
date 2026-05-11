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
mod currency;
mod date;
pub mod db;
pub mod error;
mod money;
pub mod service;
mod settings;

use crate::error::ErrorExt;
use crate::service::{AccountService, CategoryService, TransactionService};
use crate::settings::Settings;
pub use currency::Currency;
pub use error::{Error, Result};
use folio_migrate::Migrator;
pub use money::Money;
use sqlx::SqlitePool;
use sqlx::sqlite::{SqliteConnectOptions, SqliteJournalMode};
use std::path::{Path, PathBuf};
use std::sync::{Arc, MutexGuard};
use rusqlite::Connection;
use tauri::{App, WebviewUrl, WebviewWindowBuilder};
use tokio::sync::Mutex;
use tracing::info;

fn setup_app(app: &mut App) -> std::result::Result<(), Box<dyn std::error::Error>> {
    let builder = WebviewWindowBuilder::new(app, "main", WebviewUrl::default())
        .title("Folio")
        .resizable(true)
        .maximized(true);

    // Use a custom title bar on Windows
    #[cfg(windows)]
    let builder = builder.decorations(false);

    builder.build()?;
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub async fn run() -> Result<()> {
    let state = State::new()
        .await
        .context("Failed to initialise app state")?;

    let app = tauri::Builder::default()
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_prevent_default::init())
        .manage(state)
        .setup(setup_app)
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_updater::Builder::new().build());

    let app = command::handlers(app);

    app.run(tauri::generate_context!())
        .context("Failed to launch app")
}

#[derive(Clone)]
pub struct State {
    pool: SqlitePool,
    // TODO: change to std::sync::Mutex
    settings: Arc<Mutex<Settings>>,
    account_service: AccountService,
    transaction_service: TransactionService,
    category_service: CategoryService,
    connection: SqliteConnection
}

impl State {
    pub async fn new() -> Result<Self> {
        let (pool,connection) = init_database().await?;
        info!("Initialised database pool");

        let account_service = AccountService::new(pool.clone());
        let category_service = CategoryService::new(pool.clone(),connection.clone());
        let transaction_service = TransactionService::new(pool.clone());

        #[cfg(debug_assertions)]
        let mut path = PathBuf::from(".");
        #[cfg(not(debug_assertions))]
        let mut path = get_data_dir().expect("failed to get data directory");

        path = path.join("settings.json");

        let settings = Settings::open(path)?;
        Ok(Self {
            pool,
            settings: Arc::new(Mutex::new(settings)),
            account_service,
            category_service,
            transaction_service,
            connection
        })
    }
}

pub async fn init_database() -> Result<(SqlitePool,SqliteConnection)> {
    #[cfg(not(debug_assertions))]
    let path = {
        let data_dir = get_data_dir().unwrap();
        std::fs::create_dir_all(&data_dir)?;
        data_dir.join("data.db")
    };

    #[cfg(debug_assertions)]
    let path = PathBuf::from("./data.db");

    let opts = SqliteConnectOptions::new()
        .filename(&path)
        .create_if_missing(true);

    let pool = SqlitePool::connect_with(opts).await?;
    info!(path=?path,"Connected to sqlite database");

    let connection = SqliteConnection::open(&path)?;
    let mut migrator = Migrator::new();

    migrator.load_from_dir("./m2")?;
    migrator.migrate(&connection.get())?;

    // sqlx::migrate!()
    // .run(&pool)
    // .await
    // .inspect_err(|err| error!("Failed to run migration: {err}"))?;

    Ok((pool,connection))
}

/// Get the platform specific data directory.
pub fn get_data_dir() -> Option<PathBuf> {
    let base_dirs = directories::BaseDirs::new()?;
    let data_dir = base_dirs.data_dir();

    #[cfg(any(windows, target_os = "macos"))]
    let app_name = "Folio";

    // Trying to match Linux conventions
    #[cfg(target_os = "linux")]
    let app_name = "folio";

    Some(data_dir.join(app_name))
}

/// Creates an in memory sqlite database for testing.
#[cfg(test)]
fn create_test_db() -> crate::Result<rusqlite::Connection>{
    let conn = rusqlite::Connection::open_in_memory()?;
    conn.execute("PRAGMA foreign_keys = ON", ())?;

    let mut migrator = Migrator::new();

    migrator.load_from_dir("./m2")?;
    migrator.migrate(&conn)?;

    Ok(conn)
}

#[derive(Clone)]
pub struct SqliteConnection{
    connection: Arc<std::sync::Mutex<rusqlite::Connection>>
}

// TODO: add in-memory
impl SqliteConnection{
    pub fn open(path: impl AsRef<Path>) -> crate::Result<Self>{
        // TODO: use WAL journal mode
        let conn = rusqlite::Connection::open(&path)?;

        conn.execute("PRAGMA foreign_keys = ON", ())?;

        Ok(Self{
            connection: Arc::new(std::sync::Mutex::new(conn))
        })
    }

    /// Returns a reference to the sqlite connection.
    pub fn get(&self) -> MutexGuard<rusqlite::Connection> {
        self.connection.lock().unwrap()
    }
}
