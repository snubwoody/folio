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

// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tracing::error;
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{EnvFilter, fmt};

#[tokio::main]
async fn main() {
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
        .expect("Failed to setup logging");

    if let Err(err) = folio_lib::run().await {
        error!("{}", err.report())
    }
}
