import { appStore } from "$lib/state.svelte";
import ExpenseTable from "$components/home/ExpenseTable.svelte";
import TransactionsSection from "$components/home/TransactionsSection.svelte";
import { test, beforeEach, expect, describe } from "vitest";
import { render } from "vitest-browser-svelte";
import type { Account, Expense, Category } from "$lib/lib";

beforeEach(() => {
    appStore.budgets = [];
    appStore.incomes = [];
    appStore.expenses = [];
});

describe("Expense table", () => {
    test("show heading", async () => {
        const page = render(ExpenseTable);
        expect(page.getByText("Category")).toBeInTheDocument();
        expect(page.getByText("Account")).toBeInTheDocument();
        expect(page.getByText("Date")).toBeInTheDocument();
        expect(page.getByText("Amount")).toBeInTheDocument();
    });

    test("show expenses", async () => {
        appStore.settings.currencyCode = "CAD";
        const account: Account= {
            id: "24",
            startingBalance: "24",
            balance: "24",
            name: "Account"
        };
        appStore.accounts = [account];
        appStore.expenses = [
            { id: "1", amount: "0",date: "2025-10-11",currencyCode: "CAD", account },
            { id: "2", amount: "500",date: "2025-10-10",currencyCode: "CAD",account },
            { id: "3", amount: "24.24",date: "2025-09-01",currencyCode: "CAD",account }
        ];
        const page = render(ExpenseTable);
        expect(page.getByText("Oct 10, 2025")).toBeInTheDocument();
        expect(page.getByText("CA$").first()).toBeInTheDocument();
    });

    test("show expense category", async () => {
        appStore.settings.currencyCode = "CAD";
        const category: Category = {
            id: "24",
            title: "Transport",
            createdAt: "2025-01-01"
        };
        const expense: Expense = {
            id: "1",
            amount: "22.24",
            date: "2024-09-09",
            category,
            currencyCode: "USD"
        };
        appStore.categories = [category];
        appStore.expenses = [expense];
        const page = render(ExpenseTable);
        expect(page.getByText("Transport")).toBeInTheDocument();
    });

});

test("Show add transaction button", async () => {
    const page = render(TransactionsSection);
    expect(page.getByRole("button", { name: "New" })).toBeInTheDocument();
});

test("Open add expense form", async () => {
    const page = render(TransactionsSection);
    await page.getByRole("button", { name: "New" }).click();
    const fields = page.getByRole("textbox").all();
    expect(fields).toHaveLength(2);
});

