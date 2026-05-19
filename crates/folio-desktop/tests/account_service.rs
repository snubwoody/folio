use chrono::Utc;
use folio_lib::service::{AccountService, EditAccount, TransactionService};
use folio_lib::{Money, Result, SqliteConnection};
use sqlx::SqlitePool;

#[sqlx::test]
async fn edit_account(pool: SqlitePool) -> Result<()> {
    let connection = SqliteConnection::open(pool.connect_options().get_filename())?;
    let service = AccountService::new(connection.clone());
    let account = service
        .create_account("My account", Money::from_unscaled(200))?;

    let opts = EditAccount::new()
        .name("XLPE")
        .starting_balance(Money::ZERO);

    let account = service.edit_account(&account.id, opts)?;
    let record = sqlx::query!("SELECT * FROM accounts WHERE id=$1", account.id)
        .fetch_one(&pool)
        .await?;

    assert_eq!(record.name, account.name);
    assert_eq!(record.starting_balance, account.starting_balance.inner());
    Ok(())
}

#[sqlx::test]
async fn edit_account_keep_defaults(pool: SqlitePool) -> Result<()> {
    let connection = SqliteConnection::open(pool.connect_options().get_filename())?;
    let service = AccountService::new(connection.clone());
    let account = service
        .create_account("My account", Money::from_unscaled(200))?;
    let opts = EditAccount::new().starting_balance(Money::ZERO);

    let account = service.edit_account(&account.id, opts)?;
    let record = sqlx::query!("SELECT * FROM accounts WHERE id=$1", account.id)
        .fetch_one(&pool)
        .await?;

    assert_eq!(record.name, "My account");
    assert_eq!(record.starting_balance, account.starting_balance.inner());
    Ok(())
}

#[sqlx::test]
async fn create_account(pool: sqlx::SqlitePool) -> folio_lib::Result<()> {
    let connection = SqliteConnection::open(pool.connect_options().get_filename())?;
    let service = AccountService::new(connection.clone());
    let now = Utc::now().timestamp();
    service
        .create_account("My account", Money::from_unscaled(20))?;
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
    let connection = SqliteConnection::open(pool.connect_options().get_filename())?;
    let service = AccountService::new(connection.clone());
    let amount = Money::from_f64(20.0);
    let amount = amount.inner();
    let record = sqlx::query!(
        "INSERT INTO accounts(name,starting_balance) VALUES('C3PO',$1) RETURNING id",
        amount
    )
    .fetch_one(&pool)
    .await?;

    let account = service.fetch_account(&record.id)?;
    assert_eq!(account.starting_balance.inner(), 20_000_000);
    assert_eq!(account.name, "C3PO");
    Ok(())
}

#[sqlx::test]
async fn calculate_account_balance(pool: SqlitePool) -> folio_lib::Result<()> {
    let connection = SqliteConnection::open(pool.connect_options().get_filename())?;
    let service = AccountService::new(connection.clone());
    let account = service.create_account("Expense", Money::ZERO)?;
    let transaction_service = TransactionService::new(connection.clone());
    transaction_service
        .expense()
        .account_id(&account.id)
        .amount(Money::from_unscaled(20))
        .create(&connection.get())?;
    transaction_service
        .expense()
        .account_id(&account.id)
        .amount(Money::from_unscaled(20))
        .create(&connection.get())?;
    transaction_service
        .income()
        .account_id(&account.id)
        .amount(Money::from_unscaled(50))
        .create(&connection.get())?;
    let balance = service.calculate_balance(&account.id)?;
    assert_eq!(balance, Money::from_unscaled(10));
    Ok(())
}

#[sqlx::test]
async fn delete_account(pool: sqlx::SqlitePool) -> folio_lib::Result<()> {
    let connection = SqliteConnection::open(pool.connect_options().get_filename())?;
    let service = AccountService::new(connection.clone());
    service.create_account("My account", Money::ZERO)?;
    service.create_account("My account", Money::ZERO)?;

    let account = service.create_account("My account", Money::ZERO)?;

    let records = sqlx::query!("SELECT * FROM accounts")
        .fetch_all(&pool)
        .await?;
    assert_eq!(records.len(), 3);

    service.delete_account(&account.id)?;
    let records = sqlx::query!("SELECT * FROM accounts")
        .fetch_all(&pool)
        .await?;
    assert_eq!(records.len(), 2);
    Ok(())
}

#[sqlx::test]
async fn delete_account_with_expense(pool: sqlx::SqlitePool) -> folio_lib::Result<()> {
    let connection = SqliteConnection::open(pool.connect_options().get_filename())?;
    let service = AccountService::new(connection.clone());
    let transaction_service = TransactionService::new(connection.clone());
    let account = service.create_account("My account", Money::ZERO)?;

    transaction_service
        .expense()
        .account_id(&account.id)
        .create(&connection.get())?;
    let records = sqlx::query!("SELECT * FROM accounts")
        .fetch_all(&pool)
        .await?;
    assert_eq!(records.len(), 1);

    service.delete_account(&account.id)?;
    let records = sqlx::query!("SELECT * FROM accounts")
        .fetch_all(&pool)
        .await?;
    assert_eq!(records.len(), 0);
    Ok(())
}

#[sqlx::test]
async fn delete_account_with_income(pool: sqlx::SqlitePool) -> folio_lib::Result<()> {
    let connection = SqliteConnection::open(pool.connect_options().get_filename())?;
    let service = AccountService::new(connection.clone());
    let transaction_service = TransactionService::new(connection.clone());
    let account = service.create_account("My account", Money::ZERO)?;
    transaction_service
        .income()
        .account_id(&account.id)
        .create(&connection.get())?;
    let records = sqlx::query!("SELECT * FROM accounts")
        .fetch_all(&pool)
        .await?;
    assert_eq!(records.len(), 1);

    service.delete_account(&account.id)?;
    let records = sqlx::query!("SELECT * FROM accounts")
        .fetch_all(&pool)
        .await?;
    assert_eq!(records.len(), 0);
    Ok(())
}
