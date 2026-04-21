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
mod date;
pub mod db;
pub mod error;
mod money;
pub mod service;
mod settings;

use crate::settings::Settings;
pub use error::{Error, Result};
pub use money::Money;
use rusqlite::Connection;
use sqlx::SqlitePool;
use sqlx::sqlite::SqliteConnectOptions;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tauri::{App, WebviewUrl, WebviewWindowBuilder};
use tokio::sync::Mutex;
use tracing::{error, info};
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};

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
pub async fn run() {
    #[cfg(debug_assertions)]
    let log_dir = "logs";
    #[cfg(not(debug_assertions))]
    let log_dir = get_data_dir()
        .expect("failed to get data directory")
        .join("logs");

    let file_appender = RollingFileAppender::builder()
        .rotation(Rotation::DAILY)
        .filename_prefix("folio")
        .max_log_files(10)
        .filename_suffix("log")
        .build(log_dir)
        .expect("Failed to setup logging");

    // Keep guard in scope
    let (file_writer, _guard) = tracing_appender::non_blocking(file_appender);

    let std_io_layer = fmt::layer().with_writer(std::io::stdout);

    let file_layer = fmt::layer()
        .pretty()
        .with_writer(file_writer)
        .with_ansi(false);

    tracing_subscriber::registry()
        .with(EnvFilter::new("info,folio=debug"))
        .with(std_io_layer)
        .with(file_layer)
        .try_init()
        .unwrap();

    let state = State::new().await.expect("Failed to initialise state");

    let app = tauri::Builder::default()
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_prevent_default::init())
        .manage(state)
        .setup(setup_app)
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_updater::Builder::new().build());

    let app = command::handlers(app);

    app.run(tauri::generate_context!())
        .inspect_err(|err| tracing::error!("{err}"))
        .expect("error while running application");
}

#[derive(Clone)]
pub struct State {
    pool: SqlitePool,
    settings: Arc<Mutex<Settings>>,
    // TODO: might not need Arc<Mutex>
    connection: Arc<std::sync::Mutex<Connection>>,
}

impl State {
    pub async fn new() -> Result<Self> {
        // FIXME: check if foreign keys are on
        // TODO: check WAL and journal mode
        let pool = init_database().await?;
        info!("Initialised database pool");

        // TODO: global test connection
        let connection = Connection::open("data.db").unwrap();

        #[cfg(debug_assertions)]
        let mut path = PathBuf::from(".");
        // FIXME: return error
        #[cfg(not(debug_assertions))]
        let mut path = get_data_dir().expect("failed to get data directory");

        path = path.join("settings.json");

        let settings = Settings::open(path)?;
        Ok(Self {
            pool,
            settings: Arc::new(Mutex::new(settings)),
            connection: Arc::new(std::sync::Mutex::new(connection)),
        })
    }
}

pub async fn setup_test_db(path: impl AsRef<Path>) -> Connection {
    let connection = Connection::open(&path).unwrap();

    let opts = SqliteConnectOptions::new()
        .filename(&path)
        .create_if_missing(true);

    // Temporary solution
    let pool = SqlitePool::connect_with(opts).await.unwrap();

    sqlx::migrate!().run(&pool).await.unwrap();

    connection
}

// TODO: use prepared statements
pub async fn init_database() -> Result<SqlitePool> {
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

    sqlx::migrate!()
        .run(&pool)
        .await
        .inspect_err(|err| error!("Failed to run migration: {err}"))?;

    Ok(pool)
}

/// Get the platform specific data directory.
pub fn get_data_dir() -> Option<PathBuf> {
    // TODO: add message at startup on fail
    let base_dirs = directories::BaseDirs::new()?;
    let data_dir = base_dirs.data_dir();

    #[cfg(any(windows, target_os = "macos"))]
    let app_name = "Folio";

    // Trying to match linux conventions
    #[cfg(target_os = "linux")]
    let app_name = "folio";

    Some(data_dir.join(app_name))
}
