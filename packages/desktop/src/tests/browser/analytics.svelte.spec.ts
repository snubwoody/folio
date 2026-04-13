import {appStore} from "$lib/state.svelte";
import BudgetOverview from "../../components/analytics/BudgetOverview.svelte";
import {beforeEach, expect, test} from "vitest";
import {render} from "vitest-browser-svelte";
import {formatMoney} from "$lib/utils/money";

beforeEach(() => {
    appStore.budgets = [];
});

test("Show monthly budget", async () => {
    appStore.budgets = [
        {
            id: "",
            amount: "20",
            totalSpent: "10",
            remaining: "10",
            category: {id: "", title: "", createdAt: "", isIncomeStream: false}
        },
        {
            id: "",
            amount: "50",
            totalSpent: "10",
            remaining: "10",
            category: {id: "", title: "", createdAt: "", isIncomeStream: false}
        },
        {
            id: "",
            amount: "20",
            totalSpent: "10",
            remaining: "10",
            category: {id: "", title: "", createdAt: "", isIncomeStream: false}
        }
    ];

    const text = formatMoney("90", { compact: true });
    const page = render(BudgetOverview);
    await expect.element(page.getByText(text)).toBeInTheDocument();
});

