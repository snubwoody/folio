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

import type { Expense,Category } from "./lib";
import { isThisMonth, parseISO } from "date-fns";

export type SpendingAnalytic = {
    category: Category,
    total: number
};

/**
 * Calculates the amount spent per category.
 * @param expenses An array of expenses.
 */
export const calculateSpendingAnalytics = (expenses: Expense[] ): SpendingAnalytic[] => {
    expenses = expenses
        .filter(expense => expense.category !== null)
        .filter(expense => {
            try{
                return isThisMonth(parseISO(expense.date));
            } catch{
                return false;
            }
        });
    const categories = new Set<string>();
    expenses.forEach(expense => {
        categories.add(expense.category!.id);
    });
    const analytics: SpendingAnalytic[] = [];
    for (const id of categories){
        const e = expenses
            .filter(expense => expense.category!.id === id);
        const category = e[0].category!;

        const total = expenses
            .filter(expense => expense.category!.id === id)
            .reduce((previous,current) => previous + parseFloat(current.amount),0);

        analytics.push({ category,total });
    }
    return analytics;
};