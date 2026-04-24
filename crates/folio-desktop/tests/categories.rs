use chrono::Utc;
use folio_lib::service::{
    AccountService, Budget, CategoryGroup, CategoryService, Transaction, create_missing_budgets,
};
use folio_lib::{Money, Result};
use sqlx::SqlitePool;

#[sqlx::test]
async fn total_spent(pool: SqlitePool) -> Result<()> {
    let account_service = AccountService::new(pool.clone());
    let category_service = CategoryService::new(pool.clone());
    let category = category_service.create_category("").await?;
    let account = account_service.create_account("", Money::ZERO).await?;
    Transaction::expense()
        .account_id(&account.id)
        .amount(Money::from_unscaled(100))
        .category(&category.id)
        .create(&pool)
        .await?;

    Transaction::expense()
        .account_id(&account.id)
        .amount(Money::from_unscaled(20))
        .category(&category.id)
        .create(&pool)
        .await?;

    let total = category_service.total_spent(&category.id).await?;
    assert_eq!(total, Money::from_unscaled(120));
    Ok(())
}

#[sqlx::test]
async fn get_categories(pool: SqlitePool) -> Result<()> {
    let category_service = CategoryService::new(pool.clone());
    let rows = sqlx::query!("SELECT id FROM categories")
        .fetch_all(&pool)
        .await?;
    category_service.create_category("").await?;
    category_service.create_category("").await?;
    category_service.create_category("").await?;
    let categories = category_service.fetch_categories().await?;
    assert_eq!(categories.len(), rows.len() + 3);
    Ok(())
}

#[sqlx::test]
async fn create_income_stream(pool: SqlitePool) -> crate::Result<()> {
    let category_service = CategoryService::new(pool.clone());
    let now = Utc::now().timestamp();
    let category = category_service.create_income_stream("Ent").await?;
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
    let service = CategoryService::new(pool.clone());
    let now = Utc::now().timestamp();
    let category = service.create_category("Ent").await?;
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
    let service = CategoryService::new(pool.clone());
    let record = sqlx::query!("INSERT INTO categories(title) VALUES('Rent') RETURNING id")
        .fetch_one(&pool)
        .await?;

    let category = service.fetch_category(&record.id).await?;
    assert_eq!(category.title, "Rent");
    Ok(())
}

#[sqlx::test]
async fn get_category_group(pool: SqlitePool) -> Result<()> {
    let row: CategoryGroup =
        sqlx::query_as("INSERT INTO category_groups(title) VALUES('Subscriptions') RETURNING *")
            .fetch_one(&pool)
            .await?;
    let group = CategoryGroup::get(&row.id, &pool).await?;
    assert_eq!(group.title, "Subscriptions");
    Ok(())
}

#[sqlx::test]
async fn create_category_group(pool: SqlitePool) -> Result<()> {
    let row = CategoryGroup::create("Wants", &pool).await?;
    let group = CategoryGroup::get(&row.id, &pool).await?;
    assert_eq!(group.title, "Wants");
    Ok(())
}

#[sqlx::test]
async fn edit_category_group_title(pool: SqlitePool) -> Result<()> {
    let row = CategoryGroup::create("Wants", &pool).await?;
    CategoryGroup::set_title(&row.id, "Needs", &pool).await?;
    let group = CategoryGroup::get(&row.id, &pool).await?;
    assert_eq!(group.title, "Needs");
    Ok(())
}

#[sqlx::test]
async fn delete_category_group(pool: SqlitePool) -> Result<()> {
    let row = CategoryGroup::create("Wants", &pool).await?;
    CategoryGroup::delete(&row.id, &pool).await?;
    let result = sqlx::query("SELECT * FROM category_groups WHERE id=$1")
        .bind(row.id)
        .fetch_optional(&pool)
        .await?;

    assert!(result.is_none());
    Ok(())
}

#[sqlx::test]
async fn soft_delete_category(pool: SqlitePool) -> Result<()> {
    let service = CategoryService::new(pool.clone());
    let category = service.create_category("__").await?;
    service.delete_category(&category.id).await?;
    let c = service.fetch_category(&category.id).await?;
    assert!(c.deleted_at.is_some());
    Ok(())
}

#[sqlx::test]
async fn create_budgets_for_categories(pool: SqlitePool) -> Result<()> {
    let service = CategoryService::new(pool.clone());
    let c = service.create_category_raw("__").await?;
    let result = Budget::from_category(&c.id, &pool).await;
    assert!(result.is_err());
    create_missing_budgets(&pool).await?;
    let result = Budget::from_category(&c.id, &pool).await;
    assert!(result.is_ok());
    Ok(())
}

#[sqlx::test]
async fn create_budget_after_category(pool: SqlitePool) -> Result<()> {
    let service = CategoryService::new(pool.clone());
    let category = service.create_category("__").await?;

    let record = sqlx::query!("SELECT * FROM budgets WHERE category_id=$1", category.id)
        .fetch_optional(&pool)
        .await?;

    assert!(record.is_some());
    Ok(())
}

#[sqlx::test]
async fn edit_category(pool: SqlitePool) -> Result<()> {
    let service = CategoryService::new(pool.clone());
    let category = service.create_category("__").await?;
    let category = service.edit_category(&category.id, "__MINE__").await?;

    let record = sqlx::query!("SELECT * FROM categories WHERE id=$1", category.id)
        .fetch_one(&pool)
        .await?;

    assert_eq!(record.title, "__MINE__");
    Ok(())
}

#[sqlx::test]
async fn fetch_categories_not_deleted(pool: SqlitePool) -> crate::Result<()> {
    let service = CategoryService::new(pool.clone());
    let len1 = service.fetch_categories().await?.len();
    service.create_category("title").await?;
    let c = service.create_category("title").await?;
    service.delete_category(&c.id).await?;
    let len2 = service.fetch_categories().await?.len();
    assert!(len1 + 1 == len2);
    Ok(())
}
