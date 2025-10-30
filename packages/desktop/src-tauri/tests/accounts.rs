use folio_lib::service::{Account, EditAccount};
use folio_lib::{Money, Result};
use sqlx::SqlitePool;

#[sqlx::test]
async fn edit_account(pool: SqlitePool) -> Result<()> {
    let account = Account::create("My account", Money::from_unscaled(200), &pool).await?;
    let opts = EditAccount::new()
        .name("XLPE")
        .starting_balance(Money::ZERO);

    let account = Account::edit(&account.id, opts, &pool).await?;
    let record = sqlx::query!("SELECT * FROM accounts WHERE id=$1", account.id)
        .fetch_one(&pool)
        .await?;

    assert_eq!(record.name, account.name);
    assert_eq!(record.starting_balance, account.starting_balance.inner());
    Ok(())
}

#[sqlx::test]
async fn edit_account_keep_defaults(pool: SqlitePool) -> Result<()> {
    let account = Account::create("My account", Money::from_unscaled(200), &pool).await?;
    let opts = EditAccount::new().starting_balance(Money::ZERO);

    let account = Account::edit(&account.id, opts, &pool).await?;
    let record = sqlx::query!("SELECT * FROM accounts WHERE id=$1", account.id)
        .fetch_one(&pool)
        .await?;

    assert_eq!(record.name, "My account");
    assert_eq!(record.starting_balance, account.starting_balance.inner());
    Ok(())
}
