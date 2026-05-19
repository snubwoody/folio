use chrono::Utc;
use folio_lib::service::{AccountService, CategoryGroup, CategoryService, TransactionService};
use folio_lib::{Money, Result, SqliteConnection};
use sqlx::SqlitePool;

#[sqlx::test]
async fn total_spent(pool: SqlitePool) -> Result<()> {
    let connection = SqliteConnection::open(pool.connect_options().get_filename())?;
    // let connection = SqliteConnection::open(pool.connect_options().get_filename())?;

    let category_service = CategoryService::new(connection.clone());
    let account_service = AccountService::new(pool.clone());
    let transaction_service = TransactionService::new(connection.clone());

    let category = category_service.create_category("")?;
    let account = account_service.create_account("", Money::ZERO).await?;

    transaction_service
        .expense()
        .account_id(&account.id)
        .amount(Money::from_unscaled(100))
        .category(&category.id)
        .create(&connection.get())?;

    transaction_service
        .expense()
        .account_id(&account.id)
        .amount(Money::from_unscaled(20))
        .category(&category.id)
        .create(&connection.get())?;

    let total = category_service.total_spent(&category.id)?;
    assert_eq!(total, Money::from_unscaled(120));
    Ok(())
}

#[sqlx::test]
async fn get_categories(pool: SqlitePool) -> Result<()> {
    let connection = SqliteConnection::open(pool.connect_options().get_filename())?;
    let category_service = CategoryService::new(connection);
    let rows = sqlx::query!("SELECT id FROM categories")
        .fetch_all(&pool)
        .await?;
    category_service.create_category("")?;
    category_service.create_category("")?;
    category_service.create_category("")?;
    let categories = category_service.fetch_categories()?;
    assert_eq!(categories.len(), rows.len() + 3);
    Ok(())
}

#[sqlx::test]
async fn create_income_stream(pool: SqlitePool) -> crate::Result<()> {
    let connection = SqliteConnection::open(pool.connect_options().get_filename())?;
    let category_service = CategoryService::new(connection);
    let now = Utc::now().timestamp();
    let category = category_service.create_income_stream("Ent")?;
    let record = sqlx::query!("SELECT * FROM categories WHERE id=$1", category.id)
        .fetch_one(&pool)
        .await?;

    assert!(record.created_at.unwrap() >= now);
    assert_eq!(record.title, "Ent");
    assert!(record.is_income_stream.unwrap());
    Ok(())
}

#[sqlx::test]
async fn create_category(pool: SqlitePool) -> folio_lib::Result<()> {
    let connection = SqliteConnection::open(pool.connect_options().get_filename())?;
    let service = CategoryService::new(connection);
    let now = Utc::now().timestamp();
    let category = service.create_category("Ent")?;
    let record = sqlx::query!("SELECT * FROM categories WHERE id=$1", category.id)
        .fetch_one(&pool)
        .await?;

    assert!(record.created_at.unwrap() >= now);
    assert_eq!(record.title, "Ent");
    assert!(!record.is_income_stream.unwrap());
    Ok(())
}

#[sqlx::test]
async fn fetch_category(pool: SqlitePool) -> folio_lib::Result<()> {
    let connection = SqliteConnection::open(pool.connect_options().get_filename())?;
    let service = CategoryService::new(connection);
    let record = sqlx::query!("INSERT INTO categories(title) VALUES('Rent') RETURNING id")
        .fetch_one(&pool)
        .await?;

    let category = service.fetch_category(&record.id)?;
    assert_eq!(category.title, "Rent");
    Ok(())
}

#[sqlx::test]
async fn get_category_group(pool: SqlitePool) -> Result<()> {
    let connection = SqliteConnection::open(pool.connect_options().get_filename())?;
    let service = CategoryService::new(connection);

    let row: CategoryGroup =
        sqlx::query_as("INSERT INTO category_groups(title) VALUES('Subscriptions') RETURNING *")
            .fetch_one(&pool)
            .await?;
    let group = service.fetch_group(&row.id)?;
    assert_eq!(group.title, "Subscriptions");
    Ok(())
}

#[sqlx::test]
async fn create_category_group(pool: SqlitePool) -> Result<()> {
    let connection = SqliteConnection::open(pool.connect_options().get_filename())?;
    let service = CategoryService::new(connection);
    let row = service.create_group("Wants")?;
    let group = service.fetch_group(&row.id)?;
    assert_eq!(group.title, "Wants");
    Ok(())
}

#[sqlx::test]
async fn edit_category_group_title(pool: SqlitePool) -> Result<()> {
    let connection = SqliteConnection::open(pool.connect_options().get_filename())?;
    let service = CategoryService::new(connection);
    let row = service.create_group("Wants")?;
    let group = service.set_group_title(&row.id, "Needs")?;
    assert_eq!(group.title, "Needs");
    Ok(())
}

#[sqlx::test]
async fn delete_category_group(pool: SqlitePool) -> Result<()> {
    let connection = SqliteConnection::open(pool.connect_options().get_filename())?;
    let service = CategoryService::new(connection);
    let row = service.create_group("Wants")?;
    service.delete_group(&row.id)?;

    let result = sqlx::query("SELECT * FROM category_groups WHERE id=$1")
        .bind(row.id)
        .fetch_optional(&pool)
        .await?;

    assert!(result.is_none());
    Ok(())
}

#[sqlx::test]
async fn soft_delete_category(pool: SqlitePool) -> Result<()> {
    let connection = SqliteConnection::open(pool.connect_options().get_filename())?;
    let service = CategoryService::new(connection);
    let category = service.create_category("__")?;
    service.delete_category(&category.id)?;
    let c = service.fetch_category(&category.id)?;
    assert!(c.deleted_at.is_some());
    Ok(())
}

#[sqlx::test]
async fn create_budgets_for_categories(pool: SqlitePool) -> Result<()> {
    let connection = SqliteConnection::open(pool.connect_options().get_filename())?;
    let service = CategoryService::new(connection);
    let c = service.create_category_raw("__")?;
    let result = service.fetch_budget_from_category(&c.id);
    assert!(result.is_err());
    service.create_missing_budgets()?;
    let result = service.fetch_budget_from_category(&c.id);
    assert!(result.is_ok());
    Ok(())
}

#[sqlx::test]
async fn create_budget_after_category(pool: SqlitePool) -> Result<()> {
    let connection = SqliteConnection::open(pool.connect_options().get_filename())?;
    let service = CategoryService::new(connection);
    let category = service.create_category("__")?;

    let record = sqlx::query!("SELECT * FROM budgets WHERE category_id=$1", category.id)
        .fetch_optional(&pool)
        .await?;

    assert!(record.is_some());
    Ok(())
}

#[sqlx::test]
async fn edit_category(pool: SqlitePool) -> Result<()> {
    let connection = SqliteConnection::open(pool.connect_options().get_filename())?;
    let service = CategoryService::new(connection);
    let category = service.create_category("__")?;
    let category = service.edit_category(&category.id, "__MINE__")?;

    let record = sqlx::query!("SELECT * FROM categories WHERE id=$1", category.id)
        .fetch_one(&pool)
        .await?;

    assert_eq!(record.title, "__MINE__");
    Ok(())
}

#[sqlx::test]
async fn fetch_categories_not_deleted(pool: SqlitePool) -> Result<()> {
    let connection = SqliteConnection::open(pool.connect_options().get_filename())?;
    let service = CategoryService::new(connection);
    let len1 = service.fetch_categories()?.len();
    service.create_category("title")?;
    let c = service.create_category("title")?;
    service.delete_category(&c.id)?;
    let len2 = service.fetch_categories()?.len();
    assert_eq!(len1 + 1, len2);
    Ok(())
}
