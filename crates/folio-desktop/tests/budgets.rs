use folio_lib::{Result, SqliteConnection};
use folio_lib::service::CategoryService;
use sqlx::SqlitePool;

#[sqlx::test]
async fn filter_deleted_budgets(pool: SqlitePool) -> Result<()> {
    let connection = SqliteConnection::open(pool.connect_options().get_filename())?;
    let service = CategoryService::new(pool.clone(),connection);
    let c = service.create_category("").await?;
    let result = service.fetch_budget_from_category(&c.id).await;
    assert!(result.is_ok());
    service.delete_category(&c.id)?;
    let budgets = service.fetch_budgets().await?;
    assert!(budgets.is_empty());
    Ok(())
}
