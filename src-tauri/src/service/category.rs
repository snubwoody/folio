use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;

#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq, PartialOrd)]
pub struct Category {
    pub id: String,
    pub title: String,
}

impl Category {
    pub async fn create(title: &str, pool: &SqlitePool) -> Self {
        let record = sqlx::query!(
            "INSERT INTO categories(title) VALUES($1) RETURNING id",
            title
        )
        .fetch_one(pool)
        .await
        .unwrap();

        Category::from_id(&record.id, pool).await
    }

    pub async fn from_id(id: &str, pool: &SqlitePool) -> Self {
        sqlx::query_as!(Category, "SELECT * FROM categories WHERE id=$1", id)
            .fetch_one(pool)
            .await
            .unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[sqlx::test]
    async fn fetch_category(pool: SqlitePool) {
        let record = sqlx::query!("INSERT INTO categories(title) VALUES('Rent') RETURNING id")
            .fetch_one(&pool)
            .await
            .unwrap();

        let category = Category::from_id(&record.id, &pool).await;
        assert_eq!(category.title, "Rent");
    }

    #[sqlx::test]
    async fn create_category(pool: SqlitePool) {
        let category = Category::create("Ent", &pool).await;
        let record = sqlx::query!("SELECT title FROM categories WHERE id=$1", category.id)
            .fetch_one(&pool)
            .await
            .unwrap();

        assert_eq!(record.title, "Ent");
    }
}
