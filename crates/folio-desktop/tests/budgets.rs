use folio_lib::Result;
use folio_lib::service::{Budget, Category, CategoryService, fetch_budgets};
use sqlx::SqlitePool;

#[sqlx::test]
async fn filter_deleted_budgets(pool: SqlitePool) -> Result<()> {
    let service = CategoryService::new(pool.clone());
    let c = service.create_category("").await?;
    let result = Budget::from_category(&c.id, &pool).await;
    assert!(result.is_ok());
    service.delete_category(&c.id).await?;
    let budgets = fetch_budgets(&pool).await?;
    assert!(budgets.is_empty());
    Ok(())
}
