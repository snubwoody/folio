import { invoke } from "@tauri-apps/api/core";
import { logger } from "./logger";
import { SvelteMap } from "svelte/reactivity";

export type Category = {
    id: string;
    title: string;
    createdAt: string;
    isIncomeStream: boolean
};

export class CategoryStore {
    #categories: Category[] = $state([]);
    #categoryMap: SvelteMap<string,Category> = $derived(new SvelteMap(this.#categories.map(a => [a.id,a])));

    get categories(): Category[]{
        return this.#categories;
    }

    get categoryMap(){
        return this.#categoryMap;
    }

    /**
     * Delete a category from the user store. Any transactions
     * referencing this category will have their category field
     * set to `null`.
     *
     * @param id The id of the {@link Category} to delete
     */
    async deleteCategory(id: string) {
        await invoke("delete_category", { id });
        this.#categories = this.#categories.filter((c) => c.id !== id);
    }

    async editCategory(id: string, title: string) {
        const category = await invoke<Category>("edit_category", { id, title });
        const index = this.#categories.findIndex((c) => c.id === category.id);
        this.#categories[index] = category;
        await this.load();
    }

    /**
     * Creates a new {@link Category}.
     *
     * @param title The title of the category
     * @returns The new category
     */
    async createCategory(title: string = "New category"):Promise<Category> {
        const category = await invoke<Category>("create_category", {
            title
        });
        this.categories.push(category);
        return category;
    }

    async load() {
        this.#categories = await invoke<Category[]>("fetch_categories");
        logger.debug("Loaded categories from backend");
    }
}

export const categoryStore = new CategoryStore();

