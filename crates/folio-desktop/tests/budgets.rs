use folio_lib::Result;
use folio_lib::service::CategoryService;
use sqlx::SqlitePool;

#[sqlx::test]
async fn filter_deleted_budgets(pool: SqlitePool) -> Result<()> {
    let service = CategoryService::new(pool.clone());
    let c = service.create_category("").await?;
    let result = service.fetch_budget_from_category(&c.id).await;
    assert!(result.is_ok());
    service.delete_category(&c.id).await?;
    let budgets = service.fetch_budgets().await?;
    assert!(budgets.is_empty());
    Ok(())
}
