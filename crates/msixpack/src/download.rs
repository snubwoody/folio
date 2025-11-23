use anyhow::Context;
use std::fs;
use std::io::Cursor;
use std::path::Path;
use tracing::info;

const WINDOWS_TOOLKIT_URL: &str =
    "https://github.com/microsoft/MSIX-Toolkit/archive/refs/tags/v2.0.zip";

pub fn download_windows_sdk(path: impl AsRef<Path>) -> anyhow::Result<()> {
    info!("Downloading windows toolkit");
    let response = reqwest::blocking::get(WINDOWS_TOOLKIT_URL)
        .with_context(|| "Failed to fetch windows toolkit")?
        .bytes()?;
    let cursor = Cursor::new(response);
    let mut archive = zip::ZipArchive::new(cursor)?;
    archive.extract(&path)?;

    info!("Extracting toolkit to {:?}", path.as_ref());
    // Extract only the required exe and header files
    extract_sdk(path)?;
    Ok(())
}

/// Extract the sdk files
pub fn extract_sdk(path: impl AsRef<Path>) -> anyhow::Result<()> {
    let path = path.as_ref().to_path_buf();
    let toolkit_path = path.join("MSIX-Toolkit-2.0/WindowsSDK/11/10.0.22000.0/x64");
    let out = path.join("windows-toolkit");
    fs::create_dir_all(&out)?;
    for entry in fs::read_dir(&toolkit_path)? {
        let entry = entry?;
        let file_name = entry.file_name();
        let out_file = out.join(file_name);
        fs::copy(entry.path(), out_file)?;
    }

    // Delete the old toolkit folder
    fs::remove_dir_all(path.join("MSIX-Toolkit-2.0"))
        .with_context(|| "Failed to remove old windows toolkit")?;
    Ok(())
}
