use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use anyhow::{bail, Context};
use crate::data_dir;

/// Calls the `makeappx.exe` tool to create an msix package.
pub fn bundle_package(dir: impl AsRef<Path>,dest: impl AsRef<Path>) -> anyhow::Result<()> {
    let data_dir = data_dir();
    let exe_path = data_dir.join("windows-toolkit/makeappx.exe");
    // TODO: set env var
    let package = dir.as_ref().to_str().unwrap();
    let output = dest.as_ref().to_str().unwrap();
    // TODO: test existing packages
    let output = Command::new(&exe_path)
        .args(&["pack","/d",package,"/p",output,"/o"])
        .output()?;

    let stdout = String::from_utf8_lossy(&output.stdout);

    // TODO: improve error message
    if !output.status.success() {
        bail!("{stdout}");
    }
    Ok(())
}

#[cfg(test)]
mod test{
    use tempfile::tempdir;
    use crate::{Application, Config, Package};
    use super::*;

    #[test]
    fn bundle_msix() -> anyhow::Result<()> {
        let config = Config{
            package: Package{
                display_name: "Test".to_owned(),
                publisher: "CN=Test".to_owned(),
                version: "1.0.0.0".to_owned(),
                name: "Test".to_owned(),
                publisher_name: "Test".to_owned(),
                logo: "logo.png".to_owned(),
                ..Default::default()
            },
            application: Application{
                id: "Test".to_owned(),
                executable: PathBuf::from("main.exe"),
                display_name: "Test".to_owned(),
                description: String::from("A test app")
            },
            ..Default::default()
        };
        let manifest = config.create_manifest();
        let temp = tempdir()?;
        let dir = temp.path();
        let dest = dir.join("out.msix");
        let xml = quick_xml::se::to_string(&manifest)?;

        fs::write(&dir.join("logo.png"), xml.as_bytes())?;
        fs::write(&dir.join("appxmanifest.xml"), &xml)?;
        fs::write(&dir.join("main.exe"), "")?;

        bundle_package(&dir,&dest)?;
        assert!(dest.exists());
        Ok(())
    }
}