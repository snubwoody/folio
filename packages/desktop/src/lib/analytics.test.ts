import { expect, test } from "vitest";
import type { Category } from "$lib/types"
import type { Transaction } from "$lib/api/transaction";
import { getLocalTimeZone, today } from "@internationalized/date";
import { SvelteMap } from "svelte/reactivity";
import { spendingAnalytics } from "./analytics";

// TODO: probably make a seperate PR: Extract analytics into spendingAnalytics function
// TODO: test
// - Transactions with no category
// - Multiple categories
// - Exclude incomes and transfers
// - Exclude incomeStreams
test("Get total spent in a single category", () => {
    const currentDate = today(getLocalTimeZone());
    const category: Category = {
        id: "C1",
        title: "Groceries",
        createdAt: "",
        isIncomeStream: false
    };

    const transactions: Transaction[] = [
        {
            id: "A1",
            categoryId: category.id,
            toAccountId: undefined,
            fromAccountId: undefined,
            amount: "200",
            date: currentDate
        },
        {
            id: "A2",
            categoryId: category.id,
            toAccountId: undefined,
            fromAccountId: undefined,
            amount: "50",
            date: currentDate
        }
    ];

    const categoryMap = new SvelteMap<string, Category>();
    categoryMap.set("C1", category);

    const analytics = spendingAnalytics(transactions, categoryMap, { month: currentDate });
    expect(analytics.get("Groceries")).toStrictEqual(250)
});
