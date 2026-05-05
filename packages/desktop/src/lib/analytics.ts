import { SvelteMap } from "svelte/reactivity";
import { type CalendarDate, getLocalTimeZone, isSameMonth, today } from "@internationalized/date";
import type { Category } from "$lib/types";
import type { Transaction } from "$lib/api/transaction";

export type SpendingAnalyticsOptions = {
    month?: CalendarDate;
};

export type SpendingAnalytic = {
    category: Category;
    total: number;
    /**
     * A percentage of the total amount, ranging from 0-1.
     */
    percentage: number;
}
/**
 * Returns a list of total spending per category.
 */
export function spendingAnalytics(
    transactions: Transaction[],
    categoryMap: SvelteMap<string, Category>,
    opts: SpendingAnalyticsOptions
): SpendingAnalytic[] {
    const month = opts.month ?? today(getLocalTimeZone());
    let map = new SvelteMap<string,number>();
    const filteredTransactions = transactions
        .filter(t => t.categoryId !== undefined)
        .filter(t => t.toAccountId === undefined && t.fromAccountId !== undefined) // Exclude incomes and transfers
        .filter(t => isSameMonth(t.date,month));

    const totalSpent = filteredTransactions
        .map(t => parseFloat(t.amount))
        .reduce((prev, curr) => prev + curr, 0);

    for (const t of filteredTransactions) {
        if (!t.categoryId) continue;

        const categoryId = categoryMap.get(t.categoryId)?.id;
        if (!categoryId) continue;

        let value = map.get(categoryId);
        if (value === undefined){
            map.set(categoryId,parseFloat(t.amount));
            continue;
        }
        map.set(categoryId, parseFloat(t.amount) + value);
    }

    const analytics: SpendingAnalytic[] = []
    for (const [categoryId,total] of map.entries()) {
        let category = categoryMap.get(categoryId);
        if (!category) continue;

        // Prevent division by 0
        const percentage = totalSpent === 0 ? 0 : total / totalSpent;
        const analytic: SpendingAnalytic = {
            category,
            total,
            percentage
        };

        analytics.push(analytic)
    }

    return analytics;
}
