use folio_lib::Result;
use folio_lib::service::{Category, CreateExpense, CreateIncome, Expense, Income, IncomeStream};
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
async fn delete_category_with_budget(pool: SqlitePool) -> Result<()> {
    let category = Category::create("__", &pool).await?;
    // FIXME: add deleted_at or is_deleted column
    // let budget = Budget::create(Default::default(), &category.id, &pool).await?;
    // Category::delete(&category.id, &pool).await?;

    // let record = sqlx::query!("SELECT * FROM budgets WHERE id=$1", budget.id)
    //     .fetch_optional(&pool)
    //     .await?;

    // assert!(record.is_none());
    Ok(())
}

#[sqlx::test]
async fn delete_income_stream(pool: SqlitePool) -> Result<()> {
    let stream = IncomeStream::create("__", &pool).await?;
    IncomeStream::delete(&stream.id, &pool).await?;

    let record = sqlx::query!("SELECT * FROM income_streams WHERE id=$1", stream.id)
        .fetch_optional(&pool)
        .await?;

    assert!(record.is_none());
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
async fn edit_income_stream(pool: SqlitePool) -> Result<()> {
    let stream = IncomeStream::create("__", &pool).await?;
    let stream = IncomeStream::edit(&stream.id, "__MINE__", &pool).await?;

    let record = sqlx::query!("SELECT * FROM income_streams WHERE id=$1", stream.id)
        .fetch_one(&pool)
        .await?;

    assert_eq!(record.title, "__MINE__");
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
    assert!(expense.category.is_some());
    Category::delete(&category.id, &pool).await?;

    let expense = Expense::from_id(&expense.id, &pool).await?;
    assert!(expense.category.is_none());
    Ok(())
}

#[sqlx::test]
async fn delete_used_income_stream(pool: SqlitePool) -> Result<()> {
    let stream = IncomeStream::create("__", &pool).await?;
    let data = CreateIncome {
        income_stream_id: Some(stream.id.clone()),
        ..Default::default()
    };
    let income = Income::create(data, &pool).await?;
    assert!(income.income_stream.is_some());
    IncomeStream::delete(&stream.id, &pool).await?;

    let income = Income::from_id(&income.id, &pool).await?;
    assert!(income.income_stream.is_none());
    Ok(())
}
