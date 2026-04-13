import { test, expect, afterEach } from "vitest";
import { clearMocks, mockIPC } from "@tauri-apps/api/mocks";
import type { Budget } from "./lib";
import { getBudget } from "$lib/state.svelte";

afterEach(() => {
    clearMocks();
});

test("getTransactions",async() => {
    mockIPC((cmd) => {
        if (cmd === "get_budget" ) {
            const budget: Budget = {
                id: "B1",
                remaining: "0",
                totalSpent: "0",
                amount: "0",
                category: {
                    id: "C1",
                    isIncomeStream: false,
                    title: "",
                    createdAt: ""
                }
            };
            return budget;
        }
    });

    const budget = await getBudget("C1");
    expect(budget.id).toBe("B1");
});

