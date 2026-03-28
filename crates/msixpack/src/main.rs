mod bundle;
mod download;
mod manifest;

use crate::bundle::{bundle_package, create_package};
use crate::download::download_windows_sdk;
use crate::manifest::{AppxManifest, VisualElements};
use anyhow::{Context, Ok};
use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
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
    Bundle,
}

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .without_time()
        .with_file(false)
        .init();

    let cli = Cli::parse();

    match cli.command {
        Command::Bundle => {
            bundle()?;
        }
    }

    Ok(())
}

fn bundle() -> anyhow::Result<()> {
    info!("Bundling package");
    let config = Config::from_path()?;
    let temp = tempdir().with_context(|| "Failed to create temporary output directory")?;
    let temp_dir = temp.path();
    let dest = temp_dir.join(".msixpack");

    let mut file_name = config.file_name();
    file_name.push_str(".msix");
    let output = config
        .package_info
        .target_dir
        .join("release/bundle/msix")
        .join(file_name);
    validate_windows_toolkit()?;

    fs::create_dir_all(&dest).with_context(|| "Failed to create temporary directory")?;
    create_package(&config, &dest).with_context(|| "Failed to create package")?;
    bundle_package(dest, &output).with_context(|| "Failed to bundle package")?;

    info!("Created package: {output:?}");
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

/// Capitalize the first letter of a string.
fn capitalize(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
        None => String::new(),
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
struct ConfigFile {
    package: Package,
    application: Application,
}

#[derive(Default)]
pub struct Config {
    /// The path of the configuration file.
    package: Package,
    application: Application,
    package_info: PackageInfo,
}

impl Config {
    pub fn from_path() -> anyhow::Result<Config> {
        let package_info = read_metadata()?;
        let manifest_path = package_info.path.join("msixpack.toml");
        let bytes = fs::read(&manifest_path)
            .with_context(|| format!("Failed to read configuration file from {manifest_path:?}"))?;
        let config_file: ConfigFile =
            toml::from_slice(&bytes).with_context(|| "Failed to parse config".to_string())?;

        let config = Config {
            package: config_file.package,
            application: config_file.application,
            package_info,
        };

        Ok(config)
    }

    pub fn file_name(&self) -> String {
        let name = capitalize(&self.package_info.name);
        format!("{name}_{}_x86", self.package_info.version)
    }

    pub fn create_manifest(&self) -> AppxManifest {
        let mut manifest = AppxManifest::new();

        // TODO: read from package.version then package_info
        manifest.identity.version = self.package_info.version.clone();
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
        let mut executable = self.package_info.name.clone();
        executable.push_str(".exe");
        executable
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

#[derive(Default)]
struct PackageInfo {
    name: String,
    version: String,
    target_dir: PathBuf,
    path: PathBuf,
}

fn read_metadata() -> anyhow::Result<PackageInfo> {
    // TODO: bundle into release/bundle/msix or release/msix/pack
    // TODO: include package version in name when bundling
    let metadata = cargo_metadata::MetadataCommand::new()
        .exec()
        .with_context(|| "Failed to read metadata")?;

    let local_projects: Vec<cargo_metadata::Package> = metadata
        .packages
        .into_iter()
        .filter(|p| p.source.is_none())
        .collect();

    // TODO: get first package or specify with -p,--package
    let project = &local_projects[0];
    let mut version = project.version.to_string();
    let path = project.manifest_path.parent().unwrap();
    version.push_str(".0");

    let info = PackageInfo {
        path: path.to_path_buf().into_std_path_buf(),
        name: project.name.to_string(),
        version,
        target_dir: metadata.target_directory.into_std_path_buf(),
    };

    Ok(info)
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
    fn metadata() {
        let metadata = cargo_metadata::MetadataCommand::new().exec().unwrap();
        let local_projects: Vec<cargo_metadata::Package> = metadata
            .packages
            .into_iter()
            .filter(|p| p.source.is_none())
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
