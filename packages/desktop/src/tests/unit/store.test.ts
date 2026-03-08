import { test, expect, afterEach } from "vitest";
import { appStore } from "$lib/state.svelte";
import { clearMocks, mockIPC } from "@tauri-apps/api/mocks";
import { describe } from "node:test";
import type { Category } from "$lib/lib";
import { CategoryStore } from "$lib/categories.svelte";

afterEach(() => {
    clearMocks();
});

describe("CategoryStore",()=>{
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
                    createdAt: ""
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

