use iso_currency::Currency;
use serde::{Deserialize, Serialize};
use tracing::info;
use std::fs::{File, OpenOptions};
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Settings {
    #[serde(skip)]
    path: PathBuf,
    currency_code: Currency,
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
                Ok(settings)
            }
            Err(_e) => Self::init(path),
        }
    }

    fn init(path: impl AsRef<Path>) -> crate::Result<Self> {
        let settings = Settings {
            path: path.as_ref().to_path_buf(),
            currency_code: Currency::USD,
        };

        let file = File::create(&path)?;
        serde_json::to_writer_pretty(file, &settings)?;
        info!(path=?path.as_ref(),"Created settings file");
        Ok(settings)
    }
    
    pub fn set_currency_code(&mut self, currency: Currency) -> crate::Result<()> {
        self.currency_code = currency;
        self.write()?;
        info!(currency=?currency,"Updated currency code");
        Ok(())
    }

    pub fn currency_code(&self) -> Currency {
        self.currency_code
    }

    fn write(&self) -> crate::Result<()> {
        let file = OpenOptions::new().write(true).open(&self.path)?;
        serde_json::to_writer_pretty(file, &self)?;
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
        serde_json::to_writer(file, &json! ({"currencyCode":"XOF"}))?;
        let settings: Settings = Settings::open(&path)?;
        assert_eq!(settings.currency_code, Currency::XOF);
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
        };
        settings.write()?;
        let settings = Settings::open(&path)?;
        assert_eq!(settings.currency_code, Currency::ZMW);
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
