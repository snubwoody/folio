import { SvelteMap } from "svelte/reactivity";
import { type CalendarDate, getLocalTimeZone, isSameMonth, today } from "@internationalized/date";
import type { Category } from "$lib/types";
import type { Transaction } from "$lib/api/transaction";

export type SpendingAnalyticsOptions = {
    month?: CalendarDate;
};

/**
 * Returns a list of total spending per category.
 */
export function spendingAnalytics(
    transactions: Transaction[],
    categoryMap: SvelteMap<string, Category>,
    opts: SpendingAnalyticsOptions
) {
    const month = opts.month ?? today(getLocalTimeZone());
    let map = new SvelteMap<string,number>();
    const filteredTransactions = transactions
        .filter(t => isSameMonth(t.date,month));

    for (const t of filteredTransactions){
        if (!t.categoryId) continue;

        let category = categoryMap.get(t.categoryId)?.title;
        if (!category) continue;

        let value = map.get(category);
        if (value === undefined){
            map.set(category,parseFloat(t.amount));
            continue;
        }
        map.set(category, parseFloat(t.amount) + value);
    }

    return map
}
