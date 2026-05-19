use chrono::Utc;
use folio_lib::service::{
    AccountService, Budget, Category, CategoryGroup, CategoryService, Transaction,
    TransactionService,
};
use folio_lib::{Money, Result, create_test_db};

#[test]
fn total_spent() -> Result<()> {
    let connection = create_test_db()?;
    let category_service = CategoryService::new(connection.clone());
    let account_service = AccountService::new(connection.clone());
    let transaction_service = TransactionService::new(connection.clone());

    let category = category_service.create_category("")?;
    let account = account_service.create_account("Expense", Money::ZERO)?;

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

#[test]
fn get_categories() -> Result<()> {
    let connection = create_test_db()?;
    let category_service = CategoryService::new(connection.clone());

    let rows = {
        let conn = connection.get();
        let mut stmt = conn.prepare_cached("select * from categories")?;
        stmt.query_map([], |row| Category::try_from(row))?
            .collect::<Vec<_>>()
    };

    category_service.create_category("")?;
    category_service.create_category("")?;
    category_service.create_category("")?;
    let categories = category_service.fetch_categories()?;
    assert_eq!(categories.len(), rows.len() + 3);
    Ok(())
}

#[test]
fn create_income_stream() -> crate::Result<()> {
    let connection = create_test_db()?;
    let category_service = CategoryService::new(connection.clone());
    let now = Utc::now();
    let category = category_service.create_income_stream("Ent")?;

    let conn = connection.get();
    let mut stmt = conn.prepare_cached("SELECT * FROM categories WHERE id=?")?;
    let record = stmt.query_row([category.id], |row| Category::try_from(row))?;

    assert!(record.created_at.unwrap() >= now);
    assert_eq!(record.title, "Ent");
    assert!(record.is_income_stream);
    Ok(())
}

#[test]
fn create_category() -> folio_lib::Result<()> {
    let connection = create_test_db()?;
    let service = CategoryService::new(connection.clone());
    let now = Utc::now();
    let category = service.create_category("Ent")?;

    let conn = connection.get();
    let mut stmt = conn.prepare_cached("SELECT * FROM categories WHERE id=?")?;
    let record = stmt.query_row([category.id], |row| Category::try_from(row))?;

    assert!(record.created_at.unwrap() >= now);
    assert_eq!(record.title, "Ent");
    assert!(!record.is_income_stream);
    Ok(())
}

#[test]
fn fetch_category() -> folio_lib::Result<()> {
    let connection = create_test_db()?;
    let service = CategoryService::new(connection.clone());
    let record = {
        let conn = connection.get();
        let mut stmt =
            conn.prepare_cached("INSERT INTO categories(title) VALUES('Rent') RETURNING *")?;
        stmt.query_row([], |row| Category::try_from(row))?
    };

    let category = service.fetch_category(&record.id)?;
    assert_eq!(category.title, "Rent");
    Ok(())
}

#[test]
fn get_category_group() -> Result<()> {
    let connection = create_test_db()?;
    let service = CategoryService::new(connection.clone());

    let row = {
        let conn = connection.get();
        let mut stmt = conn.prepare_cached(
            "INSERT INTO category_groups(title) VALUES('Subscriptions') RETURNING *",
        )?;
        stmt.query_row([], |row| CategoryGroup::try_from(row))?
    };
    let group = service.fetch_group(&row.id)?;
    assert_eq!(group.title, "Subscriptions");
    Ok(())
}

#[test]
fn create_category_group() -> Result<()> {
    let connection = create_test_db()?;
    let service = CategoryService::new(connection);
    let row = service.create_group("Wants")?;
    let group = service.fetch_group(&row.id)?;
    assert_eq!(group.title, "Wants");
    Ok(())
}

#[test]
fn edit_category_group_title() -> Result<()> {
    let connection = create_test_db()?;
    let service = CategoryService::new(connection);
    let row = service.create_group("Wants")?;
    let group = service.set_group_title(&row.id, "Needs")?;
    assert_eq!(group.title, "Needs");
    Ok(())
}

#[test]
fn delete_category_group() -> Result<()> {
    let connection = create_test_db()?;
    let service = CategoryService::new(connection.clone());
    let row = service.create_group("Wants")?;
    service.delete_group(&row.id)?;

    let conn = connection.get();
    let mut stmt = conn.prepare_cached("select * from category_groups where id=?")?;
    stmt.query_row([row.id], |row| Budget::try_from(row))?;
    Ok(())
}

#[test]
fn soft_delete_category() -> Result<()> {
    let connection = create_test_db()?;
    let service = CategoryService::new(connection);
    let category = service.create_category("__")?;
    service.delete_category(&category.id)?;
    let c = service.fetch_category(&category.id)?;
    assert!(c.deleted_at.is_some());
    Ok(())
}

#[test]
fn create_budgets_for_categories() -> Result<()> {
    let connection = create_test_db()?;
    let service = CategoryService::new(connection);
    let c = service.create_category_raw("__")?;
    let result = service.fetch_budget_from_category(&c.id);
    assert!(result.is_err());
    service.create_missing_budgets()?;
    let result = service.fetch_budget_from_category(&c.id);
    assert!(result.is_ok());
    Ok(())
}

#[test]
fn create_budget_after_category() -> Result<()> {
    let connection = create_test_db()?;
    let service = CategoryService::new(connection.clone());
    let category = service.create_category("__")?;

    let conn = connection.get();
    let mut stmt = conn.prepare_cached("select * from budgets where category_id=?")?;
    stmt.query_row([category.id], |row| Budget::try_from(row))?;
    Ok(())
}

#[test]
fn edit_category() -> Result<()> {
    let connection = create_test_db()?;
    let service = CategoryService::new(connection.clone());
    let category = service.create_category("__")?;
    let category = service.edit_category(&category.id, "__MINE__")?;

    let conn = connection.get();
    let mut stmt = conn.prepare_cached("select * from categories where id=?")?;
    let record = stmt.query_row([category.id], |row| Category::try_from(row))?;
    assert_eq!(record.title, "__MINE__");
    Ok(())
}

#[test]
fn fetch_categories_not_deleted() -> Result<()> {
    let connection = create_test_db()?;
    let service = CategoryService::new(connection);
    let len1 = service.fetch_categories()?.len();
    service.create_category("title")?;
    let c = service.create_category("title")?;
    service.delete_category(&c.id)?;
    let len2 = service.fetch_categories()?.len();
    assert_eq!(len1 + 1, len2);
    Ok(())
}
