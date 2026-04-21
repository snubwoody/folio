use folio_lib::service::Account;
use folio_lib::{setup_test_db, Money, Result};
use sqlx::SqlitePool;

#[sqlx::test]
async fn edit_name(pool: SqlitePool) -> Result<()> {
    let conn = setup_test_db(pool.connect_options().get_filename()).await;
    let account = Account::create("My account", Money::from_unscaled(200), &conn)?;
    let account = Account::set_name(&account.id,"Absa",&conn)?;
    let new_account = Account::from_id(&account.id,&pool).await?;

    assert_eq!(new_account.name, account.name);
    Ok(())
}

#[sqlx::test]
async fn edit_starting_balance(pool: SqlitePool) -> Result<()> {
    let conn = setup_test_db(pool.connect_options().get_filename()).await;
    let account = Account::create("My account", Money::from_unscaled(200), &conn)?;
    let account = Account::set_starting_balance(&account.id,Money::from_unscaled(500),&conn)?;
    let new_account = Account::from_id(&account.id,&pool).await?;

    assert_eq!(new_account.starting_balance, account.starting_balance);
    Ok(())
}
