import { test, expect, vi } from "vitest";
import { appStore } from "$lib/state.svelte";
import { mockIPC } from "@tauri-apps/api/mocks";

test("Delete category", async () => {
    mockIPC((cmd, arg) => {
        if (cmd === "delete_category") {
            return;
        }
        if (cmd === "fetch_expenses" || cmd === "spending_analytics") {
            return [];
        }
    });
    appStore.categories = [{ id: "id-1", title: "" }];
    appStore.expenses = [
        { id: "id-1", amount: "", date: "", description: "", currencyCode: "" },
    ];
    appStore.spendingAnaltics = [
        { category: { id: "", title: "" }, total: "" },
    ];

    await appStore.deleteCategory("id-1");
    expect(appStore.categories.length).toBe(0);
    expect(appStore.expenses.length).toBe(0);
    expect(appStore.spendingAnaltics.length).toBe(0);
});
