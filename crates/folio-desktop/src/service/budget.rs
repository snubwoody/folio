// Copyright (C) 2025 Wakunguma Kalimukwa
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

use chrono::{DateTime, Utc};
use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};
use sqlx::{Row, SqlitePool};
use tracing::{debug, info, warn};

use crate::{Money, db, service::Category, setup_test_db};
use crate::error::ErrorExt;

#[derive(Debug, Clone, Serialize, Deserialize,Default)]
#[serde(rename_all = "camelCase")]
pub struct Budget {
    pub id: String,
    pub amount: Money,
    pub total_spent: Money,
    pub remaining: Money,
    pub category_id: String,
    pub created_at: DateTime<Utc>,
}

impl Budget {
    /// Inserts a new row into the `budgets` table.
    pub(crate) fn create(
        amount: Money,
        category_id: &str,
        conn: &Connection,
    ) -> crate::Result<Self> {
        let mut stmt = conn
            .prepare_cached("INSERT INTO budgets(amount,category_id) VALUES (?1,?2) RETURNING *")?;

        let budget = stmt.query_row(params![amount.inner(),category_id],|row|Self::try_from(row))?;

        info!(category_id=?category_id,id=?budget.id,"Created new budget");
        Ok(budget)
    }

    pub fn edit(id: &str, amount: Money, conn: &Connection) -> crate::Result<Self> {
        let mut stmt = conn
            .prepare_cached("UPDATE budgets SET amount = ?2 WHERE id = ?1 RETURNING *")?;

        let budget = stmt.query_row(params![id,amount.inner()],|row|Self::try_from(row))?;
        info!(id = id, "Updated budget");
        Ok(budget)
    }

    pub async fn from_id(id: &str, pool: &SqlitePool) -> crate::Result<Self> {
        // TODO: remove this type
        // FIXME
        let conn = setup_test_db(pool.connect_options().get_filename()).await;
        let record: db::Budget = sqlx::query_as("SELECT * FROM budgets WHERE id=$1")
            .bind(id)
            .fetch_one(pool)
            .await?;

        let total = Money::new(record.amount);
        let category = Category::fetch(&record.category_id, &conn)?;
        let total_spent = Category::total_spent(&category.id, pool).await?;
        let remaining = (total - total_spent).max(Money::ZERO);
        let created_at = DateTime::from_timestamp(record.created_at, 0).unwrap_or_default();

        Ok(Self {
            id: record.id,
            amount: total,
            category_id: record.category_id,
            total_spent,
            remaining,
            created_at,
        })
    }

    /// Fetches the budget with the corresponding `category_id`.
    pub fn from_category(category_id: &str, conn: &Connection) -> crate::Result<Self> {
        let mut stmt = conn
            .prepare_cached("SELECT * FROM budgets WHERE category_id=?1")?;

        let budget = stmt.query_row([category_id],|row|Self::try_from(row))?;
        Ok(budget)
    }
}

impl<'a> TryFrom<&rusqlite::Row<'a>> for Budget{
    type Error = rusqlite::Error;

    fn try_from(value: &rusqlite::Row) -> Result<Self, Self::Error> {
        let created_at = DateTime::from_timestamp(value.get(3)?,0)
            .unwrap_or_default();

        let budget = Self{
            id: value.get(0)?,
            category_id: value.get(1)?,
            amount: Money::from_scaled(value.get(2)?),
            total_spent: Default::default(), // FIXME: remove/deprecate
            created_at,
            remaining: Default::default(), // FIXME: remove/deprecate
        };

        Ok(budget)
    }
}

pub fn create_missing_budgets(conn: &Connection) -> crate::Result<()> {
    let categories = Category::fetch_all(conn)?;
    let categories = categories.iter().filter(|c|!c.is_income_stream);

    // let budgets = fetch_budgets(pool).await?;
    // FIXME
    let budgets: Vec<Budget> = vec![];

    let mut filtered = vec![];
    for c in categories {
        let mut contains = false;
        for b in &budgets {
            if b.category_id == c.id {
                contains = true;
                break;
            }
        }
        if !contains {
            filtered.push(c);
        }
    }

    let len = filtered.len();
    if len != 0 {
        debug!("Found {len} categories without budgets")
    }

    for c in filtered {
        if let Err(err) = Budget::create(Money::ZERO, &c.id, conn).context("Failed to create budget") {
            warn!("{}",err.report());
        }
    }

    Ok(())
}

pub async fn fetch_budgets(pool: &SqlitePool) -> crate::Result<Vec<Budget>> {
    // FIXME
    let conn = setup_test_db(pool.connect_options().get_filename()).await;
    let records = sqlx::query("SELECT id,category_id FROM budgets")
        .fetch_all(pool)
        .await?;

    let categories = Category::fetch_all(&conn)?;
    let mut categories = categories.iter().filter(|c|!c.is_income_stream);

    let mut budgets = vec![];
    for record in records {
        let id: String = record.get("id");
        let category_id: String = record.get("category_id");
        // Filter budgets with a deleted category
        if !categories.any(|c| c.id == category_id) {
            continue;
        }
        let budget = Budget::from_id(&id, pool).await?;
        budgets.push(budget);
    }
    Ok(budgets)
}

#[cfg(test)]
mod test {
    use crate::setup_test_db;
    use super::*;

    #[sqlx::test]
    async fn fetch_budgets(pool: SqlitePool) -> crate::Result<()> {
        let conn = setup_test_db(pool.connect_options().get_filename()).await;
        let len = super::fetch_budgets(&pool).await?.len();
        Category::create("", &conn)?;
        let budgets = super::fetch_budgets(&pool).await?;
        assert_eq!(budgets.len(), len + 1);
        Ok(())
    }

    #[sqlx::test]
    async fn edit_budget(pool: SqlitePool) -> crate::Result<()> {
        let conn = setup_test_db(pool.connect_options().get_filename()).await;
        let category = Category::create("MINE__", &conn)?;
        let budget = Budget::from_category(&category.id, &conn)?;
        let new_budget = Budget::edit(&budget.id, Money::from_f64(244.00), &conn)?;
        let b = Budget::from_category(&category.id, &conn)?;

        assert_eq!(b.amount, Money::from_f64(244.00));
        assert_eq!(b.amount, new_budget.amount);
        Ok(())
    }

    #[sqlx::test]
    async fn get_budget(pool: SqlitePool) -> crate::Result<()> {
        let conn = setup_test_db(pool.connect_options().get_filename()).await;
        let category = Category::create("__", &conn)?;
        let budget = Budget::from_category(&category.id, &conn)?;
        assert_eq!(budget.amount, Money::ZERO);
        Ok(())
    }

    #[sqlx::test]
    async fn remaining_caps_at_zero(pool: SqlitePool) -> crate::Result<()> {
        let conn = setup_test_db(pool.connect_options().get_filename()).await;
        let category = Category::create("__", &conn)?;
        let budget = Budget::from_category(&category.id, &conn)?;
        assert_eq!(budget.amount, Money::ZERO);
        Ok(())
    }
}
