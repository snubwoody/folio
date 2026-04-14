import type {Transaction} from "$lib/api/transaction";
import {CalendarDate, getLocalTimeZone, isSameMonth, today} from "@internationalized/date";
import type {Budget} from "$lib/types";
import {invoke} from "@tauri-apps/api/core";

export async function getBudget(categoryId: string){
    return await invoke<Budget>("get_budget", {categoryId});
}

export async function editBudget(id: string, amount: string) {
    await invoke("edit_budget", { id, amount });
}

/**
 * Returns the total amount spent in the provided category during in a specific month.
 *
 * @param categoryId The category id.
 * @param transactions The list of transactions.q
 * @param date The month in which the transactions occurred, defaults to the current month.
 */
export function totalSpent(
    categoryId: string,
    transactions: Transaction[],
    date: CalendarDate = today(getLocalTimeZone())
){
    // TODO: add max and min
    const sum = transactions
        .filter(t => t.categoryId === categoryId)
        .filter(t => isSameMonth(t.date,date))
        .map(t => parseFloat(t.amount))
        .reduce((prev,current) => prev + current,0);

    return sum;
}