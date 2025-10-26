use std::fs::{File, OpenOptions};
use std::path::{Path, PathBuf};
use iso_currency::Currency;
use serde::{Serialize, Deserialize};
use sqlx::SqlitePool;

#[derive(Debug,Clone,Serialize,Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Settings{
    #[serde(skip)]
    path: PathBuf,
    currency_code: Currency,
}

impl Settings{
    pub fn open(path: impl AsRef<Path>) -> crate::Result<Self> {
        let file = OpenOptions::new()
            .write(true)
            .read(true)
            .create(true)
            .open(&path)?;

        match serde_json::from_reader::<_,Settings>(file) {
            Ok(mut settings) => {
                settings.path = path.as_ref().to_path_buf();
                Ok(settings)
            }
            Err(_e) => Self::init(path),
        }
    }

    fn init(path: impl AsRef<Path>) -> crate::Result<Self> {
        let settings = Settings{
            path: path.as_ref().to_path_buf(),
            currency_code: Currency::USD,
        };

        let file = File::create(&path)?;
        serde_json::to_writer_pretty(file, &settings)?;
        Ok(settings)
    }

    pub fn set_currency_code(&mut self, currency: Currency) -> crate::Result<()> {
        self.currency_code = currency;
        self.write()?;
        Ok(())
    }

    pub fn currency_code(&self) -> Currency {
        self.currency_code
    }

    fn write(&self,) -> crate::Result<()> {
        let file = File::open(&self.path)?;
        serde_json::to_writer_pretty(file, &self)?;
        Ok(())
    }
}

pub async fn set_currency_code(currency: Currency,pool: &SqlitePool, settings: &mut Settings) -> crate::Result<()> {
    settings.set_currency_code(currency)?;
    let code = currency.to_string();
    sqlx::query!("UPDATE expenses SET currency_code=$1",code)
        .execute(pool).await?;
    sqlx::query!("UPDATE incomes SET currency_code=$1",code)
        .execute(pool).await?;
    Ok(())
}

#[cfg(test)]
mod test{
    use std::fs;
    use serde_json::json;
    use tempfile::tempdir;
    use super::*;

    #[test]
    fn init_settings() -> crate::Result<()>{
        let dir = tempdir()?;
        let path = dir.path()
            .join("settings.json");
        Settings::init(&path)?;
        let file = File::open(&path)?;
        let settings:Settings = serde_json::from_reader(file)?;
        assert_eq!(settings.currency_code, Currency::USD);
        Ok(())
    }

    #[test]
    fn open_settings_file() -> crate::Result<()>{
        let dir = tempdir()?;
        let path = dir.path()
            .join("settings.json");
        let file = File::create(&path)?;
        serde_json::to_writer(file, &json! ({"currencyCode":"XOF"}))?;
        let settings:Settings = Settings::open(&path)?;
        assert_eq!(settings.currency_code, Currency::XOF);
        assert_eq!(settings.path, path);
        Ok(())
    }

    #[test]
    fn open_or_init_settings_file() -> crate::Result<()>{
        let dir = tempdir()?;
        let path = dir.path()
            .join("settings.json");
        let settings:Settings = Settings::open(&path)?;
        assert!(fs::exists(&path)?);
        assert_eq!(settings.currency_code, Currency::USD);
        assert_eq!(settings.path, path);
        Ok(())
    }
}