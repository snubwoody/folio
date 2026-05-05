import { describe, expect, test } from "vitest";
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
// 
describe("spendingAnalytics", () => {
    test("get total spent in a single category", () => {
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
        expect(analytics[0].total).toStrictEqual(250)
    });
    
    test("exclude transactions without a category", () => {
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
                amount: "500",
                toAccountId: undefined,
                fromAccountId: undefined,
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
        expect(analytics[0].total).toStrictEqual(50)
    });

    test("only include transactions in current month", () => {
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
                amount: "500",
                toAccountId: undefined,
                fromAccountId: undefined,
                date: currentDate.add({ months:1 })
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
        expect(analytics[0].total).toStrictEqual(50)
    });

    test("exclude incomes and transfers", () => {
        const currentDate = today(getLocalTimeZone());
        const category: Category = {
            id: "C1",
            title: "Groceries",
            createdAt: "",
            isIncomeStream: false
        };

        const transactions: Transaction[] = [
            {
                id: "T1",
                amount: "500",
                toAccountId: "A1",
                fromAccountId: undefined,
                date: currentDate.add({ months:1 })
            },
            {
                id: "T2",
                categoryId: category.id,
                toAccountId: "A1",
                fromAccountId: "A2",
                amount: "50",
                date: currentDate
            },
            {
                id: "T3",
                categoryId: category.id,
                fromAccountId: "A1",
                amount: "25",
                date: currentDate
            }
        ];

        const categoryMap = new SvelteMap<string, Category>();
        categoryMap.set("C1", category);

        const analytics = spendingAnalytics(transactions, categoryMap, { month: currentDate });
        expect(analytics[0].total).toStrictEqual(25)
    });
})
