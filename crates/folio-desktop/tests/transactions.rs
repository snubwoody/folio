use sqlx::Row;
use folio_lib::{db, service, Money};
use folio_lib::service::{Account, Transaction};

#[sqlx::test]
async fn fetch_transaction(pool: sqlx::SqlitePool) -> folio_lib::Result<()>{
    let account = Account::create("",Money::ZERO,&pool).await?;
    let row = sqlx::query("
        INSERT INTO transactions(amount,transaction_date,from_account_id)
        VALUES (10,'2024-12-12',$1)
        RETURNING id"
    )
        .bind(&account.id)
        .fetch_one(&pool)
        .await?;
    
    let id: String = row.get("id");    
    let transaction = Transaction::fetch(&id, &pool).await?;
    assert_eq!(transaction.amount,0);
    Ok(())
}