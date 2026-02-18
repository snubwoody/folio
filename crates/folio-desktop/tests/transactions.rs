use folio_lib::{db, Money};
use folio_lib::service::Account;

#[sqlx::test]
async fn parse_transactions(pool: sqlx::SqlitePool) -> folio_lib::Result<()>{
    let account = Account::create("",Money::ZERO,&pool).await?;
    let transaction: db::Transaction = sqlx::query_as("
        INSERT INTO transactions(amount,transaction_date,from_account_id)
        VALUES (10,'2024-12-12',$1)
        RETURNING *"
    )
        .bind(&account.id)
        .fetch_one(&pool)
        .await?;

    assert_eq!(transaction.amount,0);
    Ok(())
}