use folio_lib::service::CategoryService;
use folio_lib::{Result, SqliteConnection};
use sqlx::SqlitePool;

#[sqlx::test]
async fn filter_deleted_budgets(pool: SqlitePool) -> Result<()> {
    let connection = SqliteConnection::open(pool.connect_options().get_filename())?;
    let service = CategoryService::new(connection);
    let c = service.create_category("")?;
    let result = service.fetch_budget_from_category(&c.id);
    assert!(result.is_ok());
    service.delete_category(&c.id)?;
    let budgets = service.fetch_budgets()?;
    assert!(budgets.is_empty());
    Ok(())
}
