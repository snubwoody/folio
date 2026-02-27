use std::fs;
use octocrab::models::repos::Release;
use serde_json::json;
use tracing::info;

// {
// "version": "",
// "notes": "",
// "pub_date": "",
// "platforms": {
// "linux-x86_64": {
// "signature": "",
// "url": ""
// },
// "windows-x86_64": {
// "signature": "",
// "url": ""
// },
// "darwin-x86_64": {
// "signature": "",
// "url": ""
// }
// }
// }

// TODO: add is updatable
/// Generates a `release-info.json` file containing information about the
/// latest release.
pub async fn release_info() -> crate::Result<()>{
    // TODO: filter drafts
    let release = octocrab::instance()
        .repos("snubwoody", "folio")
        .releases()
        .get_latest()
        .await
        .unwrap();

    // TODO: log version?
    info!("Fetched latest release from github");

    // TODO: if it fails leave it as null
    let version = release.tag_name.clone();
    let notes = release.body.clone().unwrap_or_default();
    let (windows_url,windows_sig) = windows_release_info(&release).await?;
    let date = release.published_at.unwrap();

    let json = json!({
        "version":version,
        "notes":notes,
        "pub_date":date,
        "platforms":{
            "windows-x86_64": {
              "signature": windows_sig,
              "url": windows_url
            },
        },
    });
    fs::write("release-info.json",serde_json::to_string_pretty(&json)?)?;
    info!("Generated release info");
    Ok(())
}

/// Returns the download url and signature of the latest windows release
async fn windows_release_info(release: &Release) -> crate::Result<(String,String)>{
    let exe = &release.assets.iter().find(|a|a.name.ends_with("-setup.exe")).unwrap();
    let exe_sig = &release.assets.iter().find(|a|a.name.ends_with("-setup.exe.sig")).unwrap();

    let signature = reqwest::get(exe_sig.browser_download_url.as_str())
        .await?
        .text()
        .await?;
    let download_url = exe.browser_download_url.to_string();

    Ok((download_url,signature))
}

/// Returns the download url and signature of the latest windows release
async fn macos_release_info(release: &Release) -> crate::Result<(String,String)>{
    let exe = &release.assets.iter().find(|a|a.name.ends_with("-setup.exe")).unwrap();
    let exe_sig = &release.assets.iter().find(|a|a.name.ends_with("-setup.exe.sig")).unwrap();

    let signature = reqwest::get(exe_sig.browser_download_url.as_str())
        .await?
        .text()
        .await?;
    let download_url = exe.browser_download_url.to_string();

    Ok((download_url,signature))
}
