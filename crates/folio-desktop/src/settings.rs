use crate::error::ErrorExt;
use iso_currency::Currency;
use serde::{Deserialize, Serialize};
use std::fs::{File, OpenOptions};
use std::path::{Path, PathBuf};
use tracing::info;

// Serde doesn't allow constant values e.g. true
const fn default_true() -> bool {
    true
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Settings {
    #[serde(skip)]
    path: PathBuf,
    currency_code: Currency,
    #[serde(default = "default_true")]
    sidebar_open: bool,
}

impl Settings {
    pub fn open(path: impl AsRef<Path>) -> crate::Result<Self> {
        let file = OpenOptions::new()
            .write(true)
            .read(true)
            .create(true)
            .truncate(false)
            .open(&path)?;

        match serde_json::from_reader::<_, Settings>(file) {
            Ok(mut settings) => {
                settings.path = path.as_ref().to_path_buf();

                // Make sure new settings fields are saved
                settings.write()?;
                info!("Loaded settings from {:?}", path.as_ref());
                Ok(settings)
            }
            Err(_) => Self::init(path),
        }
    }

    fn init(path: impl AsRef<Path>) -> crate::Result<Self> {
        let settings = Settings {
            path: path.as_ref().to_path_buf(),
            currency_code: Currency::USD,
            sidebar_open: true,
        };

        let file = File::create(&path)?;
        serde_json::to_writer_pretty(file, &settings)?;
        info!(path=?path.as_ref(),"Created settings file");
        Ok(settings)
    }

    pub fn set_currency_code(&mut self, currency: Currency) -> crate::Result<()> {
        self.currency_code = currency;
        self.write()?;
        info!(currency=?currency,"Set currency code");
        Ok(())
    }

    /// Sets the `sidebar_open` setting.
    pub fn set_sidebar_state(&mut self, open: bool) -> crate::Result<()> {
        self.sidebar_open = open;
        self.write()?;
        info!(open = open, "Set sidebar state");
        Ok(())
    }

    pub fn currency_code(&self) -> Currency {
        self.currency_code
    }

    fn write(&self) -> crate::Result<()> {
        let file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(&self.path)?;
        serde_json::to_writer_pretty(file, &self)
            .context(format!("Failed to write to settings to {:?}", &self.path))?;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use serde_json::json;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn truncate_write() -> crate::Result<()> {
        let dir = tempdir()?;
        let path = dir.path().join("settings.json");
        File::create(&path)?;
        let settings = Settings {
            path: path.to_path_buf(),
            currency_code: Currency::AED,
            sidebar_open: false,
        };
        settings.write()?;
        let settings = Settings {
            path: path.to_path_buf(),
            currency_code: Currency::AED,
            sidebar_open: true,
        };
        settings.write()?;
        let settings: Settings = serde_json::from_str(&fs::read_to_string(&path)?)?;
        assert_eq!(settings.currency_code, Currency::AED);
        Ok(())
    }

    #[test]
    fn init_settings() -> crate::Result<()> {
        let dir = tempdir()?;
        let path = dir.path().join("settings.json");
        Settings::init(&path)?;
        let file = File::open(&path)?;
        let settings: Settings = serde_json::from_reader(file)?;
        assert_eq!(settings.currency_code, Currency::USD);
        Ok(())
    }

    #[test]
    fn open_settings_file() -> crate::Result<()> {
        let dir = tempdir()?;
        let path = dir.path().join("settings.json");
        let file = File::create(&path)?;
        let json = json! ({
            "currencyCode":"XOF",
            "sidebarOpen":false
        });
        serde_json::to_writer(file, &json)?;
        let settings: Settings = Settings::open(&path)?;
        assert_eq!(settings.currency_code, Currency::XOF);
        assert!(!settings.sidebar_open);
        assert_eq!(settings.path, path);
        Ok(())
    }

    #[test]
    fn open_partial_settings() -> crate::Result<()> {
        let dir = tempdir()?;
        let path = dir.path().join("settings.json");
        let file = File::create(&path)?;
        serde_json::to_writer(file, &json! ({"currencyCode":"XOF"}))?;
        let settings: Settings = Settings::open(&path)?;
        assert_eq!(settings.currency_code, Currency::XOF);
        assert!(settings.sidebar_open);
        assert_eq!(settings.path, path);
        Ok(())
    }

    #[test]
    fn write_to_file() -> crate::Result<()> {
        let dir = tempdir()?;
        let path = dir.path().join("settings.json");
        File::create(&path)?;
        let settings = Settings {
            path: path.clone(),
            currency_code: Currency::ZMW,
            sidebar_open: false,
        };
        settings.write()?;
        let settings = Settings::open(&path)?;
        assert_eq!(settings.currency_code, Currency::ZMW);
        assert!(!settings.sidebar_open);
        Ok(())
    }

    #[test]
    fn open_or_init_settings_file() -> crate::Result<()> {
        let dir = tempdir()?;
        let path = dir.path().join("settings.json");
        let settings: Settings = Settings::open(&path)?;
        assert!(fs::exists(&path)?);
        assert_eq!(settings.currency_code, Currency::USD);
        assert_eq!(settings.path, path);
        Ok(())
    }
}
