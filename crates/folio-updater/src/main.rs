use anyhow::anyhow;
use chrono::{DateTime, Utc};
use octocrab::models::repos::Release;
use serde::Serialize;
use std::collections::HashMap;
use std::fs;
use tracing::info;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt().init();
    release_info().await?;
    Ok(())
}

#[derive(Serialize, Debug, Default)]
struct ReleaseJson {
    version: String,
    notes: String,
    pub_date: DateTime<Utc>,
    platforms: HashMap<String, Platform>,
}

#[derive(Serialize, Debug, Default)]
struct Platform {
    signature: String,
    url: String,
}

// TODO: add is updatable
/// Generates a `release-info.json` file containing information about the
/// latest release.
pub async fn release_info() -> anyhow::Result<()> {
    // TODO: filter drafts
    let release = octocrab::instance()
        .repos("snubwoody", "folio")
        .releases()
        .get_latest()
        .await?;

    // TODO: log version?
    info!("Fetched latest release from github");

    // TODO: if it fails leave it as null
    // TODO: add macos
    let mut json = ReleaseJson {
        version: release.tag_name.clone(),
        notes: release.body.clone().unwrap_or_default(),
        pub_date: release.published_at.unwrap(),
        ..Default::default()
    };
    if let Ok((url, signature)) = platform_info("-setup.exe", &release).await {
        let platform = Platform { signature, url };
        json.platforms.insert("windows-x86_64".to_owned(), platform);
        info!("Found windows exe release");
    }
    if let Ok((url, signature)) = platform_info(".AppImage", &release).await {
        let platform = Platform { signature, url };
        json.platforms.insert("linux-x86_64".to_owned(), platform);
        info!("Found linux AppImage release");
    }
    if let Ok((url, signature)) = platform_info(".app.tar.gz", &release).await {
        let platform = Platform { signature, url };
        json.platforms.insert("darwin-aarch64".to_owned(), platform);
        info!("Found macos app release");
    }

    fs::write("release-info.json", serde_json::to_string_pretty(&json)?)?;
    info!("Generated release info");
    Ok(())
}

/// Returns the download url and signature of the latest platform release
async fn platform_info(pattern: &str, release: &Release) -> anyhow::Result<(String, String)> {
    let exe = &release
        .assets
        .iter()
        .find(|a| a.name.ends_with(pattern))
        .ok_or(anyhow!("pattern {pattern} not found"))?;
    let exe_sig = &release
        .assets
        .iter()
        .find(|a| a.name.ends_with(&format!("{pattern}.sig")))
        .ok_or(anyhow!("pattern {pattern} not found"))?;

    let signature = reqwest::get(exe_sig.browser_download_url.as_str())
        .await?
        .text()
        .await?;
    let download_url = exe.browser_download_url.to_string();

    Ok((download_url, signature))
}
