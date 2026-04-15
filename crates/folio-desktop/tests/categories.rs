use folio_lib::Result;
use folio_lib::service::{Budget, Category, CategoryGroup, create_missing_budgets};
use sqlx::SqlitePool;

#[sqlx::test]
async fn get_category_group(pool: SqlitePool) -> Result<()> {
    let row: CategoryGroup = sqlx::query_as("INSERT INTO category_groups(title) VALUES('Subscriptions') RETURNING *")
        .fetch_one(&pool)
        .await?;
    let group = CategoryGroup::get(&row.id, &pool).await?;
    assert_eq!(group.title,"Subscriptions");
    Ok(())
}

#[sqlx::test]
async fn create_category_group(pool: SqlitePool) -> Result<()> {
    let row = CategoryGroup::create("Wants", &pool).await?;
    let group = CategoryGroup::get(&row.id, &pool).await?;
    assert_eq!(group.title,"Wants");
    Ok(())
}

#[sqlx::test]
async fn edit_category_group_title(pool: SqlitePool) -> Result<()> {
    let row = CategoryGroup::create("Wants", &pool).await?;
    CategoryGroup::set_title(&row.id, "Needs", &pool).await?;
    let group = CategoryGroup::get(&row.id, &pool).await?;
    assert_eq!(group.title,"Needs");
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
    let category = Category::create("__", &pool).await?;
    Category::delete(&category.id, &pool).await?;
    let c = Category::from_id(&category.id, &pool).await?;
    assert!(c.deleted_at.is_some());
    Ok(())
}

#[sqlx::test]
async fn create_budgets_for_categories(pool: SqlitePool) -> Result<()> {
    let c = Category::create_raw("__", &pool).await?;
    let result = Budget::from_category(&c.id, &pool).await;
    assert!(result.is_err());
    create_missing_budgets(&pool).await?;
    let result = Budget::from_category(&c.id, &pool).await;
    assert!(result.is_ok());
    Ok(())
}

#[sqlx::test]
async fn create_budget_after_category(pool: SqlitePool) -> Result<()> {
    let category = Category::create("__", &pool).await?;

    let record = sqlx::query!("SELECT * FROM budgets WHERE category_id=$1", category.id)
        .fetch_optional(&pool)
        .await?;

    assert!(record.is_some());
    Ok(())
}

#[sqlx::test]
async fn edit_category(pool: SqlitePool) -> Result<()> {
    let category = Category::create("__", &pool).await?;
    let category = Category::edit(&category.id, "__MINE__", &pool).await?;

    let record = sqlx::query!("SELECT * FROM categories WHERE id=$1", category.id)
        .fetch_one(&pool)
        .await?;

    assert_eq!(record.title, "__MINE__");
    Ok(())
}

#[sqlx::test]
async fn fetch_categories_not_deleted(pool: SqlitePool) -> crate::Result<()> {
    let len1 = Category::fetch_categories(&pool).await?.len();
    Category::create("title", &pool).await?;
    let c = Category::create("title", &pool).await?;
    Category::delete(&c.id, &pool).await?;
    let len2 = Category::fetch_categories(&pool).await?.len();
    assert!(len1 + 1 == len2);
    Ok(())
}
