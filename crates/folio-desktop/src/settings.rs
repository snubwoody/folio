use iso_currency::Currency;
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
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

    fn write(&self) -> crate::Result<()> {
        let file = OpenOptions::new().write(true).open(&self.path)?;
        serde_json::to_writer_pretty(file, &self)?;
        Ok(())
    }
}

pub async fn set_currency_code(
    currency: Currency,
    pool: &SqlitePool,
    settings: &mut Settings,
) -> crate::Result<()> {
    settings.set_currency_code(currency)?;
    let code = currency.code();
    // TODO: remove currency_code field
    sqlx::query("UPDATE expenses SET currency_code=$1")
        .bind(code)
        .execute(pool)
        .await?;
    sqlx::query("UPDATE incomes SET currency_code=$1")
        .bind(code)
        .execute(pool)
        .await?;
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::service::{Expense, Income};
    use serde_json::json;
    use std::fs;
    use tempfile::tempdir;

    #[sqlx::test]
    async fn update_transactions(pool: SqlitePool) -> crate::Result<()> {
        let dir = tempdir()?;
        let path = dir.path().join("settings.json");

        let expense = Expense::create(Default::default(), &pool).await?;
        let income = Income::create(Default::default(), &pool).await?;
        let mut settings = Settings::open(&path)?;
        set_currency_code(Currency::ZMW, &pool, &mut settings).await?;

        let income = Income::from_id(&income.id, &pool).await?;
        let expense = Expense::from_id(&expense.id, &pool).await?;
        assert_eq!(expense.currency_code, "ZMW");
        assert_eq!(income.currency_code, "ZMW");
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
