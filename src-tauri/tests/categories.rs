use chrono::NaiveDate;
use folio_lib::service::{Category, CreateExpense, Expense};
use folio_lib::{Money, Result};
use sqlx::SqlitePool;

#[sqlx::test]
async fn delete_category(pool: SqlitePool) -> Result<()> {
    let category = Category::create("__", &pool).await?;
    Category::delete(&category.id, &pool).await?;

    let record = sqlx::query!("SELECT * FROM categories WHERE id=$1", category.id)
        .fetch_optional(&pool)
        .await?;

    assert!(record.is_none());
    Ok(())
}

#[sqlx::test]
async fn delete_category_from_expense(pool: SqlitePool) -> Result<()> {
    let category = Category::create("__", &pool).await?;
    let data = CreateExpense {
        category_id: Some(category.id.clone()),
        ..Default::default()
    };
    let expense = Expense::create(data, &pool).await?;
    Category::delete(&category.id, &pool).await?;
    dbg!(&expense);

    let expense = Expense::from_id(&expense.id, &pool).await?;
    dbg!(&expense);
    Ok(())
}
