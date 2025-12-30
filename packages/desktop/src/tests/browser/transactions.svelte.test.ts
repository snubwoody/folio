import { appStore } from "$lib/state.svelte";
import ExpenseTable from "$components/home/expense/ExpenseTable.svelte";
import Expense from "$components/home/expense/Expense.svelte";
import TransactionsSection from "$components/home/TransactionsSection.svelte";
import { test, beforeEach, expect } from "vitest";
import { render } from "vitest-browser-svelte";
import type {Account} from "$lib/lib";

beforeEach(() => {
    appStore.budgets = [];
    appStore.incomes = [];
    appStore.expenses = [];
});

test("Expense table heading", async () => {
    const page = render(ExpenseTable);
    expect(page.getByText("Category")).toBeInTheDocument();
    expect(page.getByText("Account")).toBeInTheDocument();
    expect(page.getByText("Date")).toBeInTheDocument();
    expect(page.getByText("Amount")).toBeInTheDocument();
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

test("Show expenses in expense table", async () => {
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
    const table = page.getByRole("table");
    expect(table.getByText("Oct 10, 2025")).toBeInTheDocument();
    expect(table.getByText("CA$").first()).toBeInTheDocument();
});

test("Show expense category", async () => {
    // TODO: test expense component
    appStore.settings.currencyCode = "CAD";
    const expense = {
        id: "1",
        amount: "22.24",
        date: "2024-09-09",
        category: {
            id: "",
            title: "Transport",
            createdAt: ""
        },
        currencyCode: "USD"
    };

    const page = render(Expense,{ expense });
    expect(page.getByText("Transport")).toBeInTheDocument();
});
