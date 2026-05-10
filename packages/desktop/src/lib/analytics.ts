import { SvelteMap } from "svelte/reactivity";
import { type CalendarDate, getLocalTimeZone, isSameMonth, today } from "@internationalized/date";
import type { Category } from "$lib/types";
import type { Transaction } from "$lib/api/transaction";

const colors = [
     "#C3B9F8", // Purple 200
     "#5B28D4", // Purple 600
     "#7048E9", // Purple 500
     "#A796F4", // Purple 300
     "#896FEF", // Purple 400
     "#471EA9", // Purple 700
     "#EEECFD", // Purple 50
     "#E0DCFB", // Purple 100
     "#31137A", // Purple 800
     "#1D084F", // Purple 900
     "#110434", // Purple 950
];

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
    color: string;
};
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
        .filter(t => !t.toAccountId && t.fromAccountId) // Exclude incomes and transfers
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

    const analytics: SpendingAnalytic[] = [];
    let index = 0;
    for (const [categoryId,total] of map.entries()) {
        let category = categoryMap.get(categoryId);
        if (!category) continue;

        // Prevent division by 0
        const percentage = totalSpent === 0 ? 0 : total / totalSpent;
        const analytic: SpendingAnalytic = {
            category,
            total,
            percentage,
            color: colors[index % colors.length],
        };

        analytics.push(analytic);
        index += 1;
    }

    return analytics;
}
