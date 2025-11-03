import { calculateSpendingAnalytics } from "$lib/analytics";
import { expect, test } from "vitest";
import type { Category, Expense } from "$lib/lib";

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