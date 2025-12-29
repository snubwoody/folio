import { appStore } from "$lib/state.svelte";
import ExpenseTable from "$components/home/expense/ExpenseTable.svelte";
import Expense from "$components/home/expense/Expense.svelte";
import TransactionsSection from "$components/home/TransactionsSection.svelte";
import { test, beforeEach, expect } from "vitest";
import { render } from "vitest-browser-svelte";

beforeEach(() => {
    appStore.budgets = [];
    appStore.incomes = [];
    appStore.expenses = [];
});

test("Expense table heading", async () => {
    const page = render(ExpenseTable);
    await page.getByText("Expense table").click();
    const items = page.getByRole("listitem").all();
    expect(items[0]).toHaveTextContent("Category");
    expect(items[1]).toHaveTextContent("Account");
    expect(items[2]).toHaveTextContent("Date");
    expect(items[3]).toHaveTextContent("Amount");
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
    appStore.expenses = [
        { id: "1", amount: "0",date: "2025-10-11",currencyCode: "CAD" },
        { id: "2", amount: "500",date: "2025-10-10",currencyCode: "CAD" },
        { id: "3", amount: "24.24",date: "2025-09-01",currencyCode: "CAD" }
    ];
    const page = render(ExpenseTable);
    expect(page.getByText("Oct 10, 2025")).toBeInTheDocument();
    expect(page.getByText("CA$").first()).toBeInTheDocument();
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
