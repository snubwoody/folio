import { appStore } from "$lib/state.svelte";
import BudgetOverview from "../../components/analytics/BudgetOverview.svelte";
import BudgetSection from "../../components/analytics/BudgetSection.svelte";
import BudgetComponent from "../../components/analytics/Budget.svelte";
import { expect, test, beforeEach, describe } from "vitest";
import { render } from "vitest-browser-svelte";
import { formatAmount, type Budget, type Expense, type Income, type Category } from "$lib/lib";

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
        { id: "", date, amount: "20", currencyCode: "USD" },
        { id: "", date, amount: "100",  currencyCode: "USD" },
        { id: "", date, amount: "50", currencyCode: "USD" }
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
        { id: "", date, amount: "20", currencyCode: "USD" },
        { id: "", date, amount: "100",  currencyCode: "USD" },
        { id: "", date, amount: "50", currencyCode: "USD" }
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
        { id: "", date, amount: "20", currencyCode: "USD" },
        {
            id: "",
            date: "1990-1-1",
            amount: "100",
            currencyCode: "USD"
        },
        { id: "", date, amount: "50",  currencyCode: "USD" }
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
        { id: "", date, amount: "20", currencyCode: "USD" },
        {
            id: "",
            date: "1990-1-1",
            amount: "100",
            currencyCode: "USD"
        },
        { id: "", date, amount: "50", currencyCode: "USD" }
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
        { id: "", date, amount: "10", currencyCode: "USD" }
    ];
    const incomes: Income[] = [
        { id: "", date, amount: "100", currencyCode: "USD" }
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
        { id: "", date, amount: "20", currencyCode: "USD" },
        {
            id: "",
            date: "1990-1-1",
            amount: "100",
            currencyCode: "USD"
        },
        { id: "", date, amount: "50", currencyCode: "USD" }
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

describe("Budget section",async()=>{
    test("shows categories", async () => {
        const category: Category = {
            id: "1",
            title: "Groceries",
            createdAt: ""
        };
        const c2: Category = {
            id: "2",
            title: "Cinema",
            createdAt: ""
        };
        const c3: Category = {
            id: "3",
            title: "Takeout",
            createdAt: ""
        };
        appStore.settings.currencyCode = "USD";
        appStore.budgets = [
            {id: "1",category,amount: "24.00",totalSpent: "10.00",remaining: "14.00"},
            {id: "2",category:c2,amount: "24.00",totalSpent: "10.00",remaining: "14.00"},
            {id: "3",category:c3,amount: "24.00",totalSpent: "10.00",remaining: "14.00"},
        ];

        const page = render(BudgetSection);
        await expect
            .element(page.getByText("Groceries")).toBeInTheDocument();
        await expect
            .element(page.getByText("Cinema")).toBeInTheDocument();
        await expect
            .element(page.getByText("Takeout")).toBeInTheDocument();
    });
});

describe("Budget component",async()=>{
    test("shows left to spend and total", async () => {
        const category: Category = {
            id: "",
            title: "",
            createdAt: ""
        } ;
        appStore.settings.currencyCode = "USD";
        const budget: Budget = {id: "ow",category,amount: "24.00",totalSpent: "10.00",remaining: "14.00"}

        const page = render(BudgetComponent,{budget});
        await expect
            .element(page.getByText("$24.00")).toBeInTheDocument();
        await expect
            .element(page.getByText("$10.00")).toBeInTheDocument();
    });
    test("shows category title", async () => {
        const category: Category = {
            id: "",
            title: "Groceries",
            createdAt: ""
        } ;
        appStore.settings.currencyCode = "USD";
        const budget: Budget = {id: "ow",category,amount: "24.00",totalSpent: "10.00",remaining: "14.00"}

        const page = render(BudgetComponent,{budget});
        await expect
            .element(page.getByText("Groceries")).toBeInTheDocument();
    });
    test("shows amount spent", async () => {
        const category: Category = {
            id: "",
            title: "Groceries",
            createdAt: ""
        } ;
        appStore.settings.currencyCode = "USD";
        const budget: Budget = {id: "ow",category,amount: "24.00",totalSpent: "10.00",remaining: "14.00"};

        const page = render(BudgetComponent,{budget});
        await expect
            .element(page.getByText("Spent $10.00 of $24.00")).toBeInTheDocument();
    });
    test("shows fully spent", async () => {
        const category: Category = {
            id: "",
            title: "Groceries",
            createdAt: ""
        } ;
        appStore.settings.currencyCode = "USD";
        const budget: Budget = {id: "ow",category,amount: "24.00",totalSpent: "24.00",remaining: "14.00"};

        const page = render(BudgetComponent,{budget});
        await expect
            .element(page.getByText("Fully spent")).toBeInTheDocument();
    });
    test("shows overspent", async () => {
        const category: Category = {
            id: "",
            title: "Groceries",
            createdAt: ""
        } ;
        appStore.settings.currencyCode = "USD";
        const budget: Budget = {id: "ow",category,amount: "24.00",totalSpent: "25.00",remaining: "14.00"};

        const page = render(BudgetComponent,{budget});
        await expect
            .element(page.getByText("Overspent by $1.00")).toBeInTheDocument();
    });
});
