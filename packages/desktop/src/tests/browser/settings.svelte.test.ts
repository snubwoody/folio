import { beforeEach, test, expect, afterEach } from "vitest";
import { appStore } from "$lib/state.svelte";
import Sidebar from "$components/Sidebar.svelte";
import { render } from "vitest-browser-svelte";
import { clearMocks, mockIPC } from "@tauri-apps/api/mocks";

beforeEach(() => {
    appStore.categories = [];
});

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

afterEach(()=>{
    clearMocks();
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
    appStore.accounts = [
        { id: "1",name: "Account 1", startingBalance: "24.00",balance: "0.0" },
        { id: "2",name: "Account 2", startingBalance: "254.35",balance: "0.0" },
    ];
    const page = render(Sidebar);
    await page.getByLabelText("Open settings").click();
    expect(page.getByRole("heading", { name: "Accounts" })).toBeInTheDocument();
    expect(page.getByText("Account 1")).toBeInTheDocument();
    expect(page.getByText("Account 2")).toBeInTheDocument();
});

test("Show account starting balance", async () => {
    appStore.accounts = [
        { id: "1",name: "Account 1", startingBalance: "24.25",balance: "0.0" },
    ];
    appStore.settings.currencyCode = "CAD";
    const page = render(Sidebar);
    await page.getByLabelText("Open settings").click();
    expect(page.getByText("Starting balance: CA$24.25")).toBeInTheDocument();
});

test("Show categories", async () => {
    appStore.categories = [
        { id: "1", title: "Rent", createdAt: new Date().toUTCString() },
        { id: "2", title: "Groceries", createdAt: new Date().toUTCString() },
    ];
    const page = render(Sidebar);
    await page.getByLabelText("Open settings").click();
    await page.getByText("Categories").click();

    const items = page.getByRole("listitem").all();
    expect(items).toHaveLength(2);
});

test("Show income streams", async () => {
    appStore.incomeStreams = [
        { id: "1", title: "Salary", createdAt: "" },
        { id: "2", title: "Dividends", createdAt: "" },
    ];
    const page = render(Sidebar);
    await page.getByLabelText("Open settings").click();
    await page.getByText("Income streams").click();

    const items = page.getByRole("listitem").all();

    expect(items).toHaveLength(2);
});
