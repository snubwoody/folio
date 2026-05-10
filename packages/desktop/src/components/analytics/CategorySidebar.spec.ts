import { describe, expect, test } from "vitest";
import CategorySidebar from "./CategorySidebar.svelte";
import { render } from "vitest-browser-svelte";
import type { Category } from "$lib/types";
import type { SpendingAnalytic } from "$lib/analytics";
import { formatMoney } from "$lib/utils/money";

describe("Category Sidebar", () => {
    test("heading", async() => {
        const screen = await render(CategorySidebar);

        await expect.element(screen.getByRole("heading",{ name: "Categories" })).toBeVisible();
        await expect.element(screen.getByRole("heading",{ name: "Total spent" })).toBeVisible();
    });
    test("show analytic info", async() => {
        const category: Category = {
            id: "1",
            title: "Groceries",
            createdAt: "",
            isIncomeStream: false
        };

        const analytics: SpendingAnalytic[] = [
            {
                category,
                total: 100,
                percentage: 0.1,
                color: "#000"
            }
        ];
        const screen = await render(CategorySidebar, { analytics });

        await expect.element(screen.getByText(formatMoney("100"))).toBeVisible();
        await expect.element(screen.getByText("Groceries")).toBeVisible();
    });
});