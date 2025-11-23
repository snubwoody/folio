use crate::{Config, copy_executable, copy_resources, data_dir};
use anyhow::{Context, bail};
use std::fs;
use std::path::Path;
use std::process::Command;
use tracing::info;

/// Uses the `makeappx.exe` tool to create an msix package.
pub fn bundle_package(dir: impl AsRef<Path>, dest: impl AsRef<Path>) -> anyhow::Result<()> {
    let data_dir = data_dir();
    let exe_path = data_dir.join("windows-toolkit/makeappx.exe");
    // TODO: set env var
    let package = dir.as_ref().to_str().unwrap();
    let output = dest.as_ref().to_str().unwrap();
    // TODO: test existing packages
    let output = Command::new(&exe_path)
        .args(["pack", "/d", package, "/p", output, "/o"])
        .output()?;

    let stdout = String::from_utf8_lossy(&output.stdout);

    // TODO: improve error message
    // Just show the error to the user
    if !output.status.success() {
        bail!("{stdout}");
    }
    Ok(())
}

/// Creates an msix package in the `dest` directory
pub fn create_package(config: &Config, dest: impl AsRef<Path>) -> anyhow::Result<()> {
    info!("Copying assets into package directory");
    copy_executable(config, &dest)
        .with_context(|| "Failed to copy executable to destination directory")?;
    copy_resources(config, &dest)
        .with_context(|| "Failed to copy resources to destination directory")?;
    let manifest = config.create_manifest();
    let xml = quick_xml::se::to_string(&manifest)?;
    fs::write(dest.as_ref().join("appxmanifest.xml"), &xml)?;
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{Application, Config, Package, validate_windows_toolkit};
    use std::path::PathBuf;
    use tempfile::tempdir;

    #[test]
    #[cfg(windows)]
    fn bundle_msix() -> anyhow::Result<()> {
        validate_windows_toolkit()?;
        let config = Config {
            package: Package {
                display_name: "Test".to_owned(),
                publisher: "CN=Test".to_owned(),
                version: "1.0.0.0".to_owned(),
                name: "Test".to_owned(),
                publisher_name: "Test".to_owned(),
                logo: "logo.png".to_owned(),
                ..Default::default()
            },
            application: Application {
                id: "Test".to_owned(),
                executable: PathBuf::from("main.exe"),
                display_name: "Test".to_owned(),
                description: String::from("A test app"),
            },
            ..Default::default()
        };
        let manifest = config.create_manifest();
        let temp = tempdir()?;
        let dir = temp.path();
        let dest = dir.join("out.msix");
        let xml = quick_xml::se::to_string(&manifest)?;

        fs::write(dir.join("logo.png"), xml.as_bytes())?;
        fs::write(dir.join("appxmanifest.xml"), &xml)?;
        fs::write(dir.join("main.exe"), "")?;

        bundle_package(dir, &dest)?;
        assert!(dest.exists());
        Ok(())
    }
}
