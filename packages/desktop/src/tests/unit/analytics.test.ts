import { calculateSpendingAnalytics } from "$lib/analytics";
import { expect, test } from "vitest";
import type { Category, Expense, IncomeAnalytic, IncomeStream } from "$lib/lib";
import { AppStore } from "$lib/state.svelte";

test("Calculate spending analytics",() => {

    const categories: Category[] = [
        { id: "1",title: "Rent",createdAt: "" },
        { id: "2",title: "Entertainment",createdAt: "" }
    ];

    const date = new Date().toISOString();
    const expenses: Expense[] = [
        {
            id: "",
            amount: "200",
            currencyCode: "CAD",
            date,
            category: categories[0]
        },
        {
            id: "",
            amount: "250",
            currencyCode: "CAD",
            date,
            category: categories[0]
        },
        {
            id: "",
            amount: "10.25",
            currencyCode: "CAD",
            date,
            category: categories[1]
        }
    ];

    const analytics = calculateSpendingAnalytics(expenses);
    expect(analytics[0].category.title).toBe("Rent");
    expect(analytics[0].total).toBe(450);
    expect(analytics[1].category.title).toBe("Entertainment");
    expect(analytics[1].total).toBe(10.25);
});

test("Skip malformed dates",() => {

    const categories: Category[] = [
        { id: "1",title: "Rent",createdAt: "" },
        { id: "2",title: "Entertainment",createdAt: "" }
    ];

    const date = new Date();

    const expenses: Expense[] = [
        {
            id: "",
            amount: "200",
            currencyCode: "CAD",
            date: date.toISOString(),
            category: categories[0]
        },
        {
            id: "",
            amount: "250",
            currencyCode: "CAD",
            date: date.toISOString(),
            category: categories[1]
        },
        {
            id: "",
            amount: "10.25",
            currencyCode: "CAD",
            date: "not-a-date",
            category: categories[1]
        }
    ];

    const analytics = calculateSpendingAnalytics(expenses);
    expect(analytics).toHaveLength(2);
});

test("Filter by current month",() => {

    const categories: Category[] = [
        { id: "1",title: "Rent",createdAt: "" }
    ];

    const past = new Date(2000).toISOString();
    const today = new Date().toISOString();
    const future = new Date(4000).toISOString();

    const expenses: Expense[] = [
        {
            id: "",
            amount: "200",
            currencyCode: "CAD",
            date: today,
            category: categories[0]
        },
        {
            id: "",
            amount: "250",
            currencyCode: "CAD",
            date: past,
            category: categories[0]
        },
        {
            id: "",
            amount: "10.25",
            currencyCode: "CAD",
            date: future,
            category: categories[0]
        }
    ];

    const analytics = calculateSpendingAnalytics(expenses);
    expect(analytics[0].total).toBe(200);
});

test("Parse date only strings",() => {
    const categories: Category[] = [
        { id: "1",title: "Rent",createdAt: "" }
    ];

    // Strip the timezone and extract the date
    const past = new Date(2000).toISOString().split("T")[0];
    const today = new Date().toISOString().split("T")[0];
    const future = new Date(4000).toISOString().split("T")[0];

    const expenses: Expense[] = [
        {
            id: "today",
            amount: "200",
            currencyCode: "CAD",
            date: today,
            category: categories[0]
        },
        {
            id: "past",
            amount: "250",
            currencyCode: "CAD",
            date: past,
            category: categories[0]
        },
        {
            id: "future",
            amount: "10.25",
            currencyCode: "CAD",
            date: future,
            category: categories[0]
        }
    ];

    const analytics = calculateSpendingAnalytics(expenses);
    expect(analytics[0].total).toBe(200);
});

test("Sort income analytics",() => {
    const incomeStream: IncomeStream = {
        id: "",
        title: "",
        createdAt: ""
    };
    const analytics: IncomeAnalytic[] = [
        { stream: incomeStream, total: "200.00" },
        { stream: incomeStream, total: "500.00" },
        { stream: incomeStream, total: "10.00" },
        { stream: incomeStream, total: "-24.00" }
    ];
    const appStore = new AppStore();
    appStore.incomeAnalytics = analytics;
    const incomeAnalytics = appStore.sortedIncomeAnalytics();
    expect(incomeAnalytics[0].total).toBe("500.00");
    expect(incomeAnalytics[1].total).toBe("200.00");
    expect(incomeAnalytics[2].total).toBe("10.00");
    expect(incomeAnalytics[3].total).toBe("-24.00");
});