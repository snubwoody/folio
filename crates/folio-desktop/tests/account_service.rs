use chrono::Utc;
use folio_lib::service::{Account, AccountService, EditAccount, TransactionService};
use folio_lib::{Money, Result, SqliteConnection, create_test_db};

#[test]
fn edit_account() -> Result<()> {
    let connection = create_test_db()?;
    let service = AccountService::new(connection.clone());
    let account = service.create_account("My account", Money::from_unscaled(200))?;

    let opts = EditAccount::new()
        .name("XLPE")
        .starting_balance(Money::ZERO);

    let account = service.edit_account(&account.id, opts)?;

    let conn = connection.get();
    let record = conn.query_row(
        "SELECT * FROM accounts WHERE id=?",
        [account.id],
        |row|Account::try_from(row)
    )?;

    assert_eq!(record.name, account.name);
    assert_eq!(record.starting_balance, account.starting_balance);
    Ok(())
}

#[test]
fn edit_account_keep_defaults() -> Result<()> {
    let connection = create_test_db()?;
    let service = AccountService::new(connection.clone());
    let account = service.create_account("My account", Money::from_unscaled(200))?;
    let opts = EditAccount::new().starting_balance(Money::ZERO);

    let account = service.edit_account(&account.id, opts)?;

    let conn = connection.get();
    let record = conn.query_row(
        "SELECT * FROM accounts WHERE id=?",
        [account.id],
        |row|Account::try_from(row)
    )?;

    assert_eq!(record.name, "My account");
    assert_eq!(record.starting_balance, account.starting_balance);
    Ok(())
}

#[test]
fn create_account() -> folio_lib::Result<()> {
    let connection = create_test_db()?;
    let service = AccountService::new(connection.clone());
    let now = Utc::now();
    service.create_account("My account", Money::from_unscaled(20))?;

    let conn = connection.get();
    let account = conn.query_row(
        "SELECT * FROM accounts",
        [],
        |row|Account::try_from(row)
    )?;

    assert!(account.created_at.unwrap() >= now);
    assert_eq!(account.name, "My account");
    assert_eq!(account.starting_balance, Money::from_unscaled(20));
    Ok(())
}

#[test]
fn fetch_account() -> folio_lib::Result<()> {
    let connection = create_test_db()?;
    let service = AccountService::new(connection.clone());
    let amount = Money::from_f64(20.0);
    let amount = amount.inner();

    let record = {
        let conn = connection.get();
        conn.query_row(
            "INSERT INTO accounts(name,starting_balance) VALUES('C3PO',?) RETURNING *",
            [amount],
            |row|Account::try_from(row)
        )?
    };

    let account = service.fetch_account(&record.id)?;
    assert_eq!(account.starting_balance.inner(), 20_000_000);
    assert_eq!(account.name, "C3PO");
    Ok(())
}

#[test]
fn calculate_account_balance() -> folio_lib::Result<()> {
    let connection = create_test_db()?;
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

#[test]
fn delete_account() -> folio_lib::Result<()> {
    let connection = create_test_db()?;
    let service = AccountService::new(connection.clone());
    service.create_account("My account", Money::ZERO)?;
    service.create_account("My account", Money::ZERO)?;

    let account = service.create_account("My account", Money::ZERO)?;

    // Prevent deadlock
    {
        let conn = connection.get();
        let mut stmt = conn.prepare_cached("select * from accounts")?;
        let records = stmt
            .query_map((), |row| Account::try_from(row))?
            .collect::<Vec<_>>();
        assert_eq!(records.len(), 3);
    }

    service.delete_account(&account.id)?;
    let conn = connection.get();
    let mut stmt = conn.prepare_cached("select * from accounts")?;
    let records = stmt
        .query_map((), |row| Account::try_from(row))?
        .collect::<Vec<_>>();
    assert_eq!(records.len(), 2);
    Ok(())
}

#[test]
fn delete_account_with_expense() -> folio_lib::Result<()> {
    let connection = create_test_db()?;
    let service = AccountService::new(connection.clone());
    let transaction_service = TransactionService::new(connection.clone());
    let account = service.create_account("My account", Money::ZERO)?;

    transaction_service
        .expense()
        .account_id(&account.id)
        .create(&connection.get())?;

    let records = {
        let conn = connection.get();
        let mut stmt = conn.prepare_cached("select * from accounts")?;
        stmt
            .query_map((), |row| Account::try_from(row))?
            .collect::<Vec<_>>()
    };
    assert_eq!(records.len(), 1);

    service.delete_account(&account.id)?;
    let records = {
        let conn = connection.get();
        let mut stmt = conn.prepare_cached("select * from accounts")?;
        stmt
            .query_map((), |row| Account::try_from(row))?
            .collect::<Vec<_>>()
    };
    assert_eq!(records.len(), 0);
    Ok(())
}

#[test]
fn delete_account_with_income() -> folio_lib::Result<()> {
    let connection = create_test_db()?;
    let service = AccountService::new(connection.clone());
    let transaction_service = TransactionService::new(connection.clone());
    let account = service.create_account("My account", Money::ZERO)?;
    transaction_service
        .income()
        .account_id(&account.id)
        .create(&connection.get())?;

    let records = {
        let conn = connection.get();
        let mut stmt = conn.prepare_cached("select * from accounts")?;
        stmt
            .query_map((), |row| Account::try_from(row))?
            .collect::<Vec<_>>()
    };
    assert_eq!(records.len(), 1);

    service.delete_account(&account.id)?;

    let records = {
        let conn = connection.get();
        let mut stmt = conn.prepare_cached("select * from accounts")?;
        stmt
            .query_map((), |row| Account::try_from(row))?
            .collect::<Vec<_>>()
    };
    assert_eq!(records.len(), 0);
    Ok(())
}
