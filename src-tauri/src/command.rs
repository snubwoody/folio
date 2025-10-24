use crate::{
    Money, Result, State,
    analytics::{self, IncomeAnalytic, SpendingAnalytic},
    service::{self, *},
};
use std::str::FromStr;

#[tauri::command]
pub async fn create_expense(state: tauri::State<'_, State>, data: CreateExpense) -> Result<()> {
    Expense::create(data, &state.pool).await?;
    Ok(())
}

#[tauri::command]
pub async fn create_income(state: tauri::State<'_, State>, data: CreateIncome) -> Result<()> {
    Income::create(data, &state.pool).await?;
    Ok(())
}

#[tauri::command]
pub async fn edit_expense(
    state: tauri::State<'_, State>,
    id: String,
    data: EditExpense,
) -> Result<()> {
    Expense::update(&id, data, &state.pool).await?;
    Ok(())
}

#[tauri::command]
pub async fn edit_income(
    state: tauri::State<'_, State>,
    id: String,
    data: EditIncome,
) -> Result<()> {
    Income::update(&id, data, &state.pool).await?;
    Ok(())
}

#[tauri::command]
pub async fn spending_analytics(state: tauri::State<'_, State>) -> Result<Vec<SpendingAnalytic>> {
    analytics::spending_analytics(&state.pool).await
}

#[tauri::command]
pub async fn income_analytics(state: tauri::State<'_, State>) -> Result<Vec<IncomeAnalytic>> {
    analytics::income_analytics(&state.pool).await
}

#[tauri::command]
pub async fn create_account(
    state: tauri::State<'_, State>,
    name: &str,
    starting_balance: Money,
) -> Result<()> {
    Account::create(name, starting_balance, &state.pool).await?;
    Ok(())
}

#[tauri::command]
pub async fn fetch_accounts(state: tauri::State<'_, State>) -> Result<Vec<Account>> {
    service::fetch_accounts(&state.pool).await
}

#[tauri::command]
pub async fn fetch_expenses(state: tauri::State<'_, State>) -> Result<Vec<Expense>> {
    let expenses = service::fetch_expenses(&state.pool).await?;
    Ok(expenses)
}

#[tauri::command]
pub async fn fetch_incomes(state: tauri::State<'_, State>) -> Result<Vec<Income>> {
    let expenses = service::fetch_incomes(&state.pool).await?;
    Ok(expenses)
}

#[tauri::command]
pub async fn fetch_categories(state: tauri::State<'_, State>) -> Result<Vec<Category>> {
    service::fetch_categories(&state.pool).await
}

#[tauri::command]
pub async fn fetch_income_streams(state: tauri::State<'_, State>) -> Result<Vec<IncomeStream>> {
    service::fetch_income_streams(&state.pool).await
}

#[tauri::command]
pub async fn fetch_budgets(state: tauri::State<'_, State>) -> Result<Vec<Budget>> {
    service::fetch_budgets(&state.pool).await
}

#[tauri::command]
pub async fn create_budget(
    amount: &str,
    category_id: &str,
    state: tauri::State<'_, State>,
) -> Result<()> {
    Budget::create(Money::from_str(amount)?, category_id, &state.pool).await?;
    Ok(())
}

#[tauri::command]
pub async fn create_category(state: tauri::State<'_, State>, title: &str) -> Result<()> {
    Category::create(title, &state.pool).await?;
    Ok(())
}

#[tauri::command]
pub async fn create_income_stream(state: tauri::State<'_, State>, title: &str) -> Result<()> {
    IncomeStream::create(title, &state.pool).await?;
    Ok(())
}
