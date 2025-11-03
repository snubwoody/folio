import { appStore } from "$lib/state.svelte";
import BudgetOverview from "../../components/analytics/BudgetOverview.svelte";
import { expect, test, beforeEach } from "vitest";
import { render } from "vitest-browser-svelte";
import { formatAmount, type Budget, type Expense, type Income } from "$lib/lib";

beforeEach(() => {
    appStore.budgets = [];
    appStore.incomes = [];
    appStore.expenses = [];
});

test("Show monthly budget", async () => {
    const budgets: Budget[] = [
        {
            id: "",
            amount: "20",
            totalSpent: "10",
            remaining: "10",
            category: { id: "", title: "", createdAt: "" }
        },
        {
            id: "",
            amount: "50",
            totalSpent: "10",
            remaining: "10",
            category: { id: "", title: "", createdAt: "" }
        },
        {
            id: "",
            amount: "20",
            totalSpent: "10",
            remaining: "10",
            category: { id: "", title: "", createdAt: "" }
        }
    ];
    appStore.budgets = budgets;

    const text = formatAmount("90", { compact: true });
    const page = render(BudgetOverview);
    await expect.element(page.getByText(text)).toBeInTheDocument();
});

test("Total expenses", async () => {
    const now = new Date();
    const date = `${now.getFullYear()}-${now.getMonth() + 1}-${now.getDate()}`;
    const expenses: Expense[] = [
        { id: "", date, amount: "20", description: "", currencyCode: "USD" },
        { id: "", date, amount: "100", description: "", currencyCode: "USD" },
        { id: "", date, amount: "50", description: "", currencyCode: "USD" }
    ];
    appStore.expenses = expenses;

    const text = formatAmount("170", { compact: true });
    const page = render(BudgetOverview);
    await expect.element(page.getByText(text)).toBeInTheDocument();
});

test("Format large amounts", async () => {
    const now = new Date();
    const date = `${now.getFullYear()}-${now.getMonth() + 1}-${now.getDate()}`;
    const expenses: Expense[] = [
        {
            id: "",
            date,
            amount: "20000000",
            description: "",
            currencyCode: "USD"
        }
    ];
    appStore.expenses = expenses;
    const page = render(BudgetOverview);
    await expect.element(page.getByText("$20M")).toBeInTheDocument();
});

test("Total income", async () => {
    const now = new Date();
    const date = `${now.getFullYear()}-${now.getMonth() + 1}-${now.getDate()}`;
    const incomes: Income[] = [
        { id: "", date, amount: "20", description: "", currencyCode: "USD" },
        { id: "", date, amount: "100", description: "", currencyCode: "USD" },
        { id: "", date, amount: "50", description: "", currencyCode: "USD" }
    ];
    appStore.incomes = incomes;

    const text = formatAmount("170", { compact: true });
    const page = render(BudgetOverview);
    await expect.element(page.getByText(text)).toBeInTheDocument();
});

test("Only include current month expenses", async () => {
    const now = new Date();
    const date = `${now.getFullYear()}-${now.getMonth() + 1}-${now.getDate()}`;
    const expenses: Expense[] = [
        { id: "", date, amount: "20", description: "", currencyCode: "USD" },
        {
            id: "",
            date: "1990-1-1",
            amount: "100",
            description: "",
            currencyCode: "USD"
        },
        { id: "", date, amount: "50", description: "", currencyCode: "USD" }
    ];
    appStore.expenses = expenses;

    const text = formatAmount("70", { compact: true });
    const page = render(BudgetOverview);
    await expect.element(page.getByText(text)).toBeInTheDocument();
});

test("Only include current month incomes", async () => {
    const now = new Date();
    const date = `${now.getFullYear()}-${now.getMonth() + 1}-${now.getDate()}`;
    const incomes: Income[] = [
        { id: "", date, amount: "20", description: "", currencyCode: "USD" },
        {
            id: "",
            date: "1990-1-1",
            amount: "100",
            description: "",
            currencyCode: "USD"
        },
        { id: "", date, amount: "50", description: "", currencyCode: "USD" }
    ];
    appStore.incomes = incomes;

    const text = formatAmount("70", { compact: true });
    const page = render(BudgetOverview);
    await expect.element(page.getByText(text)).toBeInTheDocument();
});

test("Show percentage of income", async () => {
    const now = new Date();
    const date = `${now.getFullYear()}-${now.getMonth() + 1}-${now.getDate()}`;
    const expenses: Expense[] = [
        { id: "", date, amount: "10", description: "", currencyCode: "USD" }
    ];
    const incomes: Income[] = [
        { id: "", date, amount: "100", description: "", currencyCode: "USD" }
    ];
    appStore.expenses = expenses;
    appStore.incomes = incomes;

    const page = render(BudgetOverview);
    await expect.element(page.getByText("10% of income")).toBeInTheDocument();
});

test("Don't show NAN expense percentage", async () => {
    const now = new Date();
    const date = `${now.getFullYear()}-${now.getMonth() + 1}-${now.getDay()}`;
    const expenses: Expense[] = [
        { id: "", date, amount: "20", description: "", currencyCode: "USD" },
        {
            id: "",
            date: "1990-1-1",
            amount: "100",
            description: "",
            currencyCode: "USD"
        },
        { id: "", date, amount: "50", description: "", currencyCode: "USD" }
    ];
    appStore.expenses = expenses;

    const page = render(BudgetOverview);
    await expect
        .element(page.getByText("Infinity% of income"))
        .not.toBeInTheDocument();
    await expect
        .element(page.getByText("null% of income"))
        .not.toBeInTheDocument();
});
