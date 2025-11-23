mod manifest;
mod bundle;
mod download;

use crate::manifest::{AppxManifest, VisualElements};
use anyhow::Context;
use glob::glob;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use clap::{Parser, Subcommand};
use tempfile::tempdir;
use tracing::info;
use crate::bundle::bundle_package;
use crate::download::download_windows_sdk;

#[derive(Parser)]
struct Cli{
    #[clap(subcommand)]
    command: Command
}

#[derive(Subcommand)]
enum Command{
    /// Bundle an msix package
    Bundle{
        /// The path to the config file
        #[arg(short='c',long)]
        config: Option<PathBuf>,
        /// The path of the final msix package
        #[arg(short='o',long)]
        output: Option<PathBuf>,
    }
}

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .without_time()
        .with_file(false)
        .init();
    let cli = Cli::parse();

    match cli.command {
        Command::Bundle{config, output} => {
            let config = config.unwrap_or(PathBuf::from("./msixpack.toml"));
            let output= output.unwrap_or(PathBuf::from("./package.msixpack"));
            bundle(config,output)?;
        }
    }

    Ok(())
}

fn bundle(config: impl AsRef<Path>,output: impl AsRef<Path>) -> anyhow::Result<()> {
    info!("Bundling package");
    // TODO:
    // Copy executable and resources
    // Create appxmanifest
    // Create msix package
    let config_path = config.as_ref();
    let output_path = output.as_ref();
    let config = Config::from_path(config_path)?;
    let temp = tempdir()
        .with_context(|| "Failed to create temporary output directory")?;
    let temp_dir = temp.path();
    let dest = temp_dir.join(".msixpack");

    validate_windows_toolkit()?;

    fs::create_dir_all(&dest)
        .with_context(|| "Failed to create temporary directory")?;
    create_package(&config, &dest)
        .with_context(|| "Failed to create package")?;
    bundle_package(dest,&output)
        .with_context(|| "Failed to bundle package")?;

    info!("Created package: {output_path:?}");
    Ok(())
}

/// Installs the windows toolkit if it is not found.
fn validate_windows_toolkit() -> anyhow::Result<()> {
    if !toolkit_exists()?{
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
        config.directory = path.parent()
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

    fn create_application(&self) -> manifest::Application {
        let app = manifest::Application {
            id: self.application.id.clone(),
            executable: self.application.executable.to_str().unwrap().to_owned(),
            entry_point: String::from("Windows.FullTrustApplication"),
            visual_elements: VisualElements {
                display_name: self.application.display_name.to_owned(),
                description: self.application.description.to_owned(),
                background_color: String::from("transparent"),
                square_44_logo: self.package.logo.to_owned(),
                square_150_logo: self.package.logo.to_owned(),
                ..Default::default()
            },
            ..Default::default()
        };

        app
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

/// Creates an msix package in the `dest` directory
fn create_package(config: &Config, dest: impl AsRef<Path>) -> anyhow::Result<()> {
    dbg!(&config, &dest.as_ref());
    info!("Copying assets into package directory");
    copy_executable(config, &dest)
        .with_context(|| "Failed to copy executable to destination directory")?;
    copy_resources(config, &dest)
        .with_context(|| "Failed to copy resources to destination directory")?;
    let manifest = config.create_manifest();
    let xml = quick_xml::se::to_string(&manifest)?;
    fs::write(&dest.as_ref().join("appxmanifest.xml"), &xml)?;
    Ok(())
}



fn copy_executable(config: &Config, dest: impl AsRef<Path>) -> anyhow::Result<()> {
    let dest = dest.as_ref();
    let exe_path = config.directory.join(&config.application.executable);
    let exe = config.application.executable.file_name().unwrap();
    // FIXME: put it in the root
    fs::copy(exe_path, dest.join(&exe))?;
    Ok(())
}

/// Copy all the resources defined in the [`Config`] to the destination directory.
fn copy_resources(config: &Config, dest: impl AsRef<Path>) -> anyhow::Result<()> {
    let dir = &config.directory;
    for pattern in &config.package.resources {
        let path = dir.join(pattern);
        for entry in glob(path.to_str().unwrap())? {
            let entry = entry?;
            let base_path = entry.strip_prefix(&dir)?;

            if entry.is_dir() {
                continue;
            }
            let output = dest.as_ref().join(base_path);

            fs::create_dir_all(&output.parent().unwrap())
                .with_context(|| "Failed to create directory")?;
            fs::copy(&entry, &output)
                .with_context(|| format!("Failed to copy file {:?} to {:?}",entry.to_str().unwrap(),output.to_str().unwrap()))?;
        }
    }
    Ok(())
}

fn data_dir() -> PathBuf {
    dirs::data_dir()
        .unwrap_or(PathBuf::from("."))
        .join("msixpack")
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;
    use tempfile::{tempdir};

    #[test]
    fn get_data_dir() {
        let dir = data_dir();
        let data_dir = dirs::data_dir().unwrap();
        assert_eq!(dir,data_dir.join("msixpack"));
    }

    #[test]
    fn create_identity() {
        let package = Package {
            name: String::from("Company.App"),
            version: "1.0.0.0".to_string(),
            logo: "img.png".to_string(),
            publisher: "CN=Company".to_string(),
            publisher_name: "Company".to_string(),
            ..Default::default()
        };
        let config = Config {
            package,
            ..Default::default()
        };
        let manifest = config.create_manifest();

        assert_eq!(manifest.identity.version, "1.0.0.0");
        assert_eq!(manifest.identity.processor_architecture, "x64");
        assert_eq!(manifest.identity.publisher, "CN=Company");
        assert_eq!(manifest.identity.name, "Company.App");
    }

    #[test]
    fn create_application() {
        let application = Application {
            id: String::from("ID"),
            executable: PathBuf::from("/bin/sh"),
            ..Default::default()
        };
        let config = Config {
            application,
            ..Default::default()
        };
        let manifest = config.create_manifest();

        let app = &manifest.applications.applications[0];
        assert_eq!(app.id, "ID");
        assert_eq!(app.executable, "/bin/sh");
        assert_eq!(app.entry_point, "Windows.FullTrustApplication");
    }

    #[test]
    fn copy_resources() -> anyhow::Result<()> {
        let dir = tempdir()?;
        let icons_dir = dir.path().join("icons");
        fs::create_dir(&icons_dir)?;
        let icon1 = dir.path().join("icons").join("icon1.png");
        let icon2 = dir.path().join("icons").join("icon2.png");
        File::create(&icon1)?;
        File::create(&icon2)?;
        let config = Config {
            directory: dir.path().to_path_buf(),
            package: Package {
                resources: vec!["icons/*.png".to_string()],
                ..Default::default()
            },
            ..Default::default()
        };
        let out = tempdir()?;
        super::copy_resources(&config, out.path())?;
        assert!(fs::exists(out.path().join("icons/icon1.png"))?);
        assert!(fs::exists(out.path().join("icons/icon2.png"))?);
        Ok(())
    }
}
