use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;

#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq, PartialOrd)]
pub struct IncomeStream {
    pub id: String,
    pub title: String,
}

impl IncomeStream {
    pub async fn create(title: &str, pool: &SqlitePool) -> crate::Result<Self> {
        let record = sqlx::query!(
            "INSERT INTO income_streams(title) VALUES($1) RETURNING id",
            title
        )
        .fetch_one(pool)
        .await?;

        IncomeStream::from_id(&record.id, pool).await
    }

    pub async fn from_id(id: &str, pool: &SqlitePool) -> crate::Result<Self> {
        let income_stream = sqlx::query_as!(IncomeStream, "SELECT * FROM income_streams WHERE id=$1", id)
            .fetch_one(pool)
            .await?;

        Ok(income_stream)
    }
}

/// Fetch all the income streams from the database.
pub async fn fetch_income_streams(pool: &SqlitePool) -> Result<Vec<IncomeStream>, crate::Error> {
    let records = sqlx::query!("SELECT id FROM income_streams")
        .fetch_all(pool)
        .await?;

    let mut streams = vec![];
    for record in records {
        let income_stream = IncomeStream::from_id(&record.id, pool).await?;
        streams.push(income_stream);
    }
    Ok(streams)
}

#[cfg(test)]
mod test {
    use super::*;

    #[sqlx::test]
    async fn get_categories(pool: SqlitePool) -> Result<(), crate::Error> {
        let rows = sqlx::query!("SELECT id FROM income_streams")
            .fetch_all(&pool)
            .await?;
        IncomeStream::create("", &pool).await?;
        IncomeStream::create("", &pool).await?;
        IncomeStream::create("", &pool).await?;
        let categories = fetch_income_streams(&pool).await?;
        assert_eq!(categories.len(), rows.len() + 3);
        Ok(())
    }

    #[sqlx::test]
    async fn fetch_income_stream(pool: SqlitePool) -> crate::Result<()> {
        let record = sqlx::query!("INSERT INTO income_streams(title) VALUES('Rent') RETURNING id")
            .fetch_one(&pool)
            .await?;

        let category = IncomeStream::from_id(&record.id, &pool).await?;
        assert_eq!(category.title, "Rent");
        Ok(())
    }

    #[sqlx::test]
    async fn create_income_stream(pool: SqlitePool) -> crate::Result<()> {
        let stream = IncomeStream::create("my---stream", &pool).await?;
        let record = sqlx::query!("SELECT title FROM income_streams WHERE id=$1", stream.id)
            .fetch_one(&pool)
            .await
            .unwrap();

        assert_eq!(record.title, "my---stream");
        Ok(())
    }
}
