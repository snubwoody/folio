mod bundle;
mod download;
mod manifest;

use crate::bundle::{bundle_package, create_package};
use crate::download::download_windows_sdk;
use crate::manifest::{AppxManifest, VisualElements};
use anyhow::{Context, Ok};
use clap::{Parser, Subcommand};
use glob::glob;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use tempfile::tempdir;
use tracing::info;

#[derive(Parser)]
struct Cli {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Bundle an msix package
    Bundle {
        /// The path to the config file
        #[arg(short = 'c', long)]
        config: Option<PathBuf>,
        /// The path of the final msix package
        #[arg(short = 'o', long)]
        output: Option<PathBuf>,
    },
}

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .without_time()
        .with_file(false)
        .init();

    let cli = Cli::parse();

    match cli.command {
        Command::Bundle { config, output } => {
            let config = config.unwrap_or(PathBuf::from("msixpack.toml"));
            let output = output.unwrap_or(PathBuf::from("package.msix"));
            bundle(config, output)?;
        }
    }

    Ok(())
}

fn bundle(config: impl AsRef<Path>, output: impl AsRef<Path>) -> anyhow::Result<()> {
    info!("Bundling package");
    let config_path = config.as_ref();
    let output_path = output.as_ref();
    let config = Config::from_path(config_path)?;
    let temp = tempdir().with_context(|| "Failed to create temporary output directory")?;
    let temp_dir = temp.path();
    let dest = temp_dir.join(".msixpack");

    validate_windows_toolkit()?;

    fs::create_dir_all(&dest).with_context(|| "Failed to create temporary directory")?;
    create_package(&config, &dest).with_context(|| "Failed to create package")?;
    bundle_package(dest, &output).with_context(|| "Failed to bundle package")?;

    info!("Created package: {output_path:?}");
    Ok(())
}

/// Installs the windows toolkit if it is not found on the system.
fn validate_windows_toolkit() -> anyhow::Result<()> {
    if !toolkit_exists()? {
        let data_dir = data_dir();
        download_windows_sdk(&data_dir)?;
    }
    Ok(())
}

/// Returns true if the windows toolkit is installed.
fn toolkit_exists() -> anyhow::Result<bool> {
    let data_dir = data_dir();
    let exe_path = data_dir.join("windows-toolkit/makeappx.exe");
    Ok(exe_path.exists())
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Config {
    /// The path of the configuration file.
    #[serde(skip, default)]
    directory: PathBuf,
    package: Package,
    application: Application,
}

impl Config {
    pub fn from_path(path: impl AsRef<Path>) -> anyhow::Result<Config> {
        let path = path.as_ref();
        let bytes = fs::read(path)
            .with_context(|| format!("Failed to read configuration file from {path:?}"))?;
        let mut config: Config =
            toml::from_slice(&bytes).with_context(|| "Failed to parse config".to_string())?;
        config.directory = path
            .parent()
            .unwrap() // FIXME
            .to_path_buf();

        Ok(config)
    }

    pub fn create_manifest(&self) -> AppxManifest {
        let mut manifest = AppxManifest::new();

        manifest.identity.version = self.package.version.clone();
        manifest.identity.name = self.package.name.clone();
        manifest.identity.processor_architecture = "x64".to_owned();
        manifest.identity.publisher = self.package.publisher.to_owned();

        manifest.properties.logo = self.package.logo.to_owned();
        manifest.properties.display_name = self.package.display_name.to_owned();
        manifest.properties.publisher_display_name = self.package.publisher_name.to_owned();

        manifest
            .applications
            .applications
            .push(self.create_application());
        manifest
    }

    fn exe_path(&self) -> String {
        self.application
            .executable
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .to_owned()
    }

    fn create_application(&self) -> manifest::Application {
        manifest::Application {
            id: self.application.id.clone(),
            executable: self.exe_path(),
            entry_point: String::from("Windows.FullTrustApplication"),
            visual_elements: VisualElements {
                display_name: self.application.display_name.to_owned(),
                description: self.application.description.to_owned(),
                background_color: String::from("transparent"),
                square_44_logo: self.package.logo.to_owned(),
                square_150_logo: self.package.logo.to_owned(),
            },
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "kebab-case")]
struct Package {
    /// The name of the package.
    name: String,
    display_name: String,
    publisher_name: String,
    publisher: String,
    version: String,
    logo: String,
    /// A series of glob paths.
    resources: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "kebab-case")]
struct Application {
    id: String,
    display_name: String,
    description: String,
    executable: PathBuf,
}


fn data_dir() -> PathBuf {
    dirs::data_dir()
        .unwrap_or(PathBuf::from("."))
        .join("msixpack")
}

fn read_metadata() -> anyhow::Result<(String,String)>{
    let metadata = cargo_metadata::MetadataCommand::new()
        .exec()
        .with_context(||"Failed to read metadata")?;
    let local_projects: Vec<cargo_metadata::Package> = metadata.packages
        .into_iter()
        .filter(|p|p.source.is_none())
        .collect();

    // TODO: get first package or specify with -p,--package
    let project = &local_projects[0];
    let mut version = project.version.to_string();
    let exe_path = metadata.target_directory.join("release/folio.exe").to_string();
    version.push_str(".0");
    dbg!(project.version.to_string());
    dbg!(&local_projects[0].name);
    dbg!(&metadata.target_directory.join("release/folio.exe"));

    Ok((version,exe_path))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn get_data_dir() {
        let dir = data_dir();
        let data_dir = dirs::data_dir().unwrap();
        assert_eq!(dir, data_dir.join("msixpack"));
    }

    #[test]
    fn metadata(){
        let metadata = cargo_metadata::MetadataCommand::new().exec().unwrap();
        let local_projects: Vec<cargo_metadata::Package> = metadata.packages
            .into_iter()
            .filter(|p|p.source.is_none())
            .collect();
        // TODO: get first package or specify with -p,--package
        let project = &local_projects[0];
        let mut version = project.version.to_string();
        version.push_str(".0");
        dbg!(project.version.to_string());
        dbg!(&local_projects[0].name);
        dbg!(&metadata.target_directory.join("release/folio.exe"));
        // TODO: kind includes Bin
        // dbg!(&local_projects[0].targets);
        // dbg!(&local_projects.len());
        // dbg!(&metadata.packages[0].name);
    }
}
