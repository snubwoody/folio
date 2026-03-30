import { test, expect, afterEach } from "vitest";
import { clearMocks, mockIPC } from "@tauri-apps/api/mocks";
import { describe } from "node:test";
import { CategoryStore, type Category } from "$lib/stores/categories.svelte";

afterEach(() => {
    clearMocks();
});

void describe("CategoryStore",() => {
    test("delete category", async () => {
        mockIPC((cmd,args) => {
            if (cmd === "delete_category") {
                return;
            }
            if (cmd === "create_category") {
                const payload = args as {title:string};
                const category: Category = {
                    id: Math.random().toString(),
                    title: payload.title,
                    createdAt: "",
                    isIncomeStream: false
                };
                return category;
            }
        });
        const categoryStore = new CategoryStore();
        const category = await categoryStore.createCategory("");
        expect(categoryStore.categories.length).toBe(1);
        await categoryStore.deleteCategory(category.id);
        expect(categoryStore.categories.length).toBe(0);
    });
});

