import { test, expect } from "vitest";
import Sidebar from "$components/Sidebar.svelte";
import { render } from "vitest-browser-svelte";
import { mockIPC } from "@tauri-apps/api/mocks";
import { accountStore } from "$lib/stores/account.svelte";
import { categoryStore, type Category } from "$lib/stores/categories.svelte";


mockIPC((cmd) => {
    if (cmd === "fetch_incomes" || cmd === "fetch_expenses") {
        return [];
    }
    if (cmd === "settings") {
        return { currencyCode: "USD" };
    }
    if (cmd === "currencies") {
        return ["USD", "CAD", "ZAR", "ZMW", "TSH"];
    }
});

test("Open settings panel", async () => {
    const page = render(Sidebar);
    await page.getByLabelText("Open settings").click();
    await expect
        .element(page.getByLabelText("Settings panel"))
        .toBeInTheDocument();
});

test("Default to general section", async () => {
    const page = render(Sidebar);
    await page.getByLabelText("Open settings").click();
    const selected = page
        .getByRole("button", { name: "General" })
        .query()
        ?.getAttribute("data-selected");
    expect(selected).toBe("true");
});

test("Show accounts", async () => {
    await accountStore.createTestAccount({ name: "Account 1" });
    await accountStore.createTestAccount({ name: "Account 2" });
    const page = render(Sidebar);
    await page.getByLabelText("Open settings").click();
    expect(page.getByRole("heading", { name: "Accounts" })).toBeInTheDocument();
    expect(page.getByRole("textbox").first()).toHaveValue("Account 1");
    expect(page.getByRole("textbox").nth(2)).toHaveValue("Account 2");
});

test("Show categories", async () => {
    mockIPC((cmd,args) => {
        if (cmd === "create_category") {
            const payload = args as {title:string};
            const category: Category = {
                id: Math.random().toString(),
                title: payload.title,
                createdAt: new Date().toUTCString(),
                isIncomeStream: false
            };
            return category;
        }
    });
    categoryStore.createCategory("");
    categoryStore.createCategory("");
    const page = render(Sidebar);
    await page.getByLabelText("Open settings").click();
    await page.getByText("Categories").click();

    const items = page.getByRole("listitem").all();
    expect(items).toHaveLength(2);
});

test("Show income streams", async () => {
    mockIPC((cmd,args) => {
        if (cmd === "create_income_stream") {
            const payload = args as {title:string};
            const category: Category = {
                id: Math.random().toString(),
                title: payload.title,
                createdAt: new Date().toUTCString(),
                isIncomeStream: true
            };
            return category;
        }
    });
    await categoryStore.createIncomeStream();
    await categoryStore.createIncomeStream();
    const page = render(Sidebar);
    await page.getByLabelText("Open settings").click();
    await page.getByText("Income streams").click();

    const items = page.getByRole("listitem").all();

    expect(items).toHaveLength(2);
});
