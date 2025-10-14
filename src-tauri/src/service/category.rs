use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;

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

    pub async fn from_id(id: &str, pool: &SqlitePool) -> crate::Result<Self>{
        let category = sqlx::query_as!(Category, "SELECT * FROM categories WHERE id=$1", id)
            .fetch_one(pool)
            .await?;

        Ok(category)
    }
}

pub async fn fetch_categories(pool: &SqlitePool) -> Result<Vec<Category>,crate::Error> {
    let records = sqlx::query!("SELECT id FROM categories")
        .fetch_all(pool)
        .await?;

    let mut categories = vec![];
    for record in records{
        let category = Category::from_id(&record.id, pool).await?;
        categories.push(category);
    }
    Ok(categories)
}

#[cfg(test)]
mod test {
    use super::*;

    #[sqlx::test]
    async fn get_categories(pool: SqlitePool) -> Result<(),crate::Error> {
        let rows = sqlx::query!("SELECT id FROM categories").fetch_all(&pool).await?;
        Category::create("", &pool).await?;
        Category::create("", &pool).await?;
        Category::create("", &pool).await?;
        let categories = fetch_categories(&pool).await?;
        assert_eq!(categories.len(),rows.len() + 3);
        Ok(())
    }

    #[sqlx::test]
    async fn fetch_category(pool: SqlitePool) -> crate::Result<()>{
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
