import { test, expect, afterEach } from "vitest";
import { appStore } from "$lib/state.svelte";
import { clearMocks, mockIPC } from "@tauri-apps/api/mocks";

afterEach(() => {
    clearMocks();
});

// TODO: add more tests
test("Delete category", async () => {
    // TODO: split these
    mockIPC((cmd) => {
        if (cmd === "delete_category") {
            return;
        }
        if (cmd === "fetch_expenses" || cmd === "spending_analytics") {
            return [];
        }
    });
    appStore.categories = [{ id: "id-1", title: "", createdAt: "" }];
    // appStore.expenses = [
    //     { id: "id-1", amount: "", date: "", currencyCode: "" }
    // ];
    appStore.transactions.addExpense({ amount: "0",currencyCode: "CAD" });
    await appStore.deleteCategory("id-1");
    expect(appStore.categories.length).toBe(0);
    expect(appStore.expenses.length).toBe(0);
});
