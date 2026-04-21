use folio_lib::service::{Budget, Category, fetch_budgets};
use folio_lib::{Result, setup_test_db};
use sqlx::SqlitePool;

#[sqlx::test]
async fn filter_deleted_budgets(pool: SqlitePool) -> Result<()> {
    let conn = setup_test_db(pool.connect_options().get_filename()).await;
    let c = Category::create("", &conn)?;
    let result = Budget::from_category(&c.id, &conn);
    assert!(result.is_ok());
    Category::delete(&c.id, &conn)?;
    let budgets = fetch_budgets(&pool).await?;
    assert!(budgets.is_empty());
    Ok(())
}
