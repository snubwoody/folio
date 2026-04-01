use crate::{Config, data_dir};
use anyhow::{Context, bail};
use glob::glob;
use std::fs;
use std::path::Path;
use std::process::Command;
use tracing::info;

/// Uses the `makeappx.exe` tool to create an msix file.
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

/// Creates an msix manifest and package in the `dest` directory
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

fn copy_executable(config: &Config, dest: impl AsRef<Path>) -> anyhow::Result<()> {
    let dest = dest.as_ref();
    let exe_path = config
        .package_info
        .path
        .join(&config.application.executable);
    dbg!(&exe_path);
    let exe = config.application.executable.file_name().unwrap();
    fs::copy(exe_path, dest.join(exe))?;
    Ok(())
}

/// Copy all the resources defined in the [`Config`] to the destination directory.
fn copy_resources(config: &Config, dest: impl AsRef<Path>) -> anyhow::Result<()> {
    let dir = &config.package_info.path;
    for pattern in &config.package.resources {
        let path = dir.join(pattern);
        for entry in glob(path.to_str().unwrap())? {
            let entry = entry?;
            let base_path = entry.strip_prefix(dir)?;

            if entry.is_dir() {
                continue;
            }
            let output = dest.as_ref().join(base_path);

            fs::create_dir_all(output.parent().unwrap())
                .with_context(|| "Failed to create directory")?;
            fs::copy(&entry, &output).with_context(|| {
                format!(
                    "Failed to copy file {:?} to {:?}",
                    entry.to_str().unwrap(),
                    output.to_str().unwrap()
                )
            })?;
        }
    }
    Ok(())
}

#[cfg(all(test, windows))]
mod test {
    use super::*;
    use crate::{Application, Config, Package, PackageInfo, validate_windows_toolkit};
    use std::{fs::File, path::PathBuf};
    use tempfile::tempdir;

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
            // directory: dir.path().to_path_buf(),
            package: Package {
                resources: vec!["icons/*.png".to_string()],
                ..Default::default()
            },
            package_info: PackageInfo {
                path: dir.path().to_path_buf(),
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

    #[test]
    fn bundle_msix() -> anyhow::Result<()> {
        let temp = tempdir()?;
        let dir = temp.path();
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
            package_info: PackageInfo {
                version: "1.0.0.0".to_string(),
                name: "main".to_string(),
                path: dir.to_path_buf(),
                ..Default::default()
            },
        };
        let manifest = config.create_manifest();
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
