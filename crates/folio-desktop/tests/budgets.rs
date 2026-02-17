use folio_lib::Result;
use folio_lib::service::{
    Budget, Category, fetch_budgets,
};
use sqlx::SqlitePool;

#[sqlx::test]
async fn filter_deleted_budgets(pool: SqlitePool) -> Result<()> {
    let c = Category::create("",&pool).await?;
    let result = Budget::from_category(&c.id, &pool).await;
    assert!(result.is_ok());
    Category::delete(&c.id,&pool).await?;
    let budgets = fetch_budgets(&pool).await?;
    assert!(budgets.is_empty());
    Ok(())
}