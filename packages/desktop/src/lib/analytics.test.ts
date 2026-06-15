import { describe, expect, test } from "vitest";
import type { Category } from "$lib/types";
import type { Transaction } from "$lib/api/transaction";
import { getLocalTimeZone, today } from "@internationalized/date";
import { SvelteMap } from "svelte/reactivity";
import { spendingAnalytics } from "./analytics";

describe("spendingAnalytics", () => {
    test("get total spent in a single category", () => {
        const currentDate = today(getLocalTimeZone());
        const category: Category = {
            id: "C1",
            title: "Groceries",
            createdAt: "",
            isIncomeStream: false,
        };

        const transactions: Transaction[] = [
            {
                id: "A1",
                categoryId: category.id,
                fromAccountId: "A1",
                amount: "200",
                date: currentDate,
            },
            {
                id: "A2",
                categoryId: category.id,
                fromAccountId: "A1",
                amount: "50",
                date: currentDate,
            },
        ];

        const categoryMap = new SvelteMap<string, Category>();
        categoryMap.set("C1", category);

        const analytics = spendingAnalytics(transactions, categoryMap, {
            month: currentDate,
        });
        expect(analytics[0].category).toStrictEqual(category);
        expect(analytics[0].total).toStrictEqual(250);
    });

    test("empty transactions", () => {
        const currentDate = today(getLocalTimeZone());
        const category: Category = {
            id: "C1",
            title: "Groceries",
            createdAt: "",
            isIncomeStream: false,
        };

        const transactions: Transaction[] = [];

        const categoryMap = new SvelteMap<string, Category>();
        categoryMap.set("C1", category);

        const analytics = spendingAnalytics(transactions, categoryMap, {
            month: currentDate,
        });
        expect(analytics).toHaveLength(0);
    });

    test("empty categoryMap", () => {
        const currentDate = today(getLocalTimeZone());
        const category: Category = {
            id: "C1",
            title: "Groceries",
            createdAt: "",
            isIncomeStream: false,
        };

        const transactions: Transaction[] = [
            {
                id: "A1",
                categoryId: category.id,
                fromAccountId: "A1",
                amount: "200",
                date: currentDate,
            },
            {
                id: "A2",
                categoryId: category.id,
                fromAccountId: "A1",
                amount: "50",
                date: currentDate,
            },
        ];

        const categoryMap = new SvelteMap<string, Category>();

        const analytics = spendingAnalytics(transactions, categoryMap, {
            month: currentDate,
        });
        expect(analytics).toHaveLength(0);
    });

    test("total amount is 0", () => {
        const currentDate = today(getLocalTimeZone());
        const category: Category = {
            id: "C1",
            title: "Groceries",
            createdAt: "",
            isIncomeStream: false,
        };

        const transactions: Transaction[] = [
            {
                id: "A1",
                categoryId: category.id,
                fromAccountId: "A1",
                amount: "0",
                date: currentDate,
            },
        ];

        const categoryMap = new SvelteMap<string, Category>();
        categoryMap.set("C1", category);

        const analytics = spendingAnalytics(transactions, categoryMap, {
            month: currentDate,
        });
        expect(analytics[0].total).toStrictEqual(0);
        expect(analytics[0].percentage).toStrictEqual(0);
    });

    test("include income streams", () => {
        // Not sure whether income streams should be filterd out or kept.
        // They may be filtered out in the future, which will break user analytics.
        const currentDate = today(getLocalTimeZone());
        const c1: Category = {
            id: "C1",
            title: "Groceries",
            createdAt: "",
            isIncomeStream: true,
        };

        const c2: Category = {
            id: "C2",
            title: "Shopping",
            createdAt: "",
            isIncomeStream: false,
        };

        const transactions: Transaction[] = [
            {
                id: "A1",
                categoryId: c1.id,
                fromAccountId: "A1",
                amount: "100",
                date: currentDate,
            },
            {
                id: "A2",
                categoryId: c2.id,
                fromAccountId: "A1",
                amount: "100",
                date: currentDate,
            },
        ];

        const categoryMap = new SvelteMap<string, Category>();
        categoryMap.set("C1", c1);
        categoryMap.set("C2", c2);

        const analytics = spendingAnalytics(transactions, categoryMap, {
            month: currentDate,
        });
        expect(analytics[0].total).toStrictEqual(100);
        expect(analytics[0].percentage).toStrictEqual(0.5);
    });

    test("category percentage", () => {
        const currentDate = today(getLocalTimeZone());
        const c1: Category = {
            id: "C1",
            title: "Groceries",
            createdAt: "",
            isIncomeStream: false,
        };

        const c2: Category = {
            id: "C2",
            title: "Shopping",
            createdAt: "",
            isIncomeStream: false,
        };

        const transactions: Transaction[] = [
            {
                id: "A1",
                categoryId: c1.id,
                fromAccountId: "A1",
                amount: "100",
                date: currentDate,
            },
            {
                id: "A2",
                categoryId: c2.id,
                fromAccountId: "A1",
                amount: "100",
                date: currentDate,
            },
        ];

        const categoryMap = new SvelteMap<string, Category>();
        categoryMap.set("C1", c1);
        categoryMap.set("C2", c2);

        const analytics = spendingAnalytics(transactions, categoryMap, {
            month: currentDate,
        });
        expect(analytics[0].percentage).toStrictEqual(0.5);
        expect(analytics[1].percentage).toStrictEqual(0.5);
    });

    test("exclude transactions without a category", () => {
        const currentDate = today(getLocalTimeZone());
        const category: Category = {
            id: "C1",
            title: "Groceries",
            createdAt: "",
            isIncomeStream: false,
        };

        const transactions: Transaction[] = [
            {
                id: "A1",
                amount: "500",
                fromAccountId: "A1",
                date: currentDate,
            },
            {
                id: "A2",
                categoryId: category.id,
                fromAccountId: "A1",
                amount: "50",
                date: currentDate,
            },
        ];

        const categoryMap = new SvelteMap<string, Category>();
        categoryMap.set("C1", category);

        const analytics = spendingAnalytics(transactions, categoryMap, {
            month: currentDate,
        });
        expect(analytics[0].total).toStrictEqual(50);
    });

    test("only include transactions in current month", () => {
        const currentDate = today(getLocalTimeZone());
        const category: Category = {
            id: "C1",
            title: "Groceries",
            createdAt: "",
            isIncomeStream: false,
        };

        const transactions: Transaction[] = [
            {
                id: "A1",
                amount: "500",
                fromAccountId: "A1",
                date: currentDate.add({ months: 1 }),
            },
            {
                id: "A2",
                categoryId: category.id,
                fromAccountId: "A1",
                amount: "50",
                date: currentDate,
            },
        ];

        const categoryMap = new SvelteMap<string, Category>();
        categoryMap.set("C1", category);

        const analytics = spendingAnalytics(transactions, categoryMap, {
            month: currentDate,
        });
        expect(analytics[0].total).toStrictEqual(50);
    });

    test("exclude incomes and transfers", () => {
        const currentDate = today(getLocalTimeZone());
        const category: Category = {
            id: "C1",
            title: "Groceries",
            createdAt: "",
            isIncomeStream: false,
        };

        const transactions: Transaction[] = [
            {
                id: "T1",
                amount: "500",
                toAccountId: "A1",
                fromAccountId: undefined,
                date: currentDate.add({ months: 1 }),
            },
            {
                id: "T2",
                categoryId: category.id,
                toAccountId: "A1",
                fromAccountId: "A2",
                amount: "50",
                date: currentDate,
            },
            {
                id: "T3",
                categoryId: category.id,
                fromAccountId: "A1",
                amount: "25",
                date: currentDate,
            },
        ];

        const categoryMap = new SvelteMap<string, Category>();
        categoryMap.set("C1", category);

        const analytics = spendingAnalytics(transactions, categoryMap, {
            month: currentDate,
        });
        expect(analytics[0].total).toStrictEqual(25);
    });
});
