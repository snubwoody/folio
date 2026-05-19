use chrono::NaiveDate;
use folio_lib::service::{
    AccountService, CategoryService, Transaction, TransactionService, TransactionType,
};
use folio_lib::{Money, SqliteConnection, create_test_db};
use sqlx::{Row, SqlitePool};
use std::str::FromStr;

#[test]
fn set_inflow_for_only_one_income() -> folio_lib::Result<()> {
    let connection = create_test_db()?;
    let account_service = AccountService::new(connection.clone());
    let transaction_service = TransactionService::new(connection.clone());
    let account = account_service.create_account("__", Money::ZERO)?;
    let transaction = transaction_service
        .income()
        .amount(Money::MAX)
        .account_id(&account.id)
        .create(&connection.get())?;
    let transaction2 = transaction_service
        .income()
        .amount(Money::MAX)
        .account_id(&account.id)
        .create(&connection.get())?;

    transaction_service.set_inflow(&transaction.id, Money::from_f64(10.0))?;
    let t = transaction_service.fetch(&transaction.id)?;
    let t2 = transaction_service.fetch(&transaction2.id)?;
    assert_eq!(t2.amount, Money::MAX);
    assert_eq!(t.amount, Money::from_f64(10.0));
    assert_eq!(t.to_account_id.unwrap(), transaction.to_account_id.unwrap());
    assert!(t.from_account_id.is_none());
    Ok(())
}

#[test]
fn set_outflow_for_only_one_expense() -> folio_lib::Result<()> {
    let connection = create_test_db()?;
    let account_service = AccountService::new(connection.clone());
    let transaction_service = TransactionService::new(connection.clone());
    let account = account_service.create_account("__", Money::ZERO)?;
    let transaction = transaction_service
        .expense()
        .amount(Money::MAX)
        .account_id(&account.id)
        .create(&connection.get())?;
    let transaction2 = transaction_service
        .expense()
        .amount(Money::MAX)
        .account_id(&account.id)
        .create(&connection.get())?;

    transaction_service.set_outflow(&transaction.id, Money::from_f64(10.0))?;
    let t = transaction_service.fetch(&transaction.id)?;
    let t2 = transaction_service.fetch(&transaction2.id)?;
    assert_eq!(t.amount, Money::from_f64(10.0));
    assert_eq!(t2.amount, Money::MAX);
    assert_eq!(
        t.from_account_id.unwrap(),
        transaction.from_account_id.unwrap()
    );
    assert!(t.to_account_id.is_none());
    Ok(())
}

#[test]
fn fetch_transaction() -> folio_lib::Result<()> {
    let connection = create_test_db()?;
    let account_service = AccountService::new(connection.clone());
    let transaction_service = TransactionService::new(connection.clone());
    let account = account_service.create_account("Expense", Money::ZERO)?;

    // Prevent mutex deadlock
    let row = {
        let conn = connection.get();
        let mut stmt = conn
            .prepare_cached("insert into transactions(amount,transaction_date,from_account_id) values(10,'2024-12-12',?) returning *")?;
        stmt.query_row([account.id], |row| Transaction::try_from(row))?
    };

    let transaction = transaction_service.fetch(&row.id)?;
    assert_eq!(transaction.amount, Money::from_scaled(10));
    Ok(())
}

#[test]
fn create_expense() -> folio_lib::Result<()> {
    let connection = create_test_db()?;
    let account_service = AccountService::new(connection.clone());
    let transaction_service = TransactionService::new(connection.clone());
    let account = account_service.create_account("Expense", Money::ZERO)?;
    let date = NaiveDate::from_str("2024-12-12")?;
    let expense = transaction_service
        .expense()
        .account_id(&account.id)
        .date(date)
        .amount(Money::MAX)
        .create(&connection.get())?;

    assert_eq!(expense.amount, Money::MAX);
    assert_eq!(expense.transaction_date, date);
    assert_eq!(expense.from_account_id.unwrap(), account.id);
    Ok(())
}

#[test]
fn edit_transaction() -> folio_lib::Result<()> {
    let connection = create_test_db()?;
    let account_service = AccountService::new(connection.clone());
    let transaction_service = TransactionService::new(connection.clone());
    let account = account_service.create_account("Expense", Money::ZERO)?;
    let a2 = account_service.create_account("Expense", Money::ZERO)?;
    let a3 = account_service.create_account("Expense", Money::ZERO)?;
    let date = NaiveDate::from_str("2024-12-12")?;
    let expense = transaction_service
        .expense()
        .account_id(&account.id)
        .amount(Money::MAX)
        .create(&connection.get())?;

    let expense = transaction_service
        .edit(&expense.id)
        .amount(Money::from_f64(10.0))
        .date(date)
        .from_account(&a2.id)
        .to_account(&a3.id)
        .note("Note__")
        .update(&connection.get())?;

    assert_eq!(expense.amount, Money::from_f64(10.0));
    assert_eq!(expense.transaction_date, date);
    assert_eq!(expense.from_account_id.unwrap(), a2.id);
    assert_eq!(expense.to_account_id.unwrap(), a3.id);
    assert_eq!(expense.note.unwrap(), "Note__");
    Ok(())
}

#[test]
fn create_income() -> folio_lib::Result<()> {
    let connection = create_test_db()?;
    let account_service = AccountService::new(connection.clone());
    let transaction_service = TransactionService::new(connection.clone());
    let account = account_service.create_account("Expense", Money::ZERO)?;
    let date = NaiveDate::from_str("2024-12-12")?;
    let expense = transaction_service
        .income()
        .account_id(&account.id)
        .date(date)
        .amount(Money::MAX)
        .create(&connection.get())?;

    assert_eq!(expense.amount, Money::MAX);
    assert_eq!(expense.transaction_date, date);
    assert_eq!(expense.to_account_id.unwrap(), account.id);
    Ok(())
}

#[test]
fn create_transfer() -> folio_lib::Result<()> {
    let connection = create_test_db()?;
    let account_service = AccountService::new(connection.clone());
    let transaction_service = TransactionService::new(connection.clone());
    let a1 = account_service.create_account("Expense", Money::ZERO)?;
    let a2 = account_service.create_account("Expense", Money::ZERO)?;
    let date = NaiveDate::from_str("2024-12-12")?;
    let expense = transaction_service
        .transfer()
        .accounts(&a1.id, &a2.id)
        .date(date)
        .amount(Money::MAX)
        .create(&connection.get())?;

    assert_eq!(expense.amount, Money::MAX);
    assert_eq!(expense.transaction_date, date);
    assert_eq!(expense.from_account_id.unwrap(), a1.id);
    assert_eq!(expense.to_account_id.unwrap(), a2.id);
    Ok(())
}

#[test]
fn delete_multiple_transactions() -> folio_lib::Result<()> {
    let connection = create_test_db()?;
    let account_service = AccountService::new(connection.clone());
    let transaction_service = TransactionService::new(connection.clone());
    let account = account_service.create_account("__", Money::ZERO)?;

    let t1 = transaction_service
        .expense()
        .amount(Money::MAX)
        .account_id(&account.id)
        .create(&connection.get())?;

    let t2 = transaction_service
        .expense()
        .amount(Money::MAX)
        .account_id(&account.id)
        .create(&connection.get())?;

    let length = transaction_service.fetch_all()?.len();
    assert_eq!(length, 2);
    transaction_service.delete_all(&[t1.id, t2.id])?;
    let length = transaction_service.fetch_all()?.len();
    assert_eq!(length, 0);
    Ok(())
}

#[test]
fn delete_empty_slice() -> folio_lib::Result<()> {
    let connection = create_test_db()?;
    let account_service = AccountService::new(connection.clone());
    let transaction_service = TransactionService::new(connection.clone());
    let account = account_service.create_account("__", Money::ZERO)?;
    transaction_service
        .expense()
        .amount(Money::MAX)
        .account_id(&account.id)
        .create(&connection.get())?;

    transaction_service.delete_all::<String>(&[])?;
    let length = transaction_service.fetch_all()?.len();
    assert_eq!(length, 1);
    Ok(())
}

#[test]
fn delete_only_affected_transactions() -> folio_lib::Result<()> {
    let connection = create_test_db()?;
    let account_service = AccountService::new(connection.clone());
    let transaction_service = TransactionService::new(connection.clone());
    let account = account_service.create_account("__", Money::ZERO)?;
    let t1 = transaction_service
        .expense()
        .amount(Money::MAX)
        .account_id(&account.id)
        .create(&connection.get())?;
    transaction_service
        .expense()
        .amount(Money::MAX)
        .account_id(&account.id)
        .create(&connection.get())?;
    let length = transaction_service.fetch_all()?.len();
    assert_eq!(length, 2);
    transaction_service.delete_all(&[t1.id])?;
    let length = transaction_service.fetch_all()?.len();
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

#[test]
fn set_outflow_for_expense() -> folio_lib::Result<()> {
    let connection = create_test_db()?;
    let transaction_service = TransactionService::new(connection.clone());
    let account_service = AccountService::new(connection.clone());
    let account = account_service.create_account("__", Money::ZERO)?;
    let transaction = transaction_service
        .expense()
        .amount(Money::MAX)
        .account_id(&account.id)
        .create(&connection.get())?;

    transaction_service.set_outflow(&transaction.id, Money::from_f64(10.0))?;
    let t = transaction_service.fetch(&transaction.id)?;
    assert_eq!(t.amount, Money::from_f64(10.0));
    assert_eq!(
        t.from_account_id.unwrap(),
        transaction.from_account_id.unwrap()
    );
    assert!(t.to_account_id.is_none());
    Ok(())
}

#[test]
fn set_payee_for_expense() -> folio_lib::Result<()> {
    let connection = create_test_db()?;
    let account_service = AccountService::new(connection.clone());
    let transaction_service = TransactionService::new(connection.clone());
    let account = account_service.create_account("__", Money::ZERO)?;
    let account2 = account_service.create_account("__", Money::ZERO)?;
    let transaction = transaction_service
        .expense()
        .amount(Money::MAX)
        .account_id(&account.id)
        .create(&connection.get())?;

    transaction_service.set_payee(&transaction.id, &account2.id)?;
    let t = transaction_service.fetch(&transaction.id)?;
    assert_eq!(t.from_account_id.unwrap(), account.id);
    assert_eq!(t.to_account_id.unwrap(), account2.id);
    Ok(())
}

#[test]
fn set_account_for_expense() -> folio_lib::Result<()> {
    let connection = create_test_db()?;
    let account_service = AccountService::new(connection.clone());
    let transaction_service = TransactionService::new(connection.clone());
    let account = account_service.create_account("__", Money::ZERO)?;
    let account2 = account_service.create_account("__", Money::ZERO)?;
    let transaction = transaction_service
        .expense()
        .amount(Money::ZERO)
        .account_id(&account.id)
        .create(&connection.get())?;

    transaction_service.set_account(&transaction.id, &account2.id)?;
    let t = transaction_service.fetch(&transaction.id)?;
    assert_eq!(t.from_account_id.unwrap(), account2.id);
    assert_eq!(t.to_account_id, None);
    Ok(())
}

#[test]
fn set_account_for_income() -> folio_lib::Result<()> {
    let connection = create_test_db()?;
    let account_service = AccountService::new(connection.clone());
    let transaction_service = TransactionService::new(connection.clone());
    let account = account_service.create_account("__", Money::ZERO)?;
    let account2 = account_service.create_account("__", Money::ZERO)?;
    let transaction = transaction_service
        .income()
        .amount(Money::ZERO)
        .account_id(&account.id)
        .create(&connection.get())?;

    transaction_service.set_account(&transaction.id, &account2.id)?;
    let t = transaction_service.fetch(&transaction.id)?;
    assert_eq!(t.to_account_id.unwrap(), account2.id);
    assert_eq!(t.from_account_id, None);
    Ok(())
}

#[test]
fn set_account_for_transfer() -> folio_lib::Result<()> {
    let connection = create_test_db()?;
    let account_service = AccountService::new(connection.clone());
    let transaction_service = TransactionService::new(connection.clone());
    let account = account_service.create_account("__", Money::ZERO)?;
    let account2 = account_service.create_account("__", Money::ZERO)?;
    let account3 = account_service.create_account("__", Money::ZERO)?;
    let transaction = transaction_service
        .transfer()
        .amount(Money::ZERO)
        .accounts(&account.id, &account2.id)
        .create(&connection.get())?;

    transaction_service.set_account(&transaction.id, &account3.id)?;
    let t = transaction_service.fetch(&transaction.id)?;
    assert_eq!(t.from_account_id.unwrap(), account3.id);
    assert_eq!(t.to_account_id.unwrap(), account2.id);
    Ok(())
}

#[test]
fn set_payee_for_income() -> folio_lib::Result<()> {
    let connection = create_test_db()?;
    let account_service = AccountService::new(connection.clone());
    let transaction_service = TransactionService::new(connection.clone());

    let account = account_service.create_account("__", Money::ZERO)?;
    let account2 = account_service.create_account("__", Money::ZERO)?;
    let transaction = transaction_service
        .income()
        .amount(Money::MAX)
        .account_id(&account.id)
        .create(&connection.get())?;

    transaction_service.set_payee(&transaction.id, &account2.id)?;
    let t = transaction_service.fetch(&transaction.id)?;
    assert_eq!(t.from_account_id.unwrap(), account.id);
    assert_eq!(t.to_account_id.unwrap(), account2.id);
    Ok(())
}

#[test]
fn set_payee_for_transfer() -> folio_lib::Result<()> {
    let connection = create_test_db()?;
    let account_service = AccountService::new(connection.clone());
    let transaction_service = TransactionService::new(connection.clone());
    let account = account_service.create_account("__", Money::ZERO)?;
    let account2 = account_service.create_account("__", Money::ZERO)?;
    let account3 = account_service.create_account("__", Money::ZERO)?;
    let transaction = transaction_service
        .transfer()
        .amount(Money::MAX)
        .accounts(&account.id, &account2.id)
        .create(&connection.get())?;

    transaction_service.set_payee(&transaction.id, &account3.id)?;
    let t = transaction_service.fetch(&transaction.id)?;
    assert_eq!(t.from_account_id.unwrap(), account.id);
    assert_eq!(t.to_account_id.unwrap(), account3.id);
    Ok(())
}

#[test]
fn set_payee_removes_category() -> folio_lib::Result<()> {
    let connection = create_test_db()?;
    let service = CategoryService::new(connection.clone());
    let account_service = AccountService::new(connection.clone());
    let transaction_service = TransactionService::new(connection.clone());

    let account = account_service.create_account("__", Money::ZERO)?;
    let account2 = account_service.create_account("__", Money::ZERO)?;
    let category = service.create_category("")?;
    let transaction = transaction_service
        .income()
        .amount(Money::MAX)
        .category(&category.id)
        .account_id(&account.id)
        .create(&connection.get())?;

    transaction_service.set_payee(&transaction.id, &account2.id)?;
    let t = transaction_service.fetch(&transaction.id)?;
    assert!(t.category_id.is_none());
    Ok(())
}

#[test]
fn set_inflow_for_income() -> folio_lib::Result<()> {
    let connection = create_test_db()?;
    let transaction_service = TransactionService::new(connection.clone());
    let account_service = AccountService::new(connection.clone());
    let account = account_service.create_account("__", Money::ZERO)?;
    let transaction = transaction_service
        .income()
        .amount(Money::MAX)
        .account_id(&account.id)
        .create(&connection.get())?;

    transaction_service.set_inflow(&transaction.id, Money::from_f64(10.0))?;
    let t = transaction_service.fetch(&transaction.id)?;
    assert_eq!(t.amount, Money::from_f64(10.0));
    assert_eq!(t.to_account_id.unwrap(), transaction.to_account_id.unwrap());
    assert!(t.from_account_id.is_none());
    Ok(())
}

#[test]
fn set_inflow_for_transfer() -> folio_lib::Result<()> {
    let connection = create_test_db()?;
    let account_service = AccountService::new(connection.clone());
    let transaction_service = TransactionService::new(connection.clone());
    let account = account_service.create_account("__", Money::ZERO)?;
    let account2 = account_service.create_account("__", Money::ZERO)?;
    let transaction = transaction_service
        .transfer()
        .amount(Money::MAX)
        .accounts(&account.id, &account2.id)
        .create(&connection.get())?;

    let result = transaction_service.set_inflow(&transaction.id, Money::from_f64(10.0));
    assert!(result.is_err());
    Ok(())
}

#[test]
fn set_inflow_for_expense() -> folio_lib::Result<()> {
    let connection = create_test_db()?;
    let transaction_service = TransactionService::new(connection.clone());
    let account_service = AccountService::new(connection.clone());
    let account = account_service.create_account("__", Money::ZERO)?;
    let transaction = transaction_service
        .expense()
        .amount(Money::MAX)
        .account_id(&account.id)
        .create(&connection.get())?;

    transaction_service.set_inflow(&transaction.id, Money::from_f64(10.0))?;
    let t = transaction_service.fetch(&transaction.id)?;
    assert_eq!(t.amount, Money::from_f64(10.0));
    assert_eq!(
        t.to_account_id.unwrap(),
        transaction.from_account_id.unwrap()
    );
    assert!(t.from_account_id.is_none());
    Ok(())
}

#[test]
fn set_outflow_for_income() -> folio_lib::Result<()> {
    let connection = create_test_db()?;
    let account_service = AccountService::new(connection.clone());
    let transaction_service = TransactionService::new(connection.clone());
    // Setting outflow on an income should turn it into an expense
    let account = account_service.create_account("__", Money::ZERO)?;
    let transaction = transaction_service
        .income()
        .amount(Money::MAX)
        .account_id(&account.id)
        .create(&connection.get())?;

    transaction_service.set_outflow(&transaction.id, Money::from_f64(10.0))?;
    let t = transaction_service.fetch(&transaction.id)?;
    assert_eq!(t.amount, Money::from_f64(10.0));
    assert_eq!(
        t.from_account_id.unwrap(),
        transaction.to_account_id.unwrap()
    );
    assert!(t.to_account_id.is_none());
    Ok(())
}

#[test]
fn fetch_all() -> folio_lib::Result<()> {
    let connection = create_test_db()?;
    let account_service = AccountService::new(connection.clone());

    let transaction_service = TransactionService::new(connection.clone());

    let account = account_service.create_account("__", Money::ZERO)?;
    let transaction = transaction_service
        .income()
        .amount(Money::MAX)
        .account_id(&account.id)
        .create(&connection.get())?;

    let t = transaction_service.fetch_all()?;
    assert_eq!(t.len(), 1);
    assert_eq!(t[0].id, transaction.id);
    Ok(())
}
