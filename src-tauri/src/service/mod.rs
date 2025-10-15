mod account;
mod category;
mod expense;
mod income;
mod income_stream;

pub use account::*;
pub use category::*;
pub use expense::*;
pub use income::*;
pub use income_stream::*;

// TODO:

// struct BudgetOverview{
//     amount_spend: f32,
//     amount_left: f32,
//     total_income: f32,
//     total_expense: f32,
//     percentage_of_income: f32
// }
