use chrono::NaiveDate;
use folio_lib::Money;
use folio_lib::service::{Account, Transaction};
use sqlx::Row;
use std::str::FromStr;

#[sqlx::test]
async fn fetch_transaction(pool: sqlx::SqlitePool) -> folio_lib::Result<()> {
    let account = Account::create("", Money::ZERO, &pool).await?;
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
    let account = Account::create("", Money::ZERO, &pool).await?;
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
    let account = Account::create("", Money::ZERO, &pool).await?;
    let a2 = Account::create("", Money::ZERO, &pool).await?;
    let a3 = Account::create("", Money::ZERO, &pool).await?;
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
    let account = Account::create("", Money::ZERO, &pool).await?;
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
    let a1 = Account::create("", Money::ZERO, &pool).await?;
    let a2 = Account::create("", Money::ZERO, &pool).await?;
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
