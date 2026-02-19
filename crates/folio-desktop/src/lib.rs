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
pub mod db;
mod error;
mod money;
pub mod service;
mod settings;

use crate::settings::Settings;
pub use error::{Error, Result};
pub use money::Money;
use sqlx::SqlitePool;
use sqlx::sqlite::SqliteConnectOptions;
use std::path::PathBuf;
use std::sync::Arc;
use tauri::{App, WebviewUrl, WebviewWindowBuilder};
use tokio::sync::Mutex;
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};

// TODO: test this
fn setup_app(app: &mut App) -> std::result::Result<(), Box<dyn std::error::Error>> {
    let builder = WebviewWindowBuilder::new(app, "main", WebviewUrl::default())
        .title("Folio")
        .resizable(true)
        .maximized(true);

    // Use a custom title bar, only on windows
    #[cfg(windows)]
    let builder = builder.decorations(false);

    // FIXME: don't unwrap
    builder.build().unwrap();
    Ok(())
}

// TODO: log migrations
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub async fn run() {
    // TODO: add database backup before migrating

    #[cfg(debug_assertions)]
    let log_dir = "logs";
    #[cfg(not(debug_assertions))]
    let log_dir = get_data_dir().unwrap().join("logs");

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
    let state = State::new().await.unwrap();
    let app = tauri::Builder::default()
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_os::init())
        .manage(state)
        .setup(setup_app)
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_updater::Builder::new().build());

    let app = command::handlers(app);

    app.run(tauri::generate_context!())
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
        tracing::info!("Initialised database pool");

        #[cfg(debug_assertions)]
        let mut path = PathBuf::from(".");
        #[cfg(not(debug_assertions))]
        let mut path = get_data_dir().unwrap();

        path = path.join("settings.json");

        let settings = Settings::open(path)?;
        Ok(Self {
            pool,
            settings: Arc::new(Mutex::new(settings)),
        })
    }
}

// TODO: run this after opening the app
pub async fn init_database() -> Result<SqlitePool> {
    #[cfg(debug_assertions)]
    let opts = SqliteConnectOptions::new()
        .filename("./data.db") // FIXME
        .create_if_missing(true);

    #[cfg(not(debug_assertions))]
    let opts = {
        let data_dir = get_data_dir().unwrap();
        std::fs::create_dir_all(&data_dir)?;
        let data_dir = data_dir.join("data.db");

        SqliteConnectOptions::new()
            .filename(data_dir)
            .create_if_missing(true)
    };

    let pool = SqlitePool::connect_with(opts).await?;

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

    // Trying to match linux conventions
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
