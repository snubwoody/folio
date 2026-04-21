use chrono::Utc;
use folio_lib::service::{AccountService, EditAccount, Transaction};
use folio_lib::{Money, Result};
use sqlx::SqlitePool;

#[sqlx::test]
async fn edit_account(pool: SqlitePool) -> Result<()> {
    let service = AccountService::new(pool.clone());
    let account = service
        .create_account("My account", Money::from_unscaled(200))
        .await?;
    let opts = EditAccount::new()
        .name("XLPE")
        .starting_balance(Money::ZERO);

    let account = service.edit_account(&account.id, opts).await?;
    let record = sqlx::query!("SELECT * FROM accounts WHERE id=$1", account.id)
        .fetch_one(&pool)
        .await?;

    assert_eq!(record.name, account.name);
    assert_eq!(record.starting_balance, account.starting_balance.inner());
    Ok(())
}

#[sqlx::test]
async fn edit_account_keep_defaults(pool: SqlitePool) -> Result<()> {
    let service = AccountService::new(pool.clone());
    let account = service
        .create_account("My account", Money::from_unscaled(200))
        .await?;
    let opts = EditAccount::new().starting_balance(Money::ZERO);

    let account = service.edit_account(&account.id, opts).await?;
    let record = sqlx::query!("SELECT * FROM accounts WHERE id=$1", account.id)
        .fetch_one(&pool)
        .await?;

    assert_eq!(record.name, "My account");
    assert_eq!(record.starting_balance, account.starting_balance.inner());
    Ok(())
}

#[sqlx::test]
async fn create_account(pool: sqlx::SqlitePool) -> folio_lib::Result<()> {
    let service = AccountService::new(pool.clone());
    let now = Utc::now().timestamp();
    service
        .create_account("My account", Money::from_unscaled(20))
        .await?;
    let account = sqlx::query!("SELECT * FROM accounts")
        .fetch_one(&pool)
        .await?;
    assert!(account.created_at.unwrap() >= now);
    assert_eq!(account.name, "My account");
    assert_eq!(account.starting_balance, Money::from_unscaled(20).inner());
    Ok(())
}

#[sqlx::test]
async fn fetch_account(pool: sqlx::SqlitePool) -> folio_lib::Result<()> {
    let service = AccountService::new(pool.clone());
    let amount = Money::from_f64(20.0);
    let amount = amount.inner();
    let record = sqlx::query!(
        "INSERT INTO accounts(name,starting_balance) VALUES('C3PO',$1) RETURNING id",
        amount
    )
    .fetch_one(&pool)
    .await?;

    let account = service.fetch_account(&record.id).await?;
    assert_eq!(account.starting_balance.inner(), 20_000_000);
    assert_eq!(account.name, "C3PO");
    Ok(())
}

#[sqlx::test]
async fn calculate_account_balance(pool: SqlitePool) -> folio_lib::Result<()> {
    let service = AccountService::new(pool.clone());
    let account = service.create_account("", Money::ZERO).await?;
    Transaction::expense()
        .account_id(&account.id)
        .amount(Money::from_unscaled(20))
        .create(&pool)
        .await?;
    Transaction::expense()
        .account_id(&account.id)
        .amount(Money::from_unscaled(20))
        .create(&pool)
        .await?;
    Transaction::income()
        .account_id(&account.id)
        .amount(Money::from_unscaled(50))
        .create(&pool)
        .await?;
    let balance = service.calculate_balance(&account.id).await?;
    assert_eq!(balance, Money::from_unscaled(10));
    Ok(())
}

#[sqlx::test]
async fn delete_account(pool: sqlx::SqlitePool) -> folio_lib::Result<()> {
    let service = AccountService::new(pool.clone());
    service.create_account("My account", Money::ZERO).await?;
    service.create_account("My account", Money::ZERO).await?;

    let account = service.create_account("My account", Money::ZERO).await?;

    let records = sqlx::query!("SELECT * FROM accounts")
        .fetch_all(&pool)
        .await?;
    assert_eq!(records.len(), 3);

    service.delete_account(&account.id).await?;
    let records = sqlx::query!("SELECT * FROM accounts")
        .fetch_all(&pool)
        .await?;
    assert_eq!(records.len(), 2);
    Ok(())
}

#[sqlx::test]
async fn delete_account_with_expense(pool: sqlx::SqlitePool) -> folio_lib::Result<()> {
    let service = AccountService::new(pool.clone());
    let account = service.create_account("My account", Money::ZERO).await?;

    Transaction::expense()
        .account_id(&account.id)
        .create(&pool)
        .await?;
    let records = sqlx::query!("SELECT * FROM accounts")
        .fetch_all(&pool)
        .await?;
    assert_eq!(records.len(), 1);

    service.delete_account(&account.id).await?;
    let records = sqlx::query!("SELECT * FROM accounts")
        .fetch_all(&pool)
        .await?;
    assert_eq!(records.len(), 0);
    Ok(())
}

#[sqlx::test]
async fn delete_account_with_income(pool: sqlx::SqlitePool) -> folio_lib::Result<()> {
    let service = AccountService::new(pool.clone());
    let account = service.create_account("My account", Money::ZERO).await?;
    Transaction::income()
        .account_id(&account.id)
        .create(&pool)
        .await?;
    let records = sqlx::query!("SELECT * FROM accounts")
        .fetch_all(&pool)
        .await?;
    assert_eq!(records.len(), 1);

    service.delete_account(&account.id).await?;
    let records = sqlx::query!("SELECT * FROM accounts")
        .fetch_all(&pool)
        .await?;
    assert_eq!(records.len(), 0);
    Ok(())
}
