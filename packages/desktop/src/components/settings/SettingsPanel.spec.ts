import { test, expect } from "vitest";
import SettingsPanel from "./SettingsPanel.svelte";
import { render } from "vitest-browser-svelte";
import { mockIPC } from "@tauri-apps/api/mocks";
import { accountStore } from "$lib/stores/account.svelte";
import { categoryStore, type Category } from "$lib/stores/categories.svelte";

mockIPC((cmd) => {
    if (cmd === "settings") {
        return { currencyCode: "USD" };
    }
    if (cmd === "currencies") {
        return ["USD", "CAD", "ZAR", "ZMW", "TSH"];
    }
});

test("Default to general section", async () => {
    const screen = render(SettingsPanel);
    const selected = screen
        .getByRole("button", { name: "General" })
        .query()
        ?.getAttribute("data-selected");
    expect(selected).toBe("true");
});

test("Show accounts", async () => {
    await accountStore.createTestAccount({ name: "Account 1" });
    await accountStore.createTestAccount({ name: "Account 2" });
    const screen = render(SettingsPanel);
    expect(screen.getByRole("heading", { name: "Accounts" })).toBeInTheDocument();
    expect(screen.getByRole("textbox").first()).toHaveValue("Account 1");
    expect(screen.getByRole("textbox").nth(2)).toHaveValue("Account 2");
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
    await categoryStore.createCategory("Category");
    await categoryStore.createCategory("Category");
    const screen = render(SettingsPanel);
    await screen.getByText("Categories").click();

    const items = screen.getByRole("listitem").all();
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
    const screen = render(SettingsPanel);
    await screen.getByText("Income streams").click();

    const items = screen.getByRole("listitem").all();

    expect(items).toHaveLength(2);
});

test("Create income stream", async () => {
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

    const screen = render(SettingsPanel);
    await screen.getByText("Income streams").click();

    let items = screen.getByRole("listitem").all();
    expect(items).toHaveLength(2);
    await screen.getByRole("button",{ name: "Create income stream" }).click();
    items = screen.getByRole("listitem").all();
    expect(items).toHaveLength(3);
});
