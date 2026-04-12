import { expect, test, describe, beforeEach } from "vitest";
import { render } from "vitest-browser-svelte";
import { accountStore, mockCreateAccount } from "$lib/stores/account.svelte";
import Sidebar from "./Sidebar.svelte";
import { clearMocks, mockIPC } from "@tauri-apps/api/mocks";
import { mockSettings } from "$lib/stores/settings.svelte";

beforeEach(() => {
    accountStore.clear();
    clearMocks();
});

mockIPC((cmd) => {
    if (cmd === "settings") {
        return { currencyCode: "USD" };
    }
    if (cmd === "currencies") {
        return ["USD", "CAD", "ZAR", "ZMW", "TSH"];
    }
});

describe("Navigation Panel",() => {
    test("has navigation links",async() => {
        const screen = await render(Sidebar);
        const transactionsLink = screen.getByRole("link",{ name:"Transactions" });
        const spendingLink = screen.getByRole("link",{ name:"Spending" });

        await expect.element(transactionsLink).toBeVisible();
        await expect.element(spendingLink).toBeVisible();
        await expect.element(transactionsLink).toHaveAttribute("href","/");
        await expect.element(spendingLink).toHaveAttribute("href","/analytics");
    });

    test("open settings panel", async () => {
        mockSettings();
        const screen = await render(Sidebar);
        await screen.getByLabelText("Open settings").click();
        await expect
            .element(screen.getByLabelText("Settings panel"))
            .toBeVisible();
    });

    test("defaults to expanded", async () => {
        const screen = await render(Sidebar);
        await expect.element(screen.getByTestId("nav-panel")).toHaveAttribute("data-expanded","true");
    });

    test("expand and collapse sidebar", async () => {
        const screen = await render(Sidebar);
        await screen.getByRole("button",{ name: "Collapse sidebar" }).click();
        await expect.element(screen.getByTestId("nav-panel")).toHaveAttribute("data-expanded","false");
        await screen.getByRole("button",{ name: "Collapse sidebar" }).click();
        await expect.element(screen.getByTestId("nav-panel")).toHaveAttribute("data-expanded","true");
    });

    test("hide links in collapsed sidebar", async () => {
        const screen = await render(Sidebar);
        await screen.getByRole("button",{ name: "Collapse sidebar" }).click();
        const transactionsLink = screen.getByRole("link",{ name:"Transactions" });
        const spendingLink = screen.getByRole("link",{ name:"Spending" });

        await expect.element(transactionsLink).not.toBeInTheDocument();
        await expect.element(spendingLink).not.toBeInTheDocument();
    });

    // TODO: maybe test reactivity
    test("account list", async () => {
        mockCreateAccount();
        await accountStore.createAccount({ name: "Account 1",startingBalance: "20.00" });
        await accountStore.createAccount({ name: "Account 2",startingBalance: "500.00" });
        const screen = await render(Sidebar);

        await expect.element(screen.getByText("Account 1")).toBeVisible();
        await expect.element(screen.getByText("$20.00")).toBeVisible();
        expect(screen.getByRole("listitem").all()).toHaveLength(2);
    });

    test("show account total", async () => {
        mockCreateAccount();
        await accountStore.createAccount({ name: "Account 1",startingBalance: "20.00" });
        await accountStore.createAccount({ name: "Account 2",startingBalance: "500.00" });
        const screen = await render(Sidebar);

        await expect.element(screen.getByText("Accounts")).toBeVisible();
        await expect.element(screen.getByText("$520.00")).toBeVisible();
    });
});