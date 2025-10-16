use chrono::{Datelike, Local};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;

use crate::service::fetch_expenses;

#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq, PartialOrd)]
pub struct Category {
    pub id: String,
    pub title: String,
}

impl Category {
    pub async fn create(title: &str, pool: &SqlitePool) -> crate::Result<Self> {
        let record = sqlx::query!(
            "INSERT INTO categories(title) VALUES($1) RETURNING id",
            title
        )
        .fetch_one(pool)
        .await?;

        Category::from_id(&record.id, pool).await
    }

    pub async fn from_id(id: &str, pool: &SqlitePool) -> crate::Result<Self> {
        let category = sqlx::query_as!(Category, "SELECT * FROM categories WHERE id=$1", id)
            .fetch_one(pool)
            .await?;

        Ok(category)
    }

    /// Get the total amount spent in the month for the [`Category`].
    pub async fn total_spent(id: &str, pool: &SqlitePool) -> crate::Result<Decimal> {
        let now = Local::now();
        let total = fetch_expenses(pool)
            .await?
            .iter()
            .filter(|e| e.category.is_some())
            .filter(|e| {
                e.category.as_ref().unwrap().id == id
                    && e.date.year() == now.year()
                    && e.date.month() == now.month()
            })
            .fold(Decimal::ZERO, |acc, e| e.amount + acc);
        Ok(total)
    }
}

pub async fn fetch_categories(pool: &SqlitePool) -> Result<Vec<Category>, crate::Error> {
    let records = sqlx::query!("SELECT id FROM categories")
        .fetch_all(pool)
        .await?;

    let mut categories = vec![];
    for record in records {
        let category = Category::from_id(&record.id, pool).await?;
        categories.push(category);
    }
    Ok(categories)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::service::{CreateExpense, Expense};
    use rust_decimal::dec;

    #[sqlx::test]
    async fn total_spent(pool: SqlitePool) -> crate::Result<()> {
        let category = Category::create("", &pool).await?;
        let mut data = CreateExpense {
            amount: dec!(20).to_string(),
            category_id: Some(category.id.clone()),
            ..Default::default()
        };

        Expense::create(data.clone(), &pool).await?;
        data.amount = dec!(100).to_string();
        Expense::create(data.clone(), &pool).await?;

        let total = Category::total_spent(&category.id, &pool).await?;
        assert_eq!(total, dec!(120));
        Ok(())
    }

    #[sqlx::test]
    async fn get_categories(pool: SqlitePool) -> Result<(), crate::Error> {
        let rows = sqlx::query!("SELECT id FROM categories")
            .fetch_all(&pool)
            .await?;
        Category::create("", &pool).await?;
        Category::create("", &pool).await?;
        Category::create("", &pool).await?;
        let categories = fetch_categories(&pool).await?;
        assert_eq!(categories.len(), rows.len() + 3);
        Ok(())
    }

    #[sqlx::test]
    async fn fetch_category(pool: SqlitePool) -> crate::Result<()> {
        let record = sqlx::query!("INSERT INTO categories(title) VALUES('Rent') RETURNING id")
            .fetch_one(&pool)
            .await?;

        let category = Category::from_id(&record.id, &pool).await?;
        assert_eq!(category.title, "Rent");
        Ok(())
    }

    #[sqlx::test]
    async fn create_category(pool: SqlitePool) -> crate::Result<()> {
        let category = Category::create("Ent", &pool).await?;
        let record = sqlx::query!("SELECT title FROM categories WHERE id=$1", category.id)
            .fetch_one(&pool)
            .await
            .unwrap();

        assert_eq!(record.title, "Ent");
        Ok(())
    }
}
