use folio_lib::service::{Account, EditAccount};
use folio_lib::{setup_test_db, Money, Result};
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
async fn edit_name(pool: SqlitePool) -> Result<()> {
    let conn = setup_test_db(pool.connect_options().get_filename()).await;
    let account = Account::create("My account", Money::from_unscaled(200), &pool).await?;
    let account = Account::set_name(&account.id,"Absa",&conn)?;
    let new_account = Account::from_id(&account.id,&pool).await?;

    assert_eq!(new_account.name, account.name);
    Ok(())
}

#[sqlx::test]
async fn edit_starting_balance(pool: SqlitePool) -> Result<()> {
    let conn = setup_test_db(pool.connect_options().get_filename()).await;
    let account = Account::create("My account", Money::from_unscaled(200), &pool).await?;
    let account = Account::set_starting_balance(&account.id,Money::from_unscaled(500),&conn)?;
    let new_account = Account::from_id(&account.id,&pool).await?;

    assert_eq!(new_account.starting_balance, account.starting_balance);
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
