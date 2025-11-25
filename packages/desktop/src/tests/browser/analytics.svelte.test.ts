import { appStore } from "$lib/state.svelte";
import BudgetOverview from "../../components/analytics/BudgetOverview.svelte";
import IncomeAnalytics from "../../components/analytics/IncomeAnalytics.svelte";
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

