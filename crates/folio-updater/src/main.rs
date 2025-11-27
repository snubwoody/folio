use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::info;

/// Release information generated on each new
/// release to be used when updating Folio.
#[derive(Debug, Serialize, Deserialize)]
pub struct ReleaseInfo {
    version: String, // TODO: get cargo metadata version
    notes: String,
    pub_date: String,
    platforms: HashMap<Target, Platform>,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Serialize, Deserialize, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Target {
    #[serde(rename = "linux-x86_64")]
    Linux_x86_64,
    #[serde(rename = "windows-x86_64")]
    Windows_x86_64,
    #[serde(rename = "darwin-aarch64")]
    Darwin_AArch64,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Platform {
    /// The content of the generated `.sig` file, a path or URL does not work.
    signature: String,
    url: String,
}

fn gen_release_info() -> anyhow::Result<()> {
    let url = "https://github.com/repos/snubwoody/folio/releases/latest/download/Folio";
    let response = reqwest::blocking::get(url)?;
    // let text = response.text()?;
    dbg!(&response);
    Ok(())
}

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .without_time()
        .with_target(false)
        .with_file(false)
        .init();
    info!("Generating release info");
    gen_release_info()?;
    Ok(())
}
