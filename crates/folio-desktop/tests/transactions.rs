use chrono::NaiveDate;
use folio_lib::Money;
use folio_lib::service::{AccountService, Category, Transaction, TransactionType};
use sqlx::{Row, SqlitePool};
use std::str::FromStr;

#[sqlx::test]
async fn set_inflow_for_only_one_income(pool: SqlitePool) -> folio_lib::Result<()> {
    let account_service = AccountService::new(pool.clone());
    let account = account_service.create_account("__", Money::ZERO).await?;
    let transaction = Transaction::income()
        .amount(Money::MAX)
        .account_id(&account.id)
        .create(&pool)
        .await?;
    let transaction2 = Transaction::income()
        .amount(Money::MAX)
        .account_id(&account.id)
        .create(&pool)
        .await?;

    Transaction::set_inflow(&transaction.id, Money::from_f64(10.0), &pool).await?;
    let t = Transaction::fetch(&transaction.id, &pool).await?;
    let t2 = Transaction::fetch(&transaction2.id, &pool).await?;
    assert_eq!(t2.amount, Money::MAX);
    assert_eq!(t.amount, Money::from_f64(10.0));
    assert_eq!(t.to_account_id.unwrap(), transaction.to_account_id.unwrap());
    assert!(t.from_account_id.is_none());
    Ok(())
}

#[sqlx::test]
async fn set_outflow_for_only_one_expense(pool: SqlitePool) -> folio_lib::Result<()> {
    let account_service = AccountService::new(pool.clone());
    let account = account_service.create_account("__", Money::ZERO).await?;
    let transaction = Transaction::expense()
        .amount(Money::MAX)
        .account_id(&account.id)
        .create(&pool)
        .await?;
    let transaction2 = Transaction::expense()
        .amount(Money::MAX)
        .account_id(&account.id)
        .create(&pool)
        .await?;

    Transaction::set_outflow(&transaction.id, Money::from_f64(10.0), &pool).await?;
    let t = Transaction::fetch(&transaction.id, &pool).await?;
    let t2 = Transaction::fetch(&transaction2.id, &pool).await?;
    assert_eq!(t.amount, Money::from_f64(10.0));
    assert_eq!(t2.amount, Money::MAX);
    assert_eq!(
        t.from_account_id.unwrap(),
        transaction.from_account_id.unwrap()
    );
    assert!(t.to_account_id.is_none());
    Ok(())
}

#[sqlx::test]
async fn fetch_transaction(pool: sqlx::SqlitePool) -> folio_lib::Result<()> {
    let account_service = AccountService::new(pool.clone());
    let account = account_service.create_account("", Money::ZERO).await?;
    let row = sqlx::query(
        "
        INSERT INTO transactions(amount,transaction_date,from_account_id)
        VALUES (10,'2024-12-12',$1)
        RETURNING id",
    )
    .bind(&account.id)
    .fetch_one(&pool)
    .await?;

    let id: String = row.get("id");
    let transaction = Transaction::fetch(&id, &pool).await?;
    assert_eq!(transaction.amount, Money::from_scaled(10));
    Ok(())
}

#[sqlx::test]
async fn create_expense(pool: sqlx::SqlitePool) -> folio_lib::Result<()> {
    let account_service = AccountService::new(pool.clone());
    let account = account_service.create_account("", Money::ZERO).await?;
    let date = NaiveDate::from_str("2024-12-12")?;
    let expense = Transaction::expense()
        .account_id(&account.id)
        .date(date)
        .amount(Money::MAX)
        .create(&pool)
        .await?;

    assert_eq!(expense.amount, Money::MAX);
    assert_eq!(expense.transaction_date, date);
    assert_eq!(expense.from_account_id.unwrap(), account.id);
    Ok(())
}

#[sqlx::test]
async fn edit_transaction(pool: sqlx::SqlitePool) -> folio_lib::Result<()> {
    let account_service = AccountService::new(pool.clone());
    let account = account_service.create_account("", Money::ZERO).await?;
    let a2 = account_service.create_account("", Money::ZERO).await?;
    let a3 = account_service.create_account("", Money::ZERO).await?;
    let date = NaiveDate::from_str("2024-12-12")?;
    let expense = Transaction::expense()
        .account_id(&account.id)
        .amount(Money::MAX)
        .create(&pool)
        .await?;

    let expense = Transaction::edit(&expense.id)
        .amount(Money::from_f64(10.0))
        .date(date)
        .from_account(&a2.id)
        .to_account(&a3.id)
        .note("Note__")
        .update(&pool)
        .await?;

    assert_eq!(expense.amount, Money::from_f64(10.0));
    assert_eq!(expense.transaction_date, date);
    assert_eq!(expense.from_account_id.unwrap(), a2.id);
    assert_eq!(expense.to_account_id.unwrap(), a3.id);
    assert_eq!(expense.note.unwrap(), "Note__");
    Ok(())
}

#[sqlx::test]
async fn create_income(pool: sqlx::SqlitePool) -> folio_lib::Result<()> {
    let account_service = AccountService::new(pool.clone());
    let account = account_service.create_account("", Money::ZERO).await?;
    let date = NaiveDate::from_str("2024-12-12")?;
    let expense = Transaction::income()
        .account_id(&account.id)
        .date(date)
        .amount(Money::MAX)
        .create(&pool)
        .await?;

    assert_eq!(expense.amount, Money::MAX);
    assert_eq!(expense.transaction_date, date);
    assert_eq!(expense.to_account_id.unwrap(), account.id);
    Ok(())
}

#[sqlx::test]
async fn create_transfer(pool: sqlx::SqlitePool) -> folio_lib::Result<()> {
    let account_service = AccountService::new(pool.clone());
    let a1 = account_service.create_account("", Money::ZERO).await?;
    let a2 = account_service.create_account("", Money::ZERO).await?;
    let date = NaiveDate::from_str("2024-12-12")?;
    let expense = Transaction::transfer()
        .accounts(&a1.id, &a2.id)
        .date(date)
        .amount(Money::MAX)
        .create(&pool)
        .await?;

    assert_eq!(expense.amount, Money::MAX);
    assert_eq!(expense.transaction_date, date);
    assert_eq!(expense.from_account_id.unwrap(), a1.id);
    assert_eq!(expense.to_account_id.unwrap(), a2.id);
    Ok(())
}

#[sqlx::test]
async fn delete_multiple_transactions(pool: SqlitePool) -> folio_lib::Result<()> {
    let account_service = AccountService::new(pool.clone());
    let account = account_service.create_account("__", Money::ZERO).await?;
    let t1 = Transaction::expense()
        .amount(Money::MAX)
        .account_id(&account.id)
        .create(&pool)
        .await?;
    let t2 = Transaction::expense()
        .amount(Money::MAX)
        .account_id(&account.id)
        .create(&pool)
        .await?;
    let length = Transaction::fetch_all(&pool).await?.len();
    assert_eq!(length, 2);
    Transaction::delete(&[&t1.id, &t2.id], &pool).await?;
    let length = Transaction::fetch_all(&pool).await?.len();
    assert_eq!(length, 0);
    Ok(())
}

#[sqlx::test]
async fn delete_empty_slice(pool: SqlitePool) -> folio_lib::Result<()> {
    let account_service = AccountService::new(pool.clone());
    let account = account_service.create_account("__", Money::ZERO).await?;
    Transaction::expense()
        .amount(Money::MAX)
        .account_id(&account.id)
        .create(&pool)
        .await?;

    Transaction::delete::<String>(&[], &pool).await?;
    let length = Transaction::fetch_all(&pool).await?.len();
    assert_eq!(length, 1);
    Ok(())
}

#[sqlx::test]
async fn delete_only_affected_transactions(pool: SqlitePool) -> folio_lib::Result<()> {
    let account_service = AccountService::new(pool.clone());
    let account = account_service.create_account("__", Money::ZERO).await?;
    let t1 = Transaction::expense()
        .amount(Money::MAX)
        .account_id(&account.id)
        .create(&pool)
        .await?;
    Transaction::expense()
        .amount(Money::MAX)
        .account_id(&account.id)
        .create(&pool)
        .await?;
    let length = Transaction::fetch_all(&pool).await?.len();
    assert_eq!(length, 2);
    Transaction::delete(&[&t1.id], &pool).await?;
    let length = Transaction::fetch_all(&pool).await?.len();
    assert_eq!(length, 1);
    Ok(())
}

#[test]
fn transaction_type_expense() {
    let expense = Transaction {
        from_account_id: Some("".to_owned()),
        ..Default::default()
    };
    let income = Transaction {
        to_account_id: Some("".to_owned()),
        ..Default::default()
    };
    let transfer = Transaction {
        from_account_id: Some("".to_owned()),
        to_account_id: Some("".to_owned()),
        ..Default::default()
    };

    assert_eq!(expense.transaction_type(), TransactionType::Expense);
    assert_eq!(income.transaction_type(), TransactionType::Income);
    assert_eq!(transfer.transaction_type(), TransactionType::Transfer);
}

#[sqlx::test]
async fn set_outflow_for_expense(pool: SqlitePool) -> folio_lib::Result<()> {
    let account_service = AccountService::new(pool.clone());
    let account = account_service.create_account("__", Money::ZERO).await?;
    let transaction = Transaction::expense()
        .amount(Money::MAX)
        .account_id(&account.id)
        .create(&pool)
        .await?;

    Transaction::set_outflow(&transaction.id, Money::from_f64(10.0), &pool).await?;
    let t = Transaction::fetch(&transaction.id, &pool).await?;
    assert_eq!(t.amount, Money::from_f64(10.0));
    assert_eq!(
        t.from_account_id.unwrap(),
        transaction.from_account_id.unwrap()
    );
    assert!(t.to_account_id.is_none());
    Ok(())
}

#[sqlx::test]
async fn set_payee_for_expense(pool: SqlitePool) -> folio_lib::Result<()> {
    let account_service = AccountService::new(pool.clone());
    let account = account_service.create_account("__", Money::ZERO).await?;
    let account2 = account_service.create_account("__", Money::ZERO).await?;
    let transaction = Transaction::expense()
        .amount(Money::MAX)
        .account_id(&account.id)
        .create(&pool)
        .await?;

    Transaction::set_payee(&transaction.id, &account2.id, &pool).await?;
    let t = Transaction::fetch(&transaction.id, &pool).await?;
    assert_eq!(t.from_account_id.unwrap(), account.id);
    assert_eq!(t.to_account_id.unwrap(), account2.id);
    Ok(())
}

#[sqlx::test]
async fn set_account_for_expense(pool: SqlitePool) -> folio_lib::Result<()> {
    let account_service = AccountService::new(pool.clone());
    let account = account_service.create_account("__", Money::ZERO).await?;
    let account2 = account_service.create_account("__", Money::ZERO).await?;
    let transaction = Transaction::expense()
        .amount(Money::ZERO)
        .account_id(&account.id)
        .create(&pool)
        .await?;

    Transaction::set_account(&transaction.id, &account2.id, &pool).await?;
    let t = Transaction::fetch(&transaction.id, &pool).await?;
    assert_eq!(t.from_account_id.unwrap(), account2.id);
    assert_eq!(t.to_account_id, None);
    Ok(())
}

#[sqlx::test]
async fn set_account_for_income(pool: SqlitePool) -> folio_lib::Result<()> {
    let account_service = AccountService::new(pool.clone());
    let account = account_service.create_account("__", Money::ZERO).await?;
    let account2 = account_service.create_account("__", Money::ZERO).await?;
    let transaction = Transaction::income()
        .amount(Money::ZERO)
        .account_id(&account.id)
        .create(&pool)
        .await?;

    Transaction::set_account(&transaction.id, &account2.id, &pool).await?;
    let t = Transaction::fetch(&transaction.id, &pool).await?;
    assert_eq!(t.to_account_id.unwrap(), account2.id);
    assert_eq!(t.from_account_id, None);
    Ok(())
}

#[sqlx::test]
async fn set_account_for_transfer(pool: SqlitePool) -> folio_lib::Result<()> {
    let account_service = AccountService::new(pool.clone());
    let account = account_service.create_account("__", Money::ZERO).await?;
    let account2 = account_service.create_account("__", Money::ZERO).await?;
    let account3 = account_service.create_account("__", Money::ZERO).await?;
    let transaction = Transaction::transfer()
        .amount(Money::ZERO)
        .accounts(&account.id, &account2.id)
        .create(&pool)
        .await?;

    Transaction::set_account(&transaction.id, &account3.id, &pool).await?;
    let t = Transaction::fetch(&transaction.id, &pool).await?;
    assert_eq!(t.from_account_id.unwrap(), account3.id);
    assert_eq!(t.to_account_id.unwrap(), account2.id);
    Ok(())
}

#[sqlx::test]
async fn set_payee_for_income(pool: SqlitePool) -> folio_lib::Result<()> {
    let account_service = AccountService::new(pool.clone());
    let account = account_service.create_account("__", Money::ZERO).await?;
    let account2 = account_service.create_account("__", Money::ZERO).await?;
    let transaction = Transaction::income()
        .amount(Money::MAX)
        .account_id(&account.id)
        .create(&pool)
        .await?;

    Transaction::set_payee(&transaction.id, &account2.id, &pool).await?;
    let t = Transaction::fetch(&transaction.id, &pool).await?;
    assert_eq!(t.from_account_id.unwrap(), account.id);
    assert_eq!(t.to_account_id.unwrap(), account2.id);
    Ok(())
}

#[sqlx::test]
async fn set_payee_for_transfer(pool: SqlitePool) -> folio_lib::Result<()> {
    let account_service = AccountService::new(pool.clone());
    let account = account_service.create_account("__", Money::ZERO).await?;
    let account2 = account_service.create_account("__", Money::ZERO).await?;
    let account3 = account_service.create_account("__", Money::ZERO).await?;
    let transaction = Transaction::transfer()
        .amount(Money::MAX)
        .accounts(&account.id, &account2.id)
        .create(&pool)
        .await?;

    Transaction::set_payee(&transaction.id, &account3.id, &pool).await?;
    let t = Transaction::fetch(&transaction.id, &pool).await?;
    assert_eq!(t.from_account_id.unwrap(), account.id);
    assert_eq!(t.to_account_id.unwrap(), account3.id);
    Ok(())
}

#[sqlx::test]
async fn set_payee_removes_category(pool: SqlitePool) -> folio_lib::Result<()> {
    let account_service = AccountService::new(pool.clone());
    let account = account_service.create_account("__", Money::ZERO).await?;
    let account2 = account_service.create_account("__", Money::ZERO).await?;
    let category = Category::create("", &pool).await?;
    let transaction = Transaction::income()
        .amount(Money::MAX)
        .category(&category.id)
        .account_id(&account.id)
        .create(&pool)
        .await?;

    Transaction::set_payee(&transaction.id, &account2.id, &pool).await?;
    let t = Transaction::fetch(&transaction.id, &pool).await?;
    assert!(t.category_id.is_none());
    Ok(())
}

#[sqlx::test]
async fn set_inflow_for_income(pool: SqlitePool) -> folio_lib::Result<()> {
    let account_service = AccountService::new(pool.clone());
    let account = account_service.create_account("__", Money::ZERO).await?;
    let transaction = Transaction::income()
        .amount(Money::MAX)
        .account_id(&account.id)
        .create(&pool)
        .await?;

    Transaction::set_inflow(&transaction.id, Money::from_f64(10.0), &pool).await?;
    let t = Transaction::fetch(&transaction.id, &pool).await?;
    assert_eq!(t.amount, Money::from_f64(10.0));
    assert_eq!(t.to_account_id.unwrap(), transaction.to_account_id.unwrap());
    assert!(t.from_account_id.is_none());
    Ok(())
}

#[sqlx::test]
async fn set_inflow_for_transfer(pool: SqlitePool) -> folio_lib::Result<()> {
    let account_service = AccountService::new(pool.clone());
    let account = account_service.create_account("__", Money::ZERO).await?;
    let account2 = account_service.create_account("__", Money::ZERO).await?;
    let transaction = Transaction::transfer()
        .amount(Money::MAX)
        .accounts(&account.id, &account2.id)
        .create(&pool)
        .await?;

    let result = Transaction::set_inflow(&transaction.id, Money::from_f64(10.0), &pool).await;
    assert!(result.is_err());
    Ok(())
}

#[sqlx::test]
async fn set_inflow_for_expense(pool: SqlitePool) -> folio_lib::Result<()> {
    let account_service = AccountService::new(pool.clone());
    let account = account_service.create_account("__", Money::ZERO).await?;
    let transaction = Transaction::expense()
        .amount(Money::MAX)
        .account_id(&account.id)
        .create(&pool)
        .await?;

    Transaction::set_inflow(&transaction.id, Money::from_f64(10.0), &pool).await?;
    let t = Transaction::fetch(&transaction.id, &pool).await?;
    assert_eq!(t.amount, Money::from_f64(10.0));
    assert_eq!(
        t.to_account_id.unwrap(),
        transaction.from_account_id.unwrap()
    );
    assert!(t.from_account_id.is_none());
    Ok(())
}

#[sqlx::test]
async fn set_outflow_for_income(pool: SqlitePool) -> folio_lib::Result<()> {
    let account_service = AccountService::new(pool.clone());
    // Setting outflow on an income should turn it into an expense
    let account = account_service.create_account("__", Money::ZERO).await?;
    let transaction = Transaction::income()
        .amount(Money::MAX)
        .account_id(&account.id)
        .create(&pool)
        .await?;

    Transaction::set_outflow(&transaction.id, Money::from_f64(10.0), &pool).await?;
    let t = Transaction::fetch(&transaction.id, &pool).await?;
    assert_eq!(t.amount, Money::from_f64(10.0));
    assert_eq!(
        t.from_account_id.unwrap(),
        transaction.to_account_id.unwrap()
    );
    assert!(t.to_account_id.is_none());
    Ok(())
}
