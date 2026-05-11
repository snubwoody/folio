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
use serde::{Deserialize, Serialize};

use crate::{Money, service::Category};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Budget {
    pub id: String,
    pub amount: Money,
    pub total_spent: Money,
    pub category_id: String,
    pub created_at: DateTime<Utc>,
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::CategoryService;
    use sqlx::SqlitePool;

    #[sqlx::test]
    async fn fetch_budgets(pool: SqlitePool) -> crate::Result<()> {
        let service = CategoryService::new(pool.clone());
        let len = service.fetch_budgets().await?.len();
        service.create_category("").await?;
        let budgets = service.fetch_budgets().await?;
        assert_eq!(budgets.len(), len + 1);
        Ok(())
    }

    #[sqlx::test]
    async fn edit_budget(pool: SqlitePool) -> crate::Result<()> {
        let service = CategoryService::new(pool.clone());
        let category = service.create_category("MINE__").await?;
        let budget = service.fetch_budget_from_category(&category.id).await?;
        service
            .edit_budget(&budget.id, Money::from_f64(244.00))
            .await?;
        let b = service.fetch_budget_from_category(&category.id).await?;

        assert_eq!(b.amount, Money::from_f64(244.00));
        Ok(())
    }

    #[sqlx::test]
    async fn get_budget(pool: SqlitePool) -> crate::Result<()> {
        let service = CategoryService::new(pool.clone());
        let category = service.create_category("__").await?;
        let budget = service.fetch_budget_from_category(&category.id).await?;
        assert_eq!(budget.amount, Money::ZERO);
        Ok(())
    }

    #[sqlx::test]
    async fn remaining_caps_at_zero(pool: SqlitePool) -> crate::Result<()> {
        let service = CategoryService::new(pool.clone());
        let category = service.create_category("__").await?;
        let budget = service.fetch_budget_from_category(&category.id).await?;
        assert_eq!(budget.amount, Money::ZERO);
        Ok(())
    }
}
